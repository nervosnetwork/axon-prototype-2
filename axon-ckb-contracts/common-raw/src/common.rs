use crate::molecule::common::{BlockHeightReader, BlockSliceBuilder, BlockSliceReader};
use crate::{FromRaw, Serialize};
use molecule::prelude::*;
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
pub struct BlockSlice {
    from: BlockHeight,
    to:   BlockHeight,
}

impl FromRaw for BlockSlice {
    fn from_raw(raw: &[u8]) -> Option<Self> {
        let reader = BlockSliceReader::from_slice(raw).ok()?;
        let from = BlockHeight::from_raw(reader.from().raw_data())?;
        let to = BlockHeight::from_raw(reader.to().raw_data())?;
        return Some(BlockSlice { from, to });
    }
}

impl Serialize for BlockSlice {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let mut buf = Vec::new();
        BlockSliceBuilder::default()
            .from(BlockHeightReader::new_unchecked(&self.from.serialize()).to_entity())
            .to(BlockHeightReader::new_unchecked(&self.to.serialize()).to_entity())
            .write(&mut buf)
            .expect("Unable to write buffer while serializing BlockSlice");
        buf
    }
}
pub type ChainId = u32;

pub type RandomSeed = [u8; 32];
pub type CommittedHash = [u8; 32];
