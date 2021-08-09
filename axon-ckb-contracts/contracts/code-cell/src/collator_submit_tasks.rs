use alloc::vec::Vec;
use ckb_std::ckb_constants::Source;
use common_raw::{
    cell::{
        code::CodeCell,
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs},
        sidechain_fee::{SidechainFeeCell, SidechainFeeCellLockArgs},
        sidechain_state::{CommittedCheckerInfo, SidechainStateCell, SidechainStateCellTypeArgs},
        task::{TaskCell, TaskCellTypeArgs, TaskStatus},
    },
    common::*,
    witness::collator_submit_tasks::CollatorSubmitTasksWitness,
    FromRaw,
};
use core::convert::TryFrom;

use crate::{cell::*, common::*, error::Error};

const SIDECHAIN_CONFIG_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const SIDECHAIN_STATE_INPUT: CellOrigin = CellOrigin(2, Source::Input);
const SIDECHAIN_FEE_INPUT: CellOrigin = CellOrigin(3, Source::Input);

const SIDECHAIN_CONFIG_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const SIDECHAIN_STATE_OUTPUT: CellOrigin = CellOrigin(2, Source::Output);
const SIDECHAIN_FEE_OUTPUT: CellOrigin = CellOrigin(3, Source::Output);

const FIXED_INPUT_CELLS: usize = 4;

const DEFAULT_REVEAL_VALUE: RandomSeed = [0u8; 32];

fn is_collator_submit_tasks(sidechain_config: &SidechainConfigCell) -> Result<(), Error> {
    let global = check_global_cell()?;

    let len_input = usize::try_from(sidechain_config.commit_threshold).or(Err(Error::Encoding))? + FIXED_INPUT_CELLS;

    if is_cell_count_not_equals(len_input, Source::Input) || is_cell_count_not_equals(4, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }
    check_cells! {
        &global,
        {
            CodeCell: CODE_INPUT,
            SidechainConfigCell: SIDECHAIN_STATE_INPUT,
            SidechainStateCell: SIDECHAIN_STATE_INPUT,
            SidechainFeeCell: SIDECHAIN_FEE_INPUT,

            CodeCell: CODE_OUTPUT,
            SidechainConfigCell: SIDECHAIN_CONFIG_OUTPUT,
            SidechainStateCell: SIDECHAIN_STATE_OUTPUT,
            SidechainFeeCell: SIDECHAIN_FEE_OUTPUT,
        },
    };

    TaskCell::range_check(FIXED_INPUT_CELLS..len_input, Source::Input, &global)?;

    Ok(())
}

// TODO: refactor this long function
pub fn collator_submit_tasks(raw_witness: &[u8], signer: [u8; 20]) -> Result<(), Error> {
    /*
    CollatorSubmitTasks,

    Dep:    0 Global Config Cell

    Code Cell             -> ~
    Sidechain Config Cell -> ~
    Sidechain State Cell  -> ~
    Sidechain Fee Cell    -> ~

    [Task Cell]           -> Null
    */

    //load inputs
    let (sidechain_config_input, sidechain_config_input_type_args) = load_entities!(
        SidechainConfigCell: SIDECHAIN_CONFIG_INPUT,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_INPUT,
    );

    is_collator_submit_tasks(&sidechain_config_input)?;

    let witness = CollatorSubmitTasksWitness::from_raw(&raw_witness).ok_or(Error::Encoding)?;

    //load inputs
    let (sidechain_state_input, sidechain_state_input_type_args, sidechain_fee_input, sidechain_fee_input_lock_args) = load_entities!(
        SidechainStateCell: SIDECHAIN_STATE_INPUT,
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_INPUT,
        SidechainFeeCell: SIDECHAIN_FEE_INPUT,
        SidechainFeeCellLockArgs: SIDECHAIN_FEE_INPUT,
    );

    //load outputs
    let (
        sidechain_config_output,
        sidechain_config_output_type_args,
        sidechain_state_output,
        sidechain_state_output_type_args,
        sidechain_fee_output,
        sidechain_fee_output_lock_args,
    ) = load_entities!(
        SidechainConfigCell: SIDECHAIN_CONFIG_OUTPUT,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_OUTPUT,
        SidechainStateCell: SIDECHAIN_STATE_OUTPUT,
        SidechainStateCellTypeArgs: SIDECHAIN_STATE_OUTPUT,
        SidechainFeeCell: SIDECHAIN_FEE_OUTPUT,
        SidechainFeeCellLockArgs: SIDECHAIN_FEE_OUTPUT,
    );

    let mut sidechain_config_res = sidechain_config_input.clone();

    sidechain_config_res.activated_checkers = sidechain_config_res
        .activated_checkers
        .into_iter()
        .filter(|&lock_arg| {
            witness
                .commit
                .iter()
                .filter(|committed_checker| committed_checker.is_invalid_existed_checker())
                .find(|invalid_checker| lock_arg == invalid_checker.checker_lock_arg)
                .is_none()
        })
        .collect();

    sidechain_config_res.jailed_checkers.extend(
        witness
            .commit
            .iter()
            .filter(|committed_checker| committed_checker.is_invalid_existed_checker())
            .map(|invalid_checker| invalid_checker.checker_lock_arg),
    );

    if sidechain_config_input_type_args.chain_id != witness.chain_id
        || sidechain_config_input.collator_lock_arg != signer
        || u128::from(sidechain_config_input.commit_threshold) * witness.fee_per_checker != witness.fee
        || sidechain_config_res != sidechain_config_output
        || sidechain_config_input_type_args != sidechain_config_output_type_args
    {
        return Err(Error::SidechainConfigMismatch);
    }

    if sidechain_state_input.random_seed != witness.origin_random_seed {
        return Err(Error::SidechainStateMismatch);
    }

    let mut sidechain_state_res = sidechain_state_input.clone();
    sidechain_state_res.random_seed = witness.new_random_seed;

    for valid_checker in witness
        .commit
        .iter()
        .filter(|committed_checker| committed_checker.is_valid_existed_checker())
    {
        let index = valid_checker.index.ok_or(Error::Encoding)?;
        if index >= sidechain_state_res.random_commit.len() {
            return Err(Error::SidechainStateMismatch);
        }

        let mut saved_commit = sidechain_state_res.random_commit[index];

        if saved_commit.checker_lock_arg != valid_checker.checker_lock_arg
            || saved_commit.committed_hash != valid_checker.origin_committed_hash.ok_or(Error::SidechainStateMismatch)?
        {
            return Err(Error::SidechainStateMismatch);
        }
        saved_commit
            .committed_hash
            .copy_from_slice(&valid_checker.new_committed_hash.ok_or(Error::Encoding)?);
    }

    let mut invalid_checker_iter = witness
        .commit
        .iter()
        .filter(|committed_checker| committed_checker.is_invalid_existed_checker());
    let mut invalid_checker_opt = invalid_checker_iter.next();
    let mut invalid_checker_index = match invalid_checker_opt {
        Some(checker) => checker.index.ok_or(Error::Encoding)?,
        None => 0,
    };

    let mut valid_random_commit = Vec::new();

    for i in 0..sidechain_state_res.random_commit.len() {
        let invalid_checker = match invalid_checker_opt {
            Some(checker) => checker,
            None => break,
        };

        if invalid_checker_index >= sidechain_state_res.random_commit.len() {
            return Err(Error::SidechainStateMismatch);
        }

        let saved_commit = sidechain_state_res.random_commit[i];

        if i != invalid_checker_index {
            valid_random_commit.push(saved_commit);
            continue;
        }

        if saved_commit.checker_lock_arg != invalid_checker.checker_lock_arg
            || saved_commit.committed_hash != invalid_checker.origin_committed_hash.ok_or(Error::SidechainStateMismatch)?
        {
            return Err(Error::SidechainStateMismatch);
        }

        invalid_checker_opt = invalid_checker_iter.next();
        invalid_checker_index = invalid_checker.index.ok_or(Error::Encoding)?;
    }
    sidechain_state_res.random_commit = valid_random_commit;

    for new_checker in witness.commit.iter().filter(|committed_checker| committed_checker.is_new_checker()) {
        if sidechain_state_input
            .random_commit
            .iter()
            .find(|commit| commit.checker_lock_arg == new_checker.checker_lock_arg)
            .is_some()
        {
            return Err(Error::SidechainStateMismatch);
        }

        sidechain_state_res.random_commit.push(CommittedCheckerInfo {
            checker_lock_arg: new_checker.checker_lock_arg,
            committed_hash:   new_checker.new_committed_hash.ok_or(Error::Encoding)?,
        })
    }

    if sidechain_state_res != sidechain_state_output || sidechain_state_input_type_args != sidechain_state_output_type_args {
        return Err(Error::SidechainStateMismatch);
    }

    let mut sidechain_fee_res_lock_args = sidechain_fee_input_lock_args.clone();
    if sidechain_fee_res_lock_args.surplus < witness.fee {
        return Err(Error::SidechainFeeMismatch);
    };
    sidechain_fee_res_lock_args.surplus -= witness.fee;

    if sidechain_fee_input != sidechain_fee_output || sidechain_fee_res_lock_args != sidechain_fee_output_lock_args {
        return Err(Error::SidechainFeeMismatch);
    }

    //load tasks
    let mut random_seed_calculator = Blake2b::default();
    random_seed_calculator.update(&witness.origin_random_seed);

    let mut committed_checker_iter = witness.commit.iter();
    let len_input = usize::try_from(sidechain_config_input.commit_threshold).or(Err(Error::Encoding))? + FIXED_INPUT_CELLS;
    for i in FIXED_INPUT_CELLS..len_input {
        let (task, task_type_args) = load_entities!(
            TaskCell: CellOrigin(i, Source::Input),
            TaskCellTypeArgs: CellOrigin(i, Source::Input),
        );

        let committed_checker = committed_checker_iter.next().ok_or(Error::TaskMismatch)?;

        if committed_checker.is_valid_existed_checker() {
            let hash = committed_checker.origin_committed_hash.ok_or(Error::Encoding)?;
            if Blake2b::calculate(&task.reveal) != hash {
                return Err(Error::TaskMismatch);
            }
        }

        if committed_checker.is_new_checker() && task.reveal != DEFAULT_REVEAL_VALUE {
            return Err(Error::TaskMismatch);
        }

        if committed_checker.is_invalid_existed_checker() {
            let hash = committed_checker.origin_committed_hash.ok_or(Error::Encoding)?;
            if Blake2b::calculate(&task.reveal) == hash {
                return Err(Error::TaskMismatch);
            }
        }

        match committed_checker.new_committed_hash {
            Some(hash) => {
                if task.commit != hash {
                    return Err(Error::TaskMismatch);
                }
            }
            None => (),
        }

        if task.status != TaskStatus::TaskPassed
            || task_type_args.chain_id != witness.chain_id
            || task_type_args.checker_lock_arg != committed_checker.checker_lock_arg
        {
            return Err(Error::TaskMismatch);
        }

        if committed_checker.is_invalid_existed_checker() {
            random_seed_calculator.update(&DEFAULT_REVEAL_VALUE);
        } else {
            random_seed_calculator.update(&task.reveal);
        }
    }

    if committed_checker_iter.next().is_some() {
        return Err(Error::TaskMismatch);
    }

    let mut random_seed_res = RandomSeed::default();
    random_seed_calculator.finalize(&mut random_seed_res);
    if random_seed_res != witness.new_random_seed {
        return Err(Error::TaskMismatch);
    }

    Ok(())
}
