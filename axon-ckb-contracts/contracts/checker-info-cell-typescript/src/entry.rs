use core::result::Result;

use ckb_std::{
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    dynamic_loading_c_impl::CKBDLContext,
    high_level::load_script,
};

use crate::error::Error;

use alloc::vec::Vec;
use ckb_lib_secp256k1::LibSecp256k1;
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell, load_cell_data, load_cell_type_hash};
use common::cell::{CheckerInfoCellData, FromRaw, GlobalConfigCellData};
use common::{get_input_cell_count, get_output_cell_count, SUDT_CODEHASH, SUDT_HASHTYPE};

pub fn main() -> Result<(), Error> {
    /*
    related tx:

    1. CheckerJoinSidechain
    2. CheckerQuitSidechain
    3. CheckerSubmitTask
    4. CheckerPublishChallenge
    5. CheckerSubmitChallenge
    6. CheckerTakeBeneficiary

    7. CollatorSubmitTask
    8. CollatorSubmitChallenge
    */
    let script = load_script()?;

    /*
    CheckerJoinSidechain,

    Dep:    1 Global Config Cell

    Sidechain Config Cell*      ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Null                        ->          Checker Info Cell

    */

    /*
    CheckerQuitSidechain

    Dep:    1 Global Config Cell

    Sidechain Config Cell*      ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Checker Info Cell           ->          Null
    */

    /*
    CheckerSubmitTask,

    Dep:    1 Global Config Cell
    Dep:    2 Sidechain Config Cell

    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          Null

    */

    /*
    CheckerPublishChallenge,

    Dep:    1 Global Config Cell
    Dep:    2 Sidechain Config Cell

    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          [Task Cell]

    */

    /*
    CheckerSubmitChallenge,

    Dep:    1 Global Config Cell
    Dep:    2 Sidechain Config Cell

    Checker Info Cell           ->          Checker Info Cell
    Task Cell                   ->          Null

    */

    /*
    CheckerTakeBeneficiary,

    Dep:    1 Global Config Cell

    Checker Info Cell           ->          Checker Info Cell
    Sidechain Fee Cell          ->          Sidechain Fee Cell
    Muse Token Cell             ->          Muse Token Cell
    */

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
