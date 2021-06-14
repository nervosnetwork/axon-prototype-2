// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::vec::Vec;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use crate::error::Error;
use ckb_std::ckb_constants::Source;
use ckb_std::{
    ckb_types::prelude::*,
    high_level::{load_cell_data, load_cell_lock, load_witness_args},
};

use ckb_std::high_level::QueryIter;
use common::pattern::{
    is_admin_create_sidechain, is_checker_bond_deposit, is_checker_bond_withdraw, is_checker_join_sidechain, is_checker_publish_challenge,
    is_checker_quit_sidechain, is_checker_submit_challenge, is_checker_submit_task, is_checker_take_beneficiary, is_collator_publish_task,
    is_collator_refresh_task, is_collator_submit_challenge, is_collator_submit_task, is_collator_unlock_bond, Pattern,
};
use common::{bit_map_add, bit_map_marked, bit_map_remove, EMPTY_BIT_MAP};
use common_raw::{
    cell::{
        checker_bond::{CheckerBondCellData, CheckerBondCellLockArgs},
        checker_info::{CheckerInfoCellData, CheckerInfoCellMode},
        code::{CodeCellLockArgs, CodeCellTypeWitness},
        muse_token::MuseTokenData,
        sidechain_bond::SidechainBondCellData,
        sidechain_config::SidechainConfigCellData,
        sidechain_fee::SidechainFeeCellData,
        sidechain_state::SidechainStateCellData,
        sudt_token::SudtTokenData,
        task::{TaskCellData, TaskCellMode},
    },
    witness::{
        admin_create_sidechain::AdminCreateSidechainWitness, checker_join_sidechain::CheckerJoinSidechainWitness,
        checker_publish_challenge::CheckerPublishChallengeWitness, checker_quit_sidechain::CheckerQuitSidechainWitness,
        checker_submit_challenge::CheckerSubmitChallengeWitness, checker_take_beneficiary::CheckerTakeBeneficiaryWitness,
        collator_publish_task::CollatorPublishTaskWitness, collator_refresh_task::CollatorRefreshTaskWitness,
        collator_submit_challenge::CollatorSubmitChallengeWitness, collator_submit_task::CollatorSubmitTaskWitness,
        collator_unlock_bond::CollatorUnlockBondWitness,
    },
    FromRaw,
};

pub fn main() -> Result<(), Error> {
    /*
    the unlocker of code cell is the owner/signer of code cell
    thus code cell's lock script must be secp256k1
     */
    // of cause, the signer is correct
    let lock_args = load_cell_lock(0, Source::Input)?;
    let signer = CodeCellLockArgs::from_raw(lock_args.args().as_reader().raw_data())
        .ok_or(Error::Encoding)?
        .public_key_hash;

    let witness_args = load_witness_args(0, Source::GroupInput)?;
    let witness_args_input_type = witness_args.input_type().to_opt().ok_or(Error::MissingWitness)?;

    let pattern = CodeCellTypeWitness::from_raw(witness_args_input_type.as_reader().raw_data()).ok_or(Error::Encoding)?;

    match pattern.pattern.into() {
        /*
        CheckerBondDeposit

        Muse Token Cell             ->          Check Bond Cell

        No way to monitor this pattern, regard all check bond cell trustless

         */
        Pattern::CheckerBondDeposit => {
            is_checker_bond_deposit()?;
            checker_bond_deposit(signer)?
        }

        /*
        CheckerBondWithdraw

        Dep:    0 Global Config Cell

        Code Cell                   ->         Code Cell
        Checker Bond Cell           ->         Muse Token Cell

         */
        Pattern::CheckerBondWithdraw => {
            is_checker_bond_withdraw()?;
            checker_bond_withdraw(signer)?
        }

        /*
        CheckerJoinSidechain,

        Dep:    0 Global Config Cell

        Code Cell                   ->         Code Cell
        Sidechain Config Cell       ->          Sidechain Config Cell
        Checker Bond Cell           ->          Checker Bond Cell
        Null                        ->          Checker Info Cell

        */
        Pattern::CheckerJoinSidechain => {
            is_checker_join_sidechain()?;
            checker_join_sidechain(signer)?
        }
        /*
        CheckerQuitSidechain

        Dep:    0 Global Config Cell

        Code Cell                   ->          Code Cell
        Sidechain Config Cell       ->          Sidechain Config Cell
        Checker Bond Cell           ->          Checker Bond Cell
        Checker Info Cell           ->          Null
        */
        Pattern::CheckerQuitSidechain => {
            is_checker_quit_sidechain()?;
            checker_quit_sidechain(signer)?
        }

        /*
        CheckerSubmitTask,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell

        Code Cell                   ->         Code Cell
        Checker Info Cell           ->          Checker Info Cell
        Task Cell                   ->          Null

        */
        Pattern::CheckerSubmitTask => {
            is_checker_submit_task()?;
            checker_submit_task(signer)?
        }
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
            checker_publish_challenge(signer)?
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
            checker_submit_challenge(signer)?
        }
        /*
        CheckerTakeBeneficiary,

        Dep:    0 Global Config Cell

        Code Cell                   ->         Code Cell
        Checker Info Cell           ->          Checker Info Cell
        Sidechain Fee Cell          ->          Sidechain Fee Cell
        Muse Token Cell             ->          Muse Token Cell

        */
        Pattern::CheckerTakeBeneficiary => {
            is_checker_take_beneficiary()?;
            checker_take_beneficiary(signer)?
        }

        /*
        AdminCreateSidechain,

        Dep:    0 Global Config Cell

        Code Cell                   ->          Code Cell
        CKB Cell                    ->          Sidechain Config Cell
        Null                        ->          Sidechain State Cell

        */
        Pattern::AdminCreateSidechain => {
            is_admin_create_sidechain()?;
            admin_create_sidechain(signer)?
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
        Pattern::CollatorPublishTask => {
            is_collator_publish_task()?;
            collator_publish_task(signer)?
        }

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
            collator_submit_task(signer)?
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
            collator_submit_challenge(signer)?
        }

        /*
        CollatorRefreshTask,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell

        Code Cell                   ->          Code Cell
        [Task Cell]                 ->          [Task Cell]

        */
        Pattern::CollatorRefreshTask => {
            is_collator_refresh_task()?;
            collator_refresh_task(signer)?
        }

        /*
        CollatorUnlockBond,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell
        Dep:    2 Sidechain State Cell

        Code Cell                   ->          Code Cell
        Sidechain Bond Cell         ->          Sudt Cell

        */
        Pattern::CollatorUnlockBond => {
            is_collator_unlock_bond()?;
            collator_unlock_bond(signer)?
        }

        _ => return Err(Error::PatternRecognitionFailure),
    }

    Ok(())
}

fn checker_bond_deposit(_signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerBondDeposit

    Muse Token Cell             ->          Check Bond Cell

    No way to monitor this pattern, regard all check bond cell trustless

     */
    Ok(())
}

fn checker_bond_withdraw(signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerBondWithdraw

    Dep:    0 Global Config Cell

    Code Cell                   ->         Code Cell
    Checker Bond Cell           ->         Muse Token Cell

     */

    /*
    Job:

    1. chain_id_bitmap is 0x00

     */

    let checker_bond_cell_lock_args_input = load_cell_lock(2, Source::Input)?.args();
    let checker_bond_input = CheckerBondCellLockArgs::from_raw(checker_bond_cell_lock_args_input.as_slice()).ok_or(Error::Encoding)?;

    if checker_bond_input.chain_id_bitmap != EMPTY_BIT_MAP {
        return Err(Error::ChainIdBitMapNotZero);
    }

    //check owner
    if signer != &checker_bond_input.checker_lock_arg[..] {
        return Err(Error::SignatureMismatch);
    }

    Ok(())
}

fn checker_join_sidechain(signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerJoinSidechain,

    Dep:    0 Global Config Cell

    Code Cell                   ->          Code Cell
    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Null                        ->          Checker Info Cell

    */

    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let witness = CheckerJoinSidechainWitness::from_raw(witness.as_reader().raw_data()).ok_or(Error::Encoding)?;

    // input cells
    let config_cell_data_input = load_cell_data(1, Source::Input)?;
    let config_input = SidechainConfigCellData::from_raw(&config_cell_data_input).ok_or(Error::Encoding)?;

    let checker_bond_input_lock_args = load_cell_lock(2, Source::Input)?.args();
    let checker_bond_input_lock_args =
        CheckerBondCellLockArgs::from_raw(checker_bond_input_lock_args.as_reader().raw_data()).ok_or(Error::Encoding)?;

    let checker_bond_input = load_cell_data(2, Source::Input)?;
    let checker_bond_input = CheckerBondCellData::from_raw(&checker_bond_input).ok_or(Error::Encoding)?;

    // output cells
    let config_cell_data_output = load_cell_data(1, Source::Output)?;
    let config_output = SidechainConfigCellData::from_raw(&config_cell_data_output).ok_or(Error::Encoding)?;

    let checker_bond_output_lock_args = load_cell_lock(2, Source::Output)?.args();
    let checker_bond_output_lock_args =
        CheckerBondCellLockArgs::from_raw(checker_bond_output_lock_args.as_reader().raw_data()).ok_or(Error::Encoding)?;

    let checker_bond_output = load_cell_data(2, Source::Output)?;
    let checker_bond_output = CheckerBondCellData::from_raw(&checker_bond_output).ok_or(Error::Encoding)?;

    let checker_info_cell_data_output = load_cell_data(3, Source::Output)?;
    let checker_info_output = CheckerInfoCellData::from_raw(&checker_info_cell_data_output).ok_or(Error::Encoding)?;

    let mut config_res = config_input.clone();

    config_res.checker_total_count += 1;
    config_res.checker_bitmap = bit_map_add(&config_res.checker_bitmap, witness.checker_id)?;

    let mut checker_bond_res_lock_args = checker_bond_input_lock_args.clone();
    checker_bond_res_lock_args.chain_id_bitmap = bit_map_add(&checker_bond_res_lock_args.chain_id_bitmap, witness.chain_id)?;

    let mut checker_info_res = checker_info_output.clone();
    checker_info_res.chain_id = witness.chain_id;
    checker_info_res.checker_id = witness.checker_id;
    checker_info_res.unpaid_fee = 0;
    checker_info_res.checker_public_key_hash = signer;
    checker_info_res.mode = CheckerInfoCellMode::Idle;

    // TODO: Check update gap limit
    if config_res.chain_id != witness.chain_id || config_res != config_output {
        return Err(Error::SidechainConfigMismatch);
    }
    if checker_bond_res_lock_args != checker_bond_output_lock_args
        || checker_bond_input != checker_bond_output
        || checker_bond_output.amount < config_input.minimal_bond
    {
        return Err(Error::CheckerBondMismatch);
    }
    if checker_info_res != checker_info_output {
        return Err(Error::CheckerInfoMismatch);
    }

    Ok(())
}

fn checker_quit_sidechain(_signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerQuitSidechain

    Dep:    0 Global Config Cell

    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Checker Info Cell           ->          Null

    */

    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let witness = CheckerQuitSidechainWitness::from_raw(&witness.as_slice()[..]).ok_or(Error::Encoding)?;

    let config_cell_data_input = load_cell_data(1, Source::Input)?;
    let config_input = SidechainConfigCellData::from_raw(&config_cell_data_input).ok_or(Error::Encoding)?;

    let checker_bond_cell_lock_args_input = load_cell_lock(2, Source::Input)?.args();
    let checker_bond_input = CheckerBondCellLockArgs::from_raw(checker_bond_cell_lock_args_input.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_cell_data_input = load_cell_data(3, Source::Input)?;
    let checker_info_input = CheckerInfoCellData::from_raw(checker_info_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let config_cell_data_output = load_cell_data(1, Source::Output)?;
    let config_output = SidechainConfigCellData::from_raw(&config_cell_data_output).ok_or(Error::Encoding)?;

    let checker_bond_cell_lock_args_output = load_cell_lock(2, Source::Output)?.args();
    let checker_bond_output = CheckerBondCellLockArgs::from_raw(checker_bond_cell_lock_args_output.as_slice()).ok_or(Error::Encoding)?;

    let mut config_res = config_input.clone();
    config_res.chain_id = witness.chain_id;
    config_res.checker_total_count -= 1;
    config_res.checker_bitmap = bit_map_remove(config_res.checker_bitmap, witness.checker_id)?;

    let mut checker_bond_res = checker_bond_input.clone();
    checker_bond_res.chain_id_bitmap = bit_map_remove(checker_bond_res.chain_id_bitmap, witness.chain_id)?;

    if config_res != config_output || checker_bond_res != checker_bond_output {
        return Err(Error::Wrong);
    }

    if checker_info_input.chain_id != witness.chain_id
        || checker_info_input.checker_id != witness.checker_id
        || checker_info_input.mode != CheckerInfoCellMode::Idle
    {
        return Err(Error::Wrong);
    }

    Ok(())
}

fn checker_submit_task(_signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerSubmitTask,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->         Code Cell
    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          Null

    */

    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let witness = CheckerQuitSidechainWitness::from_raw(&witness.as_slice()[..]).ok_or(Error::Encoding)?;

    let checker_info_cell_data_input = load_cell_data(1, Source::Input)?;
    let checker_info_input = CheckerInfoCellData::from_raw(checker_info_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let task_cell_data_input = load_cell_data(2, Source::Input)?;
    let task_cell_input = TaskCellData::from_raw(task_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_cell_data_output = load_cell_data(1, Source::Output)?;
    let checker_info_output = CheckerInfoCellData::from_raw(checker_info_cell_data_output.as_slice()).ok_or(Error::Encoding)?;

    let mut checker_info_res = checker_info_input.clone();
    checker_info_res.chain_id = witness.chain_id;
    checker_info_res.checker_id = witness.checker_id;
    checker_info_res.mode = CheckerInfoCellMode::TaskPassed;

    if checker_info_res != checker_info_output {
        return Err(Error::Wrong);
    }

    if task_cell_input.chain_id != witness.chain_id || task_cell_input.mode != TaskCellMode::Task {
        return Err(Error::Wrong);
    }

    Ok(())
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

    let task_cell_data_input = load_cell_data(2, Source::Input)?;
    let task_cell_input = TaskCellData::from_raw(task_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_cell_data_output = load_cell_data(1, Source::Output)?;
    let checker_info_output = CheckerInfoCellData::from_raw(checker_info_cell_data_output.as_slice()).ok_or(Error::Encoding)?;

    let mut checker_info_res = checker_info_input.clone();

    checker_info_res.chain_id = witness.chain_id;
    checker_info_res.checker_id = witness.checker_id;
    checker_info_res.mode = CheckerInfoCellMode::ChallengeRejected;

    let mut task_cell_res = task_cell_input.clone();

    if checker_info_res != checker_info_output {
        return Err(Error::Wrong);
    }

    if task_cell_input.chain_id != witness.chain_id || task_cell_input.mode != TaskCellMode::Task {
        return Err(Error::Wrong);
    }

    task_cell_res.chain_id = witness.chain_id;
    task_cell_res.mode = TaskCellMode::Challenge;

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

    let task_cell_data_input = load_cell_data(2, Source::Input)?;
    let task_cell_input = TaskCellData::from_raw(task_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_cell_data_output = load_cell_data(1, Source::Output)?;
    let checker_info_output = CheckerInfoCellData::from_raw(checker_info_cell_data_output.as_slice()).ok_or(Error::Encoding)?;

    let mut checker_info_res = checker_info_input.clone();
    checker_info_res.chain_id = witness.chain_id;
    checker_info_res.checker_id = witness.checker_id;
    checker_info_res.mode = CheckerInfoCellMode::ChallengeRejected;

    if checker_info_res != checker_info_output {
        return Err(Error::Wrong);
    }

    if task_cell_input.chain_id != witness.chain_id || task_cell_input.mode != TaskCellMode::Challenge {
        return Err(Error::Wrong);
    }

    Ok(())
}

fn checker_take_beneficiary(_signer: [u8; 20]) -> Result<(), Error> {
    /*
    CheckerTakeBeneficiary,

    Dep:    0 Global Config Cell

    Code Cell                   ->         Code Cell
    Checker Info Cell           ->          Checker Info Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    Muse Token Cell             ->          Muse Token Cell

    */

    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let witness = CheckerTakeBeneficiaryWitness::from_raw(&witness.as_slice()[..]).ok_or(Error::Encoding)?;

    let checker_info_cell_data_input = load_cell_data(1, Source::Input)?;
    let checker_info_input = CheckerInfoCellData::from_raw(checker_info_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let sidechain_fee_cell_data_input = load_cell_data(2, Source::Input)?;
    let sidechain_fee_input = SidechainFeeCellData::from_raw(sidechain_fee_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let muse_token_data_input = load_cell_data(3, Source::Input)?;
    let muse_token_input = MuseTokenData::from_raw(muse_token_data_input.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_cell_data_output = load_cell_data(1, Source::Output)?;
    let checker_info_output = CheckerInfoCellData::from_raw(checker_info_cell_data_output.as_slice()).ok_or(Error::Encoding)?;

    let sidechain_fee_cell_data_output = load_cell_data(2, Source::Output)?;
    let sidechain_fee_output = SidechainFeeCellData::from_raw(sidechain_fee_cell_data_output.as_slice()).ok_or(Error::Encoding)?;

    let muse_token_data_output = load_cell_data(3, Source::Output)?;
    let muse_token_output = MuseTokenData::from_raw(muse_token_data_output.as_slice()).ok_or(Error::Encoding)?;

    let mut checker_info_res = checker_info_input.clone();
    checker_info_res.chain_id = witness.chain_id;
    checker_info_res.checker_id = witness.checker_id;
    checker_info_res.unpaid_fee -= witness.fee;

    let mut sidechain_fee_res = sidechain_fee_input.clone();
    sidechain_fee_res.amount -= witness.fee;

    let mut muse_token_res = muse_token_input.clone();
    muse_token_res.amount -= witness.fee;

    if checker_info_res != checker_info_output || sidechain_fee_res != sidechain_fee_output || muse_token_res != muse_token_output {
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

    let sidechain_config_cell_data_output = load_cell_data(1, Source::Output)?;
    let sidechain_config_output = SidechainConfigCellData::from_raw(sidechain_config_cell_data_output.as_slice()).ok_or(Error::Encoding)?;

    let sidechain_state_cell_data_output = load_cell_data(2, Source::Output)?;
    let sidechain_state_output = SidechainStateCellData::from_raw(sidechain_state_cell_data_output.as_slice()).ok_or(Error::Encoding)?;

    if sidechain_config_output.chain_id != witness.chain_id {
        return Err(Error::Wrong);
    }

    if sidechain_state_output.chain_id != witness.chain_id {
        return Err(Error::Wrong);
    }

    Ok(())
}

fn collator_publish_task(_signer: [u8; 20]) -> Result<(), Error> {
    /*
    CollatorPublishTask,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->          Code Cell
    Sidechain State Cell        ->          Sidechain State Cell
    Sidechain Bond Cell/Sudt    ->          Sidechain Bond Cell
    Null                        ->          [Task Cell]

    */

    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let witness = CollatorPublishTaskWitness::from_raw(&witness.as_slice()[..]).ok_or(Error::Encoding)?;

    let sidechain_config_cell_data_celldep = load_cell_data(1, Source::CellDep)?;
    let sidechain_config_celldep =
        SidechainConfigCellData::from_raw(sidechain_config_cell_data_celldep.as_slice()).ok_or(Error::Encoding)?;

    let sidechain_state_cell_data_input = load_cell_data(1, Source::Input)?;
    let sidechain_state_input = SidechainStateCellData::from_raw(sidechain_state_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let sudt_cell_data_input = load_cell_data(2, Source::Input)?;
    let sudt_input = SudtTokenData::from_raw(sudt_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let sidechain_state_cell_data_ouput = load_cell_data(1, Source::Output)?;
    let sidechain_state_output = SidechainStateCellData::from_raw(sidechain_state_cell_data_ouput.as_slice()).ok_or(Error::Encoding)?;

    let sidechain_bond_data_output = load_cell_data(2, Source::Output)?;
    let _sidechain_bond_output = SidechainBondCellData::from_raw(sidechain_bond_data_output.as_slice()).ok_or(Error::Encoding)?;

    let mut sidechain_state_res = sidechain_state_input;
    //currently always true
    sidechain_state_res.chain_id = witness.chain_id;

    let mut task_cell_res = TaskCellData::default();
    task_cell_res.chain_id = witness.chain_id;
    task_cell_res.mode = TaskCellMode::Challenge;
    //todo calc heights and block hashs from sidechain state cell

    if !QueryIter::new(load_cell_data, Source::Output).skip(3).all(|task_cell_data_output| {
        let task_cell_output = TaskCellData::from_raw(task_cell_data_output.as_slice());
        if let Some(task_cell_output) = task_cell_output {
            task_cell_output == task_cell_res
        } else {
            false
        }
    }) {
        return Err(Error::Wrong);
    }

    if sidechain_state_res != sidechain_state_output {
        return Err(Error::Wrong);
    }

    if sudt_input.amount < sidechain_config_celldep.minimal_bond {
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

    let sidechain_state_cell_data_input = load_cell_data(1, Source::Input)?;
    let sidechain_state_input = SidechainStateCellData::from_raw(sidechain_state_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let sidechain_fee_cell_data_input = load_cell_data(2, Source::Input)?;
    let sidechain_fee_input = SidechainFeeCellData::from_raw(sidechain_fee_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_inputs = QueryIter::new(load_cell_data, Source::Input)
        .skip(3)
        .map(|checker_info_cell_data_input| CheckerInfoCellData::from_raw(checker_info_cell_data_input.as_slice()))
        .collect::<Option<Vec<_>>>()
        .ok_or(Error::Encoding)?;

    let sidechain_state_cell_data_ouput = load_cell_data(1, Source::Output)?;
    let sidechain_state_output = SidechainStateCellData::from_raw(sidechain_state_cell_data_ouput.as_slice()).ok_or(Error::Encoding)?;

    let sidechain_fee_cell_data_output = load_cell_data(2, Source::Output)?;
    let sidechain_fee_output = SidechainFeeCellData::from_raw(sidechain_fee_cell_data_output.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_outputs = QueryIter::new(load_cell_data, Source::Output)
        .skip(3)
        .map(|checker_info_cell_data_input| CheckerInfoCellData::from_raw(checker_info_cell_data_input.as_slice()))
        .collect::<Option<Vec<_>>>()
        .ok_or(Error::Encoding)?;

    let mut sidechain_state_res = sidechain_state_input;
    //currently always true
    sidechain_state_res.chain_id = witness.chain_id;

    let mut sidechain_fee_res = sidechain_fee_input;
    sidechain_fee_res.amount -= witness.fee;

    if sidechain_state_res != sidechain_state_output {
        return Err(Error::Wrong);
    }

    if sidechain_fee_res != sidechain_fee_output {
        return Err(Error::Wrong);
    }

    if !checker_info_inputs.into_iter().zip(checker_info_outputs).all(|(input, output)| {
        let mut res = input;
        res.chain_id = witness.chain_id;
        res.unpaid_fee += witness.fee_per_checker;
        res.mode = CheckerInfoCellMode::Idle;

        res == output
    }) {
        return Err(Error::Wrong);
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

    let sidechain_state_cell_data_input = load_cell_data(1, Source::Input)?;
    let sidechain_state_input = SidechainStateCellData::from_raw(sidechain_state_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let sidechain_fee_cell_data_input = load_cell_data(2, Source::Input)?;
    let sidechain_fee_input = SidechainFeeCellData::from_raw(sidechain_fee_cell_data_input.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_inputs = QueryIter::new(load_cell_data, Source::Input)
        .skip(3)
        .map(|checker_info_cell_data_input| CheckerInfoCellData::from_raw(checker_info_cell_data_input.as_slice()))
        .collect::<Option<Vec<_>>>()
        .ok_or(Error::Encoding)?;

    let sidechain_state_cell_data_ouput = load_cell_data(1, Source::Output)?;
    let sidechain_state_output = SidechainStateCellData::from_raw(sidechain_state_cell_data_ouput.as_slice()).ok_or(Error::Encoding)?;

    let sidechain_fee_cell_data_output = load_cell_data(2, Source::Output)?;
    let sidechain_fee_output = SidechainFeeCellData::from_raw(sidechain_fee_cell_data_output.as_slice()).ok_or(Error::Encoding)?;

    let checker_info_outputs = QueryIter::new(load_cell_data, Source::Output)
        .skip(3)
        .map(|checker_info_cell_data_input| CheckerInfoCellData::from_raw(checker_info_cell_data_input.as_slice()))
        .collect::<Option<Vec<_>>>()
        .ok_or(Error::Encoding)?;

    let _sidechain_state_cell_data_ouput = load_cell_data(1, Source::Output)?;

    let mut sidechain_state_res = sidechain_state_input;
    //currently always true
    sidechain_state_res.chain_id = witness.chain_id;

    let mut sidechain_fee_res = sidechain_fee_input;
    sidechain_fee_res.amount -= witness.fee;

    if sidechain_state_res != sidechain_state_output {
        return Err(Error::Wrong);
    }

    if sidechain_fee_res != sidechain_fee_output {
        return Err(Error::Wrong);
    }

    let checker_info_res = checker_info_inputs
        .into_iter()
        .filter_map(|checker_info_input| {
            let result = bit_map_marked(witness.punish_checker_bitmap, checker_info_input.checker_id);

            if result.is_ok() {
                return Some(checker_info_input);
            } else {
                return None;
            }
        })
        .collect::<Vec<_>>();

    if !checker_info_res.into_iter().zip(checker_info_outputs).all(|(mut res, output)| {
        res.chain_id = witness.chain_id;
        res.unpaid_fee += witness.fee_per_checker;
        res.mode = CheckerInfoCellMode::Idle;

        res == output
    }) {
        return Err(Error::Wrong);
    }

    Ok(())
}

fn collator_refresh_task(_signer: [u8; 20]) -> Result<(), Error> {
    /*
    CollatorRefreshTask,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell

    Code Cell                   ->          Code Cell
    [Task Cell]                 ->          [Task Cell]

    */
    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let _witness = CollatorRefreshTaskWitness::from_raw(&witness.as_slice()[..]).ok_or(Error::Encoding)?;

    let checker_info_inputs = QueryIter::new(load_cell_data, Source::Input)
        .skip(1)
        .map(|task_cell_data_input| TaskCellData::from_raw(task_cell_data_input.as_slice()))
        .collect::<Option<Vec<_>>>()
        .ok_or(Error::Encoding)?;

    let checker_info_outputs = QueryIter::new(load_cell_data, Source::Output)
        .skip(1)
        .map(|task_cell_data_output| TaskCellData::from_raw(task_cell_data_output.as_slice()))
        .collect::<Option<Vec<_>>>()
        .ok_or(Error::Encoding)?;

    if !checker_info_inputs.into_iter().zip(checker_info_outputs).all(|(input, output)| {
        let res = input;
        res == output
    }) {
        return Err(Error::Wrong);
    }

    Ok(())
}

fn collator_unlock_bond(_signer: [u8; 20]) -> Result<(), Error> {
    /*
    CollatorUnlockBond,

    Dep:    0 Global Config Cell
    Dep:    1 Sidechain Config Cell
    Dep:    2 Sidechain State Cell

    Code Cell                   ->          Code Cell
    Sidechain Bond Cell         ->          Sudt Cell

    */

    let witness = load_witness_args(0, Source::Input)?;
    let witness = witness.input_type().to_opt().ok_or(Error::MissingWitness)?;
    let _witness = CollatorUnlockBondWitness::from_raw(&witness.as_slice()[..]).ok_or(Error::Encoding)?;

    Ok(())
}
