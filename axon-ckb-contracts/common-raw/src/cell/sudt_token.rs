use core::convert::{TryFrom, TryInto};

use crate::{
    check_args_len, decode_i8, decode_u128, decode_u16, decode_u64, decode_u8, FromRaw, GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH,
    SUDT_HASHTYPE, SUDT_MUSE_ARGS,
};

const SUDT_DATA_LEN: usize = 16; // u128

// which is standard sudt
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SudtTokenData {
    pub amount: u128,
}

impl FromRaw for SudtTokenData {
    fn from_raw(cell_raw_data: &[u8]) -> Option<SudtTokenData> {
        check_args_len(cell_raw_data.len(), SUDT_DATA_LEN)?;

        let sudt_amount = decode_u128(&cell_raw_data[..16])?;

        Some(SudtTokenData { amount: sudt_amount })
    }
}
