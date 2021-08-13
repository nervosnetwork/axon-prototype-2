use crate::{cell::*, common::*, error::Error};
use ckb_std::ckb_constants::Source;
use common_raw::{
    cell::{
        code::CodeCell,
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs, SidechainStatus},
        sidechain_state::{SidechainStateCell, SidechainStateCellTypeArgs},
    },
    witness::collator_shutdown_sidechain::CollatorShutdownSidechainWitness,
    FromRaw,
};
use core::convert::TryFrom;

const SIDECHAIN_STATE_DEP: CellOrigin = CellOrigin(5, Source::CellDep);

const SIDECHAIN_CONFIG_INPUT: CellOrigin = CellOrigin(1, Source::Input);

const SIDECHAIN_CONFIG_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);

pub fn collator_shutdown_sidechain(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    AnyoneShutdownSidechain,

    Dep:    0 Global Config Cell

    Code Cell                   -> ~
    Sidechain Config Cell       -> ~
    Sidechain Fee Cell          -> ~
    */
    is_collator_shutdown_sidechain()?;

    let witness = CollatorShutdownSidechainWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    //load entities
    let (state_dep_type_args, state_dep, config_input_type_args, config_input, config_output_type_args, config_output) = load_entities!(
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_DEP,
        SidechainStateCell: SIDECHAIN_STATE_DEP,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_INPUT,
        SidechainConfigCell: SIDECHAIN_CONFIG_INPUT,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_OUTPUT,
        SidechainConfigCell: SIDECHAIN_CONFIG_OUTPUT,
    );
    if !state_dep.confirmed_jobs.is_empty()
        || !state_dep.waiting_jobs.is_empty()
        || state_dep_type_args.chain_id != u32::try_from(witness.chain_id).or(Err(Error::Encoding))?
    {
        return Err(Error::SidechainStateMismatch);
    }

    let mut config_res = config_input.clone();
    config_res.sidechain_status = SidechainStatus::Shutdown;

    if config_input.sidechain_status != SidechainStatus::Relaying
        || config_res.collator_lock_arg != signer
        || config_res != config_output
        || config_input_type_args != config_output_type_args
        || config_input_type_args.chain_id != witness.chain_id
    {
        return Err(Error::SidechainConfigMismatch);
    }

    Ok(())
}

fn is_collator_shutdown_sidechain() -> Result<(), Error> {
    let global = check_global_cell()?;

    if is_cell_count_not_equals(2, Source::Input) || is_cell_count_not_equals(2, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }
    check_cells! {
        &global,
        {
            SidechainStateCell: SIDECHAIN_STATE_DEP,

            CodeCell: CODE_INPUT,
            SidechainConfigCell: SIDECHAIN_CONFIG_INPUT,

            CodeCell: CODE_OUTPUT,
            SidechainConfigCell: SIDECHAIN_CONFIG_OUTPUT,
        },
    };

    Ok(())
}
