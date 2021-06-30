use crate::{cell::*, common::*, error::Error};
use ckb_std::ckb_constants::Source;
use common_raw::FromRaw;

const SIDECHAIN_CONFIG_DEP: CellOrigin = CellOrigin(5, Source::CellDep);
const SIDECHAIN_STATE_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const SIDECHAIN_FEE_INPUT: CellOrigin = CellOrigin(2, Source::Input);
pub fn is_collator_submit_challenge() -> Result<(), Error> {
    /*
    CollatorSubmitChallenge,

    Dep:    0 Global Config Cell
            1 Sidechain Config Cell

    Code Cell                   ->          Code Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    Muse Token Cell
    [Checker Info Cell]         ->          [Checker Info Cell]

    */

    let global = check_global_cell()?;

    if is_cell_count_smaller(5, Source::Input) || is_cell_count_smaller(5, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCellData: CellOrigin(5, Source::CellDep),
            CodeCellData: CODE_INPUT,
            SidechainStateCellData: CellOrigin(1, Source::Input),
            SidechainFeeCellData: CellOrigin(2, Source::Input),
            CodeCellData: CODE_OUTPUT,
            SidechainStateCellData: CellOrigin(1, Source::Output),
            SidechainFeeCellData: CellOrigin(2, Source::Output),
        },
    };

    CheckerInfoCellData::range_check(4.., Source::Input, &global)?;
    CheckerInfoCellData::range_check(4.., Source::Output, &global)
}
