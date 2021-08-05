use core::default::Default;

use crate::{pattern::Pattern, FromRaw, Serialize};

const COLLATOR_REFRESH_TASK_WITNESS_LEN: usize = 2;

#[derive(Debug)]
pub struct CollatorRefreshTaskWitness {
    pattern:      Pattern,
    pub chain_id: u8,
}

impl Default for CollatorRefreshTaskWitness {
    fn default() -> Self {
        Self {
            pattern:  Pattern::CollatorRefreshTask,
            chain_id: 0,
        }
    }
}

impl FromRaw for CollatorRefreshTaskWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CollatorRefreshTaskWitness> {
        if witness_raw_data.len() < COLLATOR_REFRESH_TASK_WITNESS_LEN {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;

        Some(CollatorRefreshTaskWitness { pattern, chain_id })
    }
}

impl Serialize for CollatorRefreshTaskWitness {
    type RawType = [u8; COLLATOR_REFRESH_TASK_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; COLLATOR_REFRESH_TASK_WITNESS_LEN];

        buf[0..1].copy_from_slice(&self.pattern.serialize());
        buf[1..2].copy_from_slice(&self.chain_id.serialize());

        buf
    }
}
