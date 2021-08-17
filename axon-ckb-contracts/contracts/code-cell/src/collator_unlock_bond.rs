use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        code::CodeCell,
        muse_token::MuseTokenCell,
        sidechain_bond::{SidechainBondCell, SidechainBondCellLockArgs},
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs, SidechainStatus},
        sidechain_state::{SidechainStateCell, SidechainStateCellTypeArgs},
        sudt_token::SudtTokenCell,
    },
    witness::collator_unlock_bond::CollatorUnlockBondWitness,
    FromRaw,
};
use core::convert::TryFrom;

use crate::{cell::*, common::*, error::Error};

const SIDECHAIN_CONFIG_DEP: CellOrigin = CellOrigin(5, Source::CellDep);
const SIDECHAIN_STATE_DEP: CellOrigin = CellOrigin(6, Source::CellDep);

const SIDECHAIN_BOND_INPUT: CellOrigin = CellOrigin(1, Source::Input);

const SUDT_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);

pub fn collator_unlock_bond(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CollatorUnlockBond,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell
    Dep:    2 Sidechain State Cell

    Code Cell                   ->          Code Cell
    Sidechain Bond Cell         ->          Sudt Cell

    */

    let witness = CollatorUnlockBondWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    is_collator_unlock_bond()?;

    let (
        config_dep_type_args,
        config_dep,
        state_dep_type_args,
        state_dep,
        sidechain_bond_input_lock_args,
        sidechain_bond_input,
        muse_token_output,
    ) = load_entities! {
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_DEP,
        SidechainConfigCell: SIDECHAIN_CONFIG_DEP,
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_DEP,
        SidechainStateCell: SIDECHAIN_STATE_DEP,
        SidechainBondCellLockArgs: SIDECHAIN_BOND_INPUT,
        SidechainBondCell: SIDECHAIN_BOND_INPUT,
        MuseTokenCell: SUDT_OUTPUT,
    };
    if config_dep_type_args.chain_id != witness.chain_id || config_dep.sidechain_status != SidechainStatus::Shutdown {
        return Err(Error::SidechainConfigMismatch);
    }

    if state_dep_type_args.chain_id != u32::try_from(witness.chain_id).or(Err(Error::Encoding))?
        || state_dep.submit_sidechain_block_height > sidechain_bond_input_lock_args.unlock_sidechain_height
    {
        return Err(Error::SidechainStateMismatch);
    }

    if signer != sidechain_bond_input_lock_args.collator_lock_arg
        || sidechain_bond_input_lock_args.chain_id != witness.chain_id
        || sidechain_bond_input.amount != muse_token_output.amount
    {
        return Err(Error::SidechainBondMismatch);
    }

    Ok(())
}

fn is_collator_unlock_bond() -> Result<(), Error> {
    let global = check_global_cell()?;

    if is_cell_count_not_equals(2, Source::Input) || is_cell_count_not_equals(2, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCell: SIDECHAIN_CONFIG_DEP,
            SidechainStateCell: SIDECHAIN_STATE_DEP,

            CodeCell: CODE_INPUT,
            SidechainBondCell: SIDECHAIN_BOND_INPUT,

            CodeCell: CODE_OUTPUT,
            SudtTokenCell: SUDT_OUTPUT,
        },
    };

    Ok(())
}
