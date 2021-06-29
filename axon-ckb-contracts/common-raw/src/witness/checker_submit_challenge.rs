use crate::{pattern::Pattern, FromRaw, Serialize};

const CHECKER_SUBMIT_CHALLENGE_WITNESS_LEN: usize = 3;

#[derive(Debug)]
pub struct CheckerSubmitChallengeWitness {
    pattern:        Pattern,
    pub chain_id:   u8,
    pub checker_id: u8,
}

impl Default for CheckerSubmitChallengeWitness {
    fn default() -> Self {
        Self {
            pattern:    Pattern::CheckerSubmitChallenge,
            chain_id:   0,
            checker_id: 0,
        }
    }
}

impl FromRaw for CheckerSubmitChallengeWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerSubmitChallengeWitness> {
        if witness_raw_data.len() < CHECKER_SUBMIT_CHALLENGE_WITNESS_LEN {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let checker_id = u8::from_raw(&witness_raw_data[2..3])?;

        Some(CheckerSubmitChallengeWitness {
            pattern,
            chain_id,
            checker_id,
        })
    }
}

impl Serialize for CheckerSubmitChallengeWitness {
    type RawType = [u8; CHECKER_SUBMIT_CHALLENGE_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; CHECKER_SUBMIT_CHALLENGE_WITNESS_LEN];

        buf[0..1].copy_from_slice(&self.pattern.serialize());
        buf[1..2].copy_from_slice(&self.chain_id.serialize());
        buf[2..3].copy_from_slice(&self.checker_id.serialize());

        buf
    }
}
