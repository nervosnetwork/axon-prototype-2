use core::convert::TryInto;

use crate::{decode_u8, encode_u8, pattern::Pattern, FromRaw, Serialize};

const CHECKER_QUIT_SIDECHAIN_WITNESS_LEN: usize = 3;

#[derive(Debug)]
pub struct CheckerQuitSidechainWitness {
    pattern:        Pattern,
    pub chain_id:   u8,
    pub checker_id: u8,
}

impl Default for CheckerQuitSidechainWitness {
    fn default() -> Self {
        Self {
            pattern:    Pattern::CheckerQuitSidechain,
            chain_id:   0,
            checker_id: 0,
        }
    }
}

impl FromRaw for CheckerQuitSidechainWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerQuitSidechainWitness> {
        if witness_raw_data.len() < 3 {
            return None;
        }

        let pattern = decode_u8(&witness_raw_data[0..1])?.try_into().ok()?;
        let chain_id = decode_u8(&witness_raw_data[1..2])?;
        let checker_id = decode_u8(&witness_raw_data[2..3])?;

        Some(CheckerQuitSidechainWitness {
            pattern,
            chain_id,
            checker_id,
        })
    }
}

impl Serialize for CheckerQuitSidechainWitness {
    type RawType = [u8; CHECKER_QUIT_SIDECHAIN_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; CHECKER_QUIT_SIDECHAIN_WITNESS_LEN];

        buf[0..1].copy_from_slice(&encode_u8(self.pattern as u8));
        buf[1..2].copy_from_slice(&encode_u8(self.chain_id));
        buf[2..3].copy_from_slice(&encode_u8(self.checker_id));

        buf
    }
}
