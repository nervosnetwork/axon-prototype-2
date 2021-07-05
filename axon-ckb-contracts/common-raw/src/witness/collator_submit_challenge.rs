use crate::{pattern::Pattern, FromRaw, Serialize};

const COLLATOR_SUBMIT_FAILD_CHALLENGE_WITNESS_LEN: usize = 68;
#[derive(Debug)]
pub struct CollatorSubmitChallengeWitness {
    pub pattern:               Pattern,
    pub chain_id:              u8,
    pub fee:                   u128,
    pub fee_per_checker:       u128,
    pub punish_checker_bitmap: [u8; 32],
    pub task_count:            u8,
    pub valid_challenge_count: u8,
}

impl Default for CollatorSubmitChallengeWitness {
    fn default() -> Self {
        Self {
            pattern:               Pattern::CollatorSubmitFaildChallenge,
            chain_id:              0,
            fee:                   0,
            fee_per_checker:       0,
            punish_checker_bitmap: [0u8; 32],
            task_count:            0,
            valid_challenge_count: 0,
        }
    }
}

impl FromRaw for CollatorSubmitChallengeWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CollatorSubmitChallengeWitness> {
        if witness_raw_data.len() < 2 {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let fee = u128::from_raw(&witness_raw_data[2..18])?;
        let fee_per_checker = u128::from_raw(&witness_raw_data[18..34])?;

        let mut punish_checker_bitmap = [0u8; 32];
        punish_checker_bitmap.copy_from_slice(&witness_raw_data[34..66]);

        let task_count = u8::from_raw(&witness_raw_data[66..67])?;
        let valid_challenge_count = u8::from_raw(&witness_raw_data[67..68])?;
        Some(CollatorSubmitChallengeWitness {
            pattern,
            chain_id,
            fee,
            fee_per_checker,
            punish_checker_bitmap,
            task_count,
            valid_challenge_count,
        })
    }
}

impl Serialize for CollatorSubmitChallengeWitness {
    type RawType = [u8; COLLATOR_SUBMIT_FAILD_CHALLENGE_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; COLLATOR_SUBMIT_FAILD_CHALLENGE_WITNESS_LEN];

        buf[0..1].copy_from_slice(&self.pattern.serialize());
        buf[1..2].copy_from_slice(&self.chain_id.serialize());

        buf[2..18].copy_from_slice(&self.fee.serialize());
        buf[18..34].copy_from_slice(&self.fee_per_checker.serialize());
        buf[34..66].copy_from_slice(&self.punish_checker_bitmap);
        buf[66..67].copy_from_slice(&self.task_count.serialize());
        buf[67..68].copy_from_slice(&self.valid_challenge_count.serialize());

        buf
    }
}
