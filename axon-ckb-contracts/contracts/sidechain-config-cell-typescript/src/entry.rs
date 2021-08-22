use core::result::Result;

use common::check_code_cell;

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    /*
    related tx:

    1. CheckerJoinSidechain
    2. CheckerQuitSidechain
    3. CollatorSubmitChallenge
    */

    check_code_cell().ok_or(Error::CodeCellMissing)?;

    Ok(())
}
