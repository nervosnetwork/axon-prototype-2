use alloc::vec::Vec;
use core::result::Result;

use common::{
    blake2b,
    ckb_std::{
        ckb_constants::Source,
        ckb_types::{
            packed::{Byte, CellOutput},
            prelude::*,
        },
        high_level::{load_cell, load_cell_data, load_cell_lock_hash, load_cell_type_hash, load_script, load_witness_args, QueryIter},
    },
    decode_u128, decode_u64, get_cell_type_hash,
    hash::blake2b_256,
};

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
