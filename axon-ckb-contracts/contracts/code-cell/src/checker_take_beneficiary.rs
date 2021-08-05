use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        checker_info::{CheckerInfoCell, CheckerInfoCellTypeArgs},
        code::CodeCell,
        muse_token::MuseTokenCell,
        sidechain_fee::{SidechainFeeCell, SidechainFeeCellLockArgs},
    },
    witness::checker_take_beneficiary::CheckerTakeBeneficiaryWitness,
    FromRaw,
};

use crate::{cell::*, common::*, error::Error};

const CHECKER_INFO_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const FEE_INPUT: CellOrigin = CellOrigin(2, Source::Input);
const MUSE_INPUT: CellOrigin = CellOrigin(3, Source::Input);

const CHECKER_INFO_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const FEE_OUTPUT: CellOrigin = CellOrigin(2, Source::Output);
const MUSE_OUTPUT: CellOrigin = CellOrigin(3, Source::Output);

pub fn checker_take_beneficiary(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerTakeBeneficiary,

    Dep:    0 Global Config Cell

    Code Cell                   ->         Code Cell
    Checker Info Cell           ->          Checker Info Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    Muse Token Cell             ->          Muse Token Cell

    */

    is_checker_take_beneficiary()?;

    let witness = CheckerTakeBeneficiaryWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    let (checker_info_input_type_args, checker_info_input, sidechain_fee_input_lock_args, sidechain_fee_input, muse_token_input) = load_entities! {
        CheckerInfoCellTypeArgs: CHECKER_INFO_INPUT,
        CheckerInfoCell: CHECKER_INFO_INPUT,
        SidechainFeeCellLockArgs: FEE_INPUT,
        SidechainFeeCell: FEE_INPUT,
        MuseTokenCell: MUSE_INPUT,
    };
    let (checker_info_output, sidechain_fee_output_lock_args, sidechain_fee_output, muse_token_output) = load_entities! {
        CheckerInfoCell: CHECKER_INFO_OUTPUT,
        SidechainFeeCellLockArgs: FEE_OUTPUT,
        SidechainFeeCell: FEE_OUTPUT,
        MuseTokenCell: MUSE_OUTPUT,
    };

    if checker_info_input.unpaid_fee < witness.fee {
        return Err(Error::CheckerInfoMismatch);
    }

    let mut checker_info_res = checker_info_input.clone();
    checker_info_res.unpaid_fee -= witness.fee;

    if sidechain_fee_input.amount < witness.fee {
        return Err(Error::SidechainFeeMismatch);
    }

    let mut sidechain_fee_res = sidechain_fee_input.clone();
    sidechain_fee_res.amount -= witness.fee;

    let mut muse_token_res = muse_token_input.clone();
    muse_token_res.amount += witness.fee;

    if checker_info_input_type_args.chain_id != witness.chain_id
        || checker_info_input_type_args.checker_lock_arg != signer
        || checker_info_res != checker_info_output
    {
        return Err(Error::CheckerInfoMismatch);
    }
    if sidechain_fee_res != sidechain_fee_output
        || sidechain_fee_input_lock_args.chain_id != witness.chain_id
        || sidechain_fee_input_lock_args != sidechain_fee_output_lock_args
    {
        return Err(Error::SidechainFeeMismatch);
    }
    if muse_token_res != muse_token_output {
        return Err(Error::MuseTokenMismatch);
    }

    Ok(())
}

fn is_checker_take_beneficiary() -> Result<(), Error> {
    let global = check_global_cell()?;

    if is_cell_count_not_equals(4, Source::Input) || is_cell_count_not_equals(4, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            CodeCell: CODE_INPUT,
            CheckerInfoCell: CHECKER_INFO_INPUT,
            SidechainFeeCell: FEE_INPUT,
            MuseTokenCell: MUSE_INPUT,

            CodeCell: CODE_OUTPUT,
            CheckerInfoCell: CHECKER_INFO_OUTPUT,
            SidechainFeeCell: FEE_OUTPUT,
            MuseTokenCell: MUSE_OUTPUT,
        },
    };

    Ok(())
}
