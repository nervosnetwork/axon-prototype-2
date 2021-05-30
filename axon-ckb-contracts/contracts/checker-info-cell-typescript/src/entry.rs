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
use ckb_std::high_level::{load_cell, load_cell_data, load_cell_type_hash, load_witness_args, QueryIter};
use ckb_std::syscalls::load_cell_by_field;
use common::cell::CellType::SidechainConfig;
use common::cell::{CheckerInfoCellData, CheckerInfoTypeWitness, FromRaw, GlobalConfigCellData, SidechainConfigCellData};
use common::pattern::{
    is_checker_join_sidechain, is_checker_publish_challenge, is_checker_quit_sidechain, is_checker_submit_challenge,
    is_checker_submit_task, is_checker_take_beneficiary, Pattern,
};
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
    let mut pattern = Pattern::Unrecognised;

    let w = load_witness_args(0, Source::Input)?;

    for witness_args in QueryIter::new(load_witness_args, Source::GroupInput) {
        let input_type_witness = witness_args.input_type().to_opt().ok_or(Error::MissingWitness)?;
        let witness = CheckerInfoTypeWitness::from_raw(input_type_witness.as_slice())?;
        let p: Pattern = witness.pattern.into();

        if p == Pattern::Unrecognised {
            return Err(Error::UnknownPattern);
        }

        if pattern == Pattern::Unrecognised {
            pattern = p
        } else if p != pattern {
            return Err(Error::PatternCollision);
        }
    }

    match pattern {
        /*
        CheckerJoinSidechain,

        Dep:    1 Global Config Cell

        Sidechain Config Cell*      ->          Sidechain Config Cell
        Checker Bond Cell           ->          Checker Bond Cell
        Null                        ->          Checker Info Cell

        */
        Pattern::CheckerJoinSidechain => {
            is_checker_join_sidechain()?;
            checker_join()?
        }
        /*
        CheckerQuitSidechain

        Dep:    1 Global Config Cell

        Sidechain Config Cell*      ->          Sidechain Config Cell
        Checker Bond Cell           ->          Checker Bond Cell
        Checker Info Cell           ->          Null
        */
        Pattern::CheckerQuitSidechain => {
            is_checker_quit_sidechain()?;
        }
        /*
        CheckerSubmitTask,

        Dep:    1 Global Config Cell
        Dep:    2 Sidechain Config Cell

        Checker Info Cell           ->          Checker Info Cell
        Task Cell                   ->          Null

        */
        Pattern::CheckerSubmitTask => {
            is_checker_submit_task()?;
        }
        /*
        CheckerPublishChallenge,

        Dep:    1 Global Config Cell
        Dep:    2 Sidechain Config Cell

        Checker Info Cell           ->          Checker Info Cell
        Task Cell                   ->          [Task Cell]

        */
        Pattern::CheckerPublishChallenge => {
            is_checker_publish_challenge()?;
        }

        /*
        CheckerSubmitChallenge,

        Dep:    1 Global Config Cell
        Dep:    2 Sidechain Config Cell

        Checker Info Cell           ->          Checker Info Cell
        Task Cell                   ->          Null

        */
        Pattern::CheckerSubmitChallenge => {
            is_checker_submit_challenge()?;
        }
        /*
        CheckerTakeBeneficiary,

        Dep:    1 Global Config Cell

        Checker Info Cell           ->          Checker Info Cell
        Sidechain Fee Cell          ->          Sidechain Fee Cell
        Muse Token Cell             ->          Muse Token Cell
        */
        Pattern::CheckerTakeBeneficiary => {
            is_checker_take_beneficiary()?;
        }
        _ => return Err(Error::PatternInvalid),
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

fn checker_join() -> Result<(), Error> {
    let script = load_script()?;

    let sidechain_config_data = load_cell_data(0, Source::Output)?;
    let sidechain_config_data = SidechainConfigCellData::from_raw(&sidechain_config_data)?;

    let checker_info_data = load_cell_data(2, Source::Output)?;
    let checker_info_data = CheckerInfoCellData::from_raw(&checker_info_data)?;

    // todo not finished
    Ok(())
}
