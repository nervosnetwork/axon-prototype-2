use alloc::vec::Vec;
use core::result::Result;

use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    high_level::{load_cell_data, load_script, QueryIter},
};

use common::cell::{CheckerInfoCellData, FromRaw};

use crate::error::Error;

const UDT_LEN: usize = 16;

pub fn main() -> Result<(), Error> {
    // TODO: Skip checking if SSC exist (Confirming checking task / challenge task)

    /*
    related tx:

    1. CollatorSubmitTask
    2. CollatorSubmitChallenge
    3. CheckerTakeBeneficiary
    */

    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    // we have recognised the checker is taking check fee
    // Checker Info        ->          Checker Info
    // Muse Token          ->          Muse Token
    // Sidechain Fee Cell  ->          Sidechain Fee Cell

    // Chain id: 1 Byte
    // if args.len() != 1 {
    //     return Err(Error::InvalidArgument);
    // }

    Ok(())
}
