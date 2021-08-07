use core::default::Default;

use crate::{common::ChainId, pattern::Pattern, FromRaw, Serialize};

const COLLATOR_REFRESH_TASK_WITNESS_LEN: usize = 5;

#[derive(Debug)]
pub struct AnyoneRefreshTaskWitness {
    pattern:      Pattern,
    pub chain_id: ChainId,
}

impl Default for AnyoneRefreshTaskWitness {
    fn default() -> Self {
        Self {
            pattern:  Pattern::AnyoneRefreshTask,
            chain_id: 0,
        }
    }
}

impl FromRaw for AnyoneRefreshTaskWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<AnyoneRefreshTaskWitness> {
        if witness_raw_data.len() < COLLATOR_REFRESH_TASK_WITNESS_LEN {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = ChainId::from_raw(&witness_raw_data[1..5])?;
        Some(AnyoneRefreshTaskWitness { pattern, chain_id })
    }
}

impl Serialize for AnyoneRefreshTaskWitness {
    type RawType = [u8; COLLATOR_REFRESH_TASK_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; COLLATOR_REFRESH_TASK_WITNESS_LEN];

        buf[0..1].copy_from_slice(&self.pattern.serialize());
        buf[1..5].copy_from_slice(&self.chain_id.serialize());

        buf
    }
}
