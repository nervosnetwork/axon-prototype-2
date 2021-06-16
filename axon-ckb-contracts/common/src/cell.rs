use core::result::Result;

use crate::error::CommonError;
use crate::{GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH, SUDT_HASHTYPE, SUDT_MUSE_ARGS};
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell_data, load_cell_lock, load_cell_type, load_cell_type_hash};
use common_raw::{
    cell::{
        checker_bond::CheckerBondCellData, checker_info::CheckerInfoCellData, code::CodeCellData, global_config::GlobalConfigCellData,
        muse_token::MuseTokenData, sidechain_bond::SidechainBondCellData, sidechain_config::SidechainConfigCellData,
        sidechain_fee::SidechainFeeCellData, sidechain_state::SidechainStateCellData, sudt_token::SudtTokenData, task::TaskCellData,
    },
    FromRaw,
};

//the dep0 must be global cell
pub fn check_global_cell() -> Result<GlobalConfigCellData, CommonError> {
    let global_config_data = (0..)
        .find_map(|i| {
            let type_hash = match load_cell_type_hash(i, Source::CellDep) {
                Ok(hash) => hash,
                Err(err) => return Some(Err(err)),
            }?;
            if type_hash == GLOBAL_CONFIG_TYPE_HASH {
                return load_cell_data(i, Source::CellDep).ok().map(|data| Ok(data));
            }
            None
        })
        .ok_or(CommonError::GlobalConfigCellDep)??;

    let global_config_data = GlobalConfigCellData::from_raw(&global_config_data).ok_or(CommonError::Encoding)?;

    Ok(global_config_data)
}

pub struct CellOrigin(pub usize, pub Source);

macro_rules! check_script {
    ($script: expr, $code_hash: expr, $hash_type: expr) => {
        if $script.as_reader().code_hash().raw_data() != $code_hash {
            return Err(CommonError::CodeHashMismatch);
        }
        if $script.as_reader().hash_type().as_slice()[0] != $hash_type {
            return Err(CommonError::HashTypeMismatch);
        }
    };
    ($script: expr, $code_hash: expr, $hash_type: expr, $args: expr) => {
        check_script!($script, $code_hash, $hash_type);

        if $script.as_reader().args().raw_data() != $args {
            return Err(CommonError::HashTypeMismatch);
        }
    };
}

pub trait TypedCell {
    fn type_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8);

    fn check(origin: CellOrigin, global: &GlobalConfigCellData) -> Result<(), CommonError> {
        let CellOrigin(index, source) = origin;
        let script = load_cell_type(index, source)?.ok_or(CommonError::MissingTypeScript)?;

        let (code_hash, hash_type) = Self::type_script_info(global);

        check_script!(script, code_hash, hash_type);

        Ok(())
    }
}

impl TypedCell for CodeCellData {
    fn type_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.code_cell_type_codehash, global.code_cell_type_hashtype)
    }
}

impl TypedCell for SidechainConfigCellData {
    fn type_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (
            global.sidechain_config_cell_type_codehash,
            global.sidechain_config_cell_type_hashtype,
        )
    }
}

impl TypedCell for SidechainStateCellData {
    fn type_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.sidechain_state_cell_type_codehash, global.sidechain_state_cell_type_hashtype)
    }
}

impl TypedCell for TaskCellData {
    fn type_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.task_cell_type_codehash, global.task_cell_type_hashtype)
    }
}

impl TypedCell for CheckerInfoCellData {
    fn type_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.checker_info_cell_type_codehash, global.checker_info_cell_type_hashtype)
    }
}

fn check_sudt_type_script(index: usize, source: Source) -> Result<(), CommonError> {
    let script = load_cell_type(index, source)?.ok_or(CommonError::MissingTypeScript)?;
    check_script!(script, SUDT_CODEHASH, SUDT_HASHTYPE, SUDT_MUSE_ARGS);

    Ok(())
}

pub trait TypedSudtCell {
    fn check(origin: CellOrigin, _: &GlobalConfigCellData) -> Result<(), CommonError> {
        let CellOrigin(index, source) = origin;

        check_sudt_type_script(index, source)
    }
}

impl TypedSudtCell for MuseTokenData {}
impl TypedSudtCell for SudtTokenData {}

pub trait LockedTypedSudtCell {
    fn lock_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8);

    fn check(origin: CellOrigin, global: &GlobalConfigCellData) -> Result<(), CommonError> {
        let CellOrigin(index, source) = origin;

        check_sudt_type_script(index, source)?;

        let script = load_cell_lock(index, source)?;
        let (code_hash, hash_type) = Self::lock_script_info(global);
        check_script!(script, code_hash, hash_type);

        Ok(())
    }
}

impl LockedTypedSudtCell for CheckerBondCellData {
    fn lock_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.checker_bond_cell_lock_codehash, global.checker_bond_cell_lock_hashtype)
    }
}

impl LockedTypedSudtCell for SidechainBondCellData {
    fn lock_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.sidechain_bond_cell_lock_codehash, global.sidechain_bond_cell_lock_hashtype)
    }
}

impl LockedTypedSudtCell for SidechainFeeCellData {
    fn lock_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.sidechain_fee_cell_lock_codehash, global.sidechain_fee_cell_lock_hashtype)
    }
}
