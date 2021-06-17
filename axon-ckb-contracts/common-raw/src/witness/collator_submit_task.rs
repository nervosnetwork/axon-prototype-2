use core::convert::TryInto;

use crate::{pattern::Pattern, FromRaw};

#[derive(Debug)]
pub struct CollatorSubmitTaskWitness {
    pattern:             Pattern,
    pub chain_id:        u8,
    pub fee:             u128,
    pub fee_per_checker: u128,
}

impl FromRaw for CollatorSubmitTaskWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CollatorSubmitTaskWitness> {
        if witness_raw_data.len() < 2 {
            return None;
        }

        let pattern = u8::from_raw(&witness_raw_data[0..1])?.try_into().ok()?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let fee = u128::from_raw(&witness_raw_data[2..18])?;
        let fee_per_checker = u128::from_raw(&witness_raw_data[18..34])?;

        Some(CollatorSubmitTaskWitness {
            pattern,
            chain_id,
            fee,
            fee_per_checker,
        })
    }
}
