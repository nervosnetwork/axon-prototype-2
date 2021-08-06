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

impl FromRaw for PubKeyHash {
    fn from_raw(raw_data: &[u8]) -> Option<Self> {
        let mut buf = Self::default();
        buf.copy_from_slice(raw_data);
        Some(buf)
    }
}
pub type ScriptHash = [u8; 32];

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct BlockSlice {
    pub from: BlockHeight,
    pub to:   BlockHeight,
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

#[macro_export]
macro_rules! PureSudtTokenCell {
    ($type: ty) => {
        impl crate::FromRaw for $type {
            fn from_raw(cell_raw_data: &[u8]) -> Option<Self> {
                use molecule::prelude::Reader;

                let reader = crate::molecule::cell::sudt_token::SudtTokenCellReader::from_slice(cell_raw_data).ok()?;

                let amount = crate::FromRaw::from_raw(reader.amount().raw_data())?;

                Some(Self { amount })
            }
        }

        impl crate::Serialize for $type {
            type RawType = molecule::prelude::Vec<u8>;

            fn serialize(&self) -> Self::RawType {
                use molecule::prelude::{Builder, Reader};

                let amount = crate::molecule::common::Uint128Reader::new_unchecked(&crate::Serialize::serialize(&self.amount)).to_entity();

                let builder = crate::molecule::cell::sudt_token::SudtTokenCellBuilder::default().amount(amount);

                let mut buf = molecule::prelude::Vec::new();
                builder
                    .write(&mut buf)
                    .expect(concat!("Unable to write buffer while serializing ", stringify!($type)));
                buf
            }
        }
    };
}
