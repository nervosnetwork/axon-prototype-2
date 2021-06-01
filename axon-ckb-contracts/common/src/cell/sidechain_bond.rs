use core::convert::{TryFrom, TryInto};
use core::result::Result;

use ckb_std::error::SysError;

use crate::error::CommonError;
use crate::{
    check_args_len, decode_i8, decode_u128, decode_u16, decode_u64, decode_u8, FromRaw, GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH,
    SUDT_DATA_LEN, SUDT_HASHTYPE, SUDT_MUSE_ARGS,
};
use alloc::vec::Vec;
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::prelude::{Entity, Unpack};
use ckb_std::high_level::{load_cell, load_cell_data, load_cell_type_hash};

const SIDECHAIN_BOND_LOCK_ARGS_LEN: usize = 49;

/**
    Sidechain Bond Cell
    Data:
    Type:
        codehash: sudt
        hashtype: type
        args: custom sudt admin
    Lock:
        codehash: sidechain bond cell lockscript
        hashtype: type
        args: chain_id | collator_public_key | unlock_sidechain_height
*/

// which is standard sudt
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainBondCellData {
    pub amount: u128,
}

impl FromRaw for SidechainBondCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<SidechainBondCellData, SysError> {
        check_args_len(cell_raw_data.len(), SUDT_DATA_LEN)?;

        let sudt_amount = decode_u128(&cell_raw_data[0..16])?;

        Ok(SidechainBondCellData { amount: sudt_amount })
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainBondCellLockArgs {
    pub chain_id:                u8,
    pub collator_public_key:     [u8; 32],
    pub unlock_sidechain_height: u128,
}

impl FromRaw for SidechainBondCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Result<SidechainBondCellLockArgs, SysError> {
        check_args_len(arg_raw_data.len(), SIDECHAIN_BOND_LOCK_ARGS_LEN)?;

        let chain_id = decode_u8(&arg_raw_data[0..1])?;

        let mut collator_public_key = [0u8; 32];
        collator_public_key.copy_from_slice(&arg_raw_data[1..33]);

        let unlock_sidechain_height = decode_u128(&arg_raw_data[33..49])?;

        Ok(SidechainBondCellLockArgs {
            chain_id,
            collator_public_key,
            unlock_sidechain_height,
        })
    }
}
