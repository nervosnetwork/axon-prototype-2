use molecule::prelude::*;

use crate::{
    common::*,
    molecule::{
        common::{PubKeyHashListBuilder, PubKeyHashReader, Uint128Reader, Uint32Reader},
        witness::anyone_shutdown_sidechain::{AnyoneShutdownSidechainWitnessBuilder, AnyoneShutdownSidechainWitnessReader},
    },
    pattern::Pattern,
    FromRaw, Serialize,
};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct AnyoneShutdownSidechainWitness {
    pub pattern:         Pattern,
    pub challenge_times: usize,
    pub check_data_size: u128,
    pub jailed_checkers: Vec<PubKeyHash>,
}

impl Default for AnyoneShutdownSidechainWitness {
    fn default() -> Self {
        Self {
            pattern:         Pattern::AnyoneShutdownSidechain,
            challenge_times: 0,
            check_data_size: 0,
            jailed_checkers: Vec::new(),
        }
    }
}

impl FromRaw for AnyoneShutdownSidechainWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<Self> {
        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;

        let reader = AnyoneShutdownSidechainWitnessReader::from_slice(&witness_raw_data[1..]).ok()?;

        let challenge_times = u32::from_raw(reader.challenge_times().raw_data())? as usize; // TODO: Change to usize

        let check_data_size = u128::from_raw(reader.check_data_size().raw_data())?;

        let jailed_checkers = reader
            .jailed_checkers()
            .iter()
            .map(|checker_reader| PubKeyHash::from_raw(checker_reader.raw_data()))
            .collect::<Option<Vec<PubKeyHash>>>()?;

        Some(Self {
            pattern,
            challenge_times,
            check_data_size,
            jailed_checkers,
        })
    }
}

impl Serialize for AnyoneShutdownSidechainWitness {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let challenge_times = Uint32Reader::new_unchecked(&(self.challenge_times as u32).serialize()).to_entity(); // TODO: Change to usize

        let check_data_size = Uint128Reader::new_unchecked(&self.check_data_size.serialize()).to_entity();

        let mut jailed_checkers = PubKeyHashListBuilder::default();
        for checker in &self.jailed_checkers {
            let checker_entity = PubKeyHashReader::new_unchecked(checker).to_entity();
            jailed_checkers = jailed_checkers.push(checker_entity);
        }

        let builder = AnyoneShutdownSidechainWitnessBuilder::default()
            .challenge_times(challenge_times)
            .check_data_size(check_data_size)
            .jailed_checkers(jailed_checkers.build());

        let mut buf = Vec::new();
        buf.extend_from_slice(&self.pattern.serialize());

        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing AnyoneShutdownSidechainWitness");

        buf
    }
}
