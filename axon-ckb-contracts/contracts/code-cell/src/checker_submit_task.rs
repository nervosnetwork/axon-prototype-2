use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        checker_info::{CheckerInfoCellData, CheckerInfoCellMode},
        code::CodeCellData,
        sidechain_config::SidechainConfigCellData,
        task::{TaskCellData, TaskCellMode},
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
    let (checker_info_input, task_input) = load_entities! {
        CheckerInfoCellData: CHECKER_INFO_INPUT,
        TaskCellData: TASK_INPUT,
    };
    let checker_info_output = CheckerInfoCellData::load(CHECKER_INFO_OUTPUT)?;

    let mut checker_info_res = checker_info_input.clone();
    checker_info_res.mode = CheckerInfoCellMode::TaskPassed;
    checker_info_res.unpaid_fee += u128::from(config_dep.check_fee_rate) * task_input.check_data_size;

    if checker_info_res != checker_info_output
        || checker_info_input.checker_public_key_hash != signer
        || checker_info_input.chain_id != witness.chain_id
        || checker_info_input.checker_id != witness.checker_id
    {
        return Err(Error::CheckerInfoMismatch);
    }

    if task_input.chain_id != witness.chain_id || task_input.mode != TaskCellMode::Task {
        return Err(Error::TaskMismatch);
    }

    Ok(())
}

fn is_checker_submit_task(witness: &CheckerSubmitTaskWitness) -> Result<(), Error> {
    /*
    CheckerSubmitTask,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->         Code Cell
    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          Null

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 3 || output_count != 2 {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCellData: CellOrigin(witness.sidechain_config_dep_index, Source::CellDep),
            CodeCellData: CellOrigin(0, Source::Input),
            CheckerInfoCellData: CellOrigin(1, Source::Input),
            TaskCellData: CellOrigin(2, Source::Input),
            CodeCellData: CellOrigin(0, Source::Output),
            CheckerInfoCellData: CellOrigin(1, Source::Output),
        },
    };

    Ok(())
}
