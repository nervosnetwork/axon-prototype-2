use core::result::Result;

use crate::error::Error;

use common::check_code_cell;

pub fn main() -> Result<(), Error> {
    /*
    related tx:

    1. CollatorPublishTask
    2. CollatorSubmitTask
    3. CollatorSubmitChallenge
    */

    check_code_cell().ok_or(Error::CodeCellMissing)?;

    Ok(())
}
