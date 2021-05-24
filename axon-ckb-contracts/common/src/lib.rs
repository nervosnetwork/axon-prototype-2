#![no_std]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

pub use blake2b_ref;
pub use ckb_std;
use ckb_std::error::SysError;

mod cell;
pub mod error;
pub mod hash;

#[macro_export]
macro_rules! get_cell_type_hash {
    ($index: expr, $source: expr) => {
        common::ckb_std::high_level::load_cell_type_hash($index, $source)?
            .ok_or_else(|| common::error::HelperError::MissingTypeScript)?
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
