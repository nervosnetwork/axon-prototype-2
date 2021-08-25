use core::convert::TryFrom;

use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        code::CodeCell,
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs, SidechainStatus},
        sidechain_fee::{SidechainFeeCell, SidechainFeeCellLockArgs},
        task::{TaskCell, TaskCellTypeArgs, TaskStatus},
    },
    witness::anyone_shutdown_sidechain::AnyoneShutdownSidechainWitness,
    FromRaw,
};

use crate::{cell::*, common::*, error::Error};

const SIDECHAIN_CONFIG_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const SIDECHAIN_FEE_INPUT: CellOrigin = CellOrigin(2, Source::Input);

const SIDECHAIN_CONFIG_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const SIDECHAIN_FEE_OUTPUT: CellOrigin = CellOrigin(2, Source::Output);

const FIXED_INPUT_CELLS: usize = 3;

pub fn anyone_shutdown_sidechain(raw_witness: &[u8]) -> Result<(), Error> {
    /*
    AnyoneShutdownSidechain,

    Dep:    0 Global Config Cell

    Code Cell                   -> ~
    Sidechain Config Cell       -> ~
    Sidechain Fee Cell          -> ~

    [Task Cell]         -> ~
    */

    let witness = AnyoneShutdownSidechainWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    //load inputs
    let (sidechain_config_input, sidechain_config_input_type_args) = load_entities!(
        SidechainConfigCell: SIDECHAIN_CONFIG_INPUT,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_INPUT,
    );

    // prepare arguments
    let task_count = usize::try_from(sidechain_config_input.commit_threshold).or(Err(Error::Encoding))? - witness.challenge_times;
    let challenge_count = witness.challenge_times * usize::try_from(sidechain_config_input.challenge_threshold).or(Err(Error::Encoding))?;
    let job_count = task_count + challenge_count;

    let correct_vote_count = job_count - witness.jailed_checkers.len();

    let fee = u128::try_from(correct_vote_count).or(Err(Error::Encoding))?
        * u128::try_from(sidechain_config_input.check_fee_rate).or(Err(Error::Encoding))?
        * witness.check_data_size;

    let chain_id = sidechain_config_input_type_args.chain_id;

    is_anyone_shutdown_sidechain(job_count)?;

    //load inputs
    let (sidechain_fee_input, sidechain_fee_input_lock_args) =
        load_entities!(SidechainFeeCell: SIDECHAIN_FEE_INPUT, SidechainFeeCellLockArgs: SIDECHAIN_FEE_INPUT,);

    //load outputs
    let (sidechain_config_output, sidechain_config_output_type_args, sidechain_fee_output, sidechain_fee_output_lock_args) = load_entities!(
        SidechainConfigCell: SIDECHAIN_CONFIG_OUTPUT,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_OUTPUT,
        SidechainFeeCell: SIDECHAIN_FEE_OUTPUT,
        SidechainFeeCellLockArgs: SIDECHAIN_FEE_OUTPUT,
    );

    if sidechain_config_input.sidechain_status != SidechainStatus::Relaying {
        return Err(Error::SidechainConfigMismatch);
    }

    let mut sidechain_config_res = sidechain_config_input.clone();
    sidechain_config_res.sidechain_status = SidechainStatus::Shutdown;
    sidechain_config_res.checker_normal_count -= u32::try_from(witness.jailed_checkers.len()).or(Err(Error::Encoding))?;

    sidechain_config_res.activated_checkers = sidechain_config_res
        .activated_checkers
        .into_iter()
        .filter(|&lock_arg| witness.jailed_checkers.iter().find(|&&jailed| lock_arg == jailed).is_none())
        .collect();

    sidechain_config_res.jailed_checkers.extend(witness.jailed_checkers.iter());

    if sidechain_config_res != sidechain_config_output || sidechain_config_input_type_args != sidechain_config_output_type_args {
        return Err(Error::SidechainConfigMismatch);
    }

    let mut sidechain_fee_res_lock_args = sidechain_fee_input_lock_args.clone();

    if sidechain_fee_input_lock_args.surplus < fee {
        return Err(Error::SidechainFeeMismatch);
    }
    sidechain_fee_res_lock_args.surplus -= fee;

    if sidechain_fee_input != sidechain_fee_output
        || sidechain_fee_res_lock_args != sidechain_fee_output_lock_args
        || sidechain_fee_res_lock_args.chain_id != chain_id
    {
        return Err(Error::SidechainFeeMismatch);
    }

    let (task_first, task_first_type_args) = load_entities!(
        TaskCell: CellOrigin(FIXED_INPUT_CELLS, Source::Input),
        TaskCellTypeArgs: CellOrigin(FIXED_INPUT_CELLS, Source::Input),
    );

    if task_first.check_data_size != witness.check_data_size || task_first_type_args.chain_id != chain_id {
        return Err(Error::TaskMismatch);
    }

    let mut jailed_checker_iter = witness.jailed_checkers.iter();
    let mut jailed_checker_opt = jailed_checker_iter.next();

    let len_input = FIXED_INPUT_CELLS + job_count;
    for i in FIXED_INPUT_CELLS..len_input {
        let (task, task_type_args) = load_entities!(
            TaskCell: CellOrigin(i, Source::Input),
            TaskCellTypeArgs: CellOrigin(i, Source::Input),
        );

        if match task.status {
            TaskStatus::Idle => true,
            // Good checkers
            TaskStatus::ChallengePassed => match jailed_checker_opt {
                None => false,
                Some(jailed_checker) => &task_type_args.checker_lock_arg == jailed_checker,
            },
            // Bad checkers
            _ => {
                let jailed_checker = jailed_checker_opt.ok_or(Error::TaskMismatch)?;
                jailed_checker_opt = jailed_checker_iter.next();
                &task_type_args.checker_lock_arg != jailed_checker
            }
        } {
            return Err(Error::TaskMismatch);
        }

        let mut task_res = task.clone();
        let mut task_res_type_args = task_type_args.clone();

        task_res.mode = task_first.mode;
        task_res.status = task_first.status;
        task_res.commit = task_first.commit;
        task_res.reveal = task_first.reveal;

        task_res_type_args.checker_lock_arg = task_first_type_args.checker_lock_arg;

        if task_res != task_first || task_res_type_args != task_first_type_args {
            return Err(Error::TaskMismatch);
        }
    }

    Ok(())
}

pub fn is_anyone_shutdown_sidechain(job_count: usize) -> Result<(), Error> {
    let global = check_global_cell()?;

    let len_input = FIXED_INPUT_CELLS + job_count;

    if is_cell_count_not_equals(len_input, Source::Input) || is_cell_count_not_equals(3, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            CodeCell: CODE_INPUT,
            SidechainConfigCell: SIDECHAIN_CONFIG_INPUT,
            SidechainFeeCell: SIDECHAIN_FEE_INPUT,

            CodeCell: CODE_OUTPUT,
            SidechainConfigCell: SIDECHAIN_CONFIG_OUTPUT,
            SidechainFeeCell: SIDECHAIN_FEE_OUTPUT,
        },
    };

    TaskCell::range_check(FIXED_INPUT_CELLS..len_input, Source::Input, &global)
}
