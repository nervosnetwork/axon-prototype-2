use alloc::vec::Vec;
use ckb_std::ckb_constants::Source;
use common_raw::{
    cell::{
        code::CodeCell,
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs},
        sidechain_fee::{SidechainFeeCell, SidechainFeeCellLockArgs},
        sidechain_state::{CommittedCheckerInfo, SidechainStateCell, SidechainStateCellTypeArgs},
        task::{TaskCell, TaskCellTypeArgs, TaskMode, TaskStatus},
    },
    common::*,
    witness::{collator_submit_tasks::CollatorSubmitTasksWitness, common_submit_jobs::CommonSubmitJobsWitness},
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

    let witness = CollatorSubmitTasksWitness::from_raw(&raw_witness).ok_or(Error::Encoding)?.common;

    //load inputs
    let (sidechain_config_input, sidechain_config_input_type_args) = load_entities!(
        SidechainConfigCell: SIDECHAIN_CONFIG_INPUT,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_INPUT,
    );

    let task_count = usize::try_from(sidechain_config_input.commit_threshold).or(Err(Error::Encoding))? - witness.challenge_times;
    let challenge_count = witness.challenge_times * usize::try_from(sidechain_config_input.challenge_threshold).or(Err(Error::Encoding))?;
    let job_count = task_count + challenge_count;

    is_collator_submit_tasks(job_count)?;

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

    check_sidechain_config(
        &sidechain_config_input,
        &sidechain_config_input_type_args,
        &sidechain_config_output,
        &sidechain_config_output_type_args,
        &witness,
        &signer,
        job_count,
    )?;

    check_sidechain_state(
        &sidechain_state_input,
        &sidechain_state_input_type_args,
        &sidechain_state_output,
        &sidechain_state_output_type_args,
        &witness,
    )?;

    check_sidechain_fee(
        &sidechain_fee_input,
        &sidechain_fee_input_lock_args,
        &sidechain_fee_output,
        &sidechain_fee_output_lock_args,
        &witness,
    )?;

    let mut i = FIXED_INPUT_CELLS;
    let len_input = FIXED_INPUT_CELLS + job_count;
    check_tasks(
        || {
            if i >= len_input {
                return Ok(None);
            }

            let result = load_entities!(
                TaskCell: CellOrigin(i, Source::Input),
                TaskCellTypeArgs: CellOrigin(i, Source::Input),
            );

            i += 1;

            Ok(Some(result))
        },
        &witness,
        task_count,
        challenge_count,
    )?;

    Ok(())
}

fn check_sidechain_config(
    sidechain_config_input: &SidechainConfigCell,
    sidechain_config_input_type_args: &SidechainConfigCellTypeArgs,
    sidechain_config_output: &SidechainConfigCell,
    sidechain_config_output_type_args: &SidechainConfigCellTypeArgs,
    witness: &CommonSubmitJobsWitness,
    signer: &[u8; 20],
    job_count: usize,
) -> Result<(), Error> {
    let mut sidechain_config_res = sidechain_config_input.clone();

    sidechain_config_res.activated_checkers = sidechain_config_res
        .activated_checkers
        .into_iter()
        .filter(|&lock_arg| {
            witness
                .commit
                .iter()
                .filter(|committed_checker| committed_checker.is_invalid())
                .find(|invalid_checker| lock_arg == invalid_checker.checker_lock_arg)
                .is_none()
        })
        .collect();

    sidechain_config_res.jailed_checkers.extend(
        witness
            .commit
            .iter()
            .filter(|committed_checker| committed_checker.is_invalid())
            .map(|invalid_checker| invalid_checker.checker_lock_arg),
    );

    if sidechain_config_input_type_args.chain_id != witness.chain_id
        || sidechain_config_input.collator_lock_arg != *signer
        || u128::try_from(job_count).or(Err(Error::Encoding))? * witness.fee_per_checker != witness.fee
        || sidechain_config_res != *sidechain_config_output
        || sidechain_config_input_type_args != sidechain_config_output_type_args
    {
        return Err(Error::SidechainConfigMismatch);
    }

    Ok(())
}

fn check_sidechain_state(
    sidechain_state_input: &SidechainStateCell,
    sidechain_state_input_type_args: &SidechainStateCellTypeArgs,
    sidechain_state_output: &SidechainStateCell,
    sidechain_state_output_type_args: &SidechainStateCellTypeArgs,
    witness: &CommonSubmitJobsWitness,
) -> Result<(), Error> {
    if sidechain_state_input.random_seed != witness.origin_random_seed {
        return Err(Error::SidechainStateMismatch);
    }

    let mut sidechain_state_res = sidechain_state_input.clone();
    sidechain_state_res.random_seed = witness.new_random_seed;

    // check valid existed checkers
    for valid_checker in witness
        .commit
        .iter()
        .filter(|committed_checker| committed_checker.is_valid() && committed_checker.is_existed())
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

    // remove invalid existed checkers
    let mut invalid_checker_iter = witness
        .commit
        .iter()
        .filter(|committed_checker| committed_checker.is_invalid() && committed_checker.is_existed());
    let mut invalid_checker_opt = invalid_checker_iter.next();
    let mut invalid_checker_index = match invalid_checker_opt {
        Some(checker) => checker.index.ok_or(Error::Encoding)?,
        None => 0,
    };

    let mut valid_random_commit = Vec::new();

    for i in 0..sidechain_state_res.random_commit.len() {
        let saved_commit = sidechain_state_res.random_commit[i];

        if i != invalid_checker_index {
            valid_random_commit.push(saved_commit);
            continue;
        }

        let invalid_checker = match invalid_checker_opt {
            Some(checker) => checker,
            None => {
                valid_random_commit.push(saved_commit);
                continue;
            }
        };

        if invalid_checker_index >= sidechain_state_res.random_commit.len() {
            return Err(Error::SidechainStateMismatch);
        }

        if saved_commit.checker_lock_arg != invalid_checker.checker_lock_arg
            || saved_commit.committed_hash != invalid_checker.origin_committed_hash.ok_or(Error::SidechainStateMismatch)?
        {
            return Err(Error::SidechainStateMismatch);
        }

        invalid_checker_opt = invalid_checker_iter.next();
        invalid_checker_index = match invalid_checker_opt {
            Some(checker) => checker.index.ok_or(Error::Encoding)?,
            None => 0,
        };
    }
    sidechain_state_res.random_commit = valid_random_commit;

    // add valid new checkers
    for new_checker in witness
        .commit
        .iter()
        .filter(|committed_checker| committed_checker.is_valid() && committed_checker.is_new())
    {
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

    if sidechain_state_res != *sidechain_state_output || sidechain_state_input_type_args != sidechain_state_output_type_args {
        return Err(Error::SidechainStateMismatch);
    }

    Ok(())
}

fn check_sidechain_fee(
    sidechain_fee_input: &SidechainFeeCell,
    sidechain_fee_input_lock_args: &SidechainFeeCellLockArgs,
    sidechain_fee_output: &SidechainFeeCell,
    sidechain_fee_output_lock_args: &SidechainFeeCellLockArgs,
    witness: &CommonSubmitJobsWitness,
) -> Result<(), Error> {
    let mut sidechain_fee_res_lock_args = sidechain_fee_input_lock_args.clone();
    if sidechain_fee_res_lock_args.surplus < witness.fee {
        return Err(Error::SidechainFeeMismatch);
    };
    sidechain_fee_res_lock_args.surplus -= witness.fee;

    if sidechain_fee_input != sidechain_fee_output || sidechain_fee_res_lock_args != *sidechain_fee_output_lock_args {
        return Err(Error::SidechainFeeMismatch);
    }

    Ok(())
}

fn check_tasks<T: FnMut() -> Result<Option<(TaskCell, TaskCellTypeArgs)>, Error>>(
    mut next_task: T,
    witness: &CommonSubmitJobsWitness,
    mut task_count: usize,
    mut challenge_count: usize,
) -> Result<(), Error> {
    let mut settle_count = 0;
    let mut shutdown_count = 0;

    let mut random_seed_calculator = Blake2b::default();
    random_seed_calculator.update(&witness.origin_random_seed);

    let mut committed_checker_iter = witness.commit.iter();
    loop {
        let (task, task_type_args) = match next_task()? {
            Some(task) => task,
            None => break,
        };

        match task.status {
            TaskStatus::Idle => return Err(Error::TaskMismatch),
            TaskStatus::TaskPassed => {
                task_count -= 1;
                settle_count += 1;
            }
            TaskStatus::ChallengeRejected => {
                challenge_count -= 1;
                settle_count += 1;
            }
            TaskStatus::ChallengePassed => {
                challenge_count -= 1;
                shutdown_count += 1;
            }
        }

        let committed_checker = committed_checker_iter.next().ok_or(Error::TaskMismatch)?;

        if committed_checker.is_valid() {
            if committed_checker.is_new() {
                if task.reveal != DEFAULT_REVEAL_VALUE {
                    return Err(Error::TaskMismatch);
                }

                random_seed_calculator.update(&DEFAULT_REVEAL_VALUE);
            } else {
                let hash = committed_checker.origin_committed_hash.ok_or(Error::Encoding)?;
                if Blake2b::calculate(&task.reveal) != hash {
                    return Err(Error::TaskMismatch);
                }

                random_seed_calculator.update(&task.reveal);
            }

            if match task.mode {
                TaskMode::Task => task.status != TaskStatus::TaskPassed,
                TaskMode::Challenge => task.status != TaskStatus::ChallengeRejected,
            } {
                return Err(Error::TaskMismatch);
            }
        } else {
            let hash = committed_checker.origin_committed_hash.ok_or(Error::Encoding)?;

            if Blake2b::calculate(&task.reveal) == hash {
                if task.mode != TaskMode::Challenge || task.status != TaskStatus::ChallengePassed {
                    return Err(Error::TaskMismatch);
                }

                random_seed_calculator.update(&task.reveal);
            } else {
                random_seed_calculator.update(&DEFAULT_REVEAL_VALUE);
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

        if task_type_args.chain_id != witness.chain_id || task_type_args.checker_lock_arg != committed_checker.checker_lock_arg {
            return Err(Error::TaskMismatch);
        }
    }

    if task_count != 0 || challenge_count != 0 || shutdown_count >= settle_count || committed_checker_iter.next().is_some() {
        return Err(Error::TaskMismatch);
    }

    let mut random_seed_res = RandomSeed::default();
    random_seed_calculator.finalize(&mut random_seed_res);
    if random_seed_res != witness.new_random_seed {
        return Err(Error::TaskMismatch);
    }

    Ok(())
}

fn is_collator_submit_tasks(job_count: usize) -> Result<(), Error> {
    let global = check_global_cell()?;

    let len_input = FIXED_INPUT_CELLS + job_count;

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
