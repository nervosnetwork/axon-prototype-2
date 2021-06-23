use crate::{pattern::Pattern, FromRaw, Serialize};

const COLLATOR_PUBLISH_TASK_WITNESS_LEN: usize = 18;

#[derive(Debug)]
pub struct CollatorPublishTaskWitness {
    pattern:      Pattern,
    pub chain_id: u8,
    pub bond:     u128,
}

impl Default for CollatorPublishTaskWitness {
    fn default() -> Self {
        Self {
            pattern:  Pattern::CollatorPublishTask,
            chain_id: 0,
            bond:     0,
        }
    }
}

impl FromRaw for CollatorPublishTaskWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CollatorPublishTaskWitness> {
        if witness_raw_data.len() < 2 {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let bond = u128::from_raw(&witness_raw_data[2..18])?;

        Some(CollatorPublishTaskWitness { pattern, chain_id, bond })
    }
}

impl Serialize for CollatorPublishTaskWitness {
    type RawType = [u8; COLLATOR_PUBLISH_TASK_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; COLLATOR_PUBLISH_TASK_WITNESS_LEN];
        buf[0..1].copy_from_slice(&self.pattern.serialize());
        buf[1..2].copy_from_slice(&self.chain_id.serialize());
        buf[2..18].copy_from_slice(&self.bond.serialize());
        buf
    }
}
