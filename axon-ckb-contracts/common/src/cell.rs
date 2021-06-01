use core::convert::{TryFrom, TryInto};
use core::result::Result;

use ckb_std::error::SysError;

use crate::error::CommonError;
use crate::{
    check_args_len, decode_i8, decode_u128, decode_u16, decode_u64, decode_u8, GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH, SUDT_HASHTYPE,
    SUDT_MUSE_ARGS,
};
use alloc::vec::Vec;
use ckb_standalone_types::prelude::{Entity, Unpack};
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell, load_cell_data, load_cell_type_hash};

// in byte
const SUDT_DATA_LEN: usize = 16; // u128

const GLOBAL_CONFIG_DATA_LEN: usize = 296;

const CODE_TYPE_ARGS_LEN: usize = 33;
const CODE_TYPE_WITNESS_LEN_MIN: usize = 1;
const CODE_LOCK_WITNESS_LEN: usize = 33;

const CHECKER_BOND_LOCK_ARGS_LEN: usize = 64;

const SIDECHAIN_CONFIG_DATA_LEN: usize = 185;
const SIDECHAIN_CONFIG_TYPE_ARGS_LEN: usize = 1;

const SIDECHAIN_STATE_DATA_LEN: usize = 98;
const SIDECHAIN_STATE_TYPE_ARGS_LEN: usize = 1;

const CHECKER_INFO_DATA_LEN: usize = 563;
const CHECKER_INFO_TYPE_ARGS_LEN: usize = 33;

const TASK_DATA_LEN: usize = 69;
const TASK_TYPE_ARGS_LEN: usize = 1;

const SIDECHAIN_BOND_LOCK_ARGS_LEN: usize = 49;

const SIDECHAIN_FEE_LOCK_ARGS_LEN: usize = 1;

pub trait FromRaw {
    fn from_raw(cell_raw_data: &[u8]) -> Result<Self, SysError>
    where
        Self: Sized;
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum CellType {
    Unknown,
    Sudt,
    MuseToken,
    CheckerBond,
    CheckerInfo,
    SidechainConfig,
    SidechainState,
    Task,
    SidechainFee,
    SidechainBond,
    Code,
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

        Ok(MuseTokenData { amount: sudt_amount })
    }
}

/**

    Global config cell only contains data

    Global Config Cell
    Data:
    Type:
        codehash: typeid                // A.S.
        hashtype: type                  // data
        args: unique_id                 // null
    Lock:
        codehash: secp256k1
        args: admin
*/

#[derive(Debug)]
pub struct GlobalConfigCellData {
    pub admin_public_key:        [u8; 32], /* this is the authenticated admin for
                                            * sidechain config cell */
    pub code_cell_type_codehash: [u8; 32],
    pub code_cell_type_hashtype: u8,

    pub sidechain_config_cell_type_codehash: [u8; 32],
    pub sidechain_config_cell_type_hashtype: u8,

    pub sidechain_state_cell_type_codehash: [u8; 32],
    pub sidechain_state_cell_type_hashtype: u8,

    pub checker_info_cell_type_codehash: [u8; 32],
    pub checker_info_cell_type_hashtype: u8,

    pub checker_bond_cell_lock_codehash: [u8; 32],
    pub checker_bond_cell_lock_hashtype: u8,

    pub task_cell_type_codehash: [u8; 32],
    pub task_cell_type_hashtype: u8,

    pub sidechain_fee_cell_lock_codehash: [u8; 32],
    pub sidechain_fee_cell_lock_hashtype: u8,

    pub sidechain_bond_cell_lock_codehash: [u8; 32],
    pub sidechain_bond_cell_lock_hashtype: u8,
}

impl FromRaw for GlobalConfigCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<GlobalConfigCellData, SysError> {
        check_args_len(cell_raw_data.len(), GLOBAL_CONFIG_DATA_LEN)?;

        let mut admin_public_key = [0u8; 32];
        admin_public_key.copy_from_slice(&cell_raw_data[0..32]);

        let mut code_cell_type_codehash = [0u8; 32];
        code_cell_type_codehash.copy_from_slice(&cell_raw_data[32..64]);
        let code_cell_type_hashtype = decode_u8(&cell_raw_data[64..65])?;

        let mut sidechain_config_cell_type_codehash = [0u8; 32];
        sidechain_config_cell_type_codehash.copy_from_slice(&cell_raw_data[65..97]);
        let sidechain_config_cell_type_hashtype = decode_u8(&cell_raw_data[97..98])?;

        let mut sidechain_state_cell_type_codehash = [0u8; 32];
        sidechain_state_cell_type_codehash.copy_from_slice(&cell_raw_data[98..130]);
        let sidechain_state_cell_type_hashtype = decode_u8(&cell_raw_data[130..131])?;

        let mut checker_info_cell_type_codehash = [0u8; 32];
        checker_info_cell_type_codehash.copy_from_slice(&cell_raw_data[131..163]);
        let checker_info_cell_type_hashtype = decode_u8(&cell_raw_data[163..164])?;

        let mut checker_bond_cell_lock_codehash = [0u8; 32];
        checker_bond_cell_lock_codehash.copy_from_slice(&cell_raw_data[164..196]);
        let checker_bond_cell_lock_hashtype = decode_u8(&cell_raw_data[196..197])?;

        let mut task_cell_type_codehash = [0u8; 32];
        task_cell_type_codehash.copy_from_slice(&cell_raw_data[197..229]);
        let task_cell_type_hashtype = decode_u8(&cell_raw_data[229..230])?;

        let mut sidechain_fee_cell_lock_codehash = [0u8; 32];
        sidechain_fee_cell_lock_codehash.copy_from_slice(&cell_raw_data[230..262]);
        let sidechain_fee_cell_lock_hashtype = decode_u8(&cell_raw_data[262..263])?;

        let mut sidechain_bond_cell_lock_codehash = [0u8; 32];
        sidechain_bond_cell_lock_codehash.copy_from_slice(&cell_raw_data[263..295]);
        let sidechain_bond_cell_lock_hashtype = decode_u8(&cell_raw_data[295..296])?;

        Ok(GlobalConfigCellData {
            admin_public_key,
            code_cell_type_codehash,
            code_cell_type_hashtype,
            sidechain_config_cell_type_codehash,
            sidechain_config_cell_type_hashtype,
            sidechain_state_cell_type_codehash,
            sidechain_state_cell_type_hashtype,
            checker_info_cell_type_codehash,
            checker_info_cell_type_hashtype,
            checker_bond_cell_lock_codehash,
            checker_bond_cell_lock_hashtype,
            task_cell_type_codehash,
            task_cell_type_hashtype,
            sidechain_fee_cell_lock_codehash,
            sidechain_fee_cell_lock_hashtype,
            sidechain_bond_cell_lock_codehash,
            sidechain_bond_cell_lock_hashtype,
        })
    }
}

/*

    Code Cell
    Data: null
    Type:
        codehash: typeId
        hashtype: type
        args: chain_id | checker_public_key
    Lock:
        codehash: secp256k1
        hashtype: type
        args: public-key
*/

#[derive(Debug, Copy, Clone)]
pub struct CodeCellTypeArgs {
    pub chain_id:       u8,
    pub who_public_key: [u8; 32],
}

impl FromRaw for CodeCellTypeArgs {
    fn from_raw(cell_raw_data: &[u8]) -> Result<CodeCellTypeArgs, SysError> {
        check_args_len(cell_raw_data.len(), CODE_TYPE_ARGS_LEN)?;

        let chain_id = decode_u8(&cell_raw_data[0..1])?;

        let mut who_public_key = [0u8; 32];
        who_public_key.copy_from_slice(&cell_raw_data[1..33]);

        Ok(CodeCellTypeArgs { chain_id, who_public_key })
    }
}

#[derive(Debug)]
pub struct CodeCellTypeWitness {
    pub pattern: u8,
}

impl FromRaw for CodeCellTypeWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Result<CodeCellTypeWitness, SysError> {
        if witness_raw_data.len() < CODE_TYPE_WITNESS_LEN_MIN {
            return Err(SysError::Encoding);
        }

        let pattern = decode_u8(&witness_raw_data[0..1])?;

        Ok(CodeCellTypeWitness { pattern })
    }
}

#[derive(Debug)]
pub struct CodeCellLockArgs {
    pub public_key_hash: [u8; 20],
}

impl FromRaw for CodeCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Result<CodeCellLockArgs, SysError> {
        check_args_len(arg_raw_data.len(), CODE_LOCK_WITNESS_LEN)?;

        let mut public_key_hash = [0u8; 20];
        public_key_hash.copy_from_slice(&arg_raw_data[0..20]);

        Ok(CodeCellLockArgs { public_key_hash })
    }
}

/**
    Checker Bond Cell
    Data:
    Type:
        codehash: sudt
        hashtype: type
        args: muse_token_admin
    Lock:
        codehash: checker bond cell lockscript
        hashtype: type
        args: checker public key | chain id bitmap
*/

// which is standard sudt
#[derive(Debug)]
pub struct CheckerBondCellData {
    pub amount: u128,
}

impl FromRaw for CheckerBondCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<CheckerBondCellData, SysError> {
        check_args_len(cell_raw_data.len(), SUDT_DATA_LEN)?;

        let sudt_amount = decode_u128(&cell_raw_data[0..16])?;

        Ok(CheckerBondCellData { amount: sudt_amount })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CheckerBondCellLockArgs {
    pub checker_public_key: [u8; 32],
    pub chain_id_bitmap:    [u8; 32],
}

impl FromRaw for CheckerBondCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Result<CheckerBondCellLockArgs, SysError> {
        check_args_len(arg_raw_data.len(), CHECKER_BOND_LOCK_ARGS_LEN)?;

        let mut checker_address = [0u8; 32];
        checker_address.copy_from_slice(&arg_raw_data[0..32]);

        let mut chain_id_bitmap = [0u8; 32];
        chain_id_bitmap.copy_from_slice(&arg_raw_data[32..64]);

        Ok(CheckerBondCellLockArgs {
            checker_public_key: checker_address,
            chain_id_bitmap,
        })
    }
}

/**
    Sidechain Config Cell
    Data:
    Type:
        codehash: typeId
        hashtype: type
        args: chain_id(for lumos)
    Lock:
        codehash: A.S
        hashtype: data
        args: null
*/

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
    pub admin_public_key:        [u8; 32],
    pub collator_public_key:     [u8; 32],
    pub bond_sudt_type_hash:     [u8; 32],
}

impl FromRaw for SidechainConfigCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<SidechainConfigCellData, SysError> {
        check_args_len(cell_raw_data.len(), SIDECHAIN_CONFIG_DATA_LEN)?;

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

        let mut admin_public_key = [0u8; 32];
        admin_public_key.copy_from_slice(&cell_raw_data[89..121]);

        let mut collator_public_key = [0u8; 32];
        collator_public_key.copy_from_slice(&cell_raw_data[121..153]);

        let mut bond_sudt_type_hash = [0u8; 32];
        bond_sudt_type_hash.copy_from_slice(&cell_raw_data[153..185]);

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
            admin_public_key,
            collator_public_key,
            bond_sudt_type_hash,
        })
    }
}
#[derive(Debug, Copy, Clone)]
pub struct SidechainConfigCellTypeArgs {
    pub chain_id: u8,
}

impl FromRaw for SidechainConfigCellTypeArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Result<SidechainConfigCellTypeArgs, SysError> {
        check_args_len(arg_raw_data.len(), SIDECHAIN_CONFIG_TYPE_ARGS_LEN)?;

        let chain_id = decode_u8(&arg_raw_data[0..1])?;

        Ok(SidechainConfigCellTypeArgs { chain_id })
    }
}

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
        check_args_len(cell_raw_data.len(), SIDECHAIN_STATE_DATA_LEN)?;

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

#[derive(Debug, Copy, Clone)]
pub struct SidechainStateCellTypeArgs {
    pub chain_id: u8,
}

impl FromRaw for SidechainStateCellTypeArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Result<SidechainStateCellTypeArgs, SysError> {
        check_args_len(arg_raw_data.len(), SIDECHAIN_STATE_TYPE_ARGS_LEN)?;

        let chain_id = decode_u8(&arg_raw_data[0..1])?;

        Ok(SidechainStateCellTypeArgs { chain_id })
    }
}

/**
    Checker Info Cell
    Data:
    Type:
        codehash: typeId
        hashtype: type
        args: chain_id | public_key
    Lock:
        codehash: A.S.
        hashtype: type
        args: null
*/

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
        check_args_len(cell_raw_data.len(), CHECKER_INFO_DATA_LEN)?;

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

#[derive(Debug, Copy, Clone)]
pub struct CheckerInfoCellTypeArgs {
    pub chain_id:           u8,
    pub checker_public_key: [u8; 32],
}

impl FromRaw for CheckerInfoCellTypeArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Result<CheckerInfoCellTypeArgs, SysError> {
        check_args_len(arg_raw_data.len(), CHECKER_INFO_TYPE_ARGS_LEN)?;

        let chain_id = decode_u8(&arg_raw_data[0..1])?;

        let mut checker_public_key = [0u8; 32];
        checker_public_key.copy_from_slice(&arg_raw_data[1..33]);

        Ok(CheckerInfoCellTypeArgs {
            chain_id,
            checker_public_key,
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

#[derive(Debug, Copy, Clone)]
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

/**
    Sidechain Bond Cell
    Data:
    Type:
        codehash: sudt
        hashtype: type
        args: custom sudt admin
    Lock:
        codehash: sidechain bond cell lockscript
        hashtype: type
        args: chain_id | collator_public_key | unlock_sidechain_height
*/

// which is standard sudt
#[derive(Debug)]
pub struct SidechainBondCellData {
    pub amount: u128,
}

impl FromRaw for SidechainBondCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<SidechainBondCellData, SysError> {
        check_args_len(cell_raw_data.len(), SUDT_DATA_LEN)?;

        let sudt_amount = decode_u128(&cell_raw_data[0..16])?;

        Ok(SidechainBondCellData { amount: sudt_amount })
    }
}

#[derive(Debug)]
pub struct SidechainBondCellLockArgs {
    pub chain_id:                u8,
    pub collator_public_key:     [u8; 32],
    pub unlock_sidechain_height: u128,
}

impl FromRaw for SidechainBondCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Result<SidechainBondCellLockArgs, SysError> {
        check_args_len(arg_raw_data.len(), SIDECHAIN_BOND_LOCK_ARGS_LEN)?;

        let chain_id = decode_u8(&arg_raw_data[0..1])?;

        let mut collator_public_key = [0u8; 32];
        collator_public_key.copy_from_slice(&arg_raw_data[1..33]);

        let unlock_sidechain_height = decode_u128(&arg_raw_data[33..49])?;

        Ok(SidechainBondCellLockArgs {
            chain_id,
            collator_public_key,
            unlock_sidechain_height,
        })
    }
}

/**
    Sidechain Fee Cell
    Data:
    Type:
        codehash: sudt
        hashtype: type
        args: muse_token_admin
    Lock:
        codehash: sidechain fee cell lockscript
        hashtype: type
        args: chain_id
*/

// which is standard sudt
#[derive(Debug)]
pub struct SidechainFeeCellData {
    amount: u128,
}

impl FromRaw for SidechainFeeCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Result<SidechainFeeCellData, SysError> {
        check_args_len(cell_raw_data.len(), SUDT_DATA_LEN)?;

        let sudt_amount = decode_u128(&cell_raw_data[0..16])?;

        Ok(SidechainFeeCellData { amount: sudt_amount })
    }
}

#[derive(Debug)]
pub struct SidechainFeeCellLockArgs {
    pub chain_id: u8,
}

impl FromRaw for SidechainFeeCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Result<SidechainFeeCellLockArgs, SysError> {
        check_args_len(arg_raw_data.len(), SIDECHAIN_FEE_LOCK_ARGS_LEN)?;

        let chain_id = decode_u8(&arg_raw_data[0..1])?;

        Ok(SidechainFeeCellLockArgs { chain_id })
    }
}

//the dep0 must be global cell
pub fn check_global_cell() -> Result<GlobalConfigCellData, CommonError> {
    if load_cell_type_hash(0, Source::CellDep)?.ok_or(CommonError::LoadTypeHashError)? != GLOBAL_CONFIG_TYPE_HASH {
        return Err(CommonError::GlobalConfigCellDepError);
    }

    let global_config_data = load_cell_data(0, Source::CellDep)?;
    let global_config_data = GlobalConfigCellData::from_raw(&global_config_data)?;

    Ok(global_config_data)
}

pub fn check_cells(requests: Vec<(CellType, usize, Source)>, global: &GlobalConfigCellData) -> Result<(), CommonError> {
    for (cell_type, index, source) in requests {
        check_cell(cell_type, index, source, global)?;
    }

    Ok(())
}

pub fn check_cell(cell_type: CellType, index: usize, source: Source, global: &GlobalConfigCellData) -> Result<(), CommonError> {
    let cell = load_cell(index, source)?;
    let script = cell.type_().to_opt().ok_or(CommonError::MissingTypeScript)?;
    let codehash = script.code_hash().unpack();
    let hashtype = script.hash_type().as_slice()[0];

    match cell_type {
        CellType::Unknown => return Err(CommonError::UnknownCellType),
        CellType::Code => {
            if codehash != global.code_cell_type_codehash {
                return Err(CommonError::CodeHashMismatch);
            }
            if hashtype != global.code_cell_type_hashtype {
                return Err(CommonError::HashTypeMismatch);
            }
            Ok(())
        }
        CellType::Sudt => {
            if codehash != SUDT_CODEHASH {
                return Err(CommonError::CodeHashMismatch);
            }

            if hashtype != SUDT_HASHTYPE || script.args().as_slice() != SUDT_MUSE_ARGS {
                return Err(CommonError::HashTypeMismatch);
            }
            Ok(())
        }
        CellType::MuseToken => {
            if codehash != SUDT_CODEHASH {
                return Err(CommonError::CodeHashMismatch);
            }

            if hashtype != SUDT_HASHTYPE || script.args().as_slice() != SUDT_MUSE_ARGS {
                return Err(CommonError::HashTypeMismatch);
            }
            Ok(())
        }
        CellType::SidechainBond => {
            if codehash != SUDT_CODEHASH {
                return Err(CommonError::CodeHashMismatch);
            }

            if hashtype != SUDT_HASHTYPE || script.args().as_slice() != SUDT_MUSE_ARGS {
                return Err(CommonError::HashTypeMismatch);
            }

            let lock_script = cell.lock();
            let lock_codehash = lock_script.code_hash().unpack();
            let lock_hashtype = lock_script.hash_type().as_slice()[0];
            if lock_codehash != global.sidechain_bond_cell_lock_codehash {
                return Err(CommonError::CodeHashMismatch);
            }

            if lock_hashtype != global.sidechain_bond_cell_lock_hashtype {
                return Err(CommonError::HashTypeMismatch);
            }

            Ok(())
        }
        CellType::CheckerBond => {
            if codehash != SUDT_CODEHASH {
                return Err(CommonError::CodeHashMismatch);
            }

            if hashtype != SUDT_HASHTYPE || script.args().as_slice() != SUDT_MUSE_ARGS {
                return Err(CommonError::HashTypeMismatch);
            }

            let lock_script = cell.lock();
            let lock_codehash = lock_script.code_hash().unpack();
            let lock_hashtype = lock_script.hash_type().as_slice()[0];
            if lock_codehash != global.checker_bond_cell_lock_codehash {
                return Err(CommonError::CodeHashMismatch);
            }

            if lock_hashtype != global.checker_bond_cell_lock_hashtype {
                return Err(CommonError::HashTypeMismatch);
            }

            Ok(())
        }
        CellType::CheckerInfo => {
            if codehash != global.checker_info_cell_type_codehash {
                return Err(CommonError::CodeHashMismatch);
            }
            if hashtype != global.checker_info_cell_type_hashtype {
                return Err(CommonError::HashTypeMismatch);
            }
            Ok(())
        }
        CellType::SidechainConfig => {
            if codehash != global.sidechain_config_cell_type_codehash {
                return Err(CommonError::CodeHashMismatch);
            }
            if hashtype != global.sidechain_config_cell_type_hashtype {
                return Err(CommonError::HashTypeMismatch);
            }
            Ok(())
        }
        CellType::SidechainState => {
            if codehash != global.sidechain_state_cell_type_codehash {
                return Err(CommonError::CodeHashMismatch);
            }
            if hashtype != global.sidechain_state_cell_type_hashtype {
                return Err(CommonError::HashTypeMismatch);
            }
            Ok(())
        }
        CellType::Task => {
            if codehash != global.task_cell_type_codehash {
                return Err(CommonError::CodeHashMismatch);
            }
            if hashtype != global.task_cell_type_hashtype {
                return Err(CommonError::HashTypeMismatch);
            }
            Ok(())
        }
        CellType::SidechainFee => {
            if codehash != SUDT_CODEHASH {
                return Err(CommonError::CodeHashMismatch);
            }

            if hashtype != SUDT_HASHTYPE || script.args().as_slice() != SUDT_MUSE_ARGS {
                return Err(CommonError::HashTypeMismatch);
            }

            let lock_script = cell.lock();
            let lock_codehash = lock_script.code_hash().unpack();
            let lock_hashtype = lock_script.hash_type().as_slice()[0];
            if lock_codehash != global.sidechain_fee_cell_lock_codehash {
                return Err(CommonError::CodeHashMismatch);
            }

            if lock_hashtype != global.sidechain_fee_cell_lock_hashtype {
                return Err(CommonError::HashTypeMismatch);
            }
            Ok(())
        }
    }
}
