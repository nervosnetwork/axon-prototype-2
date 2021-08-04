use molecule::prelude::*;

use crate::{common::*, molecule::cell::code::CodeCellLockArgsReader, FromRaw};

/*

    Code Cell
    Data: null
    Type:
        codehash: typeId
        hashtype: type
        args: chain_id(for lumos)
    Lock:
        codehash: secp256k1
        hashtype: type
        args: public-key
*/

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct CodeCell {}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct CodeCellLockArgs {
    pub lock_arg: PubKeyHash,
}

impl FromRaw for CodeCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<CodeCellLockArgs> {
        let reader = CodeCellLockArgsReader::from_slice(arg_raw_data).ok()?;

        let mut lock_arg = [0u8; 20];
        lock_arg.copy_from_slice(reader.lock_arg().raw_data());

        Some(CodeCellLockArgs { lock_arg })
    }
}
