use alloc::vec::Vec;
use core::result::Result;

use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    dynamic_loading_c_impl::CKBDLContext,
    high_level::{load_cell_data, load_script},
};

use crate::error::Error;

use ckb_lib_secp256k1::LibSecp256k1;
use common::pattern::check_code_cell;

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
    check_code_cell()?;

    Ok(())
}
