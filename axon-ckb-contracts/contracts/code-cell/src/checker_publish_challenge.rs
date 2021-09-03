use core::convert::TryFrom;

use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        checker_info::{CheckerInfoCell, CheckerInfoCellTypeArgs},
        code::CodeCell,
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs},
        sidechain_state::{CheckerLastAcceptTaskHeight, SidechainStateCell, SidechainStateCellTypeArgs},
        task::{TaskCell, TaskCellTypeArgs, TaskMode, TaskStatus},
    },
    witness::checker_publish_challenge::CheckerPublishChallengeWitness,
    FromRaw,
};

use crate::{cell::*, common::*, error::Error};

const SIDECHAIN_STATE_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const CHECKER_INFO_INPUT: CellOrigin = CellOrigin(2, Source::Input);
const FIRST_TASK_INPUT: CellOrigin = CellOrigin(3, Source::Input);

const SIDECHAIN_STATE_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const CHECKER_INFO_OUTPUT: CellOrigin = CellOrigin(2, Source::Output);
const FIRST_TASK_OUTPUT: CellOrigin = CellOrigin(3, Source::Output);
pub fn checker_publish_challenge(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerPublishChallenge,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->         Code Cell
    Task Cell                   ->          [Task Cell]

    */

    let witness = CheckerPublishChallengeWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    is_checker_publish_challenge(&witness)?;

    let config_dep_origin = CellOrigin(witness.sidechain_config_dep_index, Source::CellDep);
    let (config_dep, config_dep_type_args) = load_entities! {
        SidechainConfigCell: config_dep_origin,
        SidechainConfigCellTypeArgs: config_dep_origin,
    };

    let (
        state_input,
        state_input_type_args,
        checker_info_input,
        checker_info_input_type_args,
        first_task_input,
        first_task_input_type_args,
    ) = load_entities! {
        SidechainStateCell: SIDECHAIN_STATE_INPUT,
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_INPUT,
        CheckerInfoCell: CHECKER_INFO_INPUT,
        CheckerInfoCellTypeArgs: CHECKER_INFO_INPUT,
        TaskCell: FIRST_TASK_INPUT,
        TaskCellTypeArgs: FIRST_TASK_INPUT,
    };

    let (
        state_output,
        state_output_type_args,
        checker_info_output,
        checker_info_output_type_args,
        first_task_output,
        first_task_output_type_args,
    ) = load_entities! {
        SidechainStateCell: SIDECHAIN_STATE_OUTPUT,
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_OUTPUT,
        CheckerInfoCell: CHECKER_INFO_OUTPUT,
        CheckerInfoCellTypeArgs: CHECKER_INFO_OUTPUT,
        TaskCell: FIRST_TASK_OUTPUT,
        TaskCellTypeArgs: FIRST_TASK_OUTPUT,
    };

    if config_dep_type_args.chain_id != witness.chain_id || config_dep.challenge_threshold != witness.challenge_count {
        return Err(Error::SidechainConfigMismatch);
    }

    let mut task_res = first_task_input.clone();
    task_res.mode = TaskMode::Challenge;
    task_res.status = TaskStatus::ChallengePassed;

    if first_task_input_type_args.chain_id != witness.chain_id
        || first_task_input_type_args != first_task_output_type_args
        || first_task_input_type_args.checker_lock_arg != signer
        || first_task_input.mode != TaskMode::Task
        || first_task_input.status != TaskStatus::Idle
        || task_res != first_task_output
    {
        return Err(Error::TaskMismatch);
    }

    task_res.status = TaskStatus::Idle;

    let output_count = usize::try_from(witness.challenge_count).or(Err(Error::Encoding))? + 3;
    // 2 + challenge_count - 1  * Since this checker already voted

    let mut state_res = state_input.clone();

    let mut checker_info_res = checker_info_input.clone();
    checker_info_res.unpaid_fee += task_res.check_data_size * u128::try_from(config_dep.check_fee_rate).or(Err(Error::Encoding))?;
    if checker_info_res != checker_info_output
        || checker_info_input_type_args != checker_info_output_type_args
        || checker_info_input_type_args.checker_lock_arg != first_task_input_type_args.checker_lock_arg
        || checker_info_input_type_args.chain_id != witness.chain_id
    {
        return Err(Error::CheckerInfoMismatch);
    }

    let mut seed = state_input.random_seed;
    seed[0] += state_input.random_offset;
    for i in 4..output_count {
        seed = Blake2b::calculate(&seed);
        let seed_number = u128::from_raw(&seed[0..16]).ok_or(Error::Encoding)?;

        let checkers_count = u128::try_from(config_dep.activated_checkers.len()).or(Err(Error::Encoding))?;
        let index = usize::try_from(seed_number % checkers_count).or(Err(Error::Encoding))?;
        let checker_lock_arg = config_dep.activated_checkers.get(index).ok_or(Error::Encoding)?;

        let (task_output, task_output_type_args) = load_entities! {
            TaskCell: CellOrigin(i, Source::Output),
            TaskCellTypeArgs: CellOrigin(i, Source::Output),
        };
        let mut task_res_type_args = task_output_type_args.clone();
        task_res_type_args.chain_id = witness.chain_id;
        task_res_type_args.checker_lock_arg = *checker_lock_arg;
        if task_res != task_output || task_res_type_args != task_output_type_args {
            return Err(Error::TaskMismatch);
        }

        match state_res
            .checker_last_task_sidechain_heights
            .iter_mut()
            .find(|info| &info.checker_lock_arg == checker_lock_arg)
        {
            Some(info) => {
                info.height = task_res.sidechain_block_height_to;
            }
            None => {
                state_res.checker_last_task_sidechain_heights.push(CheckerLastAcceptTaskHeight {
                    checker_lock_arg: *checker_lock_arg,
                    height:           task_res.sidechain_block_height_to,
                });
                ckb_std::debug!("-----");
            }
        }
    }

    state_res.random_offset += 1;

    if state_res != state_output || state_input_type_args.chain_id != witness.chain_id || state_input_type_args != state_output_type_args {
        ckb_std::debug!("{:?}, {:?}", state_res, state_output);
        return Err(Error::SidechainStateMismatch);
    }

    Ok(())
}

fn is_checker_publish_challenge(witness: &CheckerPublishChallengeWitness) -> Result<(), Error> {
    let output_count = usize::try_from(witness.challenge_count).or(Err(Error::Encoding))? + 3;
    // 2 + challenge_count - 1  * Since this checker already voted

    let global = check_global_cell()?;

    if is_cell_count_not_equals(4, Source::Input) || is_cell_count_not_equals(output_count, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCell: CellOrigin(witness.sidechain_config_dep_index, Source::CellDep),

            CodeCell: CODE_INPUT,
            SidechainStateCell: SIDECHAIN_STATE_INPUT,
            TaskCell: FIRST_TASK_INPUT,

            CodeCell: CODE_OUTPUT,
            SidechainStateCell: SIDECHAIN_STATE_OUTPUT,
        },
    };

    TaskCell::range_check(3..output_count, Source::Output, &global)
}
