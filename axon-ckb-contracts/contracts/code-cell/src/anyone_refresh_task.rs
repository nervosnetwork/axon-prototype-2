use crate::{cell::*, common::*, error::Error};
use ckb_std::ckb_constants::Source;
use common_raw::cell::sidechain_state::PunishedChecker;
use common_raw::cell::task::TaskStatus;
use common_raw::{
    cell::{
        code::CodeCell,
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs},
        sidechain_state::{SidechainStateCell, SidechainStateCellTypeArgs},
        task::{TaskCell, TaskCellTypeArgs},
    },
    witness::collator_refresh_task::CollatorRefreshTaskWitness,
    FromRaw,
};
use core::convert::TryFrom;

const CONFIG_INPUT: CellOrigin = CellOrigin(1, Source::Input);
const CONFIG_OUTPUT: CellOrigin = CellOrigin(1, Source::Output);
const STATE_INPUT: CellOrigin = CellOrigin(2, Source::Input);
const STATE_OUTPUT: CellOrigin = CellOrigin(2, Source::Output);

pub fn collator_refresh_task(raw_witness: &[u8]) -> Result<(), Error> {
    /*
    CollatorRefreshTask,

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    SidechainConfig             ->          SidechainConfig
    SidechainState              ->          SidechainState
    [Task Cell]                 ->          [Task Cell]

    */
    is_collator_refresh_task()?;

    let witness = CollatorRefreshTaskWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    let (config_input_type_args, config_input, state_input_type_args, state_input) = load_entities! {
        SidechainConfigCellTypeArgs: CONFIG_INPUT,
        SidechainConfigCell: CONFIG_INPUT,
        SidechainStateCellTypeArgs: STATE_INPUT,
        SidechainStateCell: STATE_INPUT,
    };

    let (config_output_type_args, config_output, state_output_type_args, state_output) = load_entities! {
        SidechainConfigCellTypeArgs: CONFIG_OUTPUT,
        SidechainConfigCell: CONFIG_OUTPUT,
        SidechainStateCellTypeArgs: STATE_OUTPUT,
        SidechainStateCell: STATE_OUTPUT,
    };
    if u32::try_from(config_input_type_args.chain_id).or(Err(Error::Encoding))? != witness.chain_id {
        return Err(Error::SidechainConfigMismatch);
    }
    let mut config_res = config_input.clone();
    let mut state_res = state_input.clone();

    let mut seed = state_res.random_seed.clone();
    seed[0] += state_res.random_offset;

    for i in 3.. {
        let task_input_origin = CellOrigin(i, Source::Input);
        let task_output_origin = CellOrigin(i, Source::Output);

        let task_input = match TaskCell::load(task_input_origin) {
            Ok(data) => data,
            Err(Error::IndexOutOfBound) => break,
            Err(err) => return Err(err),
        };
        let task_input_type_args = match TaskCellTypeArgs::load(task_input_origin) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let task_output = match TaskCell::load(task_output_origin) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let task_output_type_args = match TaskCellTypeArgs::load(task_output_origin) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        let mut task_res = task_input.clone();
        let mut task_res_type_args = task_input_type_args.clone();

        check_confirm_interval_and_update(&mut task_res, &mut task_res_type_args, &config_input, &state_res, &mut seed)?;

        if u32::try_from(task_input_type_args.chain_id).or(Err(Error::Encoding))? != witness.chain_id
            || task_res != task_output
            || task_res.status != TaskStatus::Idle
            || task_res_type_args != task_output_type_args
        {
            return Err(Error::TaskMismatch);
        }

        match state_res
            .punish_checkers
            .iter_mut()
            .enumerate()
            .find(|(_, checker)| checker.checker_lock_arg == task_input_type_args.checker_lock_arg)
        {
            Some((index, punish_checker)) => {
                punish_checker.punish_points += config_input.refresh_punish_points;
                let checker_lock_arg = punish_checker.checker_lock_arg;

                //put checker into jail if number of punish_points is greater than threshold.
                if punish_checker.punish_points > config_input.refresh_punish_threshold {
                    state_res.punish_checkers.remove(index);
                    config_res.jailed_checkers.push(checker_lock_arg);
                }
            }

            //add a new record in punished checkers
            //do not check punish points
            None => state_res.punish_checkers.push(PunishedChecker {
                checker_lock_arg: task_res_type_args.checker_lock_arg.clone(),
                punish_points:    config_input.refresh_punish_points,
            }),
        };
    }

    //update random_offset before refresh all task in this tx.
    state_res.random_offset += 1;
    if state_res != state_output
        || state_input_type_args != state_output_type_args
        || u32::try_from(state_input_type_args.chain_id).or(Err(Error::Encoding))? != witness.chain_id
    {
        return Err(Error::SidechainStateMismatch);
    }

    if config_res != config_output
        || config_input_type_args != config_output_type_args
        || u32::try_from(config_input_type_args.chain_id).or(Err(Error::Encoding))? != witness.chain_id
    {
        return Err(Error::SidechainConfigMismatch);
    }
    Ok(())
}

fn is_collator_refresh_task() -> Result<(), Error> {
    let global = check_global_cell()?;

    if is_cell_count_smaller(3, Source::Input) || is_cell_count_smaller(3, Source::Output) {
        return Err(Error::CellNumberMismatch);
    }

    check_cells! {
        &global,
        {
            CodeCell: CODE_INPUT,
            CodeCell: CODE_OUTPUT,
            SidechainConfigCell: CONFIG_INPUT,
            SidechainConfigCell: CONFIG_OUTPUT,
            SidechainStateCell: STATE_INPUT,
            SidechainStateCell: STATE_OUTPUT,
        },
    };

    TaskCell::one_to_one_check(3, &global)
}

pub fn check_confirm_interval_and_update(
    task: &mut TaskCell,
    task_type_args: &mut TaskCellTypeArgs,
    config: &SidechainConfigCell,
    state: &SidechainStateCell,
    seed: &mut [u8; 32],
) -> Result<(), Error> {
    //compute index of chosen checker and update seed for next task in this tx;
    *seed = Blake2b::calculate(seed);

    let seed_number = u128::from_raw(&seed[0..16]).ok_or(Error::Encoding)?;
    let checkers_count = u128::try_from(config.activated_checkers.len()).or(Err(Error::Encoding))?;
    let index = usize::try_from(seed_number % checkers_count).or(Err(Error::Encoding))?;

    let next_checker_lock_arg = config.activated_checkers.get(index).ok_or(Error::Encoding)?;

    // refresh limit reached, then anyone can check this task.
    if task.refresh_sidechain_height + config.refresh_sidechain_height_interval
        > u128::try_from(config.parallel_job_upper_bond).or(Err(Error::Encoding))?
    {
        return Err(Error::Wrong);
    }

    // count the number of confirmed jobs which height greater than this task.
    let confirm_count_before_task = u128::try_from(
        state
            .confirmed_jobs
            .iter()
            .filter(|&job| job.to > task.sidechain_block_height_to)
            .count(),
    )
    .or(Err(Error::Encoding))?;

    if confirm_count_before_task < task.refresh_sidechain_height {
        return Err(Error::Wrong);
    }

    task_type_args.checker_lock_arg = *next_checker_lock_arg;
    task.refresh_sidechain_height += config.refresh_sidechain_height_interval;

    Ok(())
}
