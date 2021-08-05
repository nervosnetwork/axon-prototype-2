use molecule::prelude::*;

use crate::{
    common::*,
    molecule::{
        cell::sidechain_fee::{SidechainFeeCellLockArgsBuilder, SidechainFeeCellLockArgsReader},
        common::ChainIdReader,
    },
    FromRaw, PureSudtTokenCell, Serialize,
};

/**
    Sidechain Fee Cell
    Data:
    Type:
        codehash: sudt
        hashtype: type
        args: muse_token_admin
    Lock:
        codehash: sidechain fee cell lockscript
        hashtype: type
        args: chain_id
*/
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainFeeCell {
    pub amount: u128,
}

PureSudtTokenCell!(SidechainFeeCell);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainFeeCellLockArgs {
    pub chain_id: u8, // TODO: Change to ChainId
}

impl FromRaw for SidechainFeeCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<Self> {
        let reader = SidechainFeeCellLockArgsReader::from_slice(arg_raw_data).ok()?;

        let chain_id = ChainId::from_raw(reader.chain_id().raw_data())? as u8;

        Some(Self { chain_id })
    }
}

impl Serialize for SidechainFeeCellLockArgs {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let chain_id = ChainIdReader::new_unchecked(&(self.chain_id as ChainId).serialize()).to_entity();

        let builder = SidechainFeeCellLockArgsBuilder::default().chain_id(chain_id);

        let mut buf = Vec::new();
        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing SidechainFeeCellLockArgs");
        buf
    }
}
