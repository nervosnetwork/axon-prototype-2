use core::convert::{TryFrom, TryInto};

use crate::cell::global_config::GlobalConfigCellData;
use crate::{
    check_args_len, decode_i8, decode_u128, decode_u16, decode_u64, decode_u8, FromRaw, GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH,
    SUDT_HASHTYPE, SUDT_MUSE_ARGS,
};

pub mod checker_bond;
pub mod checker_info;
pub mod code;
pub mod global_config;
pub mod muse_token;
pub mod sidechain_bond;
pub mod sidechain_config;
pub mod sidechain_fee;
pub mod sidechain_state;
pub mod sudt_token;
pub mod task;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum CellType {
    Unknown,
    Sudt,
    MuseToken,
    CheckerBond,
    CheckerInfo,
    SidechainConfig,
    SidechainState,
    Task,
    SidechainFee,
    SidechainBond,
    Code,
}
