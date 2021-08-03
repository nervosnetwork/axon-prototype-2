use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        checker_info::{CheckerInfoCellData, CheckerInfoCellMode, CheckerInfoCellTypeArgs},
        code::CodeCellData,
        sidechain_config::SidechainConfigCellData,
        task::{TaskCell, TaskCellTypeArgs, TaskMode},
    },
    witness::checker_submit_task::CheckerSubmitTaskWitness,
    FromRaw,
};

use crate::{cell::*, common::*, error::Error};

const CHECKER_INFO_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const TASK_INPUT: CellOrigin = CellOrigin(2, Source::Input);

const CHECKER_INFO_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);

pub fn checker_submit_task(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerSubmitTask,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->         Code Cell
    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          Null

    */

    let witness = CheckerSubmitTaskWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    is_checker_submit_task(&witness)?;

    let config_dep = SidechainConfigCellData::load(CellOrigin(witness.sidechain_config_dep_index, Source::CellDep))?;
    let (checker_info_input_type_args, checker_info_input, task_input_type_args, task_input) = load_entities! {
        CheckerInfoCellTypeArgs: CHECKER_INFO_INPUT,
        CheckerInfoCellData: CHECKER_INFO_INPUT,
        TaskCellTypeArgs: TASK_INPUT,
        TaskCell: TASK_INPUT,
    };

    let (checker_info_output, checker_info_output_type_args) = load_entities! {
        CheckerInfoCellData: CHECKER_INFO_OUTPUT,
        CheckerInfoCellTypeArgs: CHECKER_INFO_OUTPUT,
    };

    let mut checker_info_res = checker_info_input.clone();
    checker_info_res.mode = CheckerInfoCellMode::TaskPassed;
    checker_info_res.unpaid_fee += u128::from(config_dep.check_fee_rate) * task_input.check_data_size;

    if checker_info_input_type_args.chain_id != witness.chain_id
        || checker_info_input_type_args.checker_lock_arg != signer
        || checker_info_input_type_args != checker_info_output_type_args
        || checker_info_res != checker_info_output
        || checker_info_input_type_args.chain_id != witness.chain_id
        || checker_info_input.checker_id != witness.checker_id
    {
        return Err(Error::CheckerInfoMismatch);
    }

    if task_input_type_args.chain_id != witness.chain_id || task_input.mode != TaskMode::Task {
        return Err(Error::TaskMismatch);
    }

    Ok(())
}

fn is_checker_submit_task(witness: &CheckerSubmitTaskWitness) -> Result<(), Error> {
    let global = check_global_cell()?;

    if is_cell_count_not_equals(3, Source::Input) || is_cell_count_not_equals(2, Source::Output) {
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

    Ok(())
}
