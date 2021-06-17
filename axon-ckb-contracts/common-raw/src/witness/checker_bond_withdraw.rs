use core::convert::TryInto;

use crate::{decode_u8, pattern::Pattern, FromRaw};

#[derive(Debug)]
pub struct CheckerBondWithdrawWitness {
    pattern: Pattern,
}

impl FromRaw for CheckerBondWithdrawWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerBondWithdrawWitness> {
        if witness_raw_data.len() < 2 {
            return None;
        }

        let pattern = decode_u8(&witness_raw_data[0..1])?.try_into().ok()?;

        Some(CheckerBondWithdrawWitness { pattern })
    }
}
