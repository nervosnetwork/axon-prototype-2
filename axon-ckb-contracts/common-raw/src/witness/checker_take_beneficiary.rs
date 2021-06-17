use core::convert::TryInto;

use crate::{pattern::Pattern, FromRaw};

#[derive(Debug)]
pub struct CheckerTakeBeneficiaryWitness {
    pattern:        Pattern,
    pub chain_id:   u8,
    pub checker_id: u8,
    pub fee:        u128,
}

impl FromRaw for CheckerTakeBeneficiaryWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerTakeBeneficiaryWitness> {
        if witness_raw_data.len() < 4 {
            return None;
        }

        let pattern = u8::from_raw(&witness_raw_data[0..1])?.try_into().ok()?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let checker_id = u8::from_raw(&witness_raw_data[2..3])?;
        let fee = u128::from_raw(&witness_raw_data[3..19])?;

        Some(CheckerTakeBeneficiaryWitness {
            pattern,
            chain_id,
            checker_id,
            fee,
        })
    }
}
