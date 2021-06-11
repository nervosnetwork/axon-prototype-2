use core::result::Result;

use crate::error::Error;

use common::pattern::check_code_cell;

pub fn main() -> Result<(), Error> {
    /*
    related tx:

    1. CollatorPublishTask
    2. CollatorSubmitTask
    3. CollatorSubmitChallenge
    4. CollatorRefreshTask
    5. CheckerSubmitTask
    6. CheckerPublishChallenge
    7. CheckerSubmitChallenge
    */

    check_code_cell()?;

    Ok(())
}
