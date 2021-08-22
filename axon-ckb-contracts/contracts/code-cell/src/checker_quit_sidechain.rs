use core::convert::TryFrom;

use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        checker_bond::{CheckerBondCell, CheckerBondCellLockArgs},
        checker_info::{CheckerInfoCell, CheckerInfoCellTypeArgs, CheckerInfoStatus},
        code::CodeCell,
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs, SidechainStatus},
        sidechain_state::SidechainStateCell,
    },
    witness::checker_quit_sidechain::CheckerQuitSidechainWitness,
    FromRaw,
};

use crate::{cell::*, common::*, error::Error};

const STATE_DEP: CellOrigin = CellOrigin(5, Source::CellDep);

const CONFIG_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const CHECKER_BOND_INPUT: CellOrigin = CellOrigin(2, Source::Input);
const CHECKER_INFO_INPUT: CellOrigin = CellOrigin(3, Source::Input);

const CONFIG_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const CHECKER_BOND_OUTPUT: CellOrigin = CellOrigin(2, Source::Output);
const CHECKER_INFO_OUTPUT: CellOrigin = CellOrigin(3, Source::Output);

pub fn checker_quit_sidechain(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerQuitSidechain

    Dep:    0 Global Config Cell

    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Checker Info Cell           ->          Checker Info Cell

    */

    is_checker_quit_sidechain()?;

    let witness = CheckerQuitSidechainWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;
    let wit_chain_id = u32::try_from(witness.chain_id).or(Err(Error::Encoding))?;
    let (state_dep, state_dep_type_args) = load_entities!(SidechainStateCell: STATE_DEP, SidechainConfigCellTypeArgs: STATE_DEP,);
    let (
        config_input_type_args,
        config_input,
        checker_bond_input_lock_args,
        checker_bond_input,
        checker_info_input_type_args,
        checker_info_input,
    ) = load_entities! {
        SidechainConfigCellTypeArgs: CONFIG_INPUT,
        SidechainConfigCell: CONFIG_INPUT,
        CheckerBondCellLockArgs: CHECKER_BOND_INPUT,
        CheckerBondCell: CHECKER_BOND_INPUT,
        CheckerInfoCellTypeArgs: CHECKER_INFO_INPUT,
        CheckerInfoCell: CHECKER_INFO_INPUT,
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
    let index = config_res
        .activated_checkers
        .iter()
        .position(|checker_lock_arg| *checker_lock_arg == signer)
        .ok_or(Error::SidechainConfigMismatch)?;

    if config_res.sidechain_status == SidechainStatus::Shutdown {
        return Ok(());
    }

    config_res.checker_total_count -= 1;
    config_res.checker_normal_count -= 1;
    config_res.activated_checkers.remove(index);
    if config_res != config_output
        || config_input_type_args != config_output_type_args
        || config_output_type_args.chain_id != witness.chain_id
    {
        return Err(Error::SidechainConfigMismatch);
    }

    if state_dep
        .punish_checkers
        .iter()
        .find(|checker| checker.checker_lock_arg == signer)
        .is_some()
    {
        return Err(Error::SidechainStateMismatch);
    }

    if u32::try_from(state_dep_type_args.chain_id).or(Err(Error::Encoding))? != wit_chain_id {
        return Err(Error::SidechainStateMismatch);
    }

    let mut checker_bond_res_lock_args = checker_bond_input_lock_args.clone();
    let index = checker_bond_res_lock_args
        .participated_chain_id
        .iter()
        .position(|chain_id| *chain_id == wit_chain_id)
        .ok_or(Error::CheckerBondMismatch)?;
    checker_bond_res_lock_args.participated_chain_id.remove(index);

    if checker_bond_res_lock_args != checker_bond_output_lock_args
        || checker_bond_input_lock_args.checker_lock_arg != signer
        || checker_bond_input != checker_bond_output
    {
        return Err(Error::CheckerBondMismatch);
    }
    let mut checker_info_res = checker_info_input.clone();
    checker_info_res.status = CheckerInfoStatus::Quit;

    if checker_info_input_type_args.chain_id != witness.chain_id
        || checker_info_input.status != CheckerInfoStatus::Relaying
        || checker_info_input_type_args.checker_lock_arg != signer
        || checker_info_input_type_args != checker_info_output_type_args
        || checker_info_res != checker_info_output
    {
        return Err(Error::CheckerInfoMismatch);
    }

    Ok(())
}

fn is_checker_quit_sidechain() -> Result<(), Error> {
    let global = check_global_cell()?;

    if is_cell_count_not_equals(4, Source::Input) || is_cell_count_not_equals(4, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainStateCell: STATE_DEP,

            CodeCell: CODE_INPUT,
            SidechainConfigCell: CONFIG_INPUT,
            CheckerBondCell: CHECKER_BOND_INPUT,
            CheckerInfoCell: CHECKER_INFO_INPUT,

            CodeCell: CODE_OUTPUT,
            SidechainConfigCell: CONFIG_OUTPUT,
            CheckerBondCell: CHECKER_BOND_OUTPUT,
            CheckerInfoCell: CHECKER_INFO_OUTPUT,
        },
    };

    Ok(())
}
