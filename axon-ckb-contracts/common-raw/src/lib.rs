#![allow(dead_code)]
#![no_std]

use core::convert::TryFrom;

pub mod cell;
pub mod common;
pub mod molecule;
pub mod pattern;
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

macro_rules! SerializableNumber {
    ($type: ty, $size: expr) => {
        impl FromRaw for $type {
            fn from_raw(raw: &[u8]) -> Option<Self> {
                if raw.len() != $size {
                    return None;
                }

                let mut buf = [0u8; $size];

                buf.copy_from_slice(raw);
                Some(<$type>::from_le_bytes(buf))
            }
        }

        impl Serialize for $type {
            type RawType = [u8; $size];

            fn serialize(&self) -> Self::RawType {
                self.to_le_bytes()
            }
        }
    };
}

SerializableNumber!(u128, 16);
SerializableNumber!(u64, 8);
SerializableNumber!(u32, 4);
SerializableNumber!(u16, 2);
SerializableNumber!(u8, 1);

impl FromRaw for usize {
    fn from_raw(raw: &[u8]) -> Option<Self> {
        u16::from_raw(raw).map(|v| v.into())
    }
}

impl Serialize for usize {
    type RawType = [u8; 2];

    fn serialize(&self) -> Self::RawType {
        u16::try_from(self.clone()).unwrap().serialize()
    }
}
