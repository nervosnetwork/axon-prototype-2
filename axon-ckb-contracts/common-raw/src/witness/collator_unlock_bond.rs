use crate::{pattern::Pattern, FromRaw, Serialize};

const COLLATOR_UNLOCK_BOND_WITNESS_LEN: usize = 4;

#[derive(Debug)]
pub struct CollatorUnlockBondWitness {
    pattern: Pattern,
    pub chain_id: u8,
    pub sidechain_state_dep_index: usize,
}

impl Default for CollatorUnlockBondWitness {
    fn default() -> Self {
        Self {
            pattern:                   Pattern::CollatorUnlockBond,
            chain_id:                  0,
            sidechain_state_dep_index: 0,
        }
    }
}

impl FromRaw for CollatorUnlockBondWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CollatorUnlockBondWitness> {
        if witness_raw_data.len() < COLLATOR_UNLOCK_BOND_WITNESS_LEN {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let sidechain_state_dep_index = usize::from_raw(&witness_raw_data[2..4])?;

        Some(CollatorUnlockBondWitness {
            pattern,
            chain_id,
            sidechain_state_dep_index,
        })
    }
}

impl Serialize for CollatorUnlockBondWitness {
    type RawType = [u8; COLLATOR_UNLOCK_BOND_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; COLLATOR_UNLOCK_BOND_WITNESS_LEN];

        buf[0..1].copy_from_slice(&self.pattern.serialize());
        buf[1..2].copy_from_slice(&self.chain_id.serialize());

        buf[2..4].copy_from_slice(&self.sidechain_state_dep_index.serialize());

        buf
    }
}
