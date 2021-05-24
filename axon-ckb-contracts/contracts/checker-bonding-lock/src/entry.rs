use core::result::Result;

use ckb_std::{
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    dynamic_loading_c_impl::CKBDLContext,
    high_level::load_script,
};

use crate::error::Error;

use ckb_lib_secp256k1::LibSecp256k1;

pub fn main() -> Result<(), Error> {
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
    }

    Ok(())
}
