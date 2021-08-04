use crate::{pattern::Pattern, FromRaw, Serialize};

const CHECKER_VOTE_WITNESS_LEN: usize = 4;

#[derive(Debug)]
pub struct CheckerVoteWitness {
    pattern: Pattern,
    pub chain_id: u8,
    pub sidechain_config_dep_index: usize,
}

impl Default for CheckerVoteWitness {
    fn default() -> Self {
        Self {
            pattern:                    Pattern::CheckerVote,
            chain_id:                   0,
            sidechain_config_dep_index: 0,
        }
    }
}

impl FromRaw for CheckerVoteWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerVoteWitness> {
        if witness_raw_data.len() < CHECKER_VOTE_WITNESS_LEN {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let sidechain_config_dep_index = usize::from_raw(&witness_raw_data[2..4])?;

        Some(CheckerVoteWitness {
            pattern,
            chain_id,
            sidechain_config_dep_index,
        })
    }
}

impl Serialize for CheckerVoteWitness {
    type RawType = [u8; CHECKER_VOTE_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; CHECKER_VOTE_WITNESS_LEN];

        buf[0..1].copy_from_slice(&self.pattern.serialize());
        buf[1..2].copy_from_slice(&self.chain_id.serialize());
        buf[2..4].copy_from_slice(&self.sidechain_config_dep_index.serialize());

        buf
    }
}
