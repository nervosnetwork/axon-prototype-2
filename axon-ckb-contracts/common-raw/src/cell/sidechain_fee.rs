use molecule::prelude::*;

use crate::{
    common::*,
    molecule::{
        cell::{
            sidechain_fee::{SidechainFeeCellLockArgsBuilder, SidechainFeeCellLockArgsReader},
            sudt_token::{SudtTokenCellBuilder, SudtTokenCellReader},
        },
        common::{ChainIdReader, Uint128Reader},
    },
    FromRaw, Serialize,
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

impl FromRaw for SidechainFeeCell {
    fn from_raw(cell_raw_data: &[u8]) -> Option<Self> {
        let reader = SudtTokenCellReader::from_slice(cell_raw_data).ok()?;

        let amount = u128::from_raw(reader.amount().raw_data())?;

        Some(Self { amount })
    }
}

impl Serialize for SidechainFeeCell {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let amount = Uint128Reader::new_unchecked(&self.amount.serialize()).to_entity();

        let builder = SudtTokenCellBuilder::default().amount(amount);

        let mut buf = Vec::new();
        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing SidechainFeeCell");
        buf
    }
}

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
