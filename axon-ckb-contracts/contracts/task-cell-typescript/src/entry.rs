use core::result::Result;

use ckb_std::{
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    dynamic_loading_c_impl::CKBDLContext,
    high_level::load_script,
};

use crate::error::Error;

use ckb_lib_secp256k1::LibSecp256k1;
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
