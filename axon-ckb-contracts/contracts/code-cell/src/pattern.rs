use crate::{
    cell::{CellOrigin, LockedTypedSudtCell, TypedCell, TypedSudtCell},
    common::*,
    error::Error,
};

use ckb_std::ckb_constants::Source;

use common_raw::cell::{
    checker_info::CheckerInfoCellData, code::CodeCellData, sidechain_bond::SidechainBondCellData,
    sidechain_config::SidechainConfigCellData, sidechain_fee::SidechainFeeCellData, sidechain_state::SidechainStateCellData,
    sudt_token::SudtTokenData, task::TaskCellData,
};

pub fn is_checker_publish_challenge() -> Result<(), Error> {
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
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCellData: CellOrigin(1, Source::CellDep),
            CodeCellData: CellOrigin(0, Source::Input),
            CheckerInfoCellData: CellOrigin(1, Source::Input),
            TaskCellData: CellOrigin(2, Source::Input),
            CodeCellData: CellOrigin(0, Source::Output),
            CheckerInfoCellData: CellOrigin(1, Source::Output),
        },
    };

    for x in 2..output_count {
        TaskCellData::check(CellOrigin(x, Source::Output), &global)?;
    }

    Ok(())
}

pub fn is_checker_submit_challenge() -> Result<(), Error> {
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
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCellData: CellOrigin(1, Source::CellDep),
            CodeCellData: CellOrigin(0, Source::Input),
            CheckerInfoCellData: CellOrigin(1, Source::Input),
            TaskCellData: CellOrigin(2, Source::Input),
            CodeCellData: CellOrigin(0, Source::Output),
            CheckerInfoCellData: CellOrigin(1, Source::Output),
        },
    };

    Ok(())
}

pub fn is_admin_create_sidechain() -> Result<(), Error> {
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
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            CodeCellData: CellOrigin(0, Source::Input),
            CodeCellData: CellOrigin(0, Source::Output),
            SidechainConfigCellData: CellOrigin(1, Source::Output),
            SidechainStateCellData: CellOrigin(2, Source::Output),
        },
    };

    Ok(())
}

pub fn is_collator_publish_task() -> Result<(), Error> {
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
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCellData: CellOrigin(1, Source::CellDep),
            CodeCellData: CellOrigin(0, Source::Input),
            SidechainStateCellData: CellOrigin(1, Source::Input),
            SidechainBondCellData: CellOrigin(2, Source::Input),
            CodeCellData: CellOrigin(0, Source::Output),
            SidechainStateCellData: CellOrigin(1, Source::Output),
            SidechainBondCellData: CellOrigin(2, Source::Output),
        },
    };

    for x in 3..output_count {
        TaskCellData::check(CellOrigin(x as usize, Source::Output), &global)?;
    }

    Ok(())
}
pub fn is_collator_submit_task() -> Result<(), Error> {
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
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCellData: CellOrigin(1, Source::CellDep),
            CodeCellData: CellOrigin(0, Source::Input),
            SidechainStateCellData: CellOrigin(1, Source::Input),
            SidechainFeeCellData: CellOrigin(2, Source::Input),
            CodeCellData: CellOrigin(0, Source::Output),
            SidechainStateCellData: CellOrigin(1, Source::Output),
            SidechainFeeCellData: CellOrigin(2, Source::Output),
        },
    };

    for x in 3..input_count {
        CheckerInfoCellData::check(CellOrigin(x as usize, Source::Input), &global)?;
    }

    for x in 3..output_count {
        CheckerInfoCellData::check(CellOrigin(x as usize, Source::Output), &global)?;
    }

    Ok(())
}

pub fn is_collator_submit_challenge() -> Result<(), Error> {
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
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            CodeCellData: CellOrigin(0, Source::Input),
            SidechainConfigCellData: CellOrigin(1, Source::Input),
            SidechainStateCellData: CellOrigin(2, Source::Input),
            SidechainFeeCellData: CellOrigin(3, Source::Input),
            CodeCellData: CellOrigin(0, Source::Output),
            SidechainConfigCellData: CellOrigin(1, Source::Output),
            SidechainStateCellData: CellOrigin(2, Source::Output),
            SidechainFeeCellData: CellOrigin(3, Source::Output),
        },
    };

    for x in 4..input_count {
        CheckerInfoCellData::check(CellOrigin(x, Source::Input), &global)?;
    }

    for x in 4..output_count {
        CheckerInfoCellData::check(CellOrigin(x, Source::Output), &global)?;
    }

    Ok(())
}

pub fn is_collator_refresh_task() -> Result<(), Error> {
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
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCellData: CellOrigin(1, Source::CellDep),
            CodeCellData: CellOrigin(0, Source::Input),
            CodeCellData: CellOrigin(0, Source::Output),
        },
    };

    for x in 1..input_count {
        TaskCellData::check(CellOrigin(x as usize, Source::Input), &global)?;
    }

    for x in 1..output_count {
        TaskCellData::check(CellOrigin(x as usize, Source::Output), &global)?;
    }

    Ok(())
}

pub fn is_collator_unlock_bond() -> Result<(), Error> {
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
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCellData: CellOrigin(1, Source::CellDep),
            SidechainStateCellData: CellOrigin(2, Source::CellDep),
            CodeCellData: CellOrigin(0, Source::Input),
            SidechainBondCellData: CellOrigin(1, Source::Input),
            CodeCellData: CellOrigin(0, Source::Output),
            SudtTokenData: CellOrigin(1, Source::Output),
        },
    };

    Ok(())
}
