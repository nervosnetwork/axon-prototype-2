use core::{
    convert::{TryFrom, TryInto},
    default::Default,
};

use crate::{
    check_args_len, decode_u8, encode_u8, FromRaw, Serialize, GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH, SUDT_HASHTYPE, SUDT_MUSE_ARGS,
};

const CHECKER_JOIN_SIDECHAIN_WITNESS_LEN: usize = 3;

#[derive(Debug)]
pub struct CheckerJoinSidechainWitness {
    pub pattern:    u8,
    pub chain_id:   u8,
    pub checker_id: u8,
}

impl Default for CheckerJoinSidechainWitness {
    fn default() -> Self {
        Self {
            pattern:    0,
            chain_id:   0,
            checker_id: 0,
        }
    }
}

impl FromRaw for CheckerJoinSidechainWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerJoinSidechainWitness> {
        if witness_raw_data.len() < CHECKER_JOIN_SIDECHAIN_WITNESS_LEN {
            return None;
        }

        let pattern = decode_u8(&witness_raw_data[0..1])?;
        let chain_id = decode_u8(&witness_raw_data[1..2])?;
        let checker_id = decode_u8(&witness_raw_data[2..3])?;

        Some(CheckerJoinSidechainWitness {
            pattern,
            chain_id,
            checker_id,
        })
    }
}

impl Serialize for CheckerJoinSidechainWitness {
    type RawType = [u8; CHECKER_JOIN_SIDECHAIN_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; CHECKER_JOIN_SIDECHAIN_WITNESS_LEN];

        buf[0..1].copy_from_slice(&encode_u8(self.pattern));
        buf[1..2].copy_from_slice(&encode_u8(self.chain_id));
        buf[2..3].copy_from_slice(&encode_u8(self.checker_id));

        buf
    }
}
