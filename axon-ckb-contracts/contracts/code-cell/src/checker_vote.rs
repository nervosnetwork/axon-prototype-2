use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        checker_info::{CheckerInfoCell, CheckerInfoCellTypeArgs},
        code::CodeCell,
        sidechain_config::SidechainConfigCell,
        task::{TaskCell, TaskCellTypeArgs, TaskMode, TaskStatus},
    },
    witness::checker_vote::CheckerVoteWitness,
    FromRaw,
};

use crate::{cell::*, common::*, error::Error};

const CHECKER_INFO_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const TASK_INPUT: CellOrigin = CellOrigin(2, Source::Input);

const CHECKER_INFO_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const TASK_OUTPUT: CellOrigin = CellOrigin(2, Source::Output);

pub fn checker_vote(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerVote,

    Dep: 0 Global Config Cell
    Dep: 1 Sidechain Config Cell

    Code Cell         -> ~
    Checker Info Cell -> ~
    Task Cell         -> ~
    */

    let witness = CheckerVoteWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    is_checker_vote(&witness)?;

    let config_dep = SidechainConfigCell::load(CellOrigin(witness.sidechain_config_dep_index, Source::CellDep))?;

    let (checker_info_input_type_args, checker_info_input, task_input_type_args, task_input) = load_entities! {
        CheckerInfoCellTypeArgs: CHECKER_INFO_INPUT,
        CheckerInfoCell: CHECKER_INFO_INPUT,
        TaskCellTypeArgs: TASK_INPUT,
        TaskCell: TASK_INPUT,
    };

    let (checker_info_output_type_args, checker_info_output, task_output_type_args, task_output) = load_entities! {
        CheckerInfoCellTypeArgs: CHECKER_INFO_OUTPUT,
        CheckerInfoCell: CHECKER_INFO_OUTPUT,
        TaskCellTypeArgs: TASK_OUTPUT,
        TaskCell: TASK_OUTPUT,
    };

    let mut checker_info_res = checker_info_input.clone();
    checker_info_res.unpaid_fee += u128::from(config_dep.check_fee_rate) * task_input.check_data_size;

    if checker_info_input_type_args.chain_id != witness.chain_id
        || checker_info_input_type_args.checker_lock_arg != signer
        || checker_info_input_type_args != checker_info_output_type_args
        || checker_info_res != checker_info_output
    {
        return Err(Error::CheckerInfoMismatch);
    }

    let mut task_res = task_input.clone();

    if task_input.status != TaskStatus::Idle {
        return Err(Error::TaskMismatch);
    }
    task_res.status = task_output.status;

    match task_input.mode {
        TaskMode::Task => {
            if task_res.status != TaskStatus::TaskPassed {
                return Err(Error::TaskMismatch);
            }
        }
        TaskMode::Challenge => {
            if task_res.status != TaskStatus::ChallengePassed && task_res.status != TaskStatus::ChallengeRejected {
                return Err(Error::TaskMismatch);
            }
        }
    };

    task_res.commit.copy_from_slice(&task_output.commit);
    task_res.reveal.copy_from_slice(&task_output.reveal);

    if task_input_type_args.chain_id != witness.chain_id
        || task_input_type_args.checker_lock_arg != signer
        || task_res != task_output
        || task_input_type_args != task_output_type_args
    {
        return Err(Error::TaskMismatch);
    }

    Ok(())
}

fn is_checker_vote(witness: &CheckerVoteWitness) -> Result<(), Error> {
    let global = check_global_cell()?;

    if is_cell_count_not_equals(3, Source::Input) || is_cell_count_not_equals(3, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCell: CellOrigin(witness.sidechain_config_dep_index, Source::CellDep),

            CodeCell: CODE_INPUT,
            CheckerInfoCell: CHECKER_INFO_INPUT,
            TaskCell: TASK_INPUT,

            CodeCell: CODE_OUTPUT,
            CheckerInfoCell: CHECKER_INFO_OUTPUT,
            TaskCell: TASK_OUTPUT,
        },
    };

    Ok(())
}
