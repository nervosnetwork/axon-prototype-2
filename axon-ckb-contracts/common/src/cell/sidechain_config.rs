use core::convert::{TryFrom, TryInto};
use core::result::Result;

use ckb_std::error::SysError;

use crate::error::CommonError;
use crate::{
    check_args_len, decode_i8, decode_u128, decode_u16, decode_u64, decode_u8, FromRaw, GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH,
    SUDT_HASHTYPE, SUDT_MUSE_ARGS,
};
use alloc::vec::Vec;
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::prelude::{Entity, Unpack};
use ckb_std::high_level::{load_cell, load_cell_data, load_cell_type_hash};

const SIDECHAIN_CONFIG_DATA_LEN: usize = 185;
const SIDECHAIN_CONFIG_TYPE_ARGS_LEN: usize = 1;
/**
    Sidechain Config Cell
    Data:
    Type:
        codehash: typeId
        hashtype: type
        args: chain_id(for lumos)
    Lock:
        codehash: A.S
        hashtype: data
        args: null
*/
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainConfigCellData {
    pub chain_id:                u8,
    pub checker_total_count:     u8,
    // 2**8 = 256
    pub checker_bitmap:          [u8; 32],
    // 256
    pub checker_threshold:       u8,
    pub update_interval:         u16,
    pub minimal_bond:            u128,
    pub checker_data_size_limit: u128,
    pub checker_price:           u128,
    pub refresh_interval:        u16,
    pub commit_threshold:        u8,
    pub challenge_threshold:     u8,
    pub admin_public_key:        [u8; 32],
    pub collator_public_key:     [u8; 32],
    pub bond_sudt_type_hash:     [u8; 32],
}

impl FromRaw for SidechainConfigCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<SidechainConfigCellData, SysError> {
        check_args_len(cell_raw_data.len(), SIDECHAIN_CONFIG_DATA_LEN)?;

        let chain_id = decode_u8(&cell_raw_data[0..1])?;
        let checker_total_count = decode_u8(&cell_raw_data[1..2])?;

        let mut checker_bitmap = [0u8; 32];
        checker_bitmap.copy_from_slice(&cell_raw_data[2..34]);

        let checker_threshold = decode_u8(&cell_raw_data[34..35])?;
        let update_interval = decode_u16(&cell_raw_data[35..37])?;
        let minimal_bond = decode_u128(&cell_raw_data[37..53])?;
        let checker_data_size_limit = decode_u128(&cell_raw_data[53..69])?;
        let checker_price = decode_u128(&cell_raw_data[69..85])?;
        let refresh_interval = decode_u16(&cell_raw_data[85..87])?;
        let commit_threshold = decode_u8(&cell_raw_data[87..88])?;
        let challenge_threshold = decode_u8(&cell_raw_data[88..89])?;

        let mut admin_public_key = [0u8; 32];
        admin_public_key.copy_from_slice(&cell_raw_data[89..121]);

        let mut collator_public_key = [0u8; 32];
        collator_public_key.copy_from_slice(&cell_raw_data[121..153]);

        let mut bond_sudt_type_hash = [0u8; 32];
        bond_sudt_type_hash.copy_from_slice(&cell_raw_data[153..185]);

        Ok(SidechainConfigCellData {
            chain_id,
            checker_total_count,
            checker_bitmap,
            checker_threshold,
            update_interval,
            minimal_bond,
            checker_data_size_limit,
            checker_price,
            refresh_interval,
            commit_threshold,
            challenge_threshold,
            admin_public_key,
            collator_public_key,
            bond_sudt_type_hash,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainConfigCellTypeArgs {
    pub chain_id: u8,
}

impl FromRaw for SidechainConfigCellTypeArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Result<SidechainConfigCellTypeArgs, SysError> {
        check_args_len(arg_raw_data.len(), SIDECHAIN_CONFIG_TYPE_ARGS_LEN)?;

        let chain_id = decode_u8(&arg_raw_data[0..1])?;

        Ok(SidechainConfigCellTypeArgs { chain_id })
    }
}
