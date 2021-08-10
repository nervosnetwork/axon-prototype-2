use molecule::prelude::*;

use crate::{
    molecule::witness::collator_submit_tasks::{CollatorSubmitTasksWitnessBuilder, CollatorSubmitTasksWitnessReader},
    pattern::Pattern,
    witness::common_submit_jobs::CommonSubmitJobsWitness,
    FromRaw, Serialize,
};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct CollatorSubmitTasksWitness {
    pattern:    Pattern,
    pub common: CommonSubmitJobsWitness,
}

impl Default for CollatorSubmitTasksWitness {
    fn default() -> Self {
        Self {
            pattern: Pattern::CollatorSubmitTasks,
            common:  CommonSubmitJobsWitness::default(),
        }
    }
}

impl FromRaw for CollatorSubmitTasksWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CollatorSubmitTasksWitness> {
        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;

        let reader = CollatorSubmitTasksWitnessReader::from_slice(&witness_raw_data[1..]).ok()?;

        let common = CommonSubmitJobsWitness::from_reader(reader.common())?;

        Some(CollatorSubmitTasksWitness { pattern, common })
    }
}

impl Serialize for CollatorSubmitTasksWitness {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let builder = CollatorSubmitTasksWitnessBuilder::default().common(self.common.as_builder().build());

        let mut buf = Vec::new();
        buf.extend_from_slice(&self.pattern.serialize());

        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing CollatorSubmitTasksWitness");

        buf
    }
}
