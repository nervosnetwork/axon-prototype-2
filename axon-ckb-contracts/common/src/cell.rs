use core::result::Result;

use crate::error::CommonError;
use crate::{GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH, SUDT_HASHTYPE, SUDT_MUSE_ARGS};
use alloc::vec::Vec;
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::prelude::{Entity, Unpack};
use ckb_std::high_level::{load_cell, load_cell_data, load_cell_type_hash};
use common_raw::{cell::global_config::GlobalConfigCellData, FromRaw};

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

//the dep0 must be global cell
pub fn check_global_cell() -> Result<GlobalConfigCellData, CommonError> {

    let global_config_data = (0..).find_map(|i| {
        let type_hash = match load_cell_type_hash(i, Source::CellDep) {
            Ok(hash) => hash,
            Err(err) => return Some(Err(err)),
        }?;
        if type_hash == GLOBAL_CONFIG_TYPE_HASH {
            return load_cell_data(i, Source::CellDep).ok().map(|data| Ok(data));
        }
        None
    }).ok_or(CommonError::GlobalConfigCellDep)??;

    let global_config_data = GlobalConfigCellData::from_raw(&global_config_data).ok_or(CommonError::Encoding)?;

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
        CellType::Unknown => Err(CommonError::UnknownCellType),
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

            if hashtype != SUDT_HASHTYPE || script.as_reader().args().raw_data() != SUDT_MUSE_ARGS {
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
