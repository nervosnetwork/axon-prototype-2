use core::result::Result;

use ckb_std::{
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    dynamic_loading_c_impl::CKBDLContext,
    high_level::load_script,
};

use crate::error::Error;

use common::pattern::{check_code_cell, is_checker_bond_withdraw, is_checker_join_sidechain, is_checker_quit_sidechain, Pattern};

use alloc::vec::Vec;
use ckb_lib_secp256k1::LibSecp256k1;
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell, load_cell_data, load_cell_lock, load_witness_args, QueryIter};
use ckb_std::syscalls::load_witness;
use common::{bit_check, bit_op, get_input_cell_count, get_output_cell_count, EMPTY_BIT_MAP, GLOBAL_CONFIG_TYPE_HASH};

pub fn main() -> Result<(), Error> {
    check_code_cell()?;
    Ok(())
}
