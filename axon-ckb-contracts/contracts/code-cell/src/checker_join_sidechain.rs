use core::convert::TryFrom;

use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        checker_bond::{CheckerBondCell, CheckerBondCellLockArgs},
        checker_info::{CheckerInfoCell, CheckerInfoCellTypeArgs, CheckerInfoStatus},
        code::CodeCell,
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs},
    },
    witness::checker_join_sidechain::CheckerJoinSidechainWitness,
    FromRaw,
};

use crate::{cell::*, common::*, error::Error};

const CONFIG_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const CHECKER_BOND_INPUT: CellOrigin = CellOrigin(2, Source::Input);

const CONFIG_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const CHECKER_BOND_OUTPUT: CellOrigin = CellOrigin(2, Source::Output);
const CHECKER_INFO_OUTPUT: CellOrigin = CellOrigin(3, Source::Output);

pub fn checker_join_sidechain(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerJoinSidechain,

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Null                        ->          Checker Info Cell

    */
    is_checker_join_sidechain()?;

    let witness = CheckerJoinSidechainWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    let (config_input_type_args, config_input, checker_bond_input_lock_args, checker_bond_input) = load_entities! {
        SidechainConfigCellTypeArgs: CONFIG_INPUT,
        SidechainConfigCell: CONFIG_INPUT,
        CheckerBondCellLockArgs: CHECKER_BOND_INPUT,
        CheckerBondCell: CHECKER_BOND_INPUT,
    };

    let (
        config_output_type_args,
        config_output,
        checker_bond_output_lock_args,
        checker_bond_output,
        checker_info_output_type_args,
        checker_info_output,
    ) = load_entities! {
        SidechainConfigCellTypeArgs: CONFIG_OUTPUT,
        SidechainConfigCell: CONFIG_OUTPUT,
        CheckerBondCellLockArgs: CHECKER_BOND_OUTPUT,
        CheckerBondCell: CHECKER_BOND_OUTPUT,
        CheckerInfoCellTypeArgs: CHECKER_INFO_OUTPUT,
        CheckerInfoCell: CHECKER_INFO_OUTPUT,
    };

    let mut config_res = config_input.clone();
    config_res.checker_total_count += 1;
    config_res.checker_normal_count += 1;
    config_res.activated_checkers.push(signer);

    if config_input_type_args != config_output_type_args
        || config_res != config_output
        || config_input_type_args.chain_id != witness.chain_id
    {
        return Err(Error::SidechainConfigMismatch);
    }
    let mut checker_bond_res_lock_args = checker_bond_input_lock_args.clone();
    if checker_bond_res_lock_args
        .participated_chain_id
        .iter()
        .find(|chain_id| Some(**chain_id) == u32::try_from(witness.chain_id).ok())
        .is_none()
    {
        checker_bond_res_lock_args
            .participated_chain_id
            .push(u32::try_from(witness.chain_id).or(Err(Error::Encoding))?);
    } else {
        return Err(Error::CheckerBondMismatch);
    }

    if checker_bond_res_lock_args != checker_bond_output_lock_args
        || checker_bond_res_lock_args.checker_lock_arg != signer
        || checker_bond_input != checker_bond_output
        || checker_bond_input.amount < config_res.minimal_bond
    {
        return Err(Error::CheckerBondMismatch);
    }

    let mut checker_info_res = checker_info_output.clone();
    checker_info_res.unpaid_fee = 0;
    checker_info_res.status = CheckerInfoStatus::Relaying;

    let mut checker_info_res_type_args = checker_info_output_type_args.clone();
    checker_info_res_type_args.checker_lock_arg = signer;

    if checker_info_res != checker_info_output
        || checker_info_res_type_args != checker_info_output_type_args
        || checker_info_res_type_args.chain_id != witness.chain_id
    {
        return Err(Error::CheckerInfoMismatch);
    }

    Ok(())
}

fn is_checker_join_sidechain() -> Result<(), Error> {
    let global = check_global_cell()?;

    if is_cell_count_not_equals(3, Source::Input) || is_cell_count_not_equals(4, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            CodeCell: CODE_INPUT,
            SidechainConfigCell: CONFIG_INPUT,
            CheckerBondCell: CHECKER_BOND_INPUT,

            CodeCell: CODE_OUTPUT,
            SidechainConfigCell: CONFIG_OUTPUT,
            CheckerBondCell: CHECKER_BOND_OUTPUT,
            CheckerInfoCell: CHECKER_INFO_OUTPUT,
        },
    };

    Ok(())
}
