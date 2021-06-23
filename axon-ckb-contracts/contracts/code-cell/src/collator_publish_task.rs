use crate::{cell::*, common::*, error::Error};
use ckb_std::ckb_constants::Source;

use common_raw::cell::sidechain_bond::{SidechainBondCellData, SidechainBondCellLockArgs};
use common_raw::cell::sidechain_config::SidechainConfigCellTypeArgs;
use common_raw::cell::sidechain_state::{SidechainStateCellData, SidechainStateCellTypeArgs};
use common_raw::cell::task::TaskCellTypeArgs;
use common_raw::{
    cell::{code::CodeCellData, sidechain_config::SidechainConfigCellData, task::TaskCellData},
    witness::collator_publish_task::CollatorPublishTaskWitness,
    FromRaw,
};

const SIDECHAIN_STATE_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const SIDECHAIN_STATE_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const SIDECHAIN_CONFIG_DEP: CellOrigin = CellOrigin(5, Source::CellDep);
const SIDECHAIN_BOND_DEP: CellOrigin = CellOrigin(6, Source::CellDep);
const PUB_TASK_INPUT_CELL_COUNT: usize = 2;
pub fn is_collator_publish_task(sidechain_config_data: &SidechainConfigCellData) -> Result<(), Error> {
    /*
    CollatorPublishTask,

    Dep:    0 Global Config Cell
            1 Sidechain Config Cell
    Code Cell                   ->          Code Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Bond Cell/Sudt    ->          Sidechain Bond Cell
    Null                        ->          [Task Cell]

    */

    let global = check_global_cell()?;
    if is_cell_count_not_equals(PUB_TASK_INPUT_CELL_COUNT, Source::Input)
        || is_cell_count_not_equals(
            PUB_TASK_INPUT_CELL_COUNT + sidechain_config_data.commit_threshold as usize,
            Source::Output,
        )
    {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCellData: SIDECHAIN_CONFIG_DEP,
            SidechainBondCellData: SIDECHAIN_BOND_DEP,
            CodeCellData: CODE_INPUT,
            SidechainStateCellData: SIDECHAIN_STATE_INPUT,
            CodeCellData: CODE_OUTPUT,
            SidechainStateCellData: SIDECHAIN_STATE_OUTPUT,
        },
    };
    TaskCellData::range_check(3.., Source::Output, &global)
}

pub fn collator_publish_task(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CollatorPublishTask,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->          Code Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Bond Cell/Sudt    ->          Sidechain Bond Cell
    Null                        ->          [Task Cell]

    */
    let witness = CollatorPublishTaskWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;
    //load dep
    let (sidechain_config_dep, sidechain_config_type_args_dep, sidechain_bond_dep, sidechain_bond_lock_args_dep) = load_entities!(
        SidechainConfigCellData: SIDECHAIN_CONFIG_DEP,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_DEP,
        SidechainBondCellData: SIDECHAIN_BOND_DEP,
        SidechainBondCellLockArgs: SIDECHAIN_BOND_DEP,
    );

    is_collator_publish_task(&sidechain_config_dep)?;

    //load inputs
    let (sidechain_state_input, sidechain_state_type_args_input) = load_entities! {
        SidechainStateCellData: SIDECHAIN_STATE_INPUT,
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_INPUT,
    };

    //load outputs
    let (sidechain_state_output, sidechain_state_type_args_output) = load_entities!(
        SidechainStateCellData: SIDECHAIN_STATE_OUTPUT,
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_OUTPUT,
    );

    if sidechain_config_dep.checker_threshold > sidechain_config_dep.checker_total_count
        || sidechain_config_dep.collator_lock_arg != signer
        || sidechain_config_type_args_dep.chain_id != witness.chain_id
    {
        return Err(Error::SidechainConfigMismatch);
    }

    let mut sidechain_state_res = sidechain_state_input.clone();
    sidechain_state_res.latest_block_hash = sidechain_state_output.latest_block_hash;
    sidechain_state_res.latest_block_height = sidechain_state_output.latest_block_height;
    // about committed height, see below in task cell checking
    if sidechain_state_type_args_input != sidechain_state_type_args_output
        || sidechain_state_type_args_input.chain_id != witness.chain_id
        || sidechain_state_input.committed_block_height != sidechain_state_input.latest_block_height
        || sidechain_state_res != sidechain_state_output
    {
        return Err(Error::SidechainStateMismatch);
    }

    if signer != sidechain_bond_lock_args_dep.collator_lock_arg
        || sidechain_bond_lock_args_dep.chain_id != witness.chain_id
        || sidechain_config_dep.minimal_bond > sidechain_bond_dep.amount
    {
        return Err(Error::SidechainBondMismatch);
    }

    let task_cell_data = TaskCellData::load(CellOrigin(2, Source::Output))?;
    let task_cell_type_args_data = TaskCellTypeArgs::load(CellOrigin(2, Source::Output))?;

    if task_cell_type_args_data.chain_id != witness.chain_id
        || sidechain_bond_lock_args_dep.unlock_sidechain_height < task_cell_data.check_block_height_to
        || task_cell_data.check_block_hash_to != sidechain_state_output.latest_block_hash
        || (sidechain_state_input.latest_block_height + 1) != task_cell_data.check_block_height_from
        || sidechain_state_output.latest_block_height != task_cell_data.check_block_height_to
        || sidechain_state_output.latest_block_hash != task_cell_data.check_block_hash_to
        || task_cell_data.check_block_height_from >= task_cell_data.check_block_height_to
    {
        return Err(Error::TaskMismatch);
    }

    for i in 2..(sidechain_config_dep.commit_threshold + 2) as usize {
        let task_cell_output = TaskCellData::load(CellOrigin(i, Source::Output))?;
        let task_cell_type_args = TaskCellTypeArgs::load(CellOrigin(i, Source::Output))?;
        if task_cell_type_args != task_cell_type_args_data || task_cell_data != task_cell_output {
            return Err(Error::TaskMismatch);
        }
    }

    Ok(())
}
