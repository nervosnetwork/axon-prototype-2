use core::convert::TryFrom;

use ckb_std::ckb_constants::Source;

use common_raw::cell::muse_token::MuseTokenCell;
use common_raw::cell::sidechain_bond::{SidechainBondCell, SidechainBondCellLockArgs};
use common_raw::cell::sidechain_config::SidechainConfigCellTypeArgs;
use common_raw::cell::sidechain_fee::{SidechainFeeCell, SidechainFeeCellLockArgs};
use common_raw::cell::sidechain_state::{SidechainStateCell, SidechainStateCellTypeArgs};
use common_raw::cell::task::TaskCellTypeArgs;
use common_raw::common::BlockSlice;
use common_raw::{
    cell::{code::CodeCell, sidechain_config::SidechainConfigCell, task::TaskCell},
    witness::collator_publish_task::CollatorPublishTaskWitness,
    FromRaw,
};

use crate::{cell::*, common::*, error::Error};

const SIDECHAIN_CONFIG_DEP: CellOrigin = CellOrigin(5, Source::CellDep);
const SIDECHAIN_BOND_DEP: CellOrigin = CellOrigin(6, Source::CellDep);

const SIDECHAIN_STATE_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const SIDECHAIN_FEE_INPUT: CellOrigin = CellOrigin(2, Source::Input);
const TOKEN_INPUT: CellOrigin = CellOrigin(3, Source::Input);

const SIDECHAIN_STATE_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const SIDECHAIN_FEE_OUTPUT: CellOrigin = CellOrigin(2, Source::Output);

pub fn is_collator_publish_task(sidechain_config_data: &SidechainConfigCell) -> Result<(), Error> {
    /*
    CollatorPublishTask,

    Dep:    0 Global Config Cell
            1 Sidechain Config Cell
            2 Sidechain Bond Cell
    Code Cell                   ->          Code Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    Muse Token                  ->          Null
    Null                        ->          [Task Cell]

    */

    let global = check_global_cell()?;
    if is_cell_count_not_equals(4, Source::Input)
        || is_cell_count_not_equals(3 + sidechain_config_data.commit_threshold as usize, Source::Output)
    {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            SidechainConfigCell: SIDECHAIN_CONFIG_DEP,
            SidechainBondCell: SIDECHAIN_BOND_DEP,
            CodeCell: CODE_INPUT,
            SidechainStateCell: SIDECHAIN_STATE_INPUT,
            SidechainFeeCell: SIDECHAIN_FEE_INPUT,
            MuseTokenCell: TOKEN_INPUT,
            CodeCell: CODE_OUTPUT,
            SidechainStateCell: SIDECHAIN_STATE_OUTPUT,
            SidechainFeeCell: SIDECHAIN_FEE_OUTPUT,
        },
    };
    TaskCell::range_check(3.., Source::Output, &global)
}

pub fn collator_publish_task(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CollatorPublishTask,

    Dep:    0 Global Config Cell
            1 Sidechain Config Cell
            2 Sidechain Bond Cell
    Code Cell                   ->          Code Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    Muse Token                  ->          Null
    Null                        ->          [Task Cell]

    */
    let witness = CollatorPublishTaskWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;
    //load dep
    let (sidechain_config_dep, sidechain_config_dep_type_args, sidechain_bond_dep_lock_args) = load_entities!(
        SidechainConfigCell: SIDECHAIN_CONFIG_DEP,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_DEP,
        SidechainBondCellLockArgs: SIDECHAIN_BOND_DEP,
    );

    is_collator_publish_task(&sidechain_config_dep)?;

    //load inputs
    let (sidechain_state_input_type_args, sidechain_state_input, sidechain_fee_input_lock_args, sidechain_fee_input, muse_token_input) = load_entities! {
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_INPUT,
        SidechainStateCell: SIDECHAIN_STATE_INPUT,
        SidechainFeeCellLockArgs: SIDECHAIN_FEE_INPUT,
        SidechainFeeCell: SIDECHAIN_FEE_INPUT,
        MuseTokenCell: TOKEN_INPUT,
    };

    //load outputs
    let (sidechain_state_output, sidechain_state_output_type_args, sidechain_fee_output_lock_args, sidechain_fee_output) = load_entities!(
        SidechainStateCell: SIDECHAIN_STATE_OUTPUT,
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_OUTPUT,
        SidechainFeeCellLockArgs: SIDECHAIN_FEE_OUTPUT,
        SidechainFeeCell: SIDECHAIN_FEE_OUTPUT,
    );

    if sidechain_config_dep.checker_threshold > sidechain_config_dep.checker_normal_count
        || sidechain_config_dep.collator_lock_arg != signer
        || sidechain_config_dep_type_args.chain_id != witness.chain_id
        || sidechain_config_dep.check_data_size_limit < witness.check_data_size
    {
        return Err(Error::SidechainConfigMismatch);
    }

    let mut sidechain_state_res = sidechain_state_input.clone();
    sidechain_state_res.waiting_jobs.push(BlockSlice {
        from: witness.from_height,
        to:   witness.to_height,
    });
    sidechain_state_res.random_offset += 1;

    let unsubmitted_jobs = sidechain_state_input.waiting_jobs.clone();
    match unsubmitted_jobs.iter().max_by(|x, y| x.from.cmp(&y.from)) {
        Some(job) => {
            if job.to > witness.from_height {
                return Err(Error::Wrong);
            }
        }

        None => {}
    }

    if sidechain_state_input_type_args != sidechain_state_output_type_args
        || sidechain_state_res != sidechain_state_output
        || sidechain_state_input_type_args.chain_id != u32::try_from(witness.chain_id).or(Err(Error::Encoding))?
    {
        return Err(Error::SidechainStateMismatch);
    }

    if signer != sidechain_bond_dep_lock_args.collator_lock_arg
        || sidechain_bond_dep_lock_args.chain_id != witness.chain_id
        || sidechain_bond_dep_lock_args.unlock_sidechain_height < witness.to_height
    {
        return Err(Error::SidechainBondMismatch);
    }

    let mut sidechain_fee_res = sidechain_fee_input.clone();
    let mut sidechain_fee_res_lock_args = sidechain_fee_input_lock_args.clone();

    let mut max_paid = u128::try_from(sidechain_config_dep.commit_threshold).or(Err(Error::Encoding))?
        * u128::try_from(sidechain_config_dep.challenge_threshold).or(Err(Error::Encoding))?
        * u128::try_from(sidechain_config_dep.check_fee_rate).or(Err(Error::Encoding))?
        * witness.check_data_size;
    if max_paid >= sidechain_fee_input_lock_args.surplus {
        sidechain_fee_res.amount += max_paid - sidechain_fee_input_lock_args.surplus;
        sidechain_fee_res_lock_args.surplus = 0;
        max_paid -= sidechain_fee_input_lock_args.surplus;
    } else {
        sidechain_fee_res_lock_args.surplus -= max_paid;
        max_paid = 0;
    }
    if muse_token_input.amount != max_paid {
        return Err(Error::MuseTokenMismatch);
    }

    if sidechain_fee_res != sidechain_fee_output
        || sidechain_fee_res_lock_args != sidechain_fee_output_lock_args
        || sidechain_fee_res_lock_args.chain_id != witness.chain_id
    {
        return Err(Error::SidechainFeeMismatch);
    }

    let mut seed = sidechain_state_input.random_seed;
    seed[0] += sidechain_state_input.random_offset;

    for i in 3..(sidechain_config_dep.commit_threshold + 2) as usize {
        seed = Blake2b::calculate(&seed);
        let seed_number = u128::from_raw(&seed[0..16]).ok_or(Error::Encoding)?;

        let checkers_count = u128::try_from(sidechain_config_dep.activated_checkers.len()).or(Err(Error::Encoding))?;
        let index = usize::try_from(seed_number % checkers_count).or(Err(Error::Encoding))?;

        let checker_lock_arg = sidechain_config_dep.activated_checkers.get(index).ok_or(Error::Encoding)?;

        let task_output = TaskCell::load(CellOrigin(i, Source::Output))?;
        let task_output_type_args = TaskCellTypeArgs::load(CellOrigin(i, Source::Output))?;
        if task_output.sidechain_block_height_from != witness.from_height
            || task_output.sidechain_block_height_to != witness.to_height
            || task_output.sidechain_block_height_from > task_output.sidechain_block_height_to
            || task_output.check_data_size != witness.check_data_size
            || task_output_type_args.checker_lock_arg != *checker_lock_arg
            || task_output_type_args.chain_id != witness.chain_id
        {
            return Err(Error::TaskMismatch);
        }
    }

    Ok(())
}
