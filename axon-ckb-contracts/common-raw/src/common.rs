use crate::{FromRaw, Serialize};

pub type BlockHeader = [u8; 32];
pub type BlockHeight = u128;
pub type CodeHash = [u8; 32];

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[repr(u8)]
pub enum HashType {
    Data,
    Type,
}

impl Default for HashType {
    fn default() -> Self {
        Self::Data
    }
}

impl FromRaw for HashType {
    fn from_raw(raw: &[u8]) -> Option<Self> {
        let status = u8::from_raw(raw)?;
        match status {
            0u8 => Some(Self::Data),
            1u8 => Some(Self::Type),
            _ => None,
        }
    }
}

impl Serialize for HashType {
    type RawType = [u8; 1];

    fn serialize(&self) -> Self::RawType {
        (*self as u8).serialize()
    }
}

pub type MerkleHash = [u8; 32];
pub type PubKeyHash = [u8; 20];
pub type ScriptHash = [u8; 32];

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
struct BlockSlice {
    from: BlockHeight,
    to:   BlockHeight,
}

pub type ChainId = u32;

pub type RandomSeed = [u8; 32];
pub type CommittedHash = [u8; 32];
