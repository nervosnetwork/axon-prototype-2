use alloc::vec::Vec;
use core::result::Result;

use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    dynamic_loading_c_impl::CKBDLContext,
    high_level::{load_cell_data, load_script},
};

use crate::error::Error;

use ckb_lib_secp256k1::LibSecp256k1;
use schemas::sidechain_state_cell::{SSCReader, SSC};

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    // Owner lock arg | Chain id | Unlock sidechain height
    //    20 Bytes    |  1 Byte  |         8 Bytes
    if args.len() != 29 {
        return Err(Error::InvalidArgument);
    }

    let lock_arg = args.slice(0..20);
    let chain_id = args.slice(20..21);
    let unlock_sidechain_height = args.slice(21..29);

    // Load dynamic library for checking signature
    let mut context = unsafe { CKBDLContext::<[u8; 128 * 1024]>::new() };
    let lib = LibSecp256k1::load(&mut context);

    lib.check_signature(&lock_arg).map_err(|err_code| {
        debug!("secp256k1 error {}", err_code);
        Error::Secp256k1Error
    })?;

    // TODO: Find SSC by type hash
    let sidechain_state_data =
        load_cell_data(2, Source::CellDep).map_err(|err| Error::from(err))?;
    SSCReader::verify(sidechain_state_data.as_slice(), false).map_err(|err| Error::from(err))?;
    let sidechain_state = SSC::new_unchecked(sidechain_state_data.into());

    if Vec::<u8>::from(chain_id) != sidechain_state.chain_id().as_slice() {
        return Err(Error::ChainIdMismatch);
    }

    if &(*unlock_sidechain_height) < sidechain_state.confirmed_sidechain_height().as_slice() {
        return Err(Error::BlockHeightNotPassed);
    }

    Ok(())
}
