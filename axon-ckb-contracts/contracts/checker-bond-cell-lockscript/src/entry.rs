use core::result::Result;

use ckb_std::{
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    dynamic_loading_c_impl::CKBDLContext,
    high_level::load_script,
};

use crate::error::Error;

use common::pattern::{is_checker_bond_withdraw, is_checker_join_sidechain, is_checker_quit_sidechain};

use alloc::vec::Vec;
use ckb_lib_secp256k1::LibSecp256k1;
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell, load_cell_data, load_cell_lock, load_witness_args};
use ckb_std::syscalls::load_witness;
use common::cell::{CheckerBondCellLockArgs, FromRaw, SidechainConfigCellData};
use common::{bit_check, bit_or, get_input_cell_count, get_output_cell_count, EMPTY_BIT_MAP, GLOBAL_CONFIG_TYPE_HASH};

pub fn main() -> Result<(), Error> {
    /*
    related tx:

    1. CheckerBondDeposit
    2. CheckerBondWithdraw
    3. CheckerJoinSidechain
    4. CheckerQuitSidechain
     */

    let script = load_script()?;

    let args: Vec<u8> = script.args().unpack();

    let args = CheckerBondCellLockArgs::from_raw(&args)?;

    /*
    CheckerBondDeposit,

    Muse Token Cell             ->        Checker Bond Cell

     */

    // won't be triggered!!!

    /*
    Dep:    1 Global Config Cell

    CheckerBondWithdraw,

    Checker Bond Cell           ->         Muse Token Cell

     */
    if is_checker_bond_withdraw().is_ok() {
        checker_bond_withdraw(args)?;
        return Ok(());
    }

    /*
    CheckerJoinSidechain,

    Dep:    1 Global Config Cell

    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Null                        ->          Checker Info Cell

    */
    if is_checker_join_sidechain().is_ok() {
        checker_bond_join(args)?;
        return Ok(());
    }

    /*
    CheckerQuitSidechain

    Dep:    1 Global Config Cell

    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Checker Info Cell           ->          Null

    */
    if is_checker_quit_sidechain().is_ok() {
        checker_bond_quit(args)?;
        return Ok(());
    }

    /*// Load dynamic library for checking signature
    let mut context = unsafe { CKBDLContext::<[u8; 128 * 1024]>::new() };
    let lib = LibSecp256k1::load(&mut context);

    lib.check_signature(&lock_arg).map_err(|_err_code| {
        debug!("secp256k1 error {}", _err_code);
        Error::Secp256k1Error
    })?;

    // TODO: Skip checking bitmap if SCC exists (Joining or leaving sidechain)
    for bitmap in chain_bitmap.into_iter() {
        if *bitmap != 0 {
            return Err(Error::BusyChecker);
        }
    }*/

    Ok(())
}

fn checker_bond_withdraw(args: CheckerBondCellLockArgs) -> Result<(), Error> {
    /*
    Dep:    1 Global Config Cell

    CheckerBondWithdraw,

    Checker Bond Cell           ->         Muse Token Cell

    */

    /*
    Job:

    1. chain_id_bitmap is 0x00
    2. secp256k1 check

     */

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

fn checker_bond_join(args: CheckerBondCellLockArgs) -> Result<(), Error> {
    /*
    CheckerJoinSidechain,

    Dep:    1 Global Config Cell

    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
                                ->          Checker Info Cell

    */

    /*
    Job:

    1. chain_id_bitmap mask cover's current chain id
    2. secp256k1 check

     */

    let config_data = load_cell_data(0, Source::Input)?;
    let config = SidechainConfigCellData::from_raw(&config_data)?;

    // input must not cover
    if !bit_check(args.chain_id_bitmap, config.chain_id) {
        return Err(Error::ChainIdBitMapMismatch);
    }

    //output must cover, and others should not change
    let output = load_cell_lock(1, Source::Output)?;
    let output_args = CheckerBondCellLockArgs::from_raw(output.args().as_slice())?;

    if bit_or(args.chain_id_bitmap, config.chain_id) != output_args.chain_id_bitmap {
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

fn checker_bond_quit(args: CheckerBondCellLockArgs) -> Result<(), Error> {
    /*
    CheckerQuitSidechain

    Dep:    1 Global Config Cell

    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Checker Info Cell           ->

    */

    /*
    Job:

    1. chain_id_bitmap mask cover's current chain id
    2. secp256k1 check

     */

    let config_data = load_cell_data(0, Source::Input)?;
    let config = SidechainConfigCellData::from_raw(&config_data)?;

    // input must cover
    if bit_check(args.chain_id_bitmap, config.chain_id) {
        return Err(Error::ChainIdBitMapMismatch);
    }

    //output must not cover, and others should not change
    let output = load_cell_lock(1, Source::Output)?;
    let output_args = CheckerBondCellLockArgs::from_raw(output.args().as_slice())?;

    //1 output must not cover
    if !bit_check(output_args.chain_id_bitmap, config.chain_id) {
        return Err(Error::ChainIdBitMapMismatch);
    }

    //2 output | chain_id = input
    if bit_or(output_args.chain_id_bitmap, config.chain_id) != args.chain_id_bitmap {
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
