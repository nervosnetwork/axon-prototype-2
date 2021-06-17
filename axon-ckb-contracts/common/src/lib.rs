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
pub const SUDT_DATA_LEN: usize = 16; // u128

pub const EMPTY_BIT_MAP: [u8; 32] = [0; 32];

pub const GLOBAL_CONFIG_TYPE_HASH: [u8; 32] = [
    245, 208, 14, 171, 59, 152, 16, 213, 201, 255, 131, 77, 22, 90, 198, 197, 156, 60, 77, 233, 241, 191, 127, 194, 187, 229, 123, 127,
    218, 254, 226, 255,
];

#[macro_use]
extern crate alloc;

#[macro_export]
macro_rules! get_cell_type_hash {
    ($index: expr, $source: expr) => {
        common::ckb_std::high_level::load_cell_type_hash($index, $source)?.ok_or_else(|| common::error::HelperError::MissingTypeScript)?
    };
}

pub trait FromRaw {
    fn from_raw(cell_raw_data: &[u8]) -> Result<Self, SysError>
    where
        Self: Sized;
}

pub fn check_args_len(expected: usize, actual: usize) -> Result<(), SysError> {
    if actual != expected {
        return Err(SysError::Encoding);
    }
    Ok(())
}

pub fn decode_u128(data: &[u8]) -> Result<u128, SysError> {
    if data.len() != 16 {
        return Err(SysError::Encoding);
    }

    let mut buf = [0u8; 16];

    buf.copy_from_slice(data);
    Ok(u128::from_le_bytes(buf))
}

pub fn decode_u64(data: &[u8]) -> Result<u64, SysError> {
    if data.len() != 8 {
        return Err(SysError::Encoding);
    }

    let mut buf = [0u8; 8];
    buf.copy_from_slice(data);
    Ok(u64::from_le_bytes(buf))
}

pub fn decode_u16(data: &[u8]) -> Result<u16, SysError> {
    if data.len() != 2 {
        return Err(SysError::Encoding);
    }

    let mut buf = [0u8; 2];
    buf.copy_from_slice(data);
    Ok(u16::from_le_bytes(buf))
}

pub fn decode_u8(data: &[u8]) -> Result<u8, SysError> {
    if data.len() != 1 {
        return Err(SysError::Encoding);
    }

    let mut buf = [0u8; 1];
    buf.copy_from_slice(data);
    Ok(u8::from_le_bytes(buf))
}

pub fn decode_i8(data: &[u8]) -> Result<i8, SysError> {
    if data.len() != 1 {
        return Err(SysError::Encoding);
    }

    let mut buf = [0u8; 1];
    buf.copy_from_slice(data);
    Ok(i8::from_le_bytes(buf))
}

pub fn get_input_cell_count() -> usize {
    QueryIter::new(load_cell, Source::Input).count()
}

pub fn get_output_cell_count() -> usize {
    QueryIter::new(load_cell, Source::Output).count()
}

pub fn get_group_input_cell_count() -> usize {
    QueryIter::new(load_cell, Source::GroupInput).count()
}

pub fn get_group_output_cell_count() -> usize {
    QueryIter::new(load_cell, Source::GroupOutput).count()
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
