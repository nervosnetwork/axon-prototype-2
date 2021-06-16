use crate::cell::{check_cell, check_cells, check_global_cell, CellType};
use crate::error::CommonError;
use crate::{get_input_cell_count, get_output_cell_count};

use ckb_std::ckb_constants::Source;

use common_raw::witness::checker_submit_task::CheckerSubmitTaskWitness;

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

pub fn is_checker_submit_task(witness: &CheckerSubmitTaskWitness) -> Result<(), CommonError> {
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
            (CellType::SidechainConfig, witness.sidechain_config_dep_index, Source::CellDep),
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
