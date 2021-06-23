// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::vec::Vec;

use crate::{
    cell::*, checker_bond_withdraw::checker_bond_withdraw, checker_join_sidechain::checker_join_sidechain,
    checker_quit_sidechain::checker_quit_sidechain, checker_submit_task::checker_submit_task,
    checker_take_beneficiary::checker_take_beneficiary, collator_publish_task::collator_publish_task,
    collator_refresh_task::collator_refresh_task, collator_unlock_bond::collator_unlock_bond, common::*, error::Error,
};

use ckb_std::ckb_constants::Source;
use ckb_std::{
    ckb_types::prelude::*,
    high_level::{load_cell_data, load_witness_args, QueryIter},
};

use crate::pattern::{
    is_admin_create_sidechain, is_checker_publish_challenge, is_checker_submit_challenge, is_collator_submit_challenge,
    is_collator_submit_task,
};
use common_raw::{
    cell::{
        checker_info::{CheckerInfoCellData, CheckerInfoCellMode, CheckerInfoCellTypeArgs},
        code::CodeCellLockArgs,
        sidechain_config::{SidechainConfigCellData, SidechainConfigCellTypeArgs},
        sidechain_fee::SidechainFeeCellData,
        sidechain_state::SidechainStateCellTypeArgs,
        task::{TaskCellData, TaskCellMode, TaskCellTypeArgs},
    },
    pattern::Pattern,
    witness::{
        admin_create_sidechain::AdminCreateSidechainWitness, checker_publish_challenge::CheckerPublishChallengeWitness,
        checker_submit_challenge::CheckerSubmitChallengeWitness, code_cell_witness::CodeCellTypeWitness,
        collator_submit_challenge::CollatorSubmitChallengeWitness, collator_submit_task::CollatorSubmitTaskWitness,
    },
    FromRaw,
};

const CODE_INPUT: CellOrigin = CellOrigin(0, Source::Input);

pub fn main() -> Result<(), Error> {
    /*
    the unlocker of code cell is the owner/signer of code cell
    thus code cell's lock script must be secp256k1
     */
    // of cause, the signer is correct
    let signer = CodeCellLockArgs::load(CODE_INPUT)?.lock_arg;

    let witness = load_witness_args(0, Source::GroupInput)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let raw_witness = witness.as_reader().raw_data();

    let witness = CodeCellTypeWitness::from_raw(raw_witness).ok_or(Error::Encoding)?;

    match witness.pattern() {
        /*
        CheckerBondWithdraw

        Dep:    0 Global Config Cell

        Code Cell                   ->         Code Cell
        Checker Bond Cell           ->         Muse Token Cell

         */
        Pattern::CheckerBondWithdraw => checker_bond_withdraw(signer),

        /*
        CheckerJoinSidechain,

        Dep:    0 Global Config Cell

        Code Cell                   ->         Code Cell
        Sidechain Config Cell       ->          Sidechain Config Cell
        Checker Bond Cell           ->          Checker Bond Cell
        Null                        ->          Checker Info Cell

        */
        Pattern::CheckerJoinSidechain => checker_join_sidechain(raw_witness, signer),
        /*
        CheckerQuitSidechain

        Dep:    0 Global Config Cell

        Code Cell                   ->          Code Cell
        Sidechain Config Cell       ->          Sidechain Config Cell
        Checker Bond Cell           ->          Checker Bond Cell
        Checker Info Cell           ->          Null
        */
        Pattern::CheckerQuitSidechain => checker_quit_sidechain(raw_witness, signer),

        /*
        CheckerSubmitTask,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell

        Code Cell                   ->         Code Cell
        Checker Info Cell           ->          Checker Info Cell
        Task Cell                   ->          Null

        */
        Pattern::CheckerSubmitTask => checker_submit_task(raw_witness, signer),
        /*
        CheckerPublishChallenge,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell

        Code Cell                   ->         Code Cell
        Checker Info Cell           ->          Checker Info Cell
        Task Cell                   ->          [Task Cell]

        */
        Pattern::CheckerPublishChallenge => {
            is_checker_publish_challenge()?;
            checker_publish_challenge(signer)
        }

        /*
        CheckerSubmitChallenge,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell

        Code Cell                   ->         Code Cell
        Checker Info Cell           ->          Checker Info Cell
        Task Cell                   ->          Null

        */
        Pattern::CheckerSubmitChallenge => {
            is_checker_submit_challenge()?;
            checker_submit_challenge(signer)
        }
        /*
        CheckerTakeBeneficiary,

        Dep:    0 Global Config Cell

        Code Cell                   ->         Code Cell
        Checker Info Cell           ->          Checker Info Cell
        Sidechain Fee Cell          ->          Sidechain Fee Cell
        Muse Token Cell             ->          Muse Token Cell

        */
        Pattern::CheckerTakeBeneficiary => checker_take_beneficiary(raw_witness, signer),

        /*
        AdminCreateSidechain,

        Dep:    0 Global Config Cell

        Code Cell                   ->          Code Cell
        CKB Cell                    ->          Sidechain Config Cell
        Null                        ->          Sidechain State Cell

        */
        Pattern::AdminCreateSidechain => {
            is_admin_create_sidechain()?;
            admin_create_sidechain(signer)
        }

        /*
        CollatorPublishTask,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell

        Code Cell                   ->          Code Cell
        Sidechain State Cell        ->          Sidechain State Cell
        Sidechain Bond Cell/Sudt    ->          Sidechain Bond Cell
        Null                        ->          [Task Cell]

        */
        Pattern::CollatorPublishTask => collator_publish_task(raw_witness, signer),

        /*
        CollatorSubmitTask,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell

        Code Cell                   ->          Code Cell
        Sidechain State Cell        ->          Sidechain State Cell
        Sidechain Fee Cell          ->          Sidechain Fee Cell
        [Checker Info Cell]         ->          [Checker Info Cell]

        */
        Pattern::CollatorSubmitTask => {
            is_collator_submit_task()?;
            collator_submit_task(signer)
        }

        /*
        CollatorSubmitChallenge,

        Dep:    0 Global Config Cell

        Code Cell                   ->          Code Cell
        Sidechain Config Cell       ->          Sidechain Config Cell
        Sidechain State Cell        ->          Sidechain State Cell
        Sidechain Fee Cell          ->          Sidechain Fee Cell
        [Checker Info Cell]         ->          [Checker Info Cell]

        */
        Pattern::CollatorSubmitChallenge => {
            is_collator_submit_challenge()?;
            collator_submit_challenge(signer)
        }

        /*
        CollatorRefreshTask,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell

        Code Cell                   ->          Code Cell
        [Task Cell]                 ->          [Task Cell]

        */
        Pattern::CollatorRefreshTask => collator_refresh_task(raw_witness),

        /*
        CollatorUnlockBond,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell
        Dep:    2 Sidechain State Cell

        Code Cell                   ->          Code Cell
        Sidechain Bond Cell         ->          Sudt Cell

        */
        Pattern::CollatorUnlockBond => collator_unlock_bond(raw_witness, signer),
    }
}

fn checker_publish_challenge(_signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerPublishChallenge,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->         Code Cell
    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          [Task Cell]

    */

    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let witness = CheckerPublishChallengeWitness::from_raw(&witness.as_slice()[..]).ok_or(Error::Encoding)?;

    let checker_info_cell_data_input = load_cell_data(1, Source::Input)?;
    let checker_info_input = CheckerInfoCellData::from_raw(checker_info_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_input_type_args = CheckerInfoCellTypeArgs::load(CellOrigin(1, Source::Input))?;

    let task_cell_data_input = load_cell_data(2, Source::Input)?;
    let task_cell_input = TaskCellData::from_raw(task_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let task_cell_input_type_args = TaskCellTypeArgs::load(CellOrigin(2, Source::Input))?;

    let checker_info_cell_data_output = load_cell_data(1, Source::Output)?;
    let checker_info_output = CheckerInfoCellData::from_raw(checker_info_cell_data_output.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_output_type_args = CheckerInfoCellTypeArgs::load(CellOrigin(1, Source::Output))?;

    let mut checker_info_res = checker_info_input.clone();

    checker_info_res.mode = CheckerInfoCellMode::ChallengeRejected;

    let mut task_cell_res = task_cell_input.clone();
    task_cell_res.mode = TaskCellMode::Challenge;

    if checker_info_input.checker_id != witness.checker_id
        || checker_info_input_type_args.chain_id != witness.chain_id
        || checker_info_input_type_args != checker_info_output_type_args
        || checker_info_res != checker_info_output
    {
        return Err(Error::Wrong);
    }

    if task_cell_input_type_args.chain_id != witness.chain_id || task_cell_input.mode != TaskCellMode::Task {
        return Err(Error::Wrong);
    }

    if !QueryIter::new(load_cell_data, Source::Output).skip(2).all(|task_cell_data_input| {
        let task_cell_output = TaskCellData::from_raw(task_cell_data_input.as_slice());
        if let Some(task_cell_output) = task_cell_output {
            task_cell_output == task_cell_res
        } else {
            false
        }
    }) {
        return Err(Error::Wrong);
    }

    Ok(())
}

fn checker_submit_challenge(_signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerSubmitChallenge,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->         Code Cell
    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          Null

    */

    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let witness = CheckerSubmitChallengeWitness::from_raw(&witness.as_slice()[..]).ok_or(Error::Encoding)?;

    let checker_info_cell_data_input = load_cell_data(1, Source::Input)?;
    let checker_info_input = CheckerInfoCellData::from_raw(checker_info_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_input_type_args = CheckerInfoCellTypeArgs::load(CellOrigin(1, Source::Input))?;

    let task_cell_data_input = load_cell_data(2, Source::Input)?;
    let task_cell_input = TaskCellData::from_raw(task_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let task_cell_input_type_args = TaskCellTypeArgs::load(CellOrigin(2, Source::Input))?;

    let checker_info_cell_data_output = load_cell_data(1, Source::Output)?;
    let checker_info_output = CheckerInfoCellData::from_raw(checker_info_cell_data_output.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_output_type_args = CheckerInfoCellTypeArgs::load(CellOrigin(1, Source::Output))?;

    let mut checker_info_res = checker_info_input.clone();
    checker_info_res.mode = CheckerInfoCellMode::ChallengeRejected;

    if checker_info_input_type_args.chain_id != witness.chain_id
        || checker_info_input_type_args == checker_info_output_type_args
        || checker_info_input.checker_id != witness.checker_id
        || checker_info_res != checker_info_output
    {
        return Err(Error::Wrong);
    }

    if task_cell_input_type_args.chain_id != witness.chain_id || task_cell_input.mode != TaskCellMode::Challenge {
        return Err(Error::Wrong);
    }

    Ok(())
}

fn admin_create_sidechain(_signer: [u8; 20]) -> Result<(), Error> {
    /*
    AdminCreateSidechain,

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    CKB Cell                    ->          Sidechain Config Cell
    Null                        ->          Sidechain State Cell

    */
    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let witness = AdminCreateSidechainWitness::from_raw(&witness.as_slice()[..]).ok_or(Error::Encoding)?;

    let sidechain_config_output_type_args = SidechainConfigCellTypeArgs::load(CellOrigin(1, Source::Output))?;

    let sidechain_state_output_type_args = SidechainStateCellTypeArgs::load(CellOrigin(2, Source::Output))?;

    if sidechain_config_output_type_args.chain_id != witness.chain_id {
        return Err(Error::Wrong);
    }

    if sidechain_state_output_type_args.chain_id != witness.chain_id {
        return Err(Error::Wrong);
    }

    Ok(())
}

fn collator_submit_task(_signer: [u8; 20]) -> Result<(), Error> {
    /*
    CollatorSubmitTask,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->          Code Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    [Checker Info Cell]         ->          [Checker Info Cell]

    */

    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let witness = CollatorSubmitTaskWitness::from_raw(&witness.as_slice()[..]).ok_or(Error::Encoding)?;

    let sidechain_config_cell_data_celldep = load_cell_data(1, Source::CellDep)?;
    let _sidechain_config_celldep =
        SidechainConfigCellData::from_raw(sidechain_config_cell_data_celldep.as_slice()).ok_or(Error::Encoding)?;

    //==========

    let sidechain_state_input_type_args = SidechainStateCellTypeArgs::load(CellOrigin(1, Source::Input))?;

    let sidechain_fee_cell_data_input = load_cell_data(2, Source::Input)?;
    let sidechain_fee_input = SidechainFeeCellData::from_raw(sidechain_fee_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_inputs = QueryIter::new(load_cell_data, Source::Input)
        .skip(3)
        .map(|checker_info_cell_data_input| CheckerInfoCellData::from_raw(checker_info_cell_data_input.as_slice()))
        .collect::<Option<Vec<_>>>()
        .ok_or(Error::Encoding)?;

    let sidechain_state_output_type_args = SidechainStateCellTypeArgs::load(CellOrigin(1, Source::Output))?;

    let sidechain_fee_cell_data_output = load_cell_data(2, Source::Output)?;
    let sidechain_fee_output = SidechainFeeCellData::from_raw(sidechain_fee_cell_data_output.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_outputs = QueryIter::new(load_cell_data, Source::Output)
        .skip(3)
        .map(|checker_info_cell_data_input| CheckerInfoCellData::from_raw(checker_info_cell_data_input.as_slice()))
        .collect::<Option<Vec<_>>>()
        .ok_or(Error::Encoding)?;

    let mut sidechain_fee_res = sidechain_fee_input;
    sidechain_fee_res.amount -= witness.fee;

    if sidechain_state_input_type_args.chain_id != witness.chain_id || sidechain_state_input_type_args != sidechain_state_output_type_args {
        return Err(Error::Wrong);
    }

    if sidechain_fee_res != sidechain_fee_output {
        return Err(Error::Wrong);
    }

    if !checker_info_inputs.into_iter().zip(checker_info_outputs).all(|(input, output)| {
        let mut res = input;
        res.unpaid_fee += witness.fee_per_checker;
        res.mode = CheckerInfoCellMode::Idle;

        res == output
    }) {
        return Err(Error::Wrong);
    }
    for i in 3.. {
        let checker_info_input_type_args = match CheckerInfoCellTypeArgs::load(CellOrigin(i, Source::Input)) {
            Ok(data) => data,
            Err(Error::IndexOutOfBound) => break,
            Err(err) => return Err(err),
        };
        let checker_info_output_type_args = match CheckerInfoCellTypeArgs::load(CellOrigin(i, Source::Output)) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        if checker_info_input_type_args.chain_id != witness.chain_id || checker_info_input_type_args != checker_info_output_type_args {
            return Err(Error::Wrong);
        }
    }

    Ok(())
}

fn collator_submit_challenge(_signer: [u8; 20]) -> Result<(), Error> {
    /*
    CollatorSubmitChallenge,

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    Sidechain Config Cell       ->          Sidechain Config Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    [Checker Info Cell]         ->          [Checker Info Cell]

    */

    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let witness = CollatorSubmitChallengeWitness::from_raw(&witness.as_slice()[..]).ok_or(Error::Encoding)?;

    let sidechain_config_cell_data_celldep = load_cell_data(1, Source::CellDep)?;
    let _sidechain_config_celldep =
        SidechainConfigCellData::from_raw(sidechain_config_cell_data_celldep.as_slice()).ok_or(Error::Encoding)?;

    //==============

    let sidechain_state_input_type_args = SidechainStateCellTypeArgs::load(CellOrigin(1, Source::Input))?;

    let sidechain_fee_cell_data_input = load_cell_data(2, Source::Input)?;
    let sidechain_fee_input = SidechainFeeCellData::from_raw(sidechain_fee_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_inputs = QueryIter::new(load_cell_data, Source::Input)
        .skip(3)
        .map(|checker_info_cell_data_input| CheckerInfoCellData::from_raw(checker_info_cell_data_input.as_slice()))
        .collect::<Option<Vec<_>>>()
        .ok_or(Error::Encoding)?;

    let sidechain_state_output_type_args = SidechainStateCellTypeArgs::load(CellOrigin(1, Source::Output))?;

    let sidechain_fee_cell_data_output = load_cell_data(2, Source::Output)?;
    let sidechain_fee_output = SidechainFeeCellData::from_raw(sidechain_fee_cell_data_output.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_outputs = QueryIter::new(load_cell_data, Source::Output)
        .skip(3)
        .map(|checker_info_cell_data_input| CheckerInfoCellData::from_raw(checker_info_cell_data_input.as_slice()))
        .collect::<Option<Vec<_>>>()
        .ok_or(Error::Encoding)?;

    let _sidechain_state_cell_data_ouput = load_cell_data(1, Source::Output)?;

    let mut sidechain_fee_res = sidechain_fee_input;
    sidechain_fee_res.amount -= witness.fee;

    if sidechain_state_input_type_args.chain_id != witness.chain_id || sidechain_state_input_type_args != sidechain_state_output_type_args {
        return Err(Error::Wrong);
    }

    if sidechain_fee_res != sidechain_fee_output {
        return Err(Error::Wrong);
    }

    let checker_info_res = checker_info_inputs
        .into_iter()
        .filter_map(|checker_info_input| {
            let result = bit_map_marked(witness.punish_checker_bitmap, checker_info_input.checker_id);

            if result.is_some() {
                return Some(checker_info_input);
            } else {
                return None;
            }
        })
        .collect::<Vec<_>>();

    if !checker_info_res.into_iter().zip(checker_info_outputs).all(|(mut res, output)| {
        res.unpaid_fee += witness.fee_per_checker;
        res.mode = CheckerInfoCellMode::Idle;

        res == output
    }) {
        return Err(Error::Wrong);
    }
    for i in 3.. {
        let checker_info_input_type_args = match CheckerInfoCellTypeArgs::load(CellOrigin(i, Source::Input)) {
            Ok(data) => data,
            Err(Error::IndexOutOfBound) => break,
            Err(err) => return Err(err),
        };
        let checker_info_output_type_args = match CheckerInfoCellTypeArgs::load(CellOrigin(i, Source::Output)) {
            Ok(data) => data,
            Err(err) => return Err(err),
        };

        if checker_info_input_type_args.chain_id != witness.chain_id || checker_info_input_type_args != checker_info_output_type_args {
            return Err(Error::Wrong);
        }
    }

    Ok(())
}
