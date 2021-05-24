use core::convert::{TryFrom, TryInto};
use core::result::Result;

use ckb_std::error::SysError;

use crate::{check_args_len, decode_i8, decode_u128, decode_u16, decode_u64, decode_u8};

// in byte
const SUDT_DATA_LEN: usize = 16; // u128
const CHECKER_BOND_CELL_LOCK_ARGS_LEN: usize = 64;
const SIDECHAIN_CONFIG_CELL_LEN: usize = 89;
const SIDECHAIN_STATE_CELL_LEN: usize = 98;
const CHECKER_INFO_CELL_LEN: usize = 563;
const TASK_CELL_LEN: usize = 69;

pub trait FromRaw {
    fn from_raw(cell_raw_data: &[u8]) -> Result<Self, SysError>
    where
        Self: Sized;
}

// which is standard sudt
#[derive(Debug)]
pub struct MuseTokenData {
    pub amount: u128,
}

impl FromRaw for MuseTokenData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<MuseTokenData, SysError> {
        check_args_len(cell_raw_data.len(), SUDT_DATA_LEN)?;

        let sudt_amount = decode_u128(&cell_raw_data[..16])?;

        Ok(MuseTokenData {
            amount: sudt_amount,
        })
    }
}

// which is standard sudt
#[derive(Debug)]
pub struct CheckerBondCellData {
    pub amount: u128,
}

impl FromRaw for CheckerBondCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<CheckerBondCellData, SysError> {
        check_args_len(cell_raw_data.len(), SUDT_DATA_LEN)?;

        let sudt_amount = decode_u128(&cell_raw_data[..16])?;

        Ok(CheckerBondCellData {
            amount: sudt_amount,
        })
    }
}

#[derive(Debug)]
pub struct CheckerBondCellLockArgs {
    pub checker_address: [u8; 32],
    pub chain_id_bitmap: [u8; 32],
}

impl FromRaw for CheckerBondCellLockArgs {
    fn from_raw(cell_raw_data: &[u8]) -> Result<CheckerBondCellLockArgs, SysError> {
        check_args_len(cell_raw_data.len(), CHECKER_BOND_CELL_LOCK_ARGS_LEN)?;

        let mut checker_address = [0u8; 32];
        checker_address.copy_from_slice(&cell_raw_data[0..32]);

        let mut chain_id_bitmap = [0u8; 32];
        chain_id_bitmap.copy_from_slice(&cell_raw_data[32..64]);

        Ok(CheckerBondCellLockArgs {
            checker_address,
            chain_id_bitmap,
        })
    }
}

#[derive(Debug)]
pub struct SidechainConfigCellData {
    pub chain_id:                u8,
    pub checker_total_count:     u8,
    // 2**8 = 256
    pub checker_bitmap:          [u8; 32],
    // 256
    pub checker_threshold:       u8,
    pub update_interval:         u16,
    pub minimal_bond:            u128,
    pub checker_data_size_limit: u128,
    pub checker_price:           u128,
    pub refresh_interval:        u16,
    pub commit_threshold:        u8,
    pub challenge_threshold:     u8,
}

impl FromRaw for SidechainConfigCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<SidechainConfigCellData, SysError> {
        check_args_len(cell_raw_data.len(), SIDECHAIN_CONFIG_CELL_LEN)?;

        let chain_id = decode_u8(&cell_raw_data[0..1])?;
        let checker_total_count = decode_u8(&cell_raw_data[1..2])?;

        let mut checker_bitmap = [0u8; 32];
        checker_bitmap.copy_from_slice(&cell_raw_data[2..34]);

        let checker_threshold = decode_u8(&cell_raw_data[34..35])?;
        let update_interval = decode_u16(&cell_raw_data[35..37])?;
        let minimal_bond = decode_u128(&cell_raw_data[37..53])?;
        let checker_data_size_limit = decode_u128(&cell_raw_data[53..69])?;
        let checker_price = decode_u128(&cell_raw_data[69..85])?;
        let refresh_interval = decode_u16(&cell_raw_data[85..87])?;
        let commit_threshold = decode_u8(&cell_raw_data[87..88])?;
        let challenge_threshold = decode_u8(&cell_raw_data[88..89])?;

        Ok(SidechainConfigCellData {
            chain_id,
            checker_total_count,
            checker_bitmap,
            checker_threshold,
            update_interval,
            minimal_bond,
            checker_data_size_limit,
            checker_price,
            refresh_interval,
            commit_threshold,
            challenge_threshold,
        })
    }
}

#[derive(Debug)]
pub struct SidechainStateCellData {
    pub chain_id:               u8,
    pub version:                u8,
    pub latest_block_height:    u128,
    pub latest_block_hash:      [u8; 32],
    pub committed_block_height: u128,
    pub committed_block_hash:   [u8; 32],
}

impl FromRaw for SidechainStateCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<SidechainStateCellData, SysError> {
        check_args_len(cell_raw_data.len(), SIDECHAIN_STATE_CELL_LEN)?;

        let chain_id = decode_u8(&cell_raw_data[0..1])?;
        let version = decode_u8(&cell_raw_data[1..2])?;

        let latest_block_height = decode_u128(&cell_raw_data[2..18])?;
        let mut latest_block_hash = [0u8; 32];
        latest_block_hash.copy_from_slice(&cell_raw_data[18..50]);

        let committed_block_height = decode_u128(&cell_raw_data[50..66])?;
        let mut committed_block_hash = [0u8; 32];
        committed_block_hash.copy_from_slice(&cell_raw_data[66..98]);

        Ok(SidechainStateCellData {
            chain_id,
            version,
            latest_block_height,
            latest_block_hash,
            committed_block_height,
            committed_block_hash,
        })
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum CheckerInfoCellMode {
    Idle = 0u8,
    TaskPassed,
    ChallengePassed,
    ChallengeRejected,
}

impl TryFrom<u8> for CheckerInfoCellMode {
    type Error = SysError;

    fn try_from(mode: u8) -> Result<Self, Self::Error> {
        match mode {
            0u8 => Ok(Self::Idle),
            1u8 => Ok(Self::TaskPassed),
            2u8 => Ok(Self::ChallengePassed),
            3u8 => Ok(Self::ChallengeRejected),
            _ => Err(SysError::IndexOutOfBound),
        }
    }
}

#[derive(Debug)]
pub struct CheckerInfoCellData {
    pub chain_id:           u8,
    pub checker_id:         u8,
    pub unpaid_fee:         u128,
    pub rpc_url:            [u8; 512],
    pub checker_public_key: [u8; 32],
    pub mode:               CheckerInfoCellMode,
}

impl FromRaw for CheckerInfoCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<CheckerInfoCellData, SysError> {
        check_args_len(cell_raw_data.len(), CHECKER_INFO_CELL_LEN)?;

        let chain_id = decode_u8(&cell_raw_data[0..1])?;
        let checker_id = decode_u8(&cell_raw_data[1..2])?;
        let unpaid_fee = decode_u128(&cell_raw_data[2..18])?;

        let mut rpc_url = [0u8; 512];
        rpc_url.copy_from_slice(&cell_raw_data[18..530]);

        let mut checker_public_key = [0u8; 32];
        checker_public_key.copy_from_slice(&cell_raw_data[530..562]);

        let mode_u8 = decode_u8(&cell_raw_data[562..563])?;

        let mode: CheckerInfoCellMode = mode_u8.try_into()?;

        Ok(CheckerInfoCellData {
            chain_id,
            checker_id,
            unpaid_fee,
            rpc_url,
            checker_public_key,
            mode,
        })
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum TaskCellMode {
    Task = 0,
    Challenge,
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

#[derive(Debug)]
pub struct TaskCellData {
    pub chain_id:                u8,
    pub version:                 u8,
    pub check_block_height_from: u128, // 应该为ssc committed_height + 1
    pub check_block_height_to:   u128, // inclusive 应该为latest_height
    pub check_block_hash_to:     u128,
    pub check_data_size:         u128,
    pub refresh_interval:        u16,
    pub mode:                    TaskCellMode,
}

impl FromRaw for TaskCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<TaskCellData, SysError> {
        check_args_len(cell_raw_data.len(), CHECKER_INFO_CELL_LEN)?;

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

// which is standard sudt
#[derive(Debug)]
pub struct SidechainBondCellData {
    amount: u128,
}

// which is standard sudt
#[derive(Debug)]
pub struct SidechainFeeCellData {
    amount: u128,
}
