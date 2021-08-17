use crate::{pattern::Pattern, FromRaw, Serialize};

const COLLATOR_PUBLISH_TASK_WITNESS_LEN: usize = 50;

#[derive(Debug)]
pub struct CollatorPublishTaskWitness {
    pattern:             Pattern,
    pub chain_id:        u8,
    pub from_height:     u128,
    pub to_height:       u128,
    pub check_data_size: u128,
}

impl Default for CollatorPublishTaskWitness {
    fn default() -> Self {
        Self {
            pattern:         Pattern::CollatorPublishTask,
            chain_id:        0,
            from_height:     0,
            to_height:       0,
            check_data_size: 0,
        }
    }
}

impl FromRaw for CollatorPublishTaskWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CollatorPublishTaskWitness> {
        if witness_raw_data.len() != COLLATOR_PUBLISH_TASK_WITNESS_LEN {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let from_height = u128::from_raw(&witness_raw_data[2..18])?;
        let to_height = u128::from_raw(&witness_raw_data[18..34])?;
        let check_data_size = u128::from_raw(&witness_raw_data[34..50])?;

        Some(CollatorPublishTaskWitness {
            pattern,
            chain_id,
            from_height,
            to_height,
            check_data_size,
        })
    }
}

impl Serialize for CollatorPublishTaskWitness {
    type RawType = [u8; COLLATOR_PUBLISH_TASK_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; COLLATOR_PUBLISH_TASK_WITNESS_LEN];
        buf[0..1].copy_from_slice(&self.pattern.serialize());
        buf[1..2].copy_from_slice(&self.chain_id.serialize());
        buf[2..18].copy_from_slice(&self.from_height.serialize());
        buf[18..34].copy_from_slice(&self.to_height.serialize());
        buf[34..50].copy_from_slice(&self.check_data_size.serialize());
        buf
    }
}
