use crate::molecule::cell::checker_info::CheckerInfoCellTypeArgsBuilder;
use crate::molecule::cell::checker_info::{
    CheckerInfoCellBuilder, CheckerInfoCellReader, CheckerInfoCellTypeArgsReader, CheckerInfoStatusReader,
};
use crate::molecule::common::{ChainIdReader, PubKeyHashReader, StringBuilder, Uint128Reader};
use crate::{FromRaw, Serialize};
use core::convert::TryFrom;
use core::result::Result;
use molecule::prelude::*;
const CHECKER_INFO_DATA_LEN: usize = 546;
const CHECKER_INFO_TYPE_ARGS_LEN: usize = 21;
/**
    Checker Info Cell
    Data:
    Type:
        codehash: typeId
        hashtype: type
        args: chain_id | lock_arg
    Lock:
        codehash: A.S.
        hashtype: type
        args: null
*/
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[repr(u8)]
pub enum CheckerInfoStatus {
    Relaying = 0u8,
    Quit,
}

impl TryFrom<u8> for CheckerInfoStatus {
    type Error = ();

    fn try_from(mode: u8) -> Result<Self, Self::Error> {
        match mode {
            0u8 => Ok(Self::Relaying),
            1u8 => Ok(Self::Quit),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct CheckerInfoCell {
    pub unpaid_fee: u128,
    pub status:     CheckerInfoStatus,
    pub rpc_url:    Vec<u8>,
}

impl Default for CheckerInfoCell {
    fn default() -> Self {
        CheckerInfoCell {
            unpaid_fee: 0,
            status:     CheckerInfoStatus::Relaying,
            rpc_url:    Vec::default(),
        }
    }
}

impl FromRaw for CheckerInfoCell {
    fn from_raw(cell_raw_data: &[u8]) -> Option<CheckerInfoCell> {
        let reader = CheckerInfoCellReader::from_slice(cell_raw_data).expect("reader");
        let unpaid_fee = u128::from_raw(reader.unpaid_fee().raw_data()).expect("unpaid_fee");
        let rpc_url = reader.rpc_url().raw_data().to_vec();
        let status = CheckerInfoStatus::try_from(reader.status().raw_data()[0]).ok()?;
        Some(CheckerInfoCell {
            unpaid_fee,
            rpc_url,
            status,
        })
    }
}

impl Serialize for CheckerInfoCell {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let status = CheckerInfoStatusReader::new_unchecked(&(self.status as u8).serialize()).to_entity();
        let unpaid_fee = Uint128Reader::new_unchecked(&self.unpaid_fee.serialize()).to_entity();
        let mut rpc_url_builder = StringBuilder::default();
        for &v in self.rpc_url.iter() {
            rpc_url_builder = rpc_url_builder.push(Byte::new(v));
        }
        let builder = CheckerInfoCellBuilder::default()
            .rpc_url(rpc_url_builder.build())
            .status(status)
            .unpaid_fee(unpaid_fee);
        let mut buf = Vec::new();
        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing ChckerInfoCell");
        buf
    }
}

#[derive(Debug, Copy, Clone, Default, PartialOrd, PartialEq, Ord, Eq)]
pub struct CheckerInfoCellTypeArgs {
    pub chain_id:         u8,
    pub checker_lock_arg: [u8; 20],
}

impl FromRaw for CheckerInfoCellTypeArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<CheckerInfoCellTypeArgs> {
        let reader = CheckerInfoCellTypeArgsReader::from_slice(arg_raw_data).ok()?;

        let chain_id = u32::from_raw(reader.chain_id().raw_data())? as u8;

        let mut checker_lock_arg = [0u8; 20];
        checker_lock_arg.copy_from_slice(reader.checker_lock_arg().raw_data());

        Some(CheckerInfoCellTypeArgs {
            chain_id,
            checker_lock_arg,
        })
    }
}

impl Serialize for CheckerInfoCellTypeArgs {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let chain_id = ChainIdReader::new_unchecked(&(self.chain_id as u32).serialize()).to_entity();
        let checker_lock_arg = PubKeyHashReader::new_unchecked(&self.checker_lock_arg).to_entity();

        let builder = CheckerInfoCellTypeArgsBuilder::default()
            .chain_id(chain_id)
            .checker_lock_arg(checker_lock_arg);

        let mut buf = Vec::new();
        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing CheckerInfoCellTypeArgs");
        buf
    }
}
