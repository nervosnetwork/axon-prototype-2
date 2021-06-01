#![no_std]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use crate::error::CommonError;
pub use blake2b_ref;
pub use ckb_std;
use ckb_std::ckb_constants::{CellField, Source};
use ckb_std::error::SysError;
use ckb_std::high_level::{load_cell, load_cell_type, load_cell_type_hash, load_script, load_script_hash, QueryIter};
use ckb_std::syscalls::load_cell_by_field;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

pub mod cell;
pub mod error;
pub mod hash;
pub mod pattern;

pub const SUDT_CODEHASH: [u8; 32] = [0; 32];
pub const SUDT_HASHTYPE: u8 = 1u8;
pub const SUDT_MUSE_ARGS: &[u8] = &[1u8];

pub const EMPTY_BIT_MAP: [u8; 32] = [0; 32];

pub const GLOBAL_CONFIG_TYPE_HASH: [u8; 32] = [0; 32];

#[macro_use]
extern crate alloc;

#[macro_export]
macro_rules! get_cell_type_hash {
    ($index: expr, $source: expr) => {
        common::ckb_std::high_level::load_cell_type_hash($index, $source)?.ok_or_else(|| common::error::HelperError::MissingTypeScript)?
    };
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

// check if the corresponding bit is marked
pub fn bit_check(bit_map: [u8; 32], chain_id: u8, masked: bool) -> bool {
    let byte_offset = chain_id / 8;

    let target = bit_map[byte_offset as usize];

    let bit_offset = chain_id - byte_offset * 8;

    let mask: u8 = match bit_offset {
        0u8 => 0b10000000,
        1u8 => 0b01000000,
        2u8 => 0b00100000,
        3u8 => 0b00010000,
        4u8 => 0b00001000,
        5u8 => 0b00000100,
        6u8 => 0b00000010,
        7u8 => 0b00000001,
        _ => return false,
    };

    if masked {
        (target & mask) == 1u8
    } else {
        (target & mask) == 0u8
    }
}

pub fn bit_op(bit_map: &mut [u8; 32], chain_id: u8, set: bool) {
    let byte_offset = chain_id / 8;

    let mut target = &mut bit_map[byte_offset as usize];

    let bit_offset = chain_id - byte_offset * 8;

    if set {
        let mask: u8 = match bit_offset {
            0u8 => 0b10000000,
            1u8 => 0b01000000,
            2u8 => 0b00100000,
            3u8 => 0b00010000,
            4u8 => 0b00001000,
            5u8 => 0b00000100,
            6u8 => 0b00000010,
            7u8 => 0b00000001,
            _ => 0b00000000,
        };

        target.bitor_assign(mask);
    } else {
        let mask: u8 = match bit_offset {
            0u8 => 0b01111111,
            1u8 => 0b10111111,
            2u8 => 0b11011111,
            3u8 => 0b11101111,
            4u8 => 0b11110111,
            5u8 => 0b11111011,
            6u8 => 0b11111101,
            7u8 => 0b11111110,
            _ => 0b00000000,
        };
        target.bitand_assign(mask);
    }
}
