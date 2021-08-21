use crate::{common::ChainId, pattern::Pattern, FromRaw};
const ADMIN_CREATE_SIDECHAIN_WITNESS_LENGTH: usize = 5;
#[derive(Debug)]
pub struct AdminCreateSidechainWitness {
    pattern:      Pattern,
    pub chain_id: ChainId,
}

impl FromRaw for AdminCreateSidechainWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<AdminCreateSidechainWitness> {
        if witness_raw_data.len() != ADMIN_CREATE_SIDECHAIN_WITNESS_LENGTH {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = ChainId::from_raw(&witness_raw_data[1..5])?;

        Some(AdminCreateSidechainWitness { pattern, chain_id })
    }
}
