use core::convert::TryInto;

use crate::{pattern::Pattern, FromRaw};

#[derive(Debug)]
pub struct AdminCreateSidechainWitness {
    pattern:      Pattern,
    pub chain_id: u8,
}

impl FromRaw for AdminCreateSidechainWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<AdminCreateSidechainWitness> {
        if witness_raw_data.len() < 2 {
            return None;
        }

        let pattern = u8::from_raw(&witness_raw_data[0..1])?.try_into().ok()?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;

        Some(AdminCreateSidechainWitness { pattern, chain_id })
    }
}
