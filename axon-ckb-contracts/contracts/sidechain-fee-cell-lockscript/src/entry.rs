use alloc::vec::Vec;
use core::result::Result;

use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    high_level::{load_cell_data, load_script, QueryIter},
};

use common::cell::{CheckerInfoCellData, FromRaw};

use crate::error::Error;
use common::pattern::check_code_cell;

const UDT_LEN: usize = 16;

pub fn main() -> Result<(), Error> {
    /*
    related tx:

    1. CollatorSubmitTask
    2. CollatorSubmitChallenge
    3. CheckerTakeBeneficiary
    */

    check_code_cell()?;

    Ok(())
}
