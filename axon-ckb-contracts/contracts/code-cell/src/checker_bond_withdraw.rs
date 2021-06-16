use ckb_std::ckb_constants::Source;

use common::EMPTY_BIT_MAP;
use common_raw::cell::checker_bond::CheckerBondCellLockArgs;

use crate::{common::*, error::Error};

const BOND_INPUT: CellOrigin = CellOrigin(1, Source::Input);

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

    let checker_bond_input = CheckerBondCellLockArgs::load(BOND_INPUT)?;

    if checker_bond_input.chain_id_bitmap != EMPTY_BIT_MAP || signer != checker_bond_input.checker_lock_arg {
        return Err(Error::CheckerBondMismatch);
    }

    Ok(())
}
