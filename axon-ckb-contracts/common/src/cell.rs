use core::convert::{TryFrom, TryInto};
use core::result::Result;

use ckb_std::error::SysError;

use crate::{check_args_len, decode_i8, decode_u128, decode_u16, decode_u64, decode_u8};

// in byte
const SUDT_DATA_LEN: usize = 16; // u128

const GLOBAL_CONFIG_DATA_LEN: usize = 263;

const CHECKER_BOND_LOCK_ARGS_LEN: usize = 64;
const CHECKER_BOND_LOCK_WITNESS_LEN: usize = 33;

const SIDECHAIN_CONFIG_DATA_LEN: usize = 153;
const SIDECHAIN_CONFIG_TYPE_WITNESS_LEN: usize = 1;

const SIDECHAIN_STATE_DATA_LEN: usize = 164;
const SIDECHAIN_STATE_TYPE_WITNESS_LEN: usize = 1;

const CHECKER_INFO_DATA_LEN: usize = 595;
const CHECKER_INFO_TYPE_WITNESS_LEN: usize = 33;

const TASK_DATA_LEN: usize = 101;
const TASK_TYPE_WITNESS_LEN: usize = 1;

const SIDECHAIN_BOND_LOCK_ARGS_LEN: usize = 49;
const SIDECHAIN_BOND_WITNESS_LEN: usize = 33;

const SIDECHAIN_FEE_LOCK_ARGS_LEN: usize = 1;
const SIDECHAIN_FEE_WITNESS_LEN: usize = 33;

pub trait FromRaw {
    fn from_raw(cell_raw_data: &[u8]) -> Result<Self, SysError>
    where
        Self: Sized;
}

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
    Any other cells refers to Global Config Cell via its type script hash.
    If in test, the type script could be an Always-Success and blank args

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
    pub admin_public_key:                    [u8; 32], /* this is the authenticated admin for
                                                        * sidechain config cell */
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

        let mut sidechain_config_cell_type_codehash = [0u8; 32];
        sidechain_config_cell_type_codehash.copy_from_slice(&cell_raw_data[32..64]);
        let sidechain_config_cell_type_hashtype = decode_u8(&cell_raw_data[64..65])?;

        let mut sidechain_state_cell_type_codehash = [0u8; 32];
        sidechain_state_cell_type_codehash.copy_from_slice(&cell_raw_data[65..97]);
        let sidechain_state_cell_type_hashtype = decode_u8(&cell_raw_data[97..98])?;

        let mut checker_info_cell_type_codehash = [0u8; 32];
        checker_info_cell_type_codehash.copy_from_slice(&cell_raw_data[98..130]);
        let checker_info_cell_type_hashtype = decode_u8(&cell_raw_data[130..131])?;

        let mut checker_bond_cell_lock_codehash = [0u8; 32];
        checker_bond_cell_lock_codehash.copy_from_slice(&cell_raw_data[131..163]);
        let checker_bond_cell_lock_hashtype = decode_u8(&cell_raw_data[163..164])?;

        let mut task_cell_type_codehash = [0u8; 32];
        task_cell_type_codehash.copy_from_slice(&cell_raw_data[164..196]);
        let task_cell_type_hashtype = decode_u8(&cell_raw_data[196..197])?;

        let mut sidechain_fee_cell_lock_codehash = [0u8; 32];
        sidechain_fee_cell_lock_codehash.copy_from_slice(&cell_raw_data[197..229]);
        let sidechain_fee_cell_lock_hashtype = decode_u8(&cell_raw_data[229..230])?;

        let mut sidechain_bond_cell_lock_codehash = [0u8; 32];
        sidechain_bond_cell_lock_codehash.copy_from_slice(&cell_raw_data[230..262]);
        let sidechain_bond_cell_lock_hashtype = decode_u8(&cell_raw_data[262..263])?;

        Ok(GlobalConfigCellData {
            admin_public_key,
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

/**
    Checker Bond Cell
    Data:
    Type:
        codehash: sudt
        hashtype: type
        args: unique_id
    Lock:
        codehash: checker bond cell lockscript
        hashtype: type // data
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
    fn from_raw(cell_raw_data: &[u8]) -> Result<CheckerBondCellLockArgs, SysError> {
        check_args_len(cell_raw_data.len(), CHECKER_BOND_LOCK_ARGS_LEN)?;

        let mut checker_address = [0u8; 32];
        checker_address.copy_from_slice(&cell_raw_data[0..32]);

        let mut chain_id_bitmap = [0u8; 32];
        chain_id_bitmap.copy_from_slice(&cell_raw_data[32..64]);

        Ok(CheckerBondCellLockArgs {
            checker_public_key: checker_address,
            chain_id_bitmap,
        })
    }
}

#[derive(Debug)]
pub struct CheckerBondLockWitness {
    pub pattern:   u8,
    pub signature: [u8; 32],
}

impl FromRaw for CheckerBondLockWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Result<CheckerBondLockWitness, SysError> {
        check_args_len(witness_raw_data.len(), CHECKER_BOND_LOCK_WITNESS_LEN)?;

        let pattern = decode_u8(witness_raw_data)?;

        let mut signature = [0u8; 32];
        signature.copy_from_slice(&witness_raw_data[1..33]);

        Ok(CheckerBondLockWitness { pattern, signature })
    }
}

/**
    Sidechain Config Cell
    Data:
    Type:
        codehash: sidechain config typescript   // sidechain config typescript
        hashtype: type                          // data
        args: null                              // null
    Lock:
        codehash: A.S
        hashtype: data
        args: null
*/

#[derive(Debug)]
pub struct SidechainConfigCellData {
    pub chain_id:                     u8,
    pub checker_total_count:          u8,
    // 2**8 = 256
    pub checker_bitmap:               [u8; 32],
    // 256
    pub checker_threshold:            u8,
    pub update_interval:              u16,
    pub minimal_bond:                 u128,
    pub checker_data_size_limit:      u128,
    pub checker_price:                u128,
    pub refresh_interval:             u16,
    pub commit_threshold:             u8,
    pub challenge_threshold:          u8,
    pub admin_public_key:             [u8; 32], //maybe going to args is better for runtime
    pub global_config_cell_type_hash: [u8; 32],
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

        let mut global_config_cell_type_hash = [0u8; 32];
        global_config_cell_type_hash.copy_from_slice(&cell_raw_data[121..153]);

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
            global_config_cell_type_hash,
        })
    }
}

#[derive(Debug)]
pub struct SidechainConfigTypeWitness {
    pub pattern: u8,
}

impl FromRaw for SidechainConfigTypeWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Result<SidechainConfigTypeWitness, SysError> {
        check_args_len(witness_raw_data.len(), SIDECHAIN_CONFIG_TYPE_WITNESS_LEN)?;

        let pattern = decode_u8(witness_raw_data)?;

        Ok(SidechainConfigTypeWitness { pattern })
    }
}
/**
    Sidechain State Cell
    Data:
    Type:
        codehash: sidechain state typescript    // sidechain state typescript
        hashtype: type                          // data
        args: null                              // null
    Lock:
        codehash: A.S.
        hashtype: type
        args: null
*/

#[derive(Debug)]
pub struct SidechainStateCellData {
    pub chain_id: u8,
    pub version: u8,
    pub latest_block_height: u128,
    pub latest_block_hash: [u8; 32],
    pub committed_block_height: u128,
    pub committed_block_hash: [u8; 32],
    pub collator_public_key: [u8; 32],
    pub global_config_cell_type_hash: [u8; 32],
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

        let mut collator_public_key = [0u8; 32];
        collator_public_key.copy_from_slice(&cell_raw_data[98..130]);

        let mut global_config_cell_type_hash = [0u8; 32];
        global_config_cell_type_hash.copy_from_slice(&cell_raw_data[130..164]);

        Ok(SidechainStateCellData {
            chain_id,
            version,
            latest_block_height,
            latest_block_hash,
            committed_block_height,
            committed_block_hash,
            collator_public_key,
            global_config_cell_type_hash,
        })
    }
}

#[derive(Debug)]
pub struct SidechainStateTypeWitness {
    pub pattern: u8,
}

impl FromRaw for SidechainStateTypeWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Result<SidechainStateTypeWitness, SysError> {
        check_args_len(witness_raw_data.len(), SIDECHAIN_STATE_TYPE_WITNESS_LEN)?;

        let pattern = decode_u8(witness_raw_data)?;

        Ok(SidechainStateTypeWitness { pattern })
    }
}

/**
    Checker Info Cell
    Data:
    Type:
        codehash: checker info typescript       // checker info typescript
        hashtype: type                          // data
        args: null                              // null
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
    pub chain_id: u8,
    pub checker_id: u8,
    pub unpaid_fee: u128,
    pub rpc_url: [u8; 512],
    pub checker_public_key: [u8; 32],
    pub mode: CheckerInfoCellMode,
    pub global_config_cell_type_hash: [u8; 32],
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

        let mut global_config_cell_type_hash = [0u8; 32];
        global_config_cell_type_hash.copy_from_slice(&cell_raw_data[563..595]);

        Ok(CheckerInfoCellData {
            chain_id,
            checker_id,
            unpaid_fee,
            rpc_url,
            checker_public_key,
            mode,
            global_config_cell_type_hash,
        })
    }
}

#[derive(Debug)]
pub struct CheckerInfoTypeWitness {
    pub pattern:   u8,
    pub signature: [u8; 32],
}

impl FromRaw for CheckerInfoTypeWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Result<CheckerInfoTypeWitness, SysError> {
        check_args_len(witness_raw_data.len(), CHECKER_INFO_TYPE_WITNESS_LEN)?;

        let pattern = decode_u8(witness_raw_data)?;

        let mut signature = [0u8; 32];
        signature.copy_from_slice(&witness_raw_data[1..33]);

        Ok(CheckerInfoTypeWitness { pattern, signature })
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
        codehash: task cell typescript          // task typescript
        hashtype: type                          // data
        args: null                              // null
    Lock:
        codehash: A.S.
        hashtype: type
        args: null
*/

#[derive(Debug)]
pub struct TaskCellData {
    pub chain_id: u8,
    pub version: u8,
    pub check_block_height_from: u128, // 应该为ssc committed_height + 1
    pub check_block_height_to: u128,   // inclusive 应该为latest_height
    pub check_block_hash_to: u128,
    pub check_data_size: u128,
    pub refresh_interval: u16,
    pub mode: TaskCellMode,
    pub global_config_cell_type_hash: [u8; 32],
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

        let mut global_config_cell_type_hash = [0u8; 32];
        global_config_cell_type_hash.copy_from_slice(&cell_raw_data[69..101]);

        Ok(TaskCellData {
            chain_id,
            version,
            check_block_height_from,
            check_block_height_to,
            check_block_hash_to,
            check_data_size,
            refresh_interval,
            mode,
            global_config_cell_type_hash,
        })
    }
}

#[derive(Debug)]
pub struct TaskCellTypeWitness {
    pub pattern: u8,
}

impl FromRaw for TaskCellTypeWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Result<TaskCellTypeWitness, SysError> {
        check_args_len(witness_raw_data.len(), TASK_TYPE_WITNESS_LEN)?;

        let pattern = decode_u8(witness_raw_data)?;

        Ok(TaskCellTypeWitness { pattern })
    }
}

/**
    Sidechain Bond Cell
    Data:
    Type:
        codehash: sudt
        hashtype: type
        args: null
    Lock:
        codehash: sidechain bond cell lockscript    // sidechain bond cell typescript
        hashtype: type                              // data
        args: chain_id | collator_public_key | unlock_sidechain_height
*/

// which is standard sudt
#[derive(Debug)]
pub struct SidechainBondCellData {
    amount: u128,
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

#[derive(Debug)]
pub struct SidechainBondLockWitness {
    pub pattern:   u8,
    pub signature: [u8; 32],
}

impl FromRaw for SidechainBondLockWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Result<SidechainBondLockWitness, SysError> {
        check_args_len(witness_raw_data.len(), SIDECHAIN_BOND_WITNESS_LEN)?;

        let pattern = decode_u8(witness_raw_data)?;

        let mut signature = [0u8; 32];
        signature.copy_from_slice(&witness_raw_data[1..33]);

        Ok(SidechainBondLockWitness { pattern, signature })
    }
}

/**
    Sidechain Fee Cell
    Data:
    Type:
        codehash: sudt
        hashtype: type
        args: null
    Lock:
        codehash: sidechain fee cell lockscript     // sidechain fee cell typescript
        hashtype: type                              // data
        args: chain_id
*/

// which is standard sudt
#[derive(Debug)]
pub struct SidechainFeeCellData {
    amount: u128,
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

#[derive(Debug)]
pub struct SidechainFeeLockWitness {
    pub pattern:   u8,
    pub signature: [u8; 32],
}

impl FromRaw for SidechainFeeLockWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Result<SidechainFeeLockWitness, SysError> {
        check_args_len(witness_raw_data.len(), SIDECHAIN_FEE_WITNESS_LEN)?;

        let pattern = decode_u8(witness_raw_data)?;

        let mut signature = [0u8; 32];
        signature.copy_from_slice(&witness_raw_data[1..33]);

        Ok(SidechainFeeLockWitness { pattern, signature })
    }
}
