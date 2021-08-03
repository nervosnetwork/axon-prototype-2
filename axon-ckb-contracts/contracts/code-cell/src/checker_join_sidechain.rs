use ckb_std::ckb_constants::Source;

use common_raw::{
    cell::{
        checker_bond::{CheckerBondCellData, CheckerBondCellLockArgs},
        checker_info::{CheckerInfoCellData, CheckerInfoCellMode, CheckerInfoCellTypeArgs},
        code::CodeCell,
        sidechain_config::{SidechainConfigCellData, SidechainConfigCellTypeArgs},
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
        SidechainConfigCellData: CONFIG_INPUT,
        CheckerBondCellLockArgs: CHECKER_BOND_INPUT,
        CheckerBondCellData: CHECKER_BOND_INPUT,
    };
    let (config_output, checker_bond_output_lock_args, checker_bond_output, checker_info_output_type_args, checker_info_output) = load_entities! {
        SidechainConfigCellData: CONFIG_OUTPUT,
        CheckerBondCellLockArgs: CHECKER_BOND_OUTPUT,
        CheckerBondCellData: CHECKER_BOND_OUTPUT,
        CheckerInfoCellTypeArgs: CHECKER_INFO_OUTPUT,
        CheckerInfoCellData: CHECKER_INFO_OUTPUT,
    };

    let mut config_res = config_input.clone();
    config_res.checker_total_count += 1;
    config_res.checker_bitmap = bit_map_add(&config_res.checker_bitmap, witness.checker_id).ok_or(Error::SidechainConfigMismatch)?;

    let mut checker_bond_res_lock_args = checker_bond_input_lock_args.clone();
    checker_bond_res_lock_args.chain_id_bitmap =
        bit_map_add(&checker_bond_res_lock_args.chain_id_bitmap, witness.chain_id).ok_or(Error::CheckerBondMismatch)?;

    let mut checker_info_res = checker_info_output.clone();
    checker_info_res.checker_id = witness.checker_id;
    checker_info_res.unpaid_fee = 0;
    checker_info_res.mode = CheckerInfoCellMode::Idle;

    let mut checker_info_res_type_args = checker_info_output_type_args.clone();
    checker_info_res_type_args.chain_id = witness.chain_id;
    checker_info_res_type_args.checker_lock_arg = signer;

    has_sidechain_config_passed_update_interval(config_input, CONFIG_INPUT)?;
    if config_input_type_args.chain_id != witness.chain_id || config_res != config_output {
        return Err(Error::SidechainConfigMismatch);
    }
    if checker_bond_res_lock_args != checker_bond_output_lock_args
        || checker_bond_input_lock_args.checker_lock_arg != signer
        || checker_bond_input != checker_bond_output
        || checker_bond_output.amount < config_input.minimal_bond
    {
        return Err(Error::CheckerBondMismatch);
    }
    if checker_info_res != checker_info_output || checker_info_res_type_args != checker_info_output_type_args {
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
            SidechainConfigCellData: CONFIG_INPUT,
            CheckerBondCellData: CHECKER_BOND_INPUT,

            CodeCell: CODE_OUTPUT,
            SidechainConfigCellData: CONFIG_OUTPUT,
            CheckerBondCellData: CHECKER_BOND_OUTPUT,
            CheckerInfoCellData: CHECKER_INFO_OUTPUT,
        },
    };

    Ok(())
}
