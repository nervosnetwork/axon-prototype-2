use core::default::Default;

use crate::{common::ChainId, pattern::Pattern, FromRaw, Serialize};

const CHECKER_JOIN_SIDECHAIN_WITNESS_LEN: usize = 6;

#[derive(Debug)]
pub struct CheckerJoinSidechainWitness {
    pattern:        Pattern,
    pub chain_id:   ChainId,
    pub checker_id: u8,
}

impl Default for CheckerJoinSidechainWitness {
    fn default() -> Self {
        Self {
            pattern:    Pattern::CheckerJoinSidechain,
            chain_id:   ChainId::default(),
            checker_id: 0,
        }
    }
}

impl FromRaw for CheckerJoinSidechainWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerJoinSidechainWitness> {
        if witness_raw_data.len() < CHECKER_JOIN_SIDECHAIN_WITNESS_LEN {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = ChainId::from_raw(&witness_raw_data[1..5])?;
        let checker_id = u8::from_raw(&witness_raw_data[5..6])?;

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

        buf[0..1].copy_from_slice(&self.pattern.serialize());
        buf[1..5].copy_from_slice(&self.chain_id.serialize());
        buf[5..6].copy_from_slice(&self.checker_id.serialize());

        buf
    }
}
