use crate::common::has_task_passed_update_interval;
use crate::{cell::*, common::*, error::Error};
use ckb_std::{
    ckb_constants::Source,
    high_level::{load_cell_data, load_witness_args, QueryIter},
};

use common_raw::{
    cell::{code::CodeCellData, sidechain_config::SidechainConfigCellData, task::TaskCellData},
    witness::collator_refresh_task::CollatorRefreshTaskWitness,
    FromRaw,
};

pub fn is_collator_refresh_task() -> Result<(), Error> {
    /*
    CollatorRefreshTask,

    Dep:    0 Global Config Cell
            1 Sidechain Config Cell

    Code Cell                   ->          Code Cell
    [Task Cell]                 ->          [Task Cell]

    */

    let global = check_global_cell()?;

    if is_cell_count_smaller(2, Source::Input) || is_cell_count_smaller(2, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCellData: CellOrigin(5, Source::CellDep),
            CodeCellData: CellOrigin(0, Source::Input),
            CodeCellData: CellOrigin(0, Source::Output),
        },
    };

    TaskCellData::one_to_one_check(1, &global)
}

pub fn collator_refresh_task() -> Result<(), Error> {
    /*
    CollatorRefreshTask,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->          Code Cell
    [Task Cell]                 ->          [Task Cell]

    */
    is_collator_refresh_task()?;

    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let witness = CollatorRefreshTaskWitness::from_raw(&witness.as_reader().raw_data()[..]).ok_or(Error::Encoding)?;

    let scc_data = SidechainConfigCellData::load(CellOrigin(5, Source::CellDep))?;

    let task_inputs = QueryIter::new(load_cell_data, Source::Input)
        .skip(1)
        .map(|task_cell_data_input| TaskCellData::from_raw(task_cell_data_input.as_slice()));

    let task_outputs = QueryIter::new(load_cell_data, Source::Output)
        .skip(1)
        .map(|task_cell_data_output| TaskCellData::from_raw(task_cell_data_output.as_slice()));

    if !task_inputs.zip(task_outputs).enumerate().all(|(x, (input, output))| {
        if has_task_passed_update_interval(scc_data.clone(), CellOrigin(x + 1, Source::Input)).is_err() {
            return false;
        }
        let (input, output) = match (input, output) {
            (Some(input), Some(output)) => (input, output),
            _ => return false,
        };

        input.chain_id == witness.chain_id && scc_data.chain_id == witness.chain_id && input == output
    }) {
        return Err(Error::TaskMismatch);
    }

    Ok(())
}
