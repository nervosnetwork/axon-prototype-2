use core::result::Result;

use ckb_std::{
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    dynamic_loading_c_impl::CKBDLContext,
    high_level::load_script,
};

use crate::error::Error;

use alloc::vec::Vec;
use ckb_lib_secp256k1::LibSecp256k1;
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell, load_cell_data, load_cell_type_hash, load_witness_args, QueryIter};
use ckb_std::syscalls::load_cell_by_field;
use common::cell::CellType::SidechainConfig;
use common::pattern::{
    check_code_cell, is_checker_join_sidechain, is_checker_publish_challenge, is_checker_quit_sidechain, is_checker_submit_challenge,
    is_checker_submit_task, is_checker_take_beneficiary, Pattern,
};
use common::{get_input_cell_count, get_output_cell_count, SUDT_CODEHASH, SUDT_HASHTYPE};

pub fn main() -> Result<(), Error> {
    /*
    related tx:

    1. CheckerJoinSidechain
    2. CheckerQuitSidechain
    3. CheckerSubmitTask
    4. CheckerPublishChallenge
    5. CheckerSubmitChallenge
    6. CheckerTakeBeneficiary

    7. CollatorSubmitTask
    8. CollatorSubmitChallenge
    */

    check_code_cell()?;

    Ok(())
}
