use ckb_std::ckb_constants::Source;
use common_raw::{
    cell::{
        checker_info::{CheckerInfoCell, CheckerInfoCellTypeArgs},
        code::CodeCell,
        muse_token::MuseTokenData,
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs},
        sidechain_fee::{SidechainFeeCellData, SidechainFeeCellLockArgs},
        sidechain_state::{SidechainStateCellData, SidechainStateCellTypeArgs},
    },
    witness::collator_submit_task::CollatorSubmitTaskWitness,
    FromRaw,
};
use core::usize;

use crate::{cell::*, common::*, error::Error};

const SIDECHAIN_CONFIG_DEP: CellOrigin = CellOrigin(5, Source::CellDep);

const SIDECHAIN_STATE_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const SIDECHAIN_FEE_INPUT: CellOrigin = CellOrigin(2, Source::Input);
const MUSE_TOKEN_INPUT: CellOrigin = CellOrigin(3, Source::Input);

const SIDECHAIN_STATE_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const SIDECHAIN_FEE_OUTPUT: CellOrigin = CellOrigin(2, Source::Output);

fn is_collator_submit_task(sidechain_config_dep: &SidechainConfigCell) -> Result<(), Error> {
    /*
    CollatorSubmitTask,

    Dep:    0 Global Config Cell
            1 Sidechain Config Cell

    Code Cell                   ->          Code Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    Muse token Cell

    [Checker Info Cell]         ->          [Checker Info Cell]

    */
    let global = check_global_cell()?;

    if is_cell_count_not_equals(sidechain_config_dep.commit_threshold as usize + 4, Source::Input)
        || is_cell_count_not_equals(sidechain_config_dep.commit_threshold as usize + 3, Source::Output)
    {
        return Err(Error::CellNumberMismatch);
    }
    check_cells! {
        &global,
        {
            SidechainConfigCell: SIDECHAIN_CONFIG_DEP,
            CodeCell: CODE_INPUT,
            SidechainStateCellData: SIDECHAIN_STATE_INPUT,
            SidechainFeeCellData: SIDECHAIN_FEE_INPUT,
            MuseTokenData: MUSE_TOKEN_INPUT,
            CodeCell: CODE_OUTPUT,
            SidechainStateCellData: SIDECHAIN_STATE_OUTPUT,
            SidechainFeeCellData: SIDECHAIN_FEE_OUTPUT,
        },
    };

    CheckerInfoCell::range_check(4..4 + sidechain_config_dep.commit_threshold as usize, Source::Input, &global)?;
    CheckerInfoCell::range_check(3..3 + sidechain_config_dep.commit_threshold as usize, Source::Output, &global)?;
    Ok(())
}

pub fn collator_submit_task(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CollatorSubmitTask,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->          Code Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    Muse token Cell

    [Checker Info Cell]         ->          [Checker Info Cell]

    */

    let witness = CollatorSubmitTaskWitness::from_raw(&raw_witness).ok_or(Error::Encoding)?;

    //load deps
    let (sidechain_config_dep, sidechain_config_type_args_dep) = load_entities!(
        SidechainConfigCell: SIDECHAIN_CONFIG_DEP,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_DEP,
    );

    if sidechain_config_type_args_dep.chain_id != witness.chain_id
        || sidechain_config_dep.collator_lock_arg != signer
        || sidechain_config_dep.commit_threshold as u128 * witness.fee_per_checker != witness.fee
    {
        return Err(Error::SidechainConfigMismatch);
    }

    is_collator_submit_task(&sidechain_config_dep)?;
    //load inputs
    let (sidechain_state_input, sidechain_state_type_args_input, sidechain_fee_input, sidechain_fee_lock_args_input, muse_token_input) = load_entities!(
        SidechainStateCellData: SIDECHAIN_STATE_INPUT,
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_INPUT,
        SidechainFeeCellData: SIDECHAIN_FEE_INPUT,
        SidechainFeeCellLockArgs: SIDECHAIN_FEE_INPUT,
        MuseTokenData: MUSE_TOKEN_INPUT,
    );
    if muse_token_input.amount != witness.fee {
        return Err(Error::MuseTokenMismatch);
    }

    let mut sidechain_state_res = sidechain_state_input.clone();
    sidechain_state_res.committed_block_hash = sidechain_state_res.latest_block_hash;
    sidechain_state_res.committed_block_height = sidechain_state_res.latest_block_height;

    let mut sidechain_fee_res = sidechain_fee_input.clone();
    sidechain_fee_res.amount += witness.fee;

    //load outputs
    let (sidechain_state_output, sidechain_state_type_args_output, sidechain_fee_output, sidechain_fee_lock_args_output) = load_entities!(
        SidechainStateCellData: SIDECHAIN_STATE_OUTPUT,
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_OUTPUT,
        SidechainFeeCellData: SIDECHAIN_FEE_OUTPUT,
        SidechainFeeCellLockArgs: SIDECHAIN_FEE_OUTPUT,
    );

    if sidechain_state_res != sidechain_state_output || sidechain_state_type_args_input != sidechain_state_type_args_output {
        return Err(Error::SidechainStateMismatch);
    }
    if sidechain_fee_res != sidechain_fee_output || sidechain_fee_lock_args_input != sidechain_fee_lock_args_output {
        return Err(Error::SidechainFeeMismatch);
    }

    //load checker info inputs and outputs
    for i in 0..sidechain_config_dep.commit_threshold as usize {
        let (checker_info_input, checker_info_type_args_input, checker_info_output, checker_info_type_args_output) = load_entities!(
            CheckerInfoCell: CellOrigin(4 + i, Source::Input),
            CheckerInfoCellTypeArgs: CellOrigin(4 + i, Source::Input),
            CheckerInfoCell: CellOrigin(3 + i, Source::Output),
            CheckerInfoCellTypeArgs: CellOrigin(3 + i, Source::Output),
        );
        let mut checker_info_res = checker_info_input.clone();
        checker_info_res.unpaid_fee += witness.fee_per_checker;

        if checker_info_type_args_input.chain_id != witness.chain_id
            && checker_info_res != checker_info_output
            && checker_info_type_args_input != checker_info_type_args_output
        {
            return Err(Error::CheckerInfoMismatch);
        }
    }

    Ok(())
}
