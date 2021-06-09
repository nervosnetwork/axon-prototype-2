use core::convert::{TryFrom, TryInto};

use crate::{
    check_args_len, decode_i8, decode_u128, decode_u16, decode_u64, decode_u8, FromRaw, GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH,
    SUDT_HASHTYPE, SUDT_MUSE_ARGS,
};

#[derive(Debug)]
pub struct CollatorPublishTaskWitness {
    pub pattern:  u8,
    pub chain_id: u8,
    pub bond:     u128,
}

impl FromRaw for CollatorPublishTaskWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CollatorPublishTaskWitness> {
        if witness_raw_data.len() < 2 {
            return None;
        }

        let pattern = decode_u8(&witness_raw_data[0..1])?;
        let chain_id = decode_u8(&witness_raw_data[1..2])?;
        let bond = decode_u128(&witness_raw_data[2..18])?;

        Some(CollatorPublishTaskWitness { pattern, chain_id, bond })
    }
}
