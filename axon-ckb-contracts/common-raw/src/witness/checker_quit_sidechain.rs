use crate::{common::ChainId, pattern::Pattern, FromRaw, Serialize};

const CHECKER_QUIT_SIDECHAIN_WITNESS_LEN: usize = 6;

#[derive(Debug)]
pub struct CheckerQuitSidechainWitness {
    pattern:        Pattern,
    pub chain_id:   ChainId,
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
        if witness_raw_data.len() != CHECKER_QUIT_SIDECHAIN_WITNESS_LEN {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = ChainId::from_raw(&witness_raw_data[1..5])?;
        let checker_id = u8::from_raw(&witness_raw_data[5..6])?;

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

        buf[0..1].copy_from_slice(&self.pattern.serialize());
        buf[1..5].copy_from_slice(&self.chain_id.serialize());
        buf[5..6].copy_from_slice(&self.checker_id.serialize());

        buf
    }
}
