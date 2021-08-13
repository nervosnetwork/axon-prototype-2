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

    let witness = CollatorSubmitTasksWitness::from_raw(&raw_witness).ok_or(Error::Encoding)?;

    //load inputs
    let (sidechain_config_input, sidechain_config_input_type_args) = load_entities!(
        SidechainConfigCell: SIDECHAIN_CONFIG_INPUT,
        SidechainConfigCellTypeArgs: SIDECHAIN_CONFIG_INPUT,
    );

    // prepare arguments
    let task_count = usize::try_from(sidechain_config_input.commit_threshold).or(Err(Error::Encoding))? - witness.challenge_times;
    let challenge_count = witness.challenge_times * usize::try_from(sidechain_config_input.challenge_threshold).or(Err(Error::Encoding))?;
    let job_count = task_count + challenge_count;

    let correct_vote_count = witness.commit.iter().filter(|commit| commit.is_valid()).count();

    let fee = u128::try_from(correct_vote_count).or(Err(Error::Encoding))?
        * u128::try_from(sidechain_config_input.check_fee_rate).or(Err(Error::Encoding))?
        * witness.check_data_size;

    let chain_id = sidechain_config_input_type_args.chain_id;

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
    )?;

    check_sidechain_state(
        &sidechain_state_input,
        &sidechain_state_input_type_args,
        &sidechain_state_output,
        &sidechain_state_output_type_args,
        &witness,
        u32::from(chain_id), // TODO: change chain_id to u32
    )?;

    check_sidechain_fee(
        &sidechain_fee_input,
        &sidechain_fee_input_lock_args,
        &sidechain_fee_output,
        &sidechain_fee_output_lock_args,
        fee,
        chain_id,
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
        chain_id,
    )?;

    Ok(())
}

fn check_sidechain_config(
    sidechain_config_input: &SidechainConfigCell,
    sidechain_config_input_type_args: &SidechainConfigCellTypeArgs,
    sidechain_config_output: &SidechainConfigCell,
    sidechain_config_output_type_args: &SidechainConfigCellTypeArgs,
    witness: &CollatorSubmitTasksWitness,
    signer: &[u8; 20],
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

    if sidechain_config_input.collator_lock_arg != *signer
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
    witness: &CollatorSubmitTasksWitness,
    chain_id: u32,
) -> Result<(), Error> {
    if sidechain_state_input.random_seed != witness.origin_random_seed {
        return Err(Error::SidechainStateMismatch);
    }

    let mut sidechain_state_res = sidechain_state_input.clone();
    sidechain_state_res.random_seed = witness.new_random_seed;

    // verify all existed checker
    for existed_checker in witness.commit.iter().filter(|committed_checker| committed_checker.is_existed()) {
        let index = existed_checker.index.ok_or(Error::Encoding)?;
        if index >= sidechain_state_res.random_commit.len() {
            return Err(Error::SidechainStateMismatch);
        }

        let mut saved_commit = sidechain_state_res.random_commit[index];

        if saved_commit.checker_lock_arg != existed_checker.checker_lock_arg
            || saved_commit.committed_hash != existed_checker.origin_committed_hash.ok_or(Error::SidechainStateMismatch)?
        {
            return Err(Error::SidechainStateMismatch);
        }

        // edit valid existed checker
        if existed_checker.is_valid() {
            saved_commit
                .committed_hash
                .copy_from_slice(&existed_checker.new_committed_hash.ok_or(Error::Encoding)?);
        }
    }

    // remove invalid existed checkers
    let invalid_checker_iter = witness
        .commit
        .iter()
        .filter(|committed_checker| committed_checker.is_invalid() && committed_checker.is_existed());

    sidechain_state_res.random_commit = sidechain_state_res
        .random_commit
        .into_iter()
        .filter(|saved_commit| {
            invalid_checker_iter
                .clone()
                .find(|checker| checker.checker_lock_arg == saved_commit.checker_lock_arg)
                .is_none()
        })
        .collect();

    // verify all new checkers
    for new_checker in witness.commit.iter().filter(|committed_checker| committed_checker.is_new()) {
        if sidechain_state_input
            .random_commit
            .iter()
            .find(|commit| commit.checker_lock_arg == new_checker.checker_lock_arg)
            .is_some()
        {
            return Err(Error::SidechainStateMismatch);
        }

        // add valid new checker
        if new_checker.is_valid() {
            sidechain_state_res.random_commit.push(CommittedCheckerInfo {
                checker_lock_arg: new_checker.checker_lock_arg,
                committed_hash:   new_checker.new_committed_hash.ok_or(Error::Encoding)?,
            })
        }
    }

    if sidechain_state_res != *sidechain_state_output
        || sidechain_state_input_type_args.chain_id != chain_id
        || sidechain_state_input_type_args != sidechain_state_output_type_args
    {
        return Err(Error::SidechainStateMismatch);
    }

    Ok(())
}

fn check_sidechain_fee(
    sidechain_fee_input: &SidechainFeeCell,
    sidechain_fee_input_lock_args: &SidechainFeeCellLockArgs,
    sidechain_fee_output: &SidechainFeeCell,
    sidechain_fee_output_lock_args: &SidechainFeeCellLockArgs,
    fee: u128,
    chain_id: u8,
) -> Result<(), Error> {
    let mut sidechain_fee_res_lock_args = sidechain_fee_input_lock_args.clone();
    if sidechain_fee_res_lock_args.surplus < fee {
        return Err(Error::SidechainFeeMismatch);
    };
    sidechain_fee_res_lock_args.surplus -= fee;

    if sidechain_fee_input != sidechain_fee_output
        || sidechain_fee_res_lock_args.chain_id != chain_id
        || sidechain_fee_res_lock_args != *sidechain_fee_output_lock_args
    {
        return Err(Error::SidechainFeeMismatch);
    }

    Ok(())
}

fn check_tasks<T: FnMut() -> Result<Option<(TaskCell, TaskCellTypeArgs)>, Error>>(
    mut next_task: T,
    witness: &CollatorSubmitTasksWitness,
    mut task_count: usize,
    mut challenge_count: usize,
    chain_id: u8,
) -> Result<(), Error> {
    let mut settle_count = 0;
    let mut shutdown_count = 0;

    let mut random_seed_calculator = Blake2b::default();
    random_seed_calculator.update(&witness.origin_random_seed);

    let mut committed_checker_iter = witness.commit.iter();

    let mut is_first_time = true;
    let mut task_first = TaskCell::default();
    let mut task_first_type_args = TaskCellTypeArgs::default();

    loop {
        let (task, task_type_args) = match next_task()? {
            Some(task) => task,
            None => break,
        };

        if is_first_time {
            is_first_time = false;

            task_first = task.clone();
            task_first_type_args = task_type_args.clone();

            if task_first_type_args.chain_id != chain_id || task_first.check_data_size != witness.check_data_size {
                return Err(Error::TaskMismatch);
            }
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

            if task.commit != committed_checker.new_committed_hash.ok_or(Error::TaskMismatch)? {
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

        let mut task_res = task.clone();
        let mut task_res_type_args = task_type_args.clone();

        task_res.mode = task_first.mode;
        task_res.status = task_first.status;
        task_res.commit = task_first.commit;
        task_res.reveal = task_first.reveal;
        task_res.refresh_sidechain_height = task_first.refresh_sidechain_height;

        task_res_type_args.checker_lock_arg = task_first_type_args.checker_lock_arg;

        if task_type_args.checker_lock_arg != committed_checker.checker_lock_arg
            || task_res != task_first
            || task_res_type_args != task_first_type_args
        {
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
