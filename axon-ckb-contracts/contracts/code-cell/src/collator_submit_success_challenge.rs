use crate::{cell::*, common::*, error::Error};
use ckb_std::ckb_constants::Source;
use common_raw::{
    cell::{
        checker_info::{CheckerInfoCellData, CheckerInfoCellMode, CheckerInfoCellTypeArgs},
        code::CodeCell,
        sidechain_bond::{SidechainBondCellData, SidechainBondCellLockArgs},
        sidechain_config::{SidechainConfigCellData, SidechainConfigCellTypeArgs},
        sidechain_fee::{SidechainFeeCellData, SidechainFeeCellLockArgs},
    },
    witness::collator_submit_challenge::CollatorSubmitChallengeWitness,
    FromRaw,
};

const SIDECHAIN_CONFIG_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const SIDECHAIN_FEE_INPUT: CellOrigin = CellOrigin(2, Source::Input);
const SIDECHAIN_BOND_INPUT: CellOrigin = CellOrigin(3, Source::Input);

const SIDECHAIN_CONFIG_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const SIDECHAIN_FEE_OUTPUT: CellOrigin = CellOrigin(2, Source::Output);

const INPUT_NORMAL_CELL_COUNT: usize = 4;
const OUTPUT_NORMAL_CELL_COUNT: usize = INPUT_NORMAL_CELL_COUNT - 1;
pub fn is_collator_submit_success_challenge(witness: &CollatorSubmitChallengeWitness) -> Result<(), Error> {
    /*
    CollatorSubmitChallenge,

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    Sidechain Config Cell       ->          Sidechain Config Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    SidechainBondCell           ->
    [Checker Info Cell]         ->          [Checker Info Cell]

    */

    let global = check_global_cell()?;

    let cell_input_count = INPUT_NORMAL_CELL_COUNT
        + usize::from(witness.valid_challenge_count)
        + usize::from(bit_map_count(witness.punish_checker_bitmap).ok_or(Error::CollatorSubmitChallengeWitnessMismatch)?);
    let cell_output_conut = OUTPUT_NORMAL_CELL_COUNT + usize::from(witness.valid_challenge_count);

    if is_cell_count_not_equals(cell_input_count, Source::Input) || is_cell_count_not_equals(cell_output_conut, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            CodeCell: CODE_INPUT,
            SidechainConfigCellData: SIDECHAIN_CONFIG_INPUT,
            SidechainFeeCellData: SIDECHAIN_FEE_INPUT,
            SidechainBondCellData: SIDECHAIN_BOND_INPUT,
            CodeCell: CODE_OUTPUT,
            SidechainConfigCellData: SIDECHAIN_CONFIG_OUTPUT,
            SidechainFeeCellData: SIDECHAIN_FEE_OUTPUT,
        },
    };
    CheckerInfoCellData::range_check(INPUT_NORMAL_CELL_COUNT.., Source::Input, &global)?;
    CheckerInfoCellData::range_check(OUTPUT_NORMAL_CELL_COUNT.., Source::Output, &global)
}

pub fn collator_submit_success_challenge(raw_witness: &[u8]) -> Result<(), Error> {
    /*
    CollatorSubmitChallenge,

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    Sidechain Config Cell       ->          Sidechain Config Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    SidechainBondCell           ->
    [Checker Info Cell]         ->          [Checker Info Cell]

    */

    let witness = CollatorSubmitChallengeWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;
    let punish_count = bit_map_count(witness.punish_checker_bitmap).ok_or(Error::CollatorSubmitChallengeWitnessMismatch)?;
    let checker_info_count = witness.valid_challenge_count + punish_count;
    let challenge_count = witness.valid_challenge_count + punish_count - witness.task_count;

    if u128::from(witness.valid_challenge_count) * witness.fee_per_checker != witness.fee || witness.valid_challenge_count <= punish_count {
        return Err(Error::CollatorSubmitChallengeWitnessMismatch);
    }
    is_collator_submit_success_challenge(&witness)?;

    //load inputs
    let (
        sidechain_config_data_input,
        sidechain_config_type_args_input,
        sidechain_fee_data_input,
        sidechain_fee_lock_args_input,
        sidechain_bond_data_input,
        sidechain_bond_lock_args_input,
    ) = load_entities!(
        SidechainConfigCellData: SIDECHAIN_CONFIG_INPUT,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_INPUT,
        SidechainFeeCellData: SIDECHAIN_FEE_INPUT,
        SidechainFeeCellLockArgs: SIDECHAIN_FEE_INPUT,
        SidechainBondCellData: SIDECHAIN_BOND_INPUT,
        SidechainBondCellLockArgs: SIDECHAIN_BOND_INPUT,
    );

    //load outputs
    let (sidechain_config_data_output, sidechain_config_type_args_output, sidechain_fee_data_output, sidechain_fee_lock_args_output) = load_entities!(
        SidechainConfigCellData: SIDECHAIN_CONFIG_OUTPUT,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_OUTPUT,
        SidechainFeeCellData: SIDECHAIN_FEE_OUTPUT,
        SidechainFeeCellLockArgs: SIDECHAIN_FEE_OUTPUT,
    );
    if sidechain_bond_lock_args_input.collator_lock_arg != sidechain_config_data_input.collator_lock_arg {
        return Err(Error::SidechainBondMismatch);
    }

    let mut sidechain_config_data_res = sidechain_config_data_input.clone();
    sidechain_config_data_res.checker_bitmap =
        bit_map_remove_by_batch(sidechain_config_data_res.checker_bitmap, witness.punish_checker_bitmap)
            .ok_or(Error::SidechainConfigMismatch)?;
    sidechain_config_data_res.checker_total_count -= punish_count;

    if sidechain_config_data_res != sidechain_config_data_output
        || sidechain_config_type_args_input.chain_id != witness.chain_id
        || sidechain_config_type_args_input != sidechain_config_type_args_output
        || (sidechain_config_data_res.commit_threshold - witness.task_count) * sidechain_config_data_res.challenge_threshold
            != challenge_count
    {
        return Err(Error::SidechainConfigMismatch);
    }

    let mut sidechain_fee_data_res = sidechain_fee_data_input.clone();
    sidechain_fee_data_res.amount += sidechain_bond_data_input.amount;

    if sidechain_fee_data_res != sidechain_fee_data_output
        || sidechain_fee_lock_args_input != sidechain_fee_lock_args_output
        || sidechain_fee_lock_args_input.chain_id != witness.chain_id
    {
        return Err(Error::SidechainFeeMismatch);
    }

    //load CIC inputs and CIC outputs
    let mut punish_bit_map_res = [0u8; 32];
    let mut task_count = 0u8;
    let mut valid_challenge_count = 0u8;

    for i in INPUT_NORMAL_CELL_COUNT..INPUT_NORMAL_CELL_COUNT + usize::from(witness.valid_challenge_count) {
        let checker_info_data_input = CheckerInfoCellData::load(CellOrigin(i, Source::Input))?;
        let checker_info_type_args_input = CheckerInfoCellTypeArgs::load(CellOrigin(i, Source::Input))?;

        let checker_info_data_output = CheckerInfoCellData::load(CellOrigin(i - 1, Source::Output))?;
        let checker_info_type_args_output = CheckerInfoCellTypeArgs::load(CellOrigin(i - 1, Source::Output))?;

        let mut checker_info_data_res = checker_info_data_input.clone();
        checker_info_data_res.unpaid_fee += witness.fee_per_checker;
        checker_info_data_res.mode = CheckerInfoCellMode::Idle;

        let fee_per_checker = u128::from(sidechain_config_data_res.check_fee_rate) * checker_info_data_res.unpaid_check_data_size;
        if checker_info_data_res != checker_info_data_output
            || fee_per_checker != witness.fee_per_checker
            || checker_info_type_args_input != checker_info_type_args_output
            || checker_info_type_args_input.chain_id != witness.chain_id
        {
            return Err(Error::CheckerInfoMismatch);
        }

        match checker_info_data_input.mode {
            CheckerInfoCellMode::ChallengeRejected => {
                valid_challenge_count += 1;
            }
            _ => {
                return Err(Error::CheckerInfoMismatch);
            }
        }
    }

    for i in INPUT_NORMAL_CELL_COUNT + usize::from(witness.valid_challenge_count)..INPUT_NORMAL_CELL_COUNT + usize::from(checker_info_count)
    {
        let checker_info_data_input = CheckerInfoCellData::load(CellOrigin(i as usize, Source::Input))?;
        let checker_info_type_args_input = CheckerInfoCellTypeArgs::load(CellOrigin(i as usize, Source::Input))?;
        punish_bit_map_res = bit_map_add(&punish_bit_map_res, checker_info_data_input.checker_id).ok_or(Error::CheckerInfoMismatch)?;
        if checker_info_type_args_input.chain_id != witness.chain_id {
            return Err(Error::CheckerInfoMismatch);
        }

        match checker_info_data_input.mode {
            CheckerInfoCellMode::TaskPassed => {
                task_count += 1;
            }

            CheckerInfoCellMode::ChallengePassed => {}

            _ => {
                return Err(Error::CheckerInfoMismatch);
            }
        }
    }

    if task_count != witness.task_count
        || valid_challenge_count != witness.valid_challenge_count
        || punish_bit_map_res != witness.punish_checker_bitmap
    {
        return Err(Error::CheckerInfoMismatch);
    }

    Ok(())
}
