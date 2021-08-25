use molecule::prelude::*;

use crate::molecule::common::Uint64Reader;
use crate::{
    common::*,
    molecule::{
        cell::task::{
            SidechainBlockHeadersBuilder, TaskCellBuilder, TaskCellReader, TaskCellTypeArgsBuilder, TaskCellTypeArgsReader, TaskModeReader,
            TaskStatusReader,
        },
        common::{
            BlockHeaderReader, BlockHeightReader, ChainIdReader, CommittedHashReader, PubKeyHashReader, RandomSeedReader, Uint128Reader,
            Uint8Reader,
        },
    },
    FromRaw, Serialize,
};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[repr(u8)]
pub enum TaskMode {
    Task,
    Challenge,
}

impl TaskMode {
    fn from_reader(reader: TaskModeReader) -> Option<Self> {
        let mode = u8::from_raw(reader.raw_data())?;
        match mode {
            0u8 => Some(Self::Task),
            1u8 => Some(Self::Challenge),
            _ => None,
        }
    }
}

impl Default for TaskMode {
    fn default() -> Self {
        Self::Task
    }
}

impl FromRaw for TaskMode {
    fn from_raw(raw: &[u8]) -> Option<Self> {
        let reader = TaskModeReader::from_slice(raw).ok()?;
        Self::from_reader(reader)
    }
}

impl Serialize for TaskMode {
    type RawType = [u8; 1];

    fn serialize(&self) -> Self::RawType {
        (*self as u8).serialize()
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[repr(u8)]
pub enum TaskStatus {
    Idle,
    TaskPassed,
    ChallengePassed,
    ChallengeRejected,
}

impl TaskStatus {
    fn from_reader(reader: TaskStatusReader) -> Option<Self> {
        let status = u8::from_raw(reader.raw_data())?;
        match status {
            0u8 => Some(Self::Idle),
            1u8 => Some(Self::TaskPassed),
            2u8 => Some(Self::ChallengePassed),
            3u8 => Some(Self::ChallengeRejected),
            _ => None,
        }
    }
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Idle
    }
}

impl FromRaw for TaskStatus {
    fn from_raw(raw: &[u8]) -> Option<Self> {
        let reader = TaskStatusReader::from_slice(raw).ok()?;
        Self::from_reader(reader)
    }
}

impl Serialize for TaskStatus {
    type RawType = [u8; 1];

    fn serialize(&self) -> Self::RawType {
        (*self as u8).serialize()
    }
}

/**
    Task Cell
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
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct TaskCell {
    pub version: u8,
    pub sidechain_block_height_from: BlockHeight,
    pub sidechain_block_height_to: BlockHeight,
    pub refresh_timestamp: u64,
    pub check_data_size: u128,
    pub mode: TaskMode,
    pub status: TaskStatus,
    pub reveal: RandomSeed,
    pub commit: CommittedHash,
    pub sidechain_block_header: Vec<BlockHeader>,
}

impl FromRaw for TaskCell {
    fn from_raw(cell_raw_data: &[u8]) -> Option<TaskCell> {
        let reader = TaskCellReader::from_slice(cell_raw_data).ok()?;

        let version = u8::from_raw(reader.version().raw_data())?;
        let sidechain_block_height_from = u128::from_raw(reader.sidechain_block_height_from().raw_data())?;
        let sidechain_block_height_to = u128::from_raw(reader.sidechain_block_height_to().raw_data())?;
        let refresh_timestamp = u64::from_raw(reader.refresh_timestamp().raw_data())?;
        let check_data_size = u128::from_raw(reader.check_data_size().raw_data())?;
        let mode = TaskMode::from_reader(reader.mode())?;
        let status = TaskStatus::from_reader(reader.status())?;

        let mut reveal: RandomSeed = [0u8; 32];
        reveal.copy_from_slice(reader.reveal().raw_data());

        let mut commit: CommittedHash = [0u8; 32];
        commit.copy_from_slice(reader.commit().raw_data());

        let sidechain_block_header_reader = reader.sidechain_block_header();
        let sidechain_block_header_len = sidechain_block_header_reader.len();
        let mut sidechain_block_header: Vec<BlockHeader> = Vec::with_capacity(sidechain_block_header_len);

        let mut i = 0;
        sidechain_block_header.resize_with(sidechain_block_header_len, || {
            let mut result: BlockHeader = [0u8; 32];
            result.copy_from_slice(sidechain_block_header_reader.get_unchecked(i).raw_data());

            i += 1;
            result
        });

        Some(TaskCell {
            version,
            sidechain_block_height_from,
            sidechain_block_height_to,
            refresh_timestamp,
            check_data_size,
            mode,
            status,
            reveal,
            commit,
            sidechain_block_header,
        })
    }
}

impl Serialize for TaskCell {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let version = Uint8Reader::new_unchecked(&self.version.serialize()).to_entity();
        let sidechain_block_height_from = BlockHeightReader::new_unchecked(&self.sidechain_block_height_from.serialize()).to_entity();
        let sidechain_block_height_to = BlockHeightReader::new_unchecked(&self.sidechain_block_height_to.serialize()).to_entity();
        let refresh_timestamp = Uint64Reader::new_unchecked(&self.refresh_timestamp.serialize()).to_entity();
        let check_data_size = Uint128Reader::new_unchecked(&self.check_data_size.serialize()).to_entity();
        let mode = TaskModeReader::new_unchecked(&self.mode.serialize()).to_entity();
        let status = TaskStatusReader::new_unchecked(&self.status.serialize()).to_entity();
        let reveal = RandomSeedReader::new_unchecked(&self.reveal).to_entity();
        let commit = CommittedHashReader::new_unchecked(&self.commit).to_entity();

        let mut sidechain_block_header = SidechainBlockHeadersBuilder::default();
        for header in &self.sidechain_block_header {
            sidechain_block_header = sidechain_block_header.push(BlockHeaderReader::new_unchecked(header).to_entity());
        }

        let builder = TaskCellBuilder::default()
            .version(version)
            .sidechain_block_height_from(sidechain_block_height_from)
            .sidechain_block_height_to(sidechain_block_height_to)
            .refresh_timestamp(refresh_timestamp)
            .check_data_size(check_data_size)
            .mode(mode)
            .status(status)
            .reveal(reveal)
            .commit(commit)
            .sidechain_block_header(sidechain_block_header.build());

        let mut buf = Vec::new();
        builder.write(&mut buf).expect("Unable to write buffer while serializing TaskCell");
        buf
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct TaskCellTypeArgs {
    pub chain_id:         ChainId,
    pub checker_lock_arg: PubKeyHash,
}

impl FromRaw for TaskCellTypeArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<TaskCellTypeArgs> {
        let reader = TaskCellTypeArgsReader::from_slice(arg_raw_data).ok()?;

        let chain_id = ChainId::from_raw(reader.chain_id().raw_data())?;

        let mut checker_lock_arg: PubKeyHash = PubKeyHash::default();
        checker_lock_arg.copy_from_slice(reader.checker_lock_arg().raw_data());

        Some(TaskCellTypeArgs {
            chain_id,
            checker_lock_arg,
        })
    }
}

impl Serialize for TaskCellTypeArgs {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let chain_id = ChainIdReader::new_unchecked(&self.chain_id.serialize()).to_entity();
        let checker_lock_arg = PubKeyHashReader::new_unchecked(&self.checker_lock_arg).to_entity();

        let builder = TaskCellTypeArgsBuilder::default()
            .chain_id(chain_id)
            .checker_lock_arg(checker_lock_arg);

        let mut buf = Vec::new();
        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing TaskCellTypeArgs");
        buf
    }
}
