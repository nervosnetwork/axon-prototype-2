use molecule::prelude::*;

use crate::{
    molecule::{
        common::Uint8Reader,
        witness::collator_shutdown_sidechain::{CollatorShutDownSidechainWitnessBuilder, CollatorShutDownSidechainWitnessReader},
    },
    pattern::Pattern,
    FromRaw, Serialize,
};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct CollatorShutdownSidechainWitness {
    pattern:      Pattern,
    pub chain_id: u8,
}

impl Default for CollatorShutdownSidechainWitness {
    fn default() -> Self {
        CollatorShutdownSidechainWitness {
            pattern:  Pattern::CollatorShutdownSidechain,
            chain_id: 0u8,
        }
    }
}

impl FromRaw for CollatorShutdownSidechainWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<Self> {
        let reader = CollatorShutDownSidechainWitnessReader::from_slice(witness_raw_data).ok()?;
        let pattern = Pattern::from_raw(reader.pattern().raw_data())?;
        let chain_id = u8::from_raw(reader.chain_id().raw_data())?;

        Some(CollatorShutdownSidechainWitness { pattern, chain_id })
    }
}

impl Serialize for CollatorShutdownSidechainWitness {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let builder = CollatorShutDownSidechainWitnessBuilder::default();
        let pattern = Uint8Reader::new_unchecked(&self.pattern.serialize()).to_entity();
        let chain_id = Uint8Reader::new_unchecked(&self.chain_id.serialize()).to_entity();

        let builder = builder.pattern(pattern).chain_id(chain_id);

        let mut buf = Vec::new();
        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing CollatorShutdownSidechainWitness");

        buf
    }
}
