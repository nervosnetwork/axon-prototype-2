use crate::{cell::*, common::*, error::Error};
use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        code::CodeCellData,
        sidechain_config::{SidechainConfigCellData, SidechainConfigCellTypeArgs},
        task::{TaskCell, TaskCellTypeArgs},
    },
    witness::collator_refresh_task::CollatorRefreshTaskWitness,
    FromRaw,
};

pub fn collator_refresh_task(raw_witness: &[u8]) -> Result<(), Error> {
    /*
    CollatorRefreshTask,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->          Code Cell
    [Task Cell]                 ->          [Task Cell]

    */
    is_collator_refresh_task()?;

    let witness = CollatorRefreshTaskWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    let (config_input_type_args, config_input) = load_entities! {
        SidechainConfigCellTypeArgs: CellOrigin(5, Source::CellDep),
        SidechainConfigCellData: CellOrigin(5, Source::CellDep),
    };

    if config_input_type_args.chain_id != witness.chain_id {
        return Err(Error::SidechainConfigMismatch);
    }

    for i in 1.. {
        let task_input = match TaskCell::load(CellOrigin(i, Source::Input)) {
            Ok(data) => data,
            Err(Error::IndexOutOfBound) => break,
            Err(err) => return Err(err),
        };
        let task_input_type_args = match TaskCellTypeArgs::load(CellOrigin(i, Source::Input)) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let task_output = match TaskCell::load(CellOrigin(i, Source::Output)) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        has_task_passed_update_interval(&config_input, CellOrigin(i, Source::Input))?;
        if task_input_type_args.chain_id != witness.chain_id || task_input != task_output {
            return Err(Error::TaskMismatch);
        }
    }

    Ok(())
}

fn is_collator_refresh_task() -> Result<(), Error> {
    let global = check_global_cell()?;

    if is_cell_count_smaller(2, Source::Input) || is_cell_count_smaller(2, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCellData: CellOrigin(5, Source::CellDep),
            CodeCellData: CODE_INPUT,
            CodeCellData: CODE_OUTPUT,
        },
    };

    TaskCell::one_to_one_check(1, &global)
}
