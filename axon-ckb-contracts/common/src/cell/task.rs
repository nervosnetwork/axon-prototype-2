use core::convert::{TryFrom, TryInto};
use core::result::Result;

use ckb_std::error::SysError;

use crate::error::CommonError;
use crate::{
    check_args_len, decode_i8, decode_u128, decode_u16, decode_u64, decode_u8, FromRaw, GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH,
    SUDT_HASHTYPE, SUDT_MUSE_ARGS,
};
use alloc::vec::Vec;
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::prelude::{Entity, Unpack};
use ckb_std::high_level::{load_cell, load_cell_data, load_cell_type_hash};

const TASK_DATA_LEN: usize = 69;
const TASK_TYPE_ARGS_LEN: usize = 1;
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[repr(u8)]
pub enum TaskCellMode {
    Task = 0,
    Challenge,
}

impl Default for TaskCellMode {
    fn default() -> Self {
        TaskCellMode::Task
    }
}

impl TryFrom<u8> for TaskCellMode {
    type Error = SysError;

    fn try_from(mode: u8) -> Result<Self, Self::Error> {
        match mode {
            0u8 => Ok(Self::Task),
            1u8 => Ok(Self::Challenge),
            _ => Err(SysError::IndexOutOfBound),
        }
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
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct TaskCellData {
    pub chain_id:                u8,
    pub version:                 u8,
    pub check_block_height_from: u128,
    // 应该为ssc committed_height + 1
    pub check_block_height_to:   u128,
    // inclusive 应该为latest_height
    pub check_block_hash_to:     u128,
    pub check_data_size:         u128,
    pub refresh_interval:        u16,
    pub mode:                    TaskCellMode,
}

impl FromRaw for TaskCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<TaskCellData, SysError> {
        check_args_len(cell_raw_data.len(), TASK_DATA_LEN)?;

        let chain_id = decode_u8(&cell_raw_data[0..1])?;
        let version = decode_u8(&cell_raw_data[1..2])?;
        let check_block_height_from = decode_u128(&cell_raw_data[2..18])?;
        let check_block_height_to = decode_u128(&cell_raw_data[18..34])?;
        let check_block_hash_to = decode_u128(&cell_raw_data[34..50])?;
        let check_data_size = decode_u128(&cell_raw_data[50..66])?;
        let refresh_interval = decode_u16(&cell_raw_data[66..68])?;

        let mode_u8 = decode_u8(&cell_raw_data[68..69])?;
        let mode: TaskCellMode = mode_u8.try_into()?;

        Ok(TaskCellData {
            chain_id,
            version,
            check_block_height_from,
            check_block_height_to,
            check_block_hash_to,
            check_data_size,
            refresh_interval,
            mode,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct TaskCellTypeArgs {
    pub chain_id: u8,
}

impl FromRaw for TaskCellTypeArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Result<TaskCellTypeArgs, SysError> {
        check_args_len(arg_raw_data.len(), TASK_TYPE_ARGS_LEN)?;

        let chain_id = decode_u8(&arg_raw_data[0..1])?;

        Ok(TaskCellTypeArgs { chain_id })
    }
}
