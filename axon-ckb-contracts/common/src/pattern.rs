use crate::cell::{CellType, FromRaw, GlobalConfigCellData, SidechainConfigCellData};
use crate::error::CommonError;
use crate::{get_input_cell_count, get_output_cell_count, GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH, SUDT_HASHTYPE, SUDT_MUSE_ARGS};
use ckb_std::high_level::load_cell_type;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{
        packed::{Byte, CellOutput},
        prelude::*,
    },
    default_alloc,
    high_level::{load_cell, load_cell_data, load_cell_lock_hash, load_cell_type_hash, load_script, load_witness_args, QueryIter},
};

#[repr(u8)]
#[derive(PartialOrd, PartialEq)]
pub enum Pattern {
    Unrecognised = 0u8,

    CheckerBondDeposit,
    CheckerBondWithdraw,
    CheckerJoinSidechain,
    CheckerQuitSidechain,
    CheckerSubmitTask = 5u8,
    CheckerPublishChallenge,
    CheckerSubmitChallenge,
    CheckerTakeBeneficiary,

    AdminCreateSidechain,

    CollatorPublishTask = 10u8,
    CollatorSubmitTask,
    CollatorSubmitChallenge,
    CollatorRefreshTask,
    CollatorUnlockBond = 14u8,
}

impl From<u8> for Pattern {
    fn from(input: u8) -> Self {
        match input {
            0u8 => Self::Unrecognised,
            1u8 => Self::CheckerBondDeposit,
            2u8 => Self::CheckerBondWithdraw,
            3u8 => Self::CheckerJoinSidechain,
            4u8 => Self::CheckerQuitSidechain,
            5u8 => Self::CheckerSubmitTask,
            6u8 => Self::CheckerPublishChallenge,
            7u8 => Self::CheckerSubmitChallenge,
            8u8 => Self::CheckerTakeBeneficiary,
            9u8 => Self::AdminCreateSidechain,
            10u8 => Self::CollatorPublishTask,
            11u8 => Self::CollatorSubmitTask,
            12u8 => Self::CollatorSubmitChallenge,
            13u8 => Self::CollatorRefreshTask,
            14u8 => Self::CollatorUnlockBond,
            _ => Self::Unrecognised,
        }
    }
}

pub fn is_checker_bond_deposit() -> Result<(), CommonError> {
    /*
    CheckerBondDeposit,

    Muse Token Cell             ->        Checker Bond Cell

     */
    Ok(())
}

pub fn is_checker_bond_withdraw() -> Result<(), CommonError> {
    /*
    CheckerBondWithdraw,

    Checker Bond Cell           ->         Muse Token Cell

     */
    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 1 || output_count != 1 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cell(CellType::CheckerBond, 0, Source::Input, &global)?;

    check_cell(CellType::MuseToken, 0, Source::Output, &global)?;

    Ok(())
}

pub fn is_checker_join_sidechain() -> Result<(), CommonError> {
    /*
    CheckerJoinSidechain,

    Dep:    1 Global Config Cell

    Sidechain Config Cell*      ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Null                        ->          Checker Info Cell

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 2 || output_count != 3 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cell(CellType::SidechainConfig, 0, Source::Input, &global)?;
    check_cell(CellType::CheckerBond, 1, Source::Input, &global)?;

    check_cell(CellType::SidechainConfig, 0, Source::Output, &global)?;
    check_cell(CellType::CheckerBond, 1, Source::Output, &global)?;
    check_cell(CellType::CheckerInfo, 2, Source::Output, &global)?;

    Ok(())
}

pub fn is_checker_quit_sidechain() -> Result<(), CommonError> {
    /*
    CheckerQuitSidechain

    Dep:    1 Global Config Cell

    Sidechain Config Cell*      ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Checker Info Cell           ->          Null
    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 3 || output_count != 2 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cell(CellType::SidechainConfig, 0, Source::Input, &global)?;
    check_cell(CellType::CheckerBond, 1, Source::Input, &global)?;
    check_cell(CellType::CheckerInfo, 2, Source::Input, &global)?;

    check_cell(CellType::SidechainConfig, 0, Source::Output, &global)?;
    check_cell(CellType::CheckerBond, 1, Source::Output, &global)?;

    Ok(())
}

pub fn is_checker_submit_task() -> Result<(), CommonError> {
    /*
    CheckerSubmitTask,

    Dep:    1 Global Config Cell
    Dep:    2 Sidechain Config Cell

    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          Null

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 2 || output_count != 1 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cell(CellType::CheckerInfo, 0, Source::Input, &global)?;
    check_cell(CellType::Task, 1, Source::Input, &global)?;

    check_cell(CellType::CheckerInfo, 0, Source::Output, &global)?;

    Ok(())
}

pub fn is_checker_publish_challenge() -> Result<(), CommonError> {
    /*
    CheckerPublishChallenge,

    Dep:    1 Global Config Cell
    Dep:    2 Sidechain Config Cell

    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          [Task Cell]

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 2 || output_count < 2 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cell(CellType::CheckerInfo, 0, Source::Input, &global)?;
    check_cell(CellType::Task, 1, Source::Input, &global)?;

    check_cell(CellType::CheckerInfo, 0, Source::Output, &global)?;

    for x in 2..output_count {
        check_cell(CellType::Task, x as usize, Source::Output, &global)?;
    }

    Ok(())
}

pub fn is_checker_submit_challenge() -> Result<(), CommonError> {
    /*
    CheckerSubmitChallenge,

    Dep:    1 Global Config Cell
    Dep:    2 Sidechain Config Cell

    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          Null

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 2 || output_count != 1 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cell(CellType::CheckerInfo, 0, Source::Input, &global)?;
    check_cell(CellType::Task, 1, Source::Input, &global)?;

    check_cell(CellType::CheckerInfo, 0, Source::Output, &global)?;

    Ok(())
}

pub fn is_checker_take_beneficiary() -> Result<(), CommonError> {
    /*
    CheckerTakeBeneficiary,

    Dep:    1 Global Config Cell

    Checker Info Cell           ->          Checker Info Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    Muse Token Cell             ->          Muse Token Cell
    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 3 || output_count != 3 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cell(CellType::CheckerInfo, 0, Source::Input, &global)?;
    check_cell(CellType::SidechainFee, 1, Source::Input, &global)?;
    check_cell(CellType::MuseToken, 2, Source::Input, &global)?;

    check_cell(CellType::CheckerInfo, 0, Source::Output, &global)?;
    check_cell(CellType::SidechainFee, 1, Source::Output, &global)?;
    check_cell(CellType::MuseToken, 2, Source::Output, &global)?;

    Ok(())
}
pub fn is_admin_create_sidechain() -> Result<(), CommonError> {
    /*
    AdminCreateSidechain,

    Dep:    1 Global Config Cell

    Null                        ->          Sidechain Config Cell
    Null                        ->          Sidechain State Cell

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if output_count != 3 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cell(CellType::SidechainConfig, 0, Source::Output, &global)?;
    check_cell(CellType::SidechainState, 1, Source::Output, &global)?;

    Ok(())
}

pub fn is_collator_publish_task() -> Result<(), CommonError> {
    /*
    CollatorPublishTask,

    Dep:    1 Global Config Cell
    Dep:    2 Sidechain Config Cell

    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Bond Cell         ->          Sidechain Bond Cell
    Null                        ->          [Task Cell]

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 2 || output_count < 3 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cell(CellType::SidechainState, 0, Source::Input, &global)?;
    check_cell(CellType::SidechainBond, 1, Source::Input, &global)?;

    check_cell(CellType::SidechainState, 0, Source::Output, &global)?;
    check_cell(CellType::SidechainBond, 1, Source::Output, &global)?;

    for x in 2..output_count {
        check_cell(CellType::Task, x as usize, Source::Output, &global)?;
    }

    Ok(())
}
pub fn is_collator_submit_task() -> Result<(), CommonError> {
    /*
    CollatorSubmitTask,

    Dep:    1 Global Config Cell
    Dep:    2 Sidechain Config Cell

    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    [Checker Info Cell]         ->          [Checker Info Cell]

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count < 3 || output_count < 3 || input_count != output_count {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cell(CellType::SidechainState, 0, Source::Input, &global)?;
    check_cell(CellType::SidechainFee, 1, Source::Input, &global)?;

    check_cell(CellType::SidechainState, 0, Source::Output, &global)?;
    check_cell(CellType::SidechainFee, 1, Source::Output, &global)?;

    for x in 2..input_count {
        check_cell(CellType::CheckerInfo, x as usize, Source::Input, &global)?;
    }

    for x in 2..output_count {
        check_cell(CellType::CheckerInfo, x as usize, Source::Output, &global)?;
    }

    Ok(())
}

pub fn is_collator_submit_challenge() -> Result<(), CommonError> {
    /*
    CollatorSubmitChallenge,

    Dep:    1 Global Config Cell

    Sidechain Config Cell       ->          Sidechain Config Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    [Checker Info Cell]         ->          [Checker Info Cell]

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count < 4 || output_count < 4 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cell(CellType::SidechainConfig, 0, Source::Input, &global)?;
    check_cell(CellType::SidechainState, 1, Source::Input, &global)?;
    check_cell(CellType::SidechainFee, 1, Source::Input, &global)?;

    check_cell(CellType::SidechainConfig, 0, Source::Output, &global)?;
    check_cell(CellType::SidechainState, 1, Source::Output, &global)?;
    check_cell(CellType::SidechainFee, 2, Source::Output, &global)?;

    for x in 3..input_count {
        check_cell(CellType::CheckerInfo, x as usize, Source::Input, &global)?;
    }

    for x in 3..output_count {
        check_cell(CellType::CheckerInfo, x as usize, Source::Output, &global)?;
    }

    Ok(())
}

pub fn is_collator_refresh_task() -> Result<(), CommonError> {
    /*
    CollatorRefreshTask,

    Dep:    1 Global Config Cell

    [Task Cell]                 ->          [Task Cell]

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count > 0 || output_count > 0 || input_count != output_count {
        return Err(CommonError::CellNumberMismatch);
    }

    for x in 0..input_count {
        check_cell(CellType::Task, x as usize, Source::Input, &global)?;
    }

    for x in 0..output_count {
        check_cell(CellType::Task, x as usize, Source::Output, &global)?;
    }

    Ok(())
}

pub fn is_collator_unlock_bond() -> Result<(), CommonError> {
    /*
    CollatorUnlockBond,

    Dep:    1 Global Config Cell
    Dep:    2 Sidechain State Cell

    Sidechain Bond Cell         ->          Muse Token Cell

    */
    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count < 4 || output_count < 4 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cell(CellType::SidechainBond, 0, Source::Input, &global)?;

    check_cell(CellType::MuseToken, 0, Source::Output, &global)?;

    Ok(())
}

pub fn check_global_cell() -> Result<GlobalConfigCellData, CommonError> {
    let global_config_data = load_cell_data(0, Source::CellDep)?;
    let global_config_data = GlobalConfigCellData::from_raw(&global_config_data)?;

    if load_cell_type_hash(0, Source::CellDep)?.ok_or(CommonError::LoadTypeHashError)? != GLOBAL_CONFIG_TYPE_HASH {
        return Err(CommonError::GlobalConfigCellDepError);
    }
    Ok(global_config_data)
}

pub fn check_cell(cell_type: CellType, index: usize, source: Source, global: &GlobalConfigCellData) -> Result<(), CommonError> {
    let cell = load_cell(index, source)?;
    let script = cell.type_().to_opt().ok_or(CommonError::MissingTypeScript)?;
    let codehash = script.code_hash().unpack();
    let hashtype = script.hash_type().as_slice()[0];

    match cell_type {
        CellType::Unknown => return Err(CommonError::UnknownCellType),
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
