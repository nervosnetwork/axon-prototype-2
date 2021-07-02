use crate::{pattern::Pattern, FromRaw, Serialize};
const COLLATOR_SUBMIT_TASK_WITNESS_LEN: usize = 34;

#[derive(Debug)]
pub struct CollatorSubmitTaskWitness {
    pattern:             Pattern,
    pub chain_id:        u8,
    pub fee:             u128,
    pub fee_per_checker: u128,
}

impl Default for CollatorSubmitTaskWitness {
    fn default() -> Self {
        Self {
            pattern:         Pattern::CollatorSubmitTask,
            chain_id:        0,
            fee:             0,
            fee_per_checker: 0,
        }
    }
}

impl FromRaw for CollatorSubmitTaskWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CollatorSubmitTaskWitness> {
        if witness_raw_data.len() < COLLATOR_SUBMIT_TASK_WITNESS_LEN {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let fee = u128::from_raw(&witness_raw_data[2..18])?;
        let fee_per_checker = u128::from_raw(&witness_raw_data[18..34])?;

        Some(CollatorSubmitTaskWitness {
            pattern,
            chain_id,
            fee,
            fee_per_checker,
        })
    }
}

impl Serialize for CollatorSubmitTaskWitness {
    type RawType = [u8; COLLATOR_SUBMIT_TASK_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; COLLATOR_SUBMIT_TASK_WITNESS_LEN];
        buf[0..1].copy_from_slice(&self.pattern.serialize());
        buf[1..2].copy_from_slice(&self.chain_id.serialize());
        buf[2..18].copy_from_slice(&self.fee.serialize());
        buf[18..34].copy_from_slice(&self.fee_per_checker.serialize());
        buf
    }
}
