use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        checker_info::{CheckerInfoCellData, CheckerInfoCellMode, CheckerInfoCellTypeArgs},
        code::CodeCellData,
        task::{TaskCellData, TaskCellMode, TaskCellTypeArgs},
    },
    witness::checker_submit_challenge::CheckerSubmitChallengeWitness,
    FromRaw,
};

use crate::{cell::*, common::*, error::Error};

const CHECKER_INFO_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const TASK_INPUT: CellOrigin = CellOrigin(2, Source::Input);

const CHECKER_INFO_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);

pub fn checker_submit_challenge(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerSubmitChallenge,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->         Code Cell
    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          Null

    */

    is_checker_submit_challenge()?;

    let witness = CheckerSubmitChallengeWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    let (checker_info_input, checker_info_input_type_args, task_cell_input, task_cell_input_type_args) = load_entities! {
        CheckerInfoCellData: CHECKER_INFO_INPUT,
        CheckerInfoCellTypeArgs: CHECKER_INFO_INPUT,
        TaskCellData: TASK_INPUT,
        TaskCellTypeArgs: TASK_INPUT,
    };

    let (checker_info_output, checker_info_output_type_args) = load_entities! {
        CheckerInfoCellData: CHECKER_INFO_OUTPUT,
        CheckerInfoCellTypeArgs: CHECKER_INFO_OUTPUT,
    };

    let mut checker_info_res = checker_info_input.clone();
    checker_info_res.mode = checker_info_output.mode;

    if checker_info_input_type_args.chain_id != witness.chain_id
        || checker_info_input_type_args.checker_lock_arg != signer
        || checker_info_input_type_args != checker_info_output_type_args
        || checker_info_input.checker_id != witness.checker_id
        || (checker_info_output.mode != CheckerInfoCellMode::ChallengeRejected
            && checker_info_output.mode != CheckerInfoCellMode::ChallengePassed)
        || checker_info_res != checker_info_output
    {
        return Err(Error::CheckerInfoMismatch);
    }

    if task_cell_input_type_args.chain_id != witness.chain_id || task_cell_input.mode != TaskCellMode::Challenge {
        return Err(Error::TaskMismatch);
    }

    Ok(())
}

fn is_checker_submit_challenge() -> Result<(), Error> {
    let global = check_global_cell()?;

    if is_cell_count_not_equals(3, Source::Input) || is_cell_count_not_equals(2, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            CodeCellData: CODE_INPUT,
            CheckerInfoCellData: CHECKER_INFO_INPUT,
            TaskCellData: TASK_INPUT,

            CodeCellData: CODE_OUTPUT,
            CheckerInfoCellData: CHECKER_INFO_OUTPUT,
        },
    };

    Ok(())
}
