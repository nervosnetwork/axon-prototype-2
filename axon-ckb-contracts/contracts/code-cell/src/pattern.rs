use crate::{
    cell::{CellOrigin, LockedTypedSudtCell, TypedCell},
    common::*,
    error::Error,
};

use ckb_std::ckb_constants::Source;

use common_raw::cell::{
    checker_info::CheckerInfoCellData, code::CodeCellData, sidechain_config::SidechainConfigCellData, sidechain_fee::SidechainFeeCellData,
    sidechain_state::SidechainStateCellData,
};

pub fn is_admin_create_sidechain() -> Result<(), Error> {
    /*
    AdminCreateSidechain,

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    CKB Cell                    ->          Sidechain Config Cell
    Null                        ->          Sidechain State Cell

    */

    let global = check_global_cell()?;

    if is_cell_count_not_equals(2, Source::Input) || is_cell_count_not_equals(3, Source::Output) {
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

    if is_cell_count_smaller(5, Source::Input) || is_cell_count_smaller(5, Source::Output) {
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

    CheckerInfoCellData::range_check(4.., Source::Input, &global)?;
    CheckerInfoCellData::range_check(4.., Source::Output, &global)
}
