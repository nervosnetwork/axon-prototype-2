use crate::cell::{check_cell, check_cells, check_global_cell, CellType, FromRaw, GlobalConfigCellData, SidechainConfigCellData};
use crate::error::CommonError;
use crate::{get_input_cell_count, get_output_cell_count, GLOBAL_CONFIG_TYPE_HASH, SUDT_CODEHASH, SUDT_HASHTYPE, SUDT_MUSE_ARGS};
use alloc::vec::Vec;
use ckb_std::high_level::load_cell_type;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{
        packed::{Byte, CellOutput},
        prelude::*,
    },
    high_level::{load_cell, load_cell_data, load_cell_lock_hash, load_cell_type_hash, load_script, load_witness_args, QueryIter},
};

#[repr(u8)]
#[derive(PartialOrd, PartialEq)]
pub enum Pattern {
    Unrecognised = 0u8,

    CreateCodeCell = 1u8,

    AdminCreateSidechain = 2u8,

    CheckerBondDeposit = 3u8,
    CheckerBondWithdraw,
    CheckerJoinSidechain,
    CheckerQuitSidechain,
    CheckerSubmitTask,
    CheckerPublishChallenge,
    CheckerSubmitChallenge,
    CheckerTakeBeneficiary,

    CollatorPublishTask = 11u8,
    CollatorSubmitTask,
    CollatorSubmitChallenge,
    CollatorRefreshTask,
    CollatorUnlockBond,
}

impl From<u8> for Pattern {
    fn from(input: u8) -> Self {
        match input {
            0u8 => Self::Unrecognised,
            1u8 => Self::CreateCodeCell,
            2u8 => Self::AdminCreateSidechain,

            3u8 => Self::CheckerBondDeposit,
            4u8 => Self::CheckerBondWithdraw,
            5u8 => Self::CheckerJoinSidechain,
            6u8 => Self::CheckerQuitSidechain,
            7u8 => Self::CheckerSubmitTask,
            8u8 => Self::CheckerPublishChallenge,
            9u8 => Self::CheckerSubmitChallenge,
            10u8 => Self::CheckerTakeBeneficiary,

            11u8 => Self::CollatorPublishTask,
            12u8 => Self::CollatorSubmitTask,
            13u8 => Self::CollatorSubmitChallenge,
            14u8 => Self::CollatorRefreshTask,
            15u8 => Self::CollatorUnlockBond,
            _ => Self::Unrecognised,
        }
    }
}

pub fn is_checker_bond_deposit() -> Result<(), CommonError> {
    /*
    CheckerBondDeposit

    Muse Token Cell             ->          Check Bond Cell

    No way to monitor this pattern, regard all check bond cell trustless

     */
    Ok(())
}

pub fn is_checker_bond_withdraw() -> Result<(), CommonError> {
    /*
    CheckerBondWithdraw

    Dep:    0 Global Config Cell

    Code Cell                   ->         Code Cell
    Checker Bond Cell           ->         Muse Token Cell

     */
    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 2 || output_count != 2 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![
            (CellType::Code, 0, Source::Input),
            (CellType::CheckerBond, 1, Source::Input),
            (CellType::Code, 0, Source::Output),
            (CellType::MuseToken, 1, Source::Output),
        ],
        &global,
    )
}

pub fn is_checker_join_sidechain() -> Result<(), CommonError> {
    /*
    CheckerJoinSidechain,

    Dep:    0 Global Config Cell

    Code Cell                   ->         Code Cell
    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Null                        ->          Checker Info Cell

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 3 || output_count != 4 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![
            (CellType::Code, 0, Source::Input),
            (CellType::SidechainConfig, 1, Source::Input),
            (CellType::CheckerBond, 2, Source::Input),
            (CellType::Code, 0, Source::Output),
            (CellType::SidechainConfig, 1, Source::Output),
            (CellType::CheckerBond, 2, Source::Output),
            (CellType::CheckerInfo, 3, Source::Output),
        ],
        &global,
    )
}

pub fn is_checker_quit_sidechain() -> Result<(), CommonError> {
    /*
    CheckerQuitSidechain

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Checker Info Cell           ->          Null
    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 4 || output_count != 3 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![
            (CellType::Code, 0, Source::Input),
            (CellType::SidechainConfig, 1, Source::Input),
            (CellType::CheckerBond, 2, Source::Input),
            (CellType::CheckerInfo, 3, Source::Input),
            (CellType::Code, 0, Source::Output),
            (CellType::SidechainConfig, 1, Source::Output),
            (CellType::CheckerBond, 2, Source::Output),
        ],
        &global,
    )
}

pub fn is_checker_submit_task() -> Result<(), CommonError> {
    /*
    CheckerSubmitTask,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->         Code Cell
    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          Null

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 3 || output_count != 2 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![
            (CellType::SidechainConfig, 1, Source::CellDep),
            (CellType::Code, 0, Source::Input),
            (CellType::CheckerInfo, 1, Source::Input),
            (CellType::Task, 2, Source::Input),
            (CellType::Code, 0, Source::Output),
            (CellType::CheckerInfo, 1, Source::Output),
        ],
        &global,
    )
}

pub fn is_checker_publish_challenge() -> Result<(), CommonError> {
    /*
    CheckerPublishChallenge,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->         Code Cell
    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          [Task Cell]

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 3 || output_count < 4 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![
            (CellType::SidechainConfig, 1, Source::CellDep),
            (CellType::Code, 0, Source::Input),
            (CellType::CheckerInfo, 1, Source::Input),
            (CellType::Task, 2, Source::Input),
            (CellType::Code, 0, Source::Output),
            (CellType::CheckerInfo, 1, Source::Output),
        ],
        &global,
    )?;

    for x in 2..output_count {
        check_cell(CellType::Task, x, Source::Output, &global)?;
    }

    Ok(())
}

pub fn is_checker_submit_challenge() -> Result<(), CommonError> {
    /*
    CheckerSubmitChallenge,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->         Code Cell
    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          Null

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 3 || output_count != 2 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![
            (CellType::SidechainConfig, 1, Source::CellDep),
            (CellType::Code, 0, Source::Input),
            (CellType::CheckerInfo, 1, Source::Input),
            (CellType::Task, 2, Source::Input),
            (CellType::Code, 0, Source::Output),
            (CellType::CheckerInfo, 1, Source::Output),
        ],
        &global,
    )?;

    Ok(())
}

pub fn is_checker_take_beneficiary() -> Result<(), CommonError> {
    /*
    CheckerTakeBeneficiary,

    Dep:    0 Global Config Cell

    Code Cell                   ->         Code Cell
    Checker Info Cell           ->          Checker Info Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    Muse Token Cell             ->          Muse Token Cell

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 4 || output_count != 4 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![
            (CellType::Code, 0, Source::Input),
            (CellType::CheckerInfo, 1, Source::Input),
            (CellType::SidechainFee, 2, Source::Input),
            (CellType::MuseToken, 3, Source::Input),
            (CellType::Code, 0, Source::Output),
            (CellType::CheckerInfo, 1, Source::Output),
            (CellType::SidechainFee, 2, Source::Output),
            (CellType::MuseToken, 3, Source::Output),
        ],
        &global,
    )
}
pub fn is_admin_create_sidechain() -> Result<(), CommonError> {
    /*
    AdminCreateSidechain,

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    CKB Cell                    ->          Sidechain Config Cell
    Null                        ->          Sidechain State Cell

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 2 || output_count != 3 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![
            (CellType::Code, 0, Source::Input),
            (CellType::Code, 0, Source::Output),
            (CellType::SidechainConfig, 1, Source::Output),
            (CellType::SidechainState, 2, Source::Output),
        ],
        &global,
    )
}

pub fn is_collator_publish_task() -> Result<(), CommonError> {
    /*
    CollatorPublishTask,

    Dep:    0 Global Config Cell
            1 Sidechain Config Cell
    Code Cell                   ->          Code Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Bond Cell/Sudt    ->          Sidechain Bond Cell
    Null                        ->          [Task Cell]

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count != 3 || output_count < 4 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![
            (CellType::SidechainConfig, 1, Source::CellDep),
            (CellType::Code, 0, Source::Input),
            (CellType::SidechainState, 1, Source::Input),
            (CellType::SidechainBond, 2, Source::Input),
            (CellType::Code, 0, Source::Output),
            (CellType::SidechainState, 1, Source::Output),
            (CellType::SidechainBond, 2, Source::Output),
        ],
        &global,
    )?;

    for x in 3..output_count {
        check_cell(CellType::Task, x as usize, Source::Output, &global)?;
    }

    Ok(())
}
pub fn is_collator_submit_task() -> Result<(), CommonError> {
    /*
    CollatorSubmitTask,

    Dep:    0 Global Config Cell
            1 Sidechain Config Cell

    Code Cell                   ->          Code Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    [Checker Info Cell]         ->          [Checker Info Cell]

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count < 4 || output_count < 4 || input_count != output_count {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![
            (CellType::SidechainConfig, 1, Source::CellDep),
            (CellType::Code, 0, Source::Input),
            (CellType::SidechainState, 1, Source::Input),
            (CellType::SidechainFee, 2, Source::Input),
            (CellType::Code, 0, Source::Output),
            (CellType::SidechainState, 1, Source::Output),
            (CellType::SidechainFee, 2, Source::Output),
        ],
        &global,
    )?;

    for x in 3..input_count {
        check_cell(CellType::CheckerInfo, x as usize, Source::Input, &global)?;
    }

    for x in 3..output_count {
        check_cell(CellType::CheckerInfo, x as usize, Source::Output, &global)?;
    }

    Ok(())
}

pub fn is_collator_submit_challenge() -> Result<(), CommonError> {
    /*
    CollatorSubmitChallenge,

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    Sidechain Config Cell       ->          Sidechain Config Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    [Checker Info Cell]         ->          [Checker Info Cell]

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count < 5 || output_count < 5 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![
            (CellType::Code, 0, Source::Input),
            (CellType::SidechainConfig, 1, Source::Input),
            (CellType::SidechainState, 2, Source::Input),
            (CellType::SidechainFee, 3, Source::Input),
            (CellType::Code, 0, Source::Output),
            (CellType::SidechainConfig, 1, Source::Output),
            (CellType::SidechainState, 2, Source::Output),
            (CellType::SidechainFee, 3, Source::Output),
        ],
        &global,
    )?;

    for x in 4..input_count {
        check_cell(CellType::CheckerInfo, x, Source::Input, &global)?;
    }

    for x in 4..output_count {
        check_cell(CellType::CheckerInfo, x, Source::Output, &global)?;
    }

    Ok(())
}

pub fn is_collator_refresh_task() -> Result<(), CommonError> {
    /*
    CollatorRefreshTask,

    Dep:    0 Global Config Cell
            1 Sidechain Config Cell

    Code Cell                   ->          Code Cell
    [Task Cell]                 ->          [Task Cell]

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count < 2 || output_count < 2 || input_count != output_count {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![
            (CellType::SidechainConfig, 1, Source::CellDep),
            (CellType::Code, 0, Source::Input),
            (CellType::Code, 0, Source::Output),
        ],
        &global,
    )?;

    for x in 1..input_count {
        check_cell(CellType::Task, x as usize, Source::Input, &global)?;
    }

    for x in 1..output_count {
        check_cell(CellType::Task, x as usize, Source::Output, &global)?;
    }

    Ok(())
}

pub fn is_collator_unlock_bond() -> Result<(), CommonError> {
    /*
    CollatorUnlockBond,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell
    Dep:    2 Sidechain State Cell

    Code Cell                   ->          Code Cell
    Sidechain Bond Cell         ->          Sudt Cell

    */
    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count < 2 || output_count < 2 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![
            (CellType::SidechainConfig, 1, Source::CellDep),
            (CellType::SidechainState, 2, Source::CellDep),
            (CellType::Code, 0, Source::Input),
            (CellType::SidechainBond, 1, Source::Input),
            (CellType::Code, 0, Source::Output),
            (CellType::Sudt, 1, Source::Output),
        ],
        &global,
    )
}
pub fn is_create_code_cell() -> Result<(), CommonError> {
    /*
    CreateCodeCell,

    Dep:    0 Global Config Cell

    CKB Cell                    ->          Code Cell

    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();

    if input_count < 1 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(vec![(CellType::Code, 0, Source::Output)], &global)
}

pub fn check_code_cell() -> Result<(), CommonError> {
    /*
    CollatorUnlockBond,

    Dep:    0 Global Config Cell
    Dep:    1 .....
    Code Cell                   ->          Code Cell
    ...
    */

    let global = check_global_cell()?;

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();

    if input_count < 1 || output_count < 2 {
        return Err(CommonError::CellNumberMismatch);
    }

    check_cells(
        vec![(CellType::Code, 0, Source::Input), (CellType::Code, 0, Source::Output)],
        &global,
    )
}
