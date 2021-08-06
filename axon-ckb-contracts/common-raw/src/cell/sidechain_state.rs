use crate::{
    common::{BlockHeader, BlockHeight, BlockSlice, ChainId, CommittedHash, MerkleHash, PubKeyHash, RandomSeed},
    molecule::{
        cell::sidechain_state::{
            BlockHeadersBuilder, CheckerLastAcceptTaskHeightBuilder, CheckerLastAcceptTaskHeightReader,
            CheckerLastAcceptTaskHeightsBuilder, CommittedCheckerInfoBuilder, CommittedCheckerInfoReader, CommittedCheckerInfosBuilder,
            JobsBuilder, PunishedCheckerBuilder, PunishedCheckerReader, PunishedCheckersBuilder, SidechainStateCellBuilder,
            SidechainStateCellReader, SidechainStateCellTypeArgsBuilder, SidechainStateCellTypeArgsReader,
        },
        common::{
            BlockHeaderReader, BlockHeightReader, BlockSliceReader, ChainIdReader, CommittedHashReader, MerkleHashReader, PubKeyHashReader,
            RandomSeedReader, Uint32Reader, Uint8Reader,
        },
    },
    FromRaw, Serialize,
};
use molecule::prelude::*;
/**
    Sidechain State Cell
    Data:
    Type:
        codehash: typeId
        hashtype: type
        args: chain_id
    Lock:
        codehash: A.S.
        hashtype: type
        args: null
*/

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct CommittedCheckerInfo {
    checker_lock_arg: PubKeyHash,
    committed_hash:   CommittedHash,
}

impl CommittedCheckerInfo {
    fn from_reader(reader: CommittedCheckerInfoReader) -> Option<Self> {
        let mut checker_lock_arg = PubKeyHash::default();
        checker_lock_arg.copy_from_slice(reader.checker_lock_arg().raw_data());
        let mut committed_hash = CommittedHash::default();
        committed_hash.copy_from_slice(reader.committed_hash().raw_data());

        Some(Self {
            checker_lock_arg,
            committed_hash,
        })
    }
}

impl FromRaw for CommittedCheckerInfo {
    fn from_raw(raw_data: &[u8]) -> Option<Self> {
        let reader = CommittedCheckerInfoReader::from_slice(raw_data).ok()?;
        let mut checker_lock_arg = PubKeyHash::default();
        checker_lock_arg.copy_from_slice(reader.checker_lock_arg().raw_data());
        let mut committed_hash = CommittedHash::default();
        committed_hash.copy_from_slice(reader.committed_hash().raw_data());

        Some(Self {
            checker_lock_arg,
            committed_hash,
        })
    }
}

impl Serialize for CommittedCheckerInfo {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let checker_lock_arg = PubKeyHashReader::new_unchecked(&self.checker_lock_arg).to_entity();
        let committed_hash = CommittedHashReader::new_unchecked(&self.committed_hash).to_entity();

        let builder = CommittedCheckerInfoBuilder::default()
            .checker_lock_arg(checker_lock_arg)
            .committed_hash(committed_hash);
        let mut buf = Vec::new();
        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing sidechainState::CommittedCheckerInfo");
        buf
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct PunishedChecker {
    pub checker_lock_arg: PubKeyHash,
    pub punish_points:    u32,
}

impl PunishedChecker {
    fn from_reader(reader: PunishedCheckerReader) -> Option<Self> {
        let mut checker_lock_arg = PubKeyHash::default();
        checker_lock_arg.copy_from_slice(reader.checker_lock_arg().raw_data());
        let punish_points = u32::from_raw(reader.punish_points().raw_data())?;

        Some(Self {
            checker_lock_arg,
            punish_points,
        })
    }
}

impl FromRaw for PunishedChecker {
    fn from_raw(cell_raw_data: &[u8]) -> Option<Self> {
        let reader = PunishedCheckerReader::from_slice(cell_raw_data).ok()?;
        let mut checker_lock_arg = PubKeyHash::default();
        checker_lock_arg.copy_from_slice(reader.checker_lock_arg().raw_data());
        let punish_points = u32::from_raw(reader.punish_points().raw_data())?;

        Some(Self {
            checker_lock_arg,
            punish_points,
        })
    }
}

impl Serialize for PunishedChecker {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let mut buf = Vec::new();
        PunishedCheckerBuilder::default()
            .checker_lock_arg(PubKeyHashReader::new_unchecked(&self.checker_lock_arg).to_entity())
            .punish_points(Uint32Reader::new_unchecked(&self.punish_points.serialize()).to_entity())
            .write(&mut buf)
            .expect("Unable to write buffer while serializing sidechainState::PunishedChecker");
        buf
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct CheckerLastAcceptTaskHeight {
    checker_lock_arg: PubKeyHash,
    height:           BlockHeight,
}

impl CheckerLastAcceptTaskHeight {
    fn from_reader(reader: CheckerLastAcceptTaskHeightReader) -> Option<Self> {
        let mut checker_lock_arg = PubKeyHash::default();
        checker_lock_arg.copy_from_slice(reader.checker_lock_arg().raw_data());

        let height = BlockHeight::from_raw(reader.height().raw_data())?;

        Some(Self { checker_lock_arg, height })
    }
}

impl FromRaw for CheckerLastAcceptTaskHeight {
    fn from_raw(cell_raw_data: &[u8]) -> Option<Self> {
        let reader = CheckerLastAcceptTaskHeightReader::from_slice(cell_raw_data).ok()?;
        let mut checker_lock_arg = PubKeyHash::default();
        checker_lock_arg.copy_from_slice(reader.checker_lock_arg().raw_data());

        let height = BlockHeight::from_raw(reader.height().raw_data())?;

        Some(Self { checker_lock_arg, height })
    }
}

impl Serialize for CheckerLastAcceptTaskHeight {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let mut buf = Vec::new();
        CheckerLastAcceptTaskHeightBuilder::default()
            .checker_lock_arg(PubKeyHashReader::new_unchecked(&self.checker_lock_arg).to_entity())
            .height(BlockHeightReader::new_unchecked(&self.height.serialize()).to_entity())
            .write(&mut buf)
            .expect("Unable to write buffer while serializing sidechainState::CheckerLastAcceptTaskHeight");
        buf
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainStateCell {
    pub version: u8,
    pub submit_sidechain_block_height: BlockHeight,
    pub waiting_jobs: Vec<BlockSlice>,
    pub confirmed_jobs: Vec<BlockSlice>,
    pub random_seed: RandomSeed,
    pub random_offset: u8,
    pub random_commit: Vec<CommittedCheckerInfo>,
    pub punish_checkers: Vec<PunishedChecker>,
    pub recent_block_headers: Vec<BlockHeader>,
    pub ancient_block_heard_merkle_root: MerkleHash,
    pub checker_last_task_sidechain_heights: Vec<CheckerLastAcceptTaskHeight>,
}

impl FromRaw for SidechainStateCell {
    fn from_raw(cell_raw_data: &[u8]) -> Option<SidechainStateCell> {
        let reader = SidechainStateCellReader::from_slice(cell_raw_data).ok()?;

        let version = u8::from_raw(reader.version().raw_data())?;
        let submit_sidechain_block_height = BlockHeight::from_raw(reader.submit_sidechain_block_height().raw_data())?;
        let waiting_jobs: Vec<BlockSlice> = reader
            .waiting_jobs()
            .iter()
            .map(|reader| BlockSlice::from_raw(reader.as_slice()).ok_or(()))
            .collect::<Result<Vec<BlockSlice>, ()>>()
            .ok()?;

        let confirmed_jobs: Vec<BlockSlice> = reader
            .confirmed_jobs()
            .iter()
            .map(|reader| BlockSlice::from_raw(reader.as_slice()).ok_or(()))
            .collect::<Result<Vec<BlockSlice>, ()>>()
            .ok()?;

        let mut random_seed = RandomSeed::default();
        random_seed.copy_from_slice(reader.random_seed().raw_data());

        let mut buf = [0u8; 1];
        buf.copy_from_slice(reader.random_offset().raw_data());
        let random_offset = u8::from_raw(&buf)?;

        let random_commit = reader
            .random_commit()
            .iter()
            .map(|reader| CommittedCheckerInfo::from_reader(reader).ok_or(()))
            .collect::<Result<Vec<CommittedCheckerInfo>, ()>>()
            .ok()?;

        let punish_checkers = reader
            .punish_checkers()
            .iter()
            .map(|reader| PunishedChecker::from_reader(reader).ok_or(()))
            .collect::<Result<Vec<PunishedChecker>, ()>>()
            .ok()?;

        let recent_block_headers = reader
            .recent_block_headers()
            .iter()
            .map(|block_height_reader| {
                let mut buf = [0u8; 32];
                buf.copy_from_slice(block_height_reader.raw_data());
                buf
            })
            .collect::<Vec<BlockHeader>>();

        let mut ancient_block_heard_merkle_root = MerkleHash::default();
        ancient_block_heard_merkle_root.copy_from_slice(reader.ancient_block_heard_merkle_root().raw_data());

        let checker_last_task_sidechain_heights = reader
            .checker_last_task_sidechain_heights()
            .iter()
            .map(|reader| CheckerLastAcceptTaskHeight::from_reader(reader).ok_or(()))
            .collect::<Result<Vec<CheckerLastAcceptTaskHeight>, ()>>()
            .ok()?;

        Some(Self {
            version,
            submit_sidechain_block_height,
            waiting_jobs,
            confirmed_jobs,
            random_seed,
            random_offset,
            random_commit,
            punish_checkers,
            recent_block_headers,
            ancient_block_heard_merkle_root,
            checker_last_task_sidechain_heights,
        })
    }
}

impl Serialize for SidechainStateCell {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let version = Uint8Reader::new_unchecked(&self.version.serialize()).to_entity();

        let submit_sidechain_block_height = BlockHeightReader::new_unchecked(&self.submit_sidechain_block_height.serialize()).to_entity();

        let mut waiting_jobs_builder = JobsBuilder::default();
        for job in &self.waiting_jobs {
            waiting_jobs_builder = waiting_jobs_builder.push(BlockSliceReader::new_unchecked(&job.serialize()).to_entity())
        }
        let waiting_jobs = waiting_jobs_builder.build();

        let mut confirmed_jobs_builder = JobsBuilder::default();
        for job in self.confirmed_jobs.iter() {
            confirmed_jobs_builder = confirmed_jobs_builder.push(BlockSliceReader::new_unchecked(&job.serialize()).to_entity())
        }
        let confirmed_jobs = confirmed_jobs_builder.build();

        let random_seed = RandomSeedReader::new_unchecked(&self.random_seed).to_entity();

        let random_offset = Uint8Reader::new_unchecked(&self.random_offset.serialize()).to_entity();

        let mut random_commit_builder = CommittedCheckerInfosBuilder::default();
        for committed_checker_info in &self.random_commit {
            random_commit_builder =
                random_commit_builder.push(CommittedCheckerInfoReader::new_unchecked(&committed_checker_info.serialize()).to_entity());
        }
        let random_commit = random_commit_builder.build();

        let mut punish_checkers_builder = PunishedCheckersBuilder::default();
        for punish_checker in &self.punish_checkers {
            punish_checkers_builder =
                punish_checkers_builder.push(PunishedCheckerReader::new_unchecked(&punish_checker.serialize()).to_entity());
        }
        let punish_checkers = punish_checkers_builder.build();

        let mut recent_block_headers_builder = BlockHeadersBuilder::default();
        for recent_block_header in &self.recent_block_headers {
            recent_block_headers_builder =
                recent_block_headers_builder.push(BlockHeaderReader::new_unchecked(recent_block_header).to_entity());
        }
        let recent_block_headers = recent_block_headers_builder.build();

        let ancient_block_heard_merkle_root = MerkleHashReader::new_unchecked(&self.ancient_block_heard_merkle_root).to_entity();

        let mut checker_last_task_sidechain_heights_builder = CheckerLastAcceptTaskHeightsBuilder::default();
        for checker_last_task_sidechain_height in &self.checker_last_task_sidechain_heights {
            checker_last_task_sidechain_heights_builder = checker_last_task_sidechain_heights_builder
                .push(CheckerLastAcceptTaskHeightReader::new_unchecked(&checker_last_task_sidechain_height.serialize()).to_entity());
        }
        let checker_last_task_sidechain_heights = checker_last_task_sidechain_heights_builder.build();

        let mut buf = Vec::new();
        SidechainStateCellBuilder::default()
            .version(version)
            .submit_sidechain_block_height(submit_sidechain_block_height)
            .waiting_jobs(waiting_jobs)
            .confirmed_jobs(confirmed_jobs)
            .random_seed(random_seed)
            .random_offset(random_offset)
            .random_commit(random_commit)
            .punish_checkers(punish_checkers)
            .recent_block_headers(recent_block_headers)
            .ancient_block_heard_merkle_root(ancient_block_heard_merkle_root)
            .checker_last_task_sidechain_heights(checker_last_task_sidechain_heights)
            .write(&mut buf)
            .expect("Unable to write buffer while serializing sidechainState::SidechainStateCell");
        buf
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainStateCellTypeArgs {
    pub chain_id: ChainId,
}

impl FromRaw for SidechainStateCellTypeArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<SidechainStateCellTypeArgs> {
        let reader = SidechainStateCellTypeArgsReader::from_slice(arg_raw_data).ok()?;
        let chain_id = ChainId::from_raw(reader.chain_id().raw_data())?;
        Some(SidechainStateCellTypeArgs { chain_id })
    }
}

impl Serialize for SidechainStateCellTypeArgs {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let mut buf = Vec::new();
        SidechainStateCellTypeArgsBuilder::default()
            .chain_id(ChainIdReader::new_unchecked(&self.chain_id.serialize()).to_entity())
            .write(&mut buf)
            .expect("Unable to write buffer while serializing sidechainState::SidechainStateCellTypeArgs");
        buf
    }
}
