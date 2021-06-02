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

#[derive(Debug)]
pub struct CheckerSubmitTaskWitness {
    pub pattern:    u8,
    pub chain_id:   u8,
    pub checker_id: u8,
}

impl FromRaw for CheckerSubmitTaskWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Result<CheckerSubmitTaskWitness, SysError> {
        if witness_raw_data.len() < 3 {
            return Err(SysError::Encoding);
        }

        let pattern = decode_u8(&witness_raw_data[0..1])?;
        let chain_id = decode_u8(&witness_raw_data[1..2])?;
        let checker_id = decode_u8(&witness_raw_data[2..3])?;

        Ok(CheckerSubmitTaskWitness {
            pattern,
            chain_id,
            checker_id,
        })
    }
}
