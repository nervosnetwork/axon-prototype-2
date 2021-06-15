use core::convert::TryInto;

use crate::{decode_u128, decode_u8, pattern::Pattern, FromRaw};

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

        let pattern = decode_u8(&witness_raw_data[0..1])?.try_into().ok()?;
        let chain_id = decode_u8(&witness_raw_data[1..2])?;
        let fee = decode_u128(&witness_raw_data[2..18])?;
        let fee_per_checker = decode_u128(&witness_raw_data[18..34])?;

        Some(CollatorSubmitTaskWitness {
            pattern,
            chain_id,
            fee,
            fee_per_checker,
        })
    }
}
