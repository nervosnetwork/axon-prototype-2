#![no_std]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::ckb_constants::Source;

use crate::error::CommonError;
use bit_vec::*;
use ckb_std::error::SysError;
use ckb_std::high_level::{load_cell, QueryIter};

pub mod cell;
pub mod error;
pub mod hash;
pub mod pattern;

pub const SUDT_CODEHASH: [u8; 32] = [
    220, 119, 113, 76, 154, 36, 54, 83, 120, 144, 192, 65, 69, 88, 144, 235, 134, 189, 90, 138, 42, 237, 82, 25, 243, 92, 250, 145, 25, 53,
    95, 95,
];
pub const SUDT_HASHTYPE: u8 = 0u8;
pub const SUDT_MUSE_ARGS: &[u8] = &[];

pub const EMPTY_BIT_MAP: [u8; 32] = [0; 32];

pub const GLOBAL_CONFIG_TYPE_HASH: [u8; 32] = [
    245, 208, 14, 171, 59, 152, 16, 213, 201, 255, 131, 77, 22, 90, 198, 197, 156, 60, 77, 233, 241, 191, 127, 194, 187, 229, 123, 127,
    218, 254, 226, 255,
];

pub fn check_args_len(expected: usize, actual: usize) -> Result<(), SysError> {
    if actual != expected {
        return Err(SysError::Encoding);
    }
    Ok(())
}

pub fn get_input_cell_count() -> usize {
    QueryIter::new(load_cell, Source::Input).count()
}

pub fn get_output_cell_count() -> usize {
    QueryIter::new(load_cell, Source::Output).count()
}

pub fn bit_map_add(input: &[u8; 32], checker_id: u8) -> Result<[u8; 32], CommonError> {
    let mut input = BitVec::from_bytes(&input[..]);

    //should be false
    if input.get(checker_id as usize).ok_or(CommonError::BitOperator)? {
        return Err(CommonError::BitOperator);
    }

    input.set(checker_id as usize, true);

    let mut ret = [0u8; 32];
    ret.copy_from_slice(input.to_bytes().as_slice());

    Ok(ret)
}

pub fn bit_map_remove(input: [u8; 32], checker_id: u8) -> Result<[u8; 32], CommonError> {
    let mut input = BitVec::from_bytes(&input[..]);

    //should be true
    if !input.get(checker_id as usize).ok_or(CommonError::BitOperator)? {
        return Err(CommonError::BitOperator);
    }

    input.set(checker_id as usize, false);

    let mut ret = [0u8; 32];
    ret.copy_from_slice(&input.to_bytes().as_slice()[0..32]);

    Ok(ret)
}

//check if given number is bit-marked in input array
pub fn bit_map_marked(input: [u8; 32], checker_id: u8) -> Result<bool, CommonError> {
    let input = BitVec::from_bytes(&input[..]);

    Ok(input.get(checker_id as usize).ok_or(CommonError::BitOperator)?)
}
