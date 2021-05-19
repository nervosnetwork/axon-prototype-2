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

    // Accept only owner address
    if args.len() != 20 {
        return Err(Error::InvalidArgument);
    }

    let lock_arg = args;

    // Load dynamic library for checking signature
    let mut context = unsafe { CKBDLContext::<[u8; 128 * 1024]>::new() };
    let lib = LibSecp256k1::load(&mut context);

    lib.check_signature(&lock_arg).map_err(|_err_code| {
        debug!("secp256k1 error {}", _err_code);
        Error::Secp256k1Error
    })
}
