#![allow(dead_code)]
#![no_std]

pub mod cell;
pub mod witness;

pub const SUDT_CODEHASH: [u8; 32] = [0; 32];
pub const SUDT_HASHTYPE: u8 = 1u8;
pub const SUDT_MUSE_ARGS: &[u8] = &[1u8];
pub const SUDT_DATA_LEN: usize = 16; // u128

pub const EMPTY_BIT_MAP: [u8; 32] = [0; 32];

pub const GLOBAL_CONFIG_TYPE_HASH: [u8; 32] = [0; 32];

pub trait FromRaw {
    fn from_raw(cell_raw_data: &[u8]) -> Option<Self>
    where
        Self: Sized;
}

pub trait Serialize {
    type RawType: AsRef<[u8]>;

    fn serialize(&self) -> Self::RawType;
}

pub fn check_args_len(expected: usize, actual: usize) -> Option<()> {
    if actual != expected {
        return None;
    }
    Some(())
}

pub fn decode_u128(data: &[u8]) -> Option<u128> {
    if data.len() != 16 {
        return None;
    }

    let mut buf = [0u8; 16];

    buf.copy_from_slice(data);
    Some(u128::from_le_bytes(buf))
}

pub fn encode_u128(num: u128) -> [u8; 16] {
    num.to_le_bytes()
}

pub fn decode_u64(data: &[u8]) -> Option<u64> {
    if data.len() != 8 {
        return None;
    }

    let mut buf = [0u8; 8];
    buf.copy_from_slice(data);
    Some(u64::from_le_bytes(buf))
}

pub fn encode_u64(num: u64) -> [u8; 8] {
    num.to_le_bytes()
}

pub fn decode_u32(data: &[u8]) -> Option<u32> {
    if data.len() != 4 {
        return None;
    }

    let mut buf = [0u8; 4];
    buf.copy_from_slice(data);
    Some(u32::from_le_bytes(buf))
}

pub fn encode_u32(num: u32) -> [u8; 4] {
    num.to_le_bytes()
}

pub fn decode_u16(data: &[u8]) -> Option<u16> {
    if data.len() != 2 {
        return None;
    }

    let mut buf = [0u8; 2];
    buf.copy_from_slice(data);
    Some(u16::from_le_bytes(buf))
}

pub fn encode_u16(num: u16) -> [u8; 2] {
    num.to_le_bytes()
}

pub fn decode_u8(data: &[u8]) -> Option<u8> {
    if data.len() != 1 {
        return None;
    }

    let mut buf = [0u8; 1];
    buf.copy_from_slice(data);
    Some(u8::from_le_bytes(buf))
}

pub fn encode_u8(num: u8) -> [u8; 1] {
    num.to_le_bytes()
}

pub fn decode_i8(data: &[u8]) -> Option<i8> {
    if data.len() != 1 {
        return None;
    }

    let mut buf = [0u8; 1];
    buf.copy_from_slice(data);
    Some(i8::from_le_bytes(buf))
}

pub fn encode_i8(num: i8) -> [u8; 1] {
    num.to_le_bytes()
}
