use core::result::Result;

use crate::error::Error;

use common::check_code_cell;

pub fn main() -> Result<(), Error> {
    check_code_cell().ok_or(Error::CodeCellMissing)?;
    Ok(())
}
