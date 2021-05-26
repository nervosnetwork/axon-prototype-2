use core::result::Result;

use ckb_std::{
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    dynamic_loading_c_impl::CKBDLContext,
    high_level::load_script,
};

use crate::error::Error;

use ckb_lib_secp256k1::LibSecp256k1;
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell, load_cell_data, load_cell_type_hash};
use common::cell::{CheckerInfoCellData, FromRaw, GlobalConfigCellData};
use common::pattern::{get_input_cell_count, get_output_cell_count};
use common::{SUDT_CODEHASH, SUDT_HASHTYPE};

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

    /*let input_count = get_input_cell_count();
    let output_count = get_output_cell_count();
    let script = load_script()?;
    let args = script.args().as_slice();
    let data = CheckerInfoCellData::from_raw(args).unwrap();

    let global_config_cell_type_hash = load_cell_type_hash(0, Source::CellDep).unwrap().unwrap();

    if (global_config_cell_type_hash != data.global_config_cell_type_hash) {
        //error
    }

    let global_cell = load_cell_data(0, Source::CellDep)?;
    let global_cell_data = GlobalConfigCellData::from_raw(&global_cell)?;
    /*
    CheckerJoinSidechain,

    Dep:    1 Global Config Cell

    Sidechain Config Cell*      ->          Sidechain Config Cell
    Checker Bond Cell           ->          Checker Bond Cell
    Null                        ->          Checker Info Cell

    */
    if (input_count == 2 && output_count == 3) {
        let input1 = load_cell(0, Source::Input).unwrap();
        if input1.type_().is_none() {
            //error
        }
        if input1.type_().to_opt().unwrap().code_hash().as_slice() != global_cell_data.sidechain_config_cell_type_hashtype
            || input1.type_().to_opt().unwrap().hash_type().as_slice() != global_cell_data.sidechain_config_cell_type_hashtype
        {
            //error
        }

        let input2 = load_cell(0, Source::Input).unwrap();
        if input2.type_().is_none() {
            //error
        }
        if input2.type_().to_opt().unwrap().code_hash().as_slice() != SUDT_CODEHASH
            || input2.type_().to_opt().unwrap().hash_type().as_slice() != SUDT_HASHTYPE
        {
            //error
        }

        let output1 = load_cell(0, Source::Output).unwrap();
        if output1.type_().is_none() {
            //error
        }
        if output1.type_().to_opt().unwrap().code_hash().as_slice() != global_cell_data.sidechain_config_cell_type_hashtype
            || output1.type_().to_opt().unwrap().hash_type().as_slice() != global_cell_data.sidechain_config_cell_type_hashtype
        {
            //error
        }

        let output2 = load_cell(0, Source::Output).unwrap();
        if output2.type_().is_none() {
            //error
        }
        if output2.type_().to_opt().unwrap().code_hash().as_slice() != SUDT_CODEHASH
            || output2.type_().to_opt().unwrap().hash_type().as_slice() != SUDT_HASHTYPE
        {
            //error
        }
    }

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
