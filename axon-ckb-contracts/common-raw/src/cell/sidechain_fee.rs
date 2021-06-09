use core::convert::{TryFrom, TryInto};

use crate::{
    check_args_len, decode_i8, decode_u128, decode_u16, decode_u64, decode_u8, FromRaw, GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH,
    SUDT_DATA_LEN, SUDT_HASHTYPE, SUDT_MUSE_ARGS,
};

const SIDECHAIN_FEE_LOCK_ARGS_LEN: usize = 1;

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

// which is standard sudt
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainFeeCellData {
    pub amount: u128,
}

impl FromRaw for SidechainFeeCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Option<SidechainFeeCellData> {
        check_args_len(cell_raw_data.len(), SUDT_DATA_LEN)?;

        let sudt_amount = decode_u128(&cell_raw_data[0..16])?;

        Some(SidechainFeeCellData { amount: sudt_amount })
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainFeeCellLockArgs {
    pub chain_id: u8,
}

impl FromRaw for SidechainFeeCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<SidechainFeeCellLockArgs> {
        check_args_len(arg_raw_data.len(), SIDECHAIN_FEE_LOCK_ARGS_LEN)?;

        let chain_id = decode_u8(&arg_raw_data[0..1])?;

        Some(SidechainFeeCellLockArgs { chain_id })
    }
}
