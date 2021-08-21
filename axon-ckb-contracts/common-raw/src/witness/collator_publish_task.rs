use crate::{
    common::{BlockHeight, ChainId},
    pattern::Pattern,
    FromRaw, Serialize,
};

const COLLATOR_PUBLISH_TASK_WITNESS_LEN: usize = 53;

#[derive(Debug)]
pub struct CollatorPublishTaskWitness {
    pattern:             Pattern,
    pub chain_id:        ChainId,
    pub from_height:     BlockHeight,
    pub to_height:       BlockHeight,
    pub check_data_size: u128,
}

impl Default for CollatorPublishTaskWitness {
    fn default() -> Self {
        Self {
            pattern:         Pattern::CollatorPublishTask,
            chain_id:        ChainId::default(),
            from_height:     BlockHeight::default(),
            to_height:       BlockHeight::default(),
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
        let chain_id = ChainId::from_raw(&witness_raw_data[1..5])?;
        let from_height = BlockHeight::from_raw(&witness_raw_data[5..21])?;
        let to_height = BlockHeight::from_raw(&witness_raw_data[21..37])?;
        let check_data_size = u128::from_raw(&witness_raw_data[37..53])?;

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
        buf[1..5].copy_from_slice(&self.chain_id.serialize());
        buf[5..21].copy_from_slice(&self.from_height.serialize());
        buf[21..37].copy_from_slice(&self.to_height.serialize());
        buf[37..53].copy_from_slice(&self.check_data_size.serialize());
        buf
    }
}
