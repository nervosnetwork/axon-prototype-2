use crate::{pattern::Pattern, FromRaw};

#[derive(Debug)]
pub struct CollatorRefreshTaskWitness {
    pattern:      Pattern,
    pub chain_id: u8,
}

impl FromRaw for CollatorRefreshTaskWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CollatorRefreshTaskWitness> {
        if witness_raw_data.len() < 2 {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;

        Some(CollatorRefreshTaskWitness { pattern, chain_id })
    }
}
