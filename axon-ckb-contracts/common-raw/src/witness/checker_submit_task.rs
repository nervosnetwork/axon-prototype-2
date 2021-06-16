use core::convert::{TryFrom, TryInto};

use crate::{pattern::Pattern, FromRaw, Serialize};

const CHECKER_SUBMIT_TASK_WITNESS_LEN: usize = 5;

#[derive(Debug)]
pub struct CheckerSubmitTaskWitness {
    pattern: Pattern,
    pub chain_id: u8,
    pub checker_id: u8,
    pub sidechain_config_dep_index: usize,
}

impl Default for CheckerSubmitTaskWitness {
    fn default() -> Self {
        Self {
            pattern:                    Pattern::CheckerSubmitTask,
            chain_id:                   0,
            checker_id:                 0,
            sidechain_config_dep_index: 0,
        }
    }
}

impl FromRaw for CheckerSubmitTaskWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerSubmitTaskWitness> {
        if witness_raw_data.len() < CHECKER_SUBMIT_TASK_WITNESS_LEN {
            return None;
        }

        let pattern = u8::from_raw(&witness_raw_data[0..1])?.try_into().ok()?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let checker_id = u8::from_raw(&witness_raw_data[2..3])?;
        let sidechain_config_dep_index = u16::from_raw(&witness_raw_data[3..5])?.into();

        Some(CheckerSubmitTaskWitness {
            pattern,
            chain_id,
            checker_id,
            sidechain_config_dep_index,
        })
    }
}

impl Serialize for CheckerSubmitTaskWitness {
    type RawType = [u8; CHECKER_SUBMIT_TASK_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; CHECKER_SUBMIT_TASK_WITNESS_LEN];

        buf[0..1].copy_from_slice(&(self.pattern as u8).serialize());
        buf[1..2].copy_from_slice(&self.chain_id.serialize());
        buf[2..3].copy_from_slice(&self.checker_id.serialize());
        buf[3..5].copy_from_slice(&u16::try_from(self.sidechain_config_dep_index).unwrap().serialize());

        buf
    }
}
