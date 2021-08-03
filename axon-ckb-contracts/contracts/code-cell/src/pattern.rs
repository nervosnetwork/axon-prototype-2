use crate::{
    cell::{CellOrigin, TypedCell},
    common::*,
    error::Error,
};

use ckb_std::ckb_constants::Source;

use common_raw::cell::{code::CodeCell, sidechain_config::SidechainConfigCellData, sidechain_state::SidechainStateCellData};

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
            CodeCell: CellOrigin(0, Source::Input),
            CodeCell: CellOrigin(0, Source::Output),
            SidechainConfigCellData: CellOrigin(1, Source::Output),
            SidechainStateCellData: CellOrigin(2, Source::Output),
        },
    };

    Ok(())
}
