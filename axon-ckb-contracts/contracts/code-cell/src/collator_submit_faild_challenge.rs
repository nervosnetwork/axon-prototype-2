use crate::{cell::*, common::*, error::Error};
use ckb_std::ckb_constants::Source;
use common_raw::{
    cell::{
        checker_info::{CheckerInfoCellData, CheckerInfoCellMode, CheckerInfoCellTypeArgs},
        code::CodeCell,
        muse_token::MuseTokenData,
        sidechain_config::{SidechainConfigCellData, SidechainConfigCellTypeArgs},
        sidechain_fee::{SidechainFeeCellData, SidechainFeeCellLockArgs},
        sidechain_state::{SidechainStateCellData, SidechainStateCellTypeArgs},
    },
    witness::collator_submit_challenge::CollatorSubmitChallengeWitness,
    FromRaw,
};

const SIDECHAIN_CONFIG_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const SIDECHAIN_STATE_INPUT: CellOrigin = CellOrigin(2, Source::Input);
const SIDECHAIN_FEE_INPUT: CellOrigin = CellOrigin(3, Source::Input);
const MUSE_TOKEN_INPUT: CellOrigin = CellOrigin(4, Source::Input);

const SIDECHAIN_CONFIG_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const SIDECHAIN_STATE_OUTPUT: CellOrigin = CellOrigin(2, Source::Output);
const SIDECHAIN_FEE_OUTPUT: CellOrigin = CellOrigin(3, Source::Output);

const INPUT_NORMAL_CELL_COUNT: usize = 5;
const OUTPUT_NORMAL_CELL_COUNT: usize = INPUT_NORMAL_CELL_COUNT - 1;

pub fn is_collator_submit_faild_challenge(witness: &CollatorSubmitChallengeWitness) -> Result<(), Error> {
    /*
    CollatorSubmitChallenge,

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    Sidechain Config Cell       ->          Sidechain Config Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    Muse Token Cell
    [Checker Info Cell]         ->          [Checker Info Cell]

    */

    let global = check_global_cell()?;

    let checker_info_count = usize::from(
        witness.task_count
            + witness.valid_challenge_count
            + bit_map_count(witness.punish_checker_bitmap).ok_or(Error::CollatorSubmitChallengeWitnessMismatch)?,
    );

    let cell_input_count = INPUT_NORMAL_CELL_COUNT + checker_info_count;
    let cell_output_conut = OUTPUT_NORMAL_CELL_COUNT + usize::from(witness.task_count) + usize::from(witness.valid_challenge_count);

    if is_cell_count_not_equals(cell_input_count, Source::Input) || is_cell_count_not_equals(cell_output_conut, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            CodeCell: CODE_INPUT,
            SidechainConfigCellData: SIDECHAIN_CONFIG_INPUT,
            SidechainStateCellData: SIDECHAIN_STATE_INPUT,
            SidechainFeeCellData: SIDECHAIN_FEE_INPUT,
            MuseTokenData: MUSE_TOKEN_INPUT,
            CodeCell: CODE_OUTPUT,
            SidechainConfigCellData: SIDECHAIN_CONFIG_OUTPUT,
            SidechainStateCellData: SIDECHAIN_STATE_OUTPUT,
            SidechainFeeCellData: SIDECHAIN_FEE_OUTPUT,
        },
    };

    CheckerInfoCellData::range_check(INPUT_NORMAL_CELL_COUNT.., Source::Input, &global)?;
    CheckerInfoCellData::range_check(OUTPUT_NORMAL_CELL_COUNT.., Source::Output, &global)
}

pub fn collator_submit_faild_challenge(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CollatorSubmitChallenge,

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    Sidechain Config Cell       ->          Sidechain Config Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    Muse Token Cell
    [Checker Info Cell]         ->          [Checker Info Cell]

    */

    let witness = CollatorSubmitChallengeWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;
    let punish_challenge_count = bit_map_count(witness.punish_checker_bitmap).ok_or(Error::CollatorSubmitChallengeWitnessMismatch)?;
    let checker_info_count = witness.task_count + witness.valid_challenge_count + punish_challenge_count;
    let valid_checker_info_count = witness.task_count + witness.valid_challenge_count;
    let challenge_count = witness.valid_challenge_count + punish_challenge_count;

    if u128::from(witness.task_count + witness.valid_challenge_count) * witness.fee_per_checker != witness.fee
        || valid_checker_info_count <= punish_challenge_count
    {
        return Err(Error::CollatorSubmitChallengeWitnessMismatch);
    }
    is_collator_submit_faild_challenge(&witness)?;

    //load inputs
    let (
        sidechain_config_data_input,
        sidechain_config_type_args_input,
        sidechain_state_data_input,
        sidechain_state_type_args_input,
        sidechain_fee_data_input,
        sidechain_fee_lock_args_input,
        muse_token_data_input,
    ) = load_entities!(
        SidechainConfigCellData: SIDECHAIN_CONFIG_INPUT,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_INPUT,
        SidechainStateCellData: SIDECHAIN_STATE_INPUT,
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_INPUT,
        SidechainFeeCellData: SIDECHAIN_FEE_INPUT,
        SidechainFeeCellLockArgs: SIDECHAIN_FEE_INPUT,
        MuseTokenData: MUSE_TOKEN_INPUT,
    );

    //load outputs
    let (
        sidechain_config_data_output,
        sidechain_config_type_args_output,
        sidechain_state_data_output,
        sidechain_state_type_args_output,
        sidechain_fee_data_output,
        sidechain_fee_lock_args_output,
    ) = load_entities!(
        SidechainConfigCellData: SIDECHAIN_CONFIG_OUTPUT,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_OUTPUT,
        SidechainStateCellData: SIDECHAIN_STATE_OUTPUT,
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_OUTPUT,
        SidechainFeeCellData: SIDECHAIN_FEE_OUTPUT,
        SidechainFeeCellLockArgs: SIDECHAIN_FEE_OUTPUT,
    );

    if muse_token_data_input.amount != witness.fee {
        return Err(Error::MuseTokenMismatch);
    }

    let mut sidechain_config_data_res = sidechain_config_data_input.clone();
    sidechain_config_data_res.checker_bitmap =
        bit_map_remove_by_batch(sidechain_config_data_res.checker_bitmap, witness.punish_checker_bitmap)
            .ok_or(Error::SidechainConfigMismatch)?;
    sidechain_config_data_res.checker_total_count -= punish_challenge_count;

    if sidechain_config_data_res != sidechain_config_data_output
        || sidechain_config_data_res.collator_lock_arg != signer
        || sidechain_config_type_args_input.chain_id != witness.chain_id
        || sidechain_config_type_args_input != sidechain_config_type_args_output
        || (sidechain_config_data_res.commit_threshold - witness.task_count) * sidechain_config_data_res.challenge_threshold
            != challenge_count
    {
        return Err(Error::SidechainConfigMismatch);
    }

    let mut sidechain_state_data_res = sidechain_state_data_input.clone();
    sidechain_state_data_res.committed_block_height = sidechain_state_data_input.latest_block_height;
    sidechain_state_data_res.committed_block_hash = sidechain_state_data_input.latest_block_hash;

    if sidechain_state_data_res != sidechain_state_data_output
        || sidechain_state_type_args_input != sidechain_state_type_args_output
        || sidechain_state_type_args_input.chain_id != witness.chain_id
    {
        return Err(Error::SidechainStateMismatch);
    }

    let mut sidechain_fee_data_res = sidechain_fee_data_input.clone();
    sidechain_fee_data_res.amount += witness.fee;

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
    for i in INPUT_NORMAL_CELL_COUNT..INPUT_NORMAL_CELL_COUNT + usize::from(witness.valid_challenge_count + witness.task_count) {
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
            CheckerInfoCellMode::TaskPassed => {
                task_count += 1;
            }
            CheckerInfoCellMode::ChallengePassed => {
                valid_challenge_count += 1;
            }
            _ => {
                return Err(Error::CheckerInfoMismatch);
            }
        }
    }

    for i in INPUT_NORMAL_CELL_COUNT + usize::from(valid_checker_info_count)..INPUT_NORMAL_CELL_COUNT + usize::from(checker_info_count) {
        let checker_info_data_input = CheckerInfoCellData::load(CellOrigin(i as usize, Source::Input))?;
        let checker_info_type_args_input = CheckerInfoCellTypeArgs::load(CellOrigin(i as usize, Source::Input))?;
        punish_bit_map_res = bit_map_add(&punish_bit_map_res, checker_info_data_input.checker_id).ok_or(Error::CheckerInfoMismatch)?;
        if checker_info_data_input.mode != CheckerInfoCellMode::ChallengeRejected
            || checker_info_type_args_input.chain_id != witness.chain_id
        {
            return Err(Error::CheckerInfoMismatch);
        }
    }

    if punish_bit_map_res != witness.punish_checker_bitmap
        || valid_challenge_count != witness.valid_challenge_count
        || task_count != witness.task_count
    {
        return Err(Error::CheckerInfoMismatch);
    }

    Ok(())
}
