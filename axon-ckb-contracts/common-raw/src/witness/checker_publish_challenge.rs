use crate::{pattern::Pattern, FromRaw, Serialize};

const CHECKER_PUBLISH_CHALLENGE_WITNESS_LEN: usize = 6;

#[derive(Debug)]
pub struct CheckerPublishChallengeWitness {
    pattern: Pattern,
    pub chain_id: u8,
    pub checker_id: u8,
    pub challenge_count: u8,
    pub sidechain_config_dep_index: usize,
}

impl Default for CheckerPublishChallengeWitness {
    fn default() -> Self {
        Self {
            pattern:                    Pattern::CheckerPublishChallenge,
            chain_id:                   0,
            checker_id:                 0,
            challenge_count:            0,
            sidechain_config_dep_index: 0,
        }
    }
}

impl FromRaw for CheckerPublishChallengeWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerPublishChallengeWitness> {
        if witness_raw_data.len() < CHECKER_PUBLISH_CHALLENGE_WITNESS_LEN {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let checker_id = u8::from_raw(&witness_raw_data[2..3])?;
        let challenge_count = u8::from_raw(&witness_raw_data[3..4])?;
        let sidechain_config_dep_index = usize::from_raw(&witness_raw_data[4..6])?;

        Some(CheckerPublishChallengeWitness {
            pattern,
            chain_id,
            checker_id,
            challenge_count,
            sidechain_config_dep_index,
        })
    }
}

impl Serialize for CheckerPublishChallengeWitness {
    type RawType = [u8; CHECKER_PUBLISH_CHALLENGE_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; CHECKER_PUBLISH_CHALLENGE_WITNESS_LEN];

        buf[0..1].copy_from_slice(&self.pattern.serialize());
        buf[1..2].copy_from_slice(&self.chain_id.serialize());
        buf[2..3].copy_from_slice(&self.checker_id.serialize());
        buf[3..4].copy_from_slice(&self.challenge_count.serialize());
        buf[4..6].copy_from_slice(&self.sidechain_config_dep_index.serialize());

        buf
    }
}
