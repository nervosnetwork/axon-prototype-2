use crate::{cell::*, common::*, error::Error};
use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        checker_info::{CheckerInfoCellData, CheckerInfoCellMode, CheckerInfoCellTypeArgs},
        code::CodeCellData,
        sidechain_config::{SidechainConfigCellData, SidechainConfigCellTypeArgs},
        task::{TaskCell, TaskCellTypeArgs, TaskMode},
    },
    witness::checker_publish_challenge::CheckerPublishChallengeWitness,
    FromRaw,
};

const CHECKER_INFO_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const TASK_INPUT: CellOrigin = CellOrigin(2, Source::Input);

const CHECKER_INFO_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);

pub fn checker_publish_challenge(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerPublishChallenge,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->         Code Cell
    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          [Task Cell]

    */

    let witness = CheckerPublishChallengeWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    is_checker_publish_challenge(&witness)?;

    let config_dep_origin = CellOrigin(witness.sidechain_config_dep_index, Source::CellDep);
    let (config_dep, config_dep_type_args) = load_entities! {
        SidechainConfigCellData: config_dep_origin,
        SidechainConfigCellTypeArgs: config_dep_origin,
    };

    let (checker_info_input, checker_info_input_type_args, task_input, task_input_type_args) = load_entities! {
        CheckerInfoCellData: CHECKER_INFO_INPUT,
        CheckerInfoCellTypeArgs: CHECKER_INFO_INPUT,
        TaskCell: TASK_INPUT,
        TaskCellTypeArgs: TASK_INPUT,
    };

    let (checker_info_output, checker_info_output_type_args) = load_entities! {
        CheckerInfoCellData: CHECKER_INFO_OUTPUT,
        CheckerInfoCellTypeArgs: CHECKER_INFO_OUTPUT,
    };

    if config_dep_type_args.chain_id != witness.chain_id || config_dep.challenge_threshold != witness.challenge_count {
        return Err(Error::SidechainConfigMismatch);
    }

    let mut checker_info_res = checker_info_input.clone();
    checker_info_res.mode = CheckerInfoCellMode::ChallengePassed;

    let mut task_res = task_input.clone();
    task_res.mode = TaskMode::Challenge;

    if checker_info_input.checker_id != witness.checker_id
        || checker_info_input_type_args.chain_id != witness.chain_id
        || checker_info_input_type_args.checker_lock_arg != signer
        || checker_info_input_type_args != checker_info_output_type_args
        || checker_info_res != checker_info_output
    {
        return Err(Error::CheckerInfoMismatch);
    }

    if task_input_type_args.chain_id != witness.chain_id || task_input.mode != TaskMode::Task {
        return Err(Error::TaskMismatch);
    }

    let output_count = usize::from(witness.challenge_count) + 1;
    // 2 + challenge_count - 1  * Since this checker already voted

    for i in 2..output_count {
        let (task_output, task_output_type_args) = load_entities! {
            TaskCell: CellOrigin(i, Source::Output),
            TaskCellTypeArgs: CellOrigin(i, Source::Output),
        };

        if task_res != task_output || task_input_type_args != task_output_type_args {
            return Err(Error::TaskMismatch);
        }
    }

    Ok(())
}

fn is_checker_publish_challenge(witness: &CheckerPublishChallengeWitness) -> Result<(), Error> {
    let output_count = usize::from(witness.challenge_count) + 1;
    // 2 + challenge_count - 1  * Since this checker already voted

    let global = check_global_cell()?;

    if is_cell_count_not_equals(3, Source::Input) || is_cell_count_not_equals(output_count, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCellData: CellOrigin(witness.sidechain_config_dep_index, Source::CellDep),

            CodeCellData: CODE_INPUT,
            CheckerInfoCellData: CHECKER_INFO_INPUT,
            TaskCell: TASK_INPUT,

            CodeCellData: CODE_OUTPUT,
            CheckerInfoCellData: CHECKER_INFO_OUTPUT,
        },
    };

    TaskCell::range_check(2..output_count, Source::Output, &global)
}
