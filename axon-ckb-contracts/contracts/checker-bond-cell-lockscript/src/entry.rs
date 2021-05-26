use core::result::Result;

use ckb_std::{
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    dynamic_loading_c_impl::CKBDLContext,
    high_level::load_script,
};

use crate::error::Error;

use common::pattern::{get_input_cell_count, get_output_cell_count};

use ckb_lib_secp256k1::LibSecp256k1;
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::load_cell;
use common::cell::{CheckerBondCellLockArgs, FromRaw};

pub fn main() -> Result<(), Error> {
    /*
    related tx:

    1. CheckerBondDeposit
    2. CheckerBondWithdraw
    3. CheckerJoinSidechain
    4. CheckerQuitSidechain
     */

    let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();
    // let script = load_script()?;
    // let args = script.args().as_slice();
    //
    // let checker_bond_lockscript_args = CheckerBondCellLockArgs::from_raw(args).unwrap();

    /*
    CheckerBondDeposit,

    Muse Token Cell             ->        Checker Bond Cell

     */

    // won't be triggered!!!
    // if chain_id_bitmap is not zero, nothing bad, check threshold every time!

    /*
    CheckerBondWithdraw,

    Checker Bond Cell           ->         Muse Token Cell

     */

    if (input_count == 1 && output_count == 1) {
        // check self is run in 1st input lock script

        // todo

        // task:
        // check witness
        // check bitmap is zero
    }

    /*
    CheckerJoinSidechain,

    Dep:    1 Global Config Cell

    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Null                        ->          Checker Info Cell

    */

    // won't be triggered

    /*
    CheckerQuitSidechain

    Dep:    1 Global Config Cell

    Sidechain Config Cell       ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Checker Info Cell           ->          Null

    */

    /* if input_count == 3 && output_count == 2 {
        // check self is run in 3rd input lock script

        // todo

        let hash = load_cell(2, Source::Input).unwrap().lock().code_hash().unpack();

        if (script.code_hash().unpack() != hash) {}

        // task:
        // check witness
        // check bitmap is zero
    }

    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    // Owner lock arg | Chain bitmap
    //    20 Bytes    |   2 Bytes
    if args.len() != 22 {
        return Err(Error::InvalidArgument);
    }

    let lock_arg = args.slice(0..20);
    let chain_bitmap = args.slice(20..22);

    // Load dynamic library for checking signature
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
