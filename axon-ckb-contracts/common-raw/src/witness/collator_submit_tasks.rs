use molecule::prelude::*;

use crate::{
    common::*,
    molecule::{
        common::{
            ChainIdReader, CommittedHashOptBuilder, CommittedHashReader, PubKeyHashReader, RandomSeedReader, Uint128Reader,
            Uint32OptBuilder, Uint32Reader,
        },
        witness::collator_submit_tasks::{
            CollatorSubmitTasksWitnessBuilder, CollatorSubmitTasksWitnessReader, ExistedCommittedCheckerInfoBuilder,
            ExistedCommittedCheckerInfoReader, ExistedCommittedCheckerInfosBuilder,
        },
    },
    pattern::Pattern,
    FromRaw, Serialize,
};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct ExistedCommittedCheckerInfo {
    pub index:                 Option<usize>,
    pub checker_lock_arg:      PubKeyHash,
    pub origin_committed_hash: Option<CommittedHash>,
    pub new_committed_hash:    Option<CommittedHash>,
}

impl ExistedCommittedCheckerInfo {
    fn from_reader(reader: ExistedCommittedCheckerInfoReader) -> Option<Self> {
        let index = reader
            .index()
            .to_opt()
            .map_or(None, |index_reader| Some(u32::from_raw(index_reader.raw_data())? as usize));

        let mut checker_lock_arg: PubKeyHash = PubKeyHash::default();
        checker_lock_arg.copy_from_slice(reader.checker_lock_arg().raw_data());

        let origin_committed_hash = reader.origin_committed_hash().to_opt().map(|origin_committed_hash_reader| {
            let mut buf: CommittedHash = CommittedHash::default();
            buf.copy_from_slice(origin_committed_hash_reader.raw_data());
            buf
        });

        let new_committed_hash = reader.new_committed_hash().to_opt().map(|new_committed_hash_reader| {
            let mut buf: CommittedHash = CommittedHash::default();
            buf.copy_from_slice(new_committed_hash_reader.raw_data());
            buf
        });

        Some(Self {
            index,
            checker_lock_arg,
            origin_committed_hash,
            new_committed_hash,
        })
    }

    fn as_builder(&self) -> ExistedCommittedCheckerInfoBuilder {
        let index = Uint32OptBuilder::default()
            .set(
                self.index
                    .map(|value| Uint32Reader::new_unchecked(&(value as u32).serialize()).to_entity()),
            )
            .build();
        let checker_lock_arg = PubKeyHashReader::new_unchecked(&self.checker_lock_arg).to_entity();
        let origin_committed_hash = CommittedHashOptBuilder::default()
            .set(
                self.origin_committed_hash
                    .map(|value| CommittedHashReader::new_unchecked(&value).to_entity()),
            )
            .build();
        let new_committed_hash = CommittedHashOptBuilder::default()
            .set(
                self.new_committed_hash
                    .map(|value| CommittedHashReader::new_unchecked(&value).to_entity()),
            )
            .build();

        ExistedCommittedCheckerInfoBuilder::default()
            .index(index)
            .checker_lock_arg(checker_lock_arg)
            .origin_committed_hash(origin_committed_hash)
            .new_committed_hash(new_committed_hash)
    }

    pub fn is_valid_existed_checker(&self) -> bool {
        self.index.is_some() && self.new_committed_hash.is_some()
    }

    pub fn is_invalid_existed_checker(&self) -> bool {
        self.index.is_some() && self.new_committed_hash.is_none()
    }

    pub fn is_new_checker(&self) -> bool {
        self.index.is_none()
    }
}

impl FromRaw for ExistedCommittedCheckerInfo {
    fn from_raw(raw: &[u8]) -> Option<Self> {
        let reader = ExistedCommittedCheckerInfoReader::from_slice(raw).ok()?;

        Self::from_reader(reader)
    }
}

impl Serialize for ExistedCommittedCheckerInfo {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let builder = self.as_builder();

        let mut buf = Vec::new();
        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing ExistedCommittedCheckerInfo");
        buf
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct CollatorSubmitTasksWitness {
    pattern:                Pattern,
    pub chain_id:           u8, // TODO: Change to ChainId
    pub commit:             Vec<ExistedCommittedCheckerInfo>,
    pub origin_random_seed: RandomSeed,
    pub new_random_seed:    RandomSeed,
    pub fee:                u128,
    pub fee_per_checker:    u128,
}

impl Default for CollatorSubmitTasksWitness {
    fn default() -> Self {
        Self {
            pattern:            Pattern::CollatorSubmitTasks,
            chain_id:           0,
            commit:             Vec::default(),
            origin_random_seed: RandomSeed::default(),
            new_random_seed:    RandomSeed::default(),
            fee:                0,
            fee_per_checker:    0,
        }
    }
}

impl FromRaw for CollatorSubmitTasksWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CollatorSubmitTasksWitness> {
        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;

        let reader = CollatorSubmitTasksWitnessReader::from_slice(&witness_raw_data[1..]).ok()?;

        let chain_id = ChainId::from_raw(reader.chain_id().raw_data())? as u8;

        let commit = reader
            .commit()
            .iter()
            .map(|commit_reader| ExistedCommittedCheckerInfo::from_reader(commit_reader).ok_or(()))
            .collect::<Result<Vec<ExistedCommittedCheckerInfo>, ()>>()
            .ok()?;

        let mut origin_random_seed = RandomSeed::default();
        origin_random_seed.copy_from_slice(reader.origin_random_seed().raw_data());

        let mut new_random_seed = RandomSeed::default();
        new_random_seed.copy_from_slice(reader.new_random_seed().raw_data());

        let fee = u128::from_raw(reader.fee().raw_data())?;
        let fee_per_checker = u128::from_raw(reader.fee_per_checker().raw_data())?;

        Some(CollatorSubmitTasksWitness {
            pattern,
            chain_id,
            commit,
            origin_random_seed,
            new_random_seed,
            fee,
            fee_per_checker,
        })
    }
}

impl Serialize for CollatorSubmitTasksWitness {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let chain_id = ChainIdReader::new_unchecked(&(self.chain_id as u32).serialize()).to_entity();

        let mut commit = ExistedCommittedCheckerInfosBuilder::default();
        for info in &self.commit {
            commit = commit.push(info.as_builder().build());
        }

        let origin_random_seed = RandomSeedReader::new_unchecked(&self.origin_random_seed).to_entity();
        let new_random_seed = RandomSeedReader::new_unchecked(&self.new_random_seed).to_entity();

        let fee = Uint128Reader::new_unchecked(&self.fee.serialize()).to_entity();
        let fee_per_checker = Uint128Reader::new_unchecked(&self.fee_per_checker.serialize()).to_entity();

        let builder = CollatorSubmitTasksWitnessBuilder::default()
            .chain_id(chain_id)
            .commit(commit.build())
            .origin_random_seed(origin_random_seed)
            .new_random_seed(new_random_seed)
            .fee(fee)
            .fee_per_checker(fee_per_checker);

        let mut buf = Vec::new();
        buf.extend_from_slice(&self.pattern.serialize());

        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing CollatorSubmitTasksWitness");

        buf
    }
}
