use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        checker_bond::{CheckerBondCellData, CheckerBondCellLockArgs},
        checker_info::{CheckerInfoCellData, CheckerInfoCellMode, CheckerInfoCellTypeArgs},
        code::CodeCellData,
        sidechain_config::{SidechainConfigCellData, SidechainConfigCellTypeArgs},
    },
    witness::checker_quit_sidechain::CheckerQuitSidechainWitness,
    FromRaw,
};

use crate::{cell::*, common::*, error::Error};

const CONFIG_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const CHECKER_BOND_INPUT: CellOrigin = CellOrigin(2, Source::Input);
const CHECKER_INFO_INPUT: CellOrigin = CellOrigin(3, Source::Input);

const CONFIG_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const CHECKER_BOND_OUTPUT: CellOrigin = CellOrigin(2, Source::Output);

pub fn checker_quit_sidechain(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerQuitSidechain

    Dep:    0 Global Config Cell

    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Checker Info Cell           ->          Null

    */

    is_checker_quit_sidechain()?;

    let witness = CheckerQuitSidechainWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    let (
        config_input_type_args,
        config_input,
        checker_bond_input_lock_args,
        checker_bond_input,
        checker_info_input_type_args,
        checker_info_input,
    ) = load_entities! {
        SidechainConfigCellTypeArgs: CONFIG_INPUT,
        SidechainConfigCellData: CONFIG_INPUT,
        CheckerBondCellLockArgs: CHECKER_BOND_INPUT,
        CheckerBondCellData: CHECKER_BOND_INPUT,
        CheckerInfoCellTypeArgs: CHECKER_INFO_INPUT,
        CheckerInfoCellData: CHECKER_INFO_INPUT,
    };
    let (config_output, checker_bond_output_lock_args, checker_bond_output) = load_entities! {
        SidechainConfigCellData: CONFIG_OUTPUT,
        CheckerBondCellLockArgs: CHECKER_BOND_OUTPUT,
        CheckerBondCellData: CHECKER_BOND_OUTPUT,
    };

    let mut config_res = config_input.clone();
    if config_res.checker_total_count <= 0 {
        return Err(Error::SidechainConfigMismatch);
    }
    config_res.checker_total_count -= 1;
    config_res.checker_bitmap = bit_map_remove(config_res.checker_bitmap, witness.checker_id).ok_or(Error::SidechainConfigMismatch)?;

    let mut checker_bond_res_lock_args = checker_bond_input_lock_args.clone();
    checker_bond_res_lock_args.chain_id_bitmap =
        bit_map_remove(checker_bond_res_lock_args.chain_id_bitmap, witness.chain_id).ok_or(Error::CheckerBondMismatch)?;

    has_sidechain_config_passed_update_interval(config_input, CONFIG_INPUT)?;
    if config_input_type_args.chain_id != witness.chain_id || config_res != config_output {
        return Err(Error::SidechainConfigMismatch);
    }
    if checker_bond_res_lock_args != checker_bond_output_lock_args
        || checker_bond_input_lock_args.checker_lock_arg != signer
        || checker_bond_input != checker_bond_output
    {
        return Err(Error::CheckerBondMismatch);
    }
    if checker_info_input_type_args.chain_id != witness.chain_id
        || checker_info_input.checker_id != witness.checker_id
        || checker_info_input_type_args.checker_lock_arg != signer
        || checker_info_input.mode != CheckerInfoCellMode::Idle
    {
        return Err(Error::CheckerInfoMismatch);
    }

    Ok(())
}

fn is_checker_quit_sidechain() -> Result<(), Error> {
    let global = check_global_cell()?;

    if is_cell_count_not_equals(4, Source::Input) || is_cell_count_not_equals(3, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            CodeCellData: CODE_INPUT,
            SidechainConfigCellData: CONFIG_INPUT,
            CheckerBondCellData: CHECKER_BOND_INPUT,
            CheckerInfoCellData: CHECKER_INFO_INPUT,

            CodeCellData: CODE_OUTPUT,
            SidechainConfigCellData: CONFIG_OUTPUT,
            CheckerBondCellData: CHECKER_BOND_OUTPUT,
        },
    };

    Ok(())
}
