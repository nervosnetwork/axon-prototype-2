use crate::{decode_u8, FromRaw};

#[derive(Debug)]
pub struct CollatorUnlockBondWitness {
    pub pattern:  u8,
    pub chain_id: u8,
}

impl FromRaw for CollatorUnlockBondWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CollatorUnlockBondWitness> {
        if witness_raw_data.len() < 2 {
            return None;
        }

        let pattern = decode_u8(&witness_raw_data[0..1])?;
        let chain_id = decode_u8(&witness_raw_data[1..2])?;

        Some(CollatorUnlockBondWitness { pattern, chain_id })
    }
}
