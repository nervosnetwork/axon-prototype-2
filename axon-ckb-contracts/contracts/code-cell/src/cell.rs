use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell_data, load_cell_lock, load_cell_type};

use common_raw::{
    cell::{
        checker_bond::{CheckerBondCell, CheckerBondCellLockArgs},
        checker_info::{CheckerInfoCell, CheckerInfoCellTypeArgs},
        code::{CodeCell, CodeCellLockArgs},
        global_config::GlobalConfigCellData,
        muse_token::MuseTokenCell,
        sidechain_bond::{SidechainBondCell, SidechainBondCellLockArgs},
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs},
        sidechain_fee::{SidechainFeeCell, SidechainFeeCellLockArgs},
        sidechain_state::{SidechainStateCell, SidechainStateCellTypeArgs},
        sudt_token::SudtTokenCell,
        task::{TaskCell, TaskCellTypeArgs},
    },
    FromRaw,
};

use crate::error::Error;

pub const SUDT_CODEHASH: [u8; 32] = common::SUDT_TYPE_HASH;
pub const SUDT_HASHTYPE: u8 = 0u8;
pub const SUDT_MUSE_ARGS: &[u8] = &[];

#[derive(Debug, Copy, Clone)]
pub struct CellOrigin(pub usize, pub Source);

pub trait LoadableCell {
    fn load(origin: CellOrigin) -> Result<Self, Error>
    where
        Self: Sized + FromRaw,
    {
        let CellOrigin(index, source) = origin;
        let data = load_cell_data(index, source)?;
        Self::from_raw(&data).ok_or(Error::Encoding)
    }
}

impl LoadableCell for CheckerBondCell {}

impl LoadableCell for CheckerInfoCell {}

impl LoadableCell for GlobalConfigCellData {}

impl LoadableCell for MuseTokenCell {}

impl LoadableCell for SidechainBondCell {}

impl LoadableCell for SidechainConfigCell {}

impl LoadableCell for SidechainFeeCell {}

impl LoadableCell for SidechainStateCell {}

impl LoadableCell for SudtTokenCell {}

impl LoadableCell for TaskCell {}

pub trait LoadableLockArgs {
    fn load(origin: CellOrigin) -> Result<Self, Error>
    where
        Self: Sized + FromRaw,
    {
        let CellOrigin(index, source) = origin;
        let data = load_cell_lock(index, source)?.args();
        Self::from_raw(data.as_reader().raw_data()).ok_or(Error::Encoding)
    }
}

impl LoadableLockArgs for CheckerBondCellLockArgs {}

impl LoadableLockArgs for CodeCellLockArgs {}

impl LoadableLockArgs for SidechainBondCellLockArgs {}

impl LoadableLockArgs for SidechainFeeCellLockArgs {}

pub trait LoadableTypeArgs {
    fn load(origin: CellOrigin) -> Result<Self, Error>
    where
        Self: Sized + FromRaw,
    {
        let CellOrigin(index, source) = origin;
        let data = load_cell_type(index, source)?.ok_or(Error::TypeScriptMissed)?.args();
        Self::from_raw(data.as_reader().raw_data()).ok_or(Error::Encoding)
    }
}

impl LoadableTypeArgs for CheckerInfoCellTypeArgs {}

impl LoadableTypeArgs for SidechainConfigCellTypeArgs {}

impl LoadableTypeArgs for SidechainStateCellTypeArgs {}

impl LoadableTypeArgs for TaskCellTypeArgs {}

#[macro_export]
macro_rules! load_entities {
    ($($type: ty: $origin: expr), * $(,)?) => {
        (
            $(<$type>::load($origin)?,)*
        )
    }
}

macro_rules! check_script {
    ($script: expr, $code_hash: expr, $hash_type: expr) => {
        if $script.as_reader().code_hash().raw_data() != $code_hash {
            return Err(Error::CodeHashMismatch);
        }
        if $script.as_reader().hash_type().as_slice()[0] != $hash_type {
            return Err(Error::HashTypeMismatch);
        }
    };
    ($script: expr, $code_hash: expr, $hash_type: expr, $args: expr) => {
        check_script!($script, $code_hash, $hash_type);

        if $script.as_reader().args().raw_data() != $args {
            return Err(Error::HashTypeMismatch);
        }
    };
}

macro_rules! CheckableHelpers {
    () => {
        fn range_check<T: Iterator<Item = usize>>(range: T, source: Source, global: &GlobalConfigCellData) -> Result<(), Error> {
            for x in range {
                match Self::check(CellOrigin(x, source), &global) {
                    Ok(_) => (),
                    Err(Error::IndexOutOfBound) => break,
                    Err(err) => return Err(err),
                }
            }

            Ok(())
        }

        fn one_to_one_check(start: usize, global: &GlobalConfigCellData) -> Result<(), Error> {
            for x in start.. {
                let input_ended = match Self::check(CellOrigin(x, Source::Input), &global) {
                    Ok(_) => false,
                    Err(Error::IndexOutOfBound) => true,
                    Err(err) => return Err(err),
                };
                let output_ended = match TaskCell::check(CellOrigin(x, Source::Output), &global) {
                    Ok(_) => false,
                    Err(Error::IndexOutOfBound) => true,
                    Err(err) => return Err(err),
                };
                if input_ended && output_ended {
                    break;
                }
                if input_ended || output_ended {
                    return Err(Error::CellNumberMismatch);
                }
            }

            Ok(())
        }
    };
}

pub trait TypedCell {
    fn type_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8);

    fn check(origin: CellOrigin, global: &GlobalConfigCellData) -> Result<(), Error> {
        let CellOrigin(index, source) = origin;
        let script = load_cell_type(index, source)?.ok_or(Error::MissingTypeScript)?;

        let (code_hash, hash_type) = Self::type_script_info(global);

        check_script!(script, code_hash, hash_type);

        Ok(())
    }

    CheckableHelpers! {}
}

impl TypedCell for CodeCell {
    fn type_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.code_cell_type_codehash, global.code_cell_type_hashtype)
    }
}

impl TypedCell for SidechainConfigCell {
    fn type_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (
            global.sidechain_config_cell_type_codehash,
            global.sidechain_config_cell_type_hashtype,
        )
    }
}

impl TypedCell for SidechainStateCell {
    fn type_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.sidechain_state_cell_type_codehash, global.sidechain_state_cell_type_hashtype)
    }
}

impl TypedCell for TaskCell {
    fn type_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.task_cell_type_codehash, global.task_cell_type_hashtype)
    }
}

impl TypedCell for CheckerInfoCell {
    fn type_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.checker_info_cell_type_codehash, global.checker_info_cell_type_hashtype)
    }
}

fn check_sudt_type_script(index: usize, source: Source) -> Result<(), Error> {
    let script = load_cell_type(index, source)?.ok_or(Error::MissingTypeScript)?;
    check_script!(script, SUDT_CODEHASH, SUDT_HASHTYPE, SUDT_MUSE_ARGS);

    Ok(())
}

pub trait TypedSudtCell {
    fn check(origin: CellOrigin, _: &GlobalConfigCellData) -> Result<(), Error> {
        let CellOrigin(index, source) = origin;

        check_sudt_type_script(index, source)
    }

    CheckableHelpers! {}
}

impl TypedSudtCell for MuseTokenCell {}

impl TypedSudtCell for SudtTokenCell {}

pub trait LockedTypedSudtCell {
    fn lock_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8);

    fn check(origin: CellOrigin, global: &GlobalConfigCellData) -> Result<(), Error> {
        let CellOrigin(index, source) = origin;

        check_sudt_type_script(index, source)?;

        let script = load_cell_lock(index, source)?;
        let (code_hash, hash_type) = Self::lock_script_info(global);
        check_script!(script, code_hash, hash_type);

        Ok(())
    }

    CheckableHelpers! {}
}

impl LockedTypedSudtCell for CheckerBondCell {
    fn lock_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.checker_bond_cell_lock_codehash, global.checker_bond_cell_lock_hashtype)
    }
}

impl LockedTypedSudtCell for SidechainBondCell {
    fn lock_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.sidechain_bond_cell_lock_codehash, global.sidechain_bond_cell_lock_hashtype)
    }
}

impl LockedTypedSudtCell for SidechainFeeCell {
    fn lock_script_info(global: &GlobalConfigCellData) -> ([u8; 32], u8) {
        (global.sidechain_fee_cell_lock_codehash, global.sidechain_fee_cell_lock_hashtype)
    }
}
