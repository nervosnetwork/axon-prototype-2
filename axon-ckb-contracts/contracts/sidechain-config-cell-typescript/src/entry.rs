use core::result::Result;

use crate::error::Error;

use common::pattern::check_code_cell;

pub fn main() -> Result<(), Error> {
    /*
    related tx:

    1. CheckerJoinSidechain
    2. CheckerQuitSidechain
    3. CollatorSubmitChallenge
    */

    check_code_cell()?;

    Ok(())
}
