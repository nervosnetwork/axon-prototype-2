use crate::{pattern::Pattern, FromRaw};

#[derive(Debug)]
pub struct CollatorSubmitChallengeWitness {
    pattern:                   Pattern,
    pub chain_id:              u8,
    pub fee:                   u128,
    pub fee_per_checker:       u128,
    pub punish_checker_bitmap: [u8; 32],
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

        Some(CollatorSubmitChallengeWitness {
            pattern,
            chain_id,
            fee,
            fee_per_checker,
            punish_checker_bitmap,
        })
    }
}
