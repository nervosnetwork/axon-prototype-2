use core::result::Result;

use crate::error::Error;

use common::check_code_cell;

pub fn main() -> Result<(), Error> {
    /*
    related tx:

    1. CollatorPublishTask
    2. CollatorUnlockBond
    */

    /*
    CollatorUnlockBond,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain State Cell

    Sidechain Bond Cell         ->          Muse Token Cell

    */
    check_code_cell().ok_or(Error::CodeCellMissing)?;

    Ok(())
}
