use alloc::vec::Vec;
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use common::{
    blake2b,
    ckb_std::{
        ckb_constants::Source,
        ckb_types::{
            packed::{Byte, CellOutput},
            prelude::*,
        },
        default_alloc,
        high_level::{
            load_cell, load_cell_data, load_cell_lock_hash, load_cell_type_hash, load_script,
            load_witness_args, QueryIter,
        },
    },
    decode_u128, decode_u64, get_cell_type_hash,
    hash::blake2b_256,
};

use crate::error::Error;

// Alloc 4K fast HEAP + 2M HEAP to receives PrefilledData
default_alloc!(4 * 1024, 2048 * 1024, 64);

pub fn main() -> Result<(), Error> {
    Ok(())
}
