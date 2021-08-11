use molecule::prelude::Reader;
use molecule::prelude::*;

use crate::{
    common::{ChainId, PubKeyHash},
    molecule::{
        cell::checker_bond::{CheckerBondCellLockArgsBuilder, CheckerBondCellLockArgsReader},
        common::{ChainIdListBuilder, ChainIdReader, PubKeyHashReader},
    },
    FromRaw, PureSudtTokenCell, Serialize,
};

const CHECKER_BOND_LOCK_ARGS_LEN: usize = 52;

/**
    Checker Bond Cell
    Data:
    Type:
        codehash: sudt
        hashtype: type
        args: muse_token_admin
    Lock:
        codehash: checker bond cell lockscript
        hashtype: type
        args: checker public key | chain id bitmap
*/

// which is standard sudt
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct CheckerBondCell {
    pub amount: u128,
}

PureSudtTokenCell!(CheckerBondCell);

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct CheckerBondCellLockArgs {
    pub checker_lock_arg:      PubKeyHash,
    pub participated_chain_id: Vec<ChainId>,
}

impl FromRaw for CheckerBondCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<CheckerBondCellLockArgs> {
        let reader = CheckerBondCellLockArgsReader::from_slice(arg_raw_data).ok()?;

        let mut checker_lock_arg = [0u8; 20];
        checker_lock_arg.copy_from_slice(reader.checker_lock_arg().raw_data());

        let participated_chain_id = reader
            .participated_chain_id()
            .iter()
            .fold(Vec::new(), |mut participated_chain_id, chain_id_reader| {
                participated_chain_id.push(ChainId::from_raw(chain_id_reader.raw_data()));
                participated_chain_id
            })
            .into_iter()
            .collect::<Option<Vec<ChainId>>>()?;

        Some(CheckerBondCellLockArgs {
            checker_lock_arg,
            participated_chain_id,
        })
    }
}

impl Serialize for CheckerBondCellLockArgs {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let lock_arg = PubKeyHashReader::new_unchecked(&self.checker_lock_arg).to_entity();
        let participated_chain_id = self
            .participated_chain_id
            .iter()
            .fold(ChainIdListBuilder::default(), |chain_id_list, chain_id| {
                chain_id_list.push(ChainIdReader::new_unchecked(&chain_id.serialize()).to_entity())
            })
            .build();

        let builder = CheckerBondCellLockArgsBuilder::default()
            .checker_lock_arg(lock_arg)
            .participated_chain_id(participated_chain_id);

        let mut buf = Vec::new();
        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing CheckerBondCellLockArgs");
        buf
    }
}
