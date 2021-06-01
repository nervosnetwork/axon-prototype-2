// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::{vec, vec::Vec};

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    high_level::{load_script, load_tx_hash},
};

use crate::error::Error;
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell_data, load_cell_lock, load_witness_args};
use ckb_std::syscalls::load_witness;
use common::cell::{CheckerBondCellLockArgs, CodeCellLockArgs, CodeCellTypeWitness, FromRaw, SidechainConfigCellData};
use common::pattern::{
    is_admin_create_sidechain, is_checker_bond_deposit, is_checker_bond_withdraw, is_checker_join_sidechain, is_checker_publish_challenge,
    is_checker_quit_sidechain, is_checker_submit_challenge, is_checker_submit_task, is_checker_take_beneficiary, is_collator_publish_task,
    is_collator_refresh_task, is_collator_submit_challenge, is_collator_submit_task, is_collator_unlock_bond, Pattern,
};
use common::{bit_check, bit_op, get_group_input_cell_count, get_group_output_cell_count, EMPTY_BIT_MAP};

pub fn main() -> Result<(), Error> {
    // of cause, the signer is correct
    let lock_args = load_cell_lock(0, Source::Input)?;
    let signer = CodeCellLockArgs::from_raw(lock_args.args().as_slice())?.public_key_hash;

    let mut witness_payload = [0u8; 128];
    let witness_len = load_witness(&mut witness_payload[..], 0, 0, Source::GroupInput)?;
    let witness_payload = &witness_payload[..witness_len];

    let pattern = CodeCellTypeWitness::from_raw(witness_payload)?;

    match pattern.pattern.into() {
        /*
        CheckerBondDeposit

        Muse Token Cell             ->          Check Bond Cell

        No way to monitor this pattern, regard all check bond cell trustless

         */
        Pattern::CheckerBondDeposit => {
            is_checker_bond_deposit()?;
            checker_bond_deposit()?
        }

        /*
        CheckerBondWithdraw

        Dep:    0 Global Config Cell

        Code Cell                   ->         Code Cell
        Checker Bond Cell           ->         Muse Token Cell

         */
        Pattern::CheckerBondWithdraw => {
            is_checker_bond_withdraw()?;
            checker_bond_withdraw()?
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
            checker_join_sidechain()?
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
            checker_quit_sidechain()?
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
            checker_submit_task()?
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
            checker_publish_challenge()?
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
            checker_submit_challenge()?
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
            checker_take_beneficiary()?
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
            admin_create_sidechain()?
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
            collator_publish_task()?
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
            collator_submit_task()?
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
            collator_submit_challenge()?
        }

        /*
        CollatorRefreshTask,

        Dep:    0 Global Config Cell
        Dep:    1 Sidechain Config Cell

        Code Cell                   ->          Code Cell
        Task Cell                   ->          Task Cell

        */
        Pattern::CollatorRefreshTask => {
            is_collator_refresh_task()?;
            collator_refresh_task()?
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
            collator_unlock_bond()?
        }

        _ => return Err(Error::PatternRecognitionFailure),
    }

    Ok(())
}

fn checker_bond_deposit() -> Result<(), Error> {
    Ok(())
}

fn checker_bond_withdraw() -> Result<(), Error> {
    /*
    Dep:    0 Global Config Cell

    CheckerBondWithdraw,

    Checker Bond Cell           ->         Muse Token Cell

    */

    /*
    Job:

    1. chain_id_bitmap is 0x00
    2. secp256k1 check

     */
    let script = load_script()?;

    let args: Vec<u8> = script.args().unpack();

    let args = CheckerBondCellLockArgs::from_raw(&args)?;

    if args.chain_id_bitmap != EMPTY_BIT_MAP {
        return Err(Error::ChainIdBitMapNotZero);
    }

    // todo check secp256k1
    let witness = load_witness_args(0, Source::Input)?;
    let signature = witness.input_type().to_opt().ok_or(Error::MissingSignature)?;

    let signature = signature.as_slice();
    if signature != &[0u8] {
        return Err(Error::SignatureMismatch);
    }

    Ok(())
}

fn checker_join_sidechain() -> Result<(), Error> {
    /*
    CheckerJoinSidechain,

    Dep:    0 Global Config Cell

    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
                                ->          Checker Info Cell

    */

    /*
    Job:

    1. chain_id_bitmap mask cover's current chain id
    2. secp256k1 check

     */
    let script = load_script()?;

    let args: Vec<u8> = script.args().unpack();

    let args = CheckerBondCellLockArgs::from_raw(&args)?;

    let config_data = load_cell_data(0, Source::Input)?;
    let config = SidechainConfigCellData::from_raw(&config_data)?;

    // input must not cover
    if bit_check(args.chain_id_bitmap, config.chain_id, false) {
        return Err(Error::ChainIdBitMapMismatch);
    }

    //output must cover, and others should not change
    let output = load_cell_lock(1, Source::Output)?;
    let output_args = CheckerBondCellLockArgs::from_raw(output.args().as_slice())?;

    if bit_check(output_args.chain_id_bitmap, config.chain_id, true) {
        return Err(Error::ChainIdBitMapMismatch);
    }

    let mut add_chain_id = args.chain_id_bitmap.clone();
    bit_op(&mut add_chain_id, config.chain_id, true);

    if add_chain_id != output_args.chain_id_bitmap.clone() {
        return Err(Error::ChainIdBitMapMistransfer);
    }

    // todo check secp256k1
    let witness = load_witness_args(0, Source::Input)?;
    let signature = witness.input_type().to_opt().ok_or(Error::MissingSignature)?;

    let signature = signature.as_slice();
    if signature != &[0u8] {
        return Err(Error::SignatureMismatch);
    }

    Ok(())
}

fn checker_quit_sidechain() -> Result<(), Error> {
    /*
    CheckerQuitSidechain

    Dep:    0 Global Config Cell

    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Checker Info Cell           ->

    */

    /*
    Job:

    1. chain_id_bitmap mask cover's current chain id
    2. secp256k1 check

     */
    let script = load_script()?;

    let args: Vec<u8> = script.args().unpack();

    let args = CheckerBondCellLockArgs::from_raw(&args)?;

    let config_data = load_cell_data(0, Source::Input)?;
    let config = SidechainConfigCellData::from_raw(&config_data)?;

    // input must cover
    if bit_check(args.chain_id_bitmap, config.chain_id, true) {
        return Err(Error::ChainIdBitMapMismatch);
    }

    //output must not cover, and others should not change
    let output = load_cell_lock(1, Source::Output)?;
    let output_args = CheckerBondCellLockArgs::from_raw(output.args().as_slice())?;

    //1 output must not cover
    if !bit_check(output_args.chain_id_bitmap, config.chain_id, false) {
        return Err(Error::ChainIdBitMapMismatch);
    }

    let mut remove_chain_id = args.chain_id_bitmap.clone();
    bit_op(&mut remove_chain_id, config.chain_id, false);

    //2 output | chain_id = input
    if remove_chain_id != args.chain_id_bitmap {
        return Err(Error::ChainIdBitMapMismatch);
    }

    // todo check secp256k1
    let witness = load_witness_args(0, Source::Input)?;
    let signature = witness.input_type().to_opt().ok_or(Error::MissingSignature)?;

    let signature = signature.as_slice();
    if signature != &[0u8] {
        return Err(Error::SignatureMismatch);
    }

    Ok(())
}

fn checker_submit_task() -> Result<(), Error> {
    Ok(())
}

fn checker_publish_challenge() -> Result<(), Error> {
    Ok(())
}

fn checker_submit_challenge() -> Result<(), Error> {
    Ok(())
}

fn checker_take_beneficiary() -> Result<(), Error> {
    Ok(())
}

fn admin_create_sidechain() -> Result<(), Error> {
    Ok(())
}

fn collator_publish_task() -> Result<(), Error> {
    Ok(())
}

fn collator_submit_task() -> Result<(), Error> {
    Ok(())
}

fn collator_submit_challenge() -> Result<(), Error> {
    Ok(())
}

fn collator_refresh_task() -> Result<(), Error> {
    Ok(())
}

fn collator_unlock_bond() -> Result<(), Error> {
    Ok(())
}
