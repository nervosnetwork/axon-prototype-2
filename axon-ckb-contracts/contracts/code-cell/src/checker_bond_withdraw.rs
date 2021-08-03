use ckb_std::ckb_constants::Source;

use common_raw::cell::{
    checker_bond::{CheckerBondCellData, CheckerBondCellLockArgs},
    code::CodeCell,
    muse_token::MuseTokenData,
};

use crate::{cell::*, common::*, error::Error};

const BOND_INPUT: CellOrigin = CellOrigin(1, Source::Input);

const TOKEN_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);

pub fn checker_bond_withdraw(signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerBondWithdraw

    Dep:    0 Global Config Cell

    Code Cell                   ->         Code Cell
    Checker Bond Cell           ->         Muse Token Cell

     */

    /*
    Job:

    1. chain_id_bitmap is 0x00

     */

    is_checker_bond_withdraw()?;

    let checker_bond_input = CheckerBondCellLockArgs::load(BOND_INPUT)?;

    if checker_bond_input.chain_id_bitmap != EMPTY_BIT_MAP || signer != checker_bond_input.checker_lock_arg {
        return Err(Error::CheckerBondMismatch);
    }

    Ok(())
}

fn is_checker_bond_withdraw() -> Result<(), Error> {
    let global = check_global_cell()?;

    if is_cell_count_not_equals(2, Source::Input) || is_cell_count_not_equals(2, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            CodeCell: CODE_INPUT,
            CheckerBondCellData: BOND_INPUT,
            CodeCell: CODE_OUTPUT,
            MuseTokenData: TOKEN_OUTPUT,
        },
    };

    Ok(())
}
