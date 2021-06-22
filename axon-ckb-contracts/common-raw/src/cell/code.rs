use crate::{check_args_len, FromRaw};

const CODE_LOCK_ARGS_LEN: usize = 20;
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
pub struct CodeCellData {}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct CodeCellLockArgs {
    pub lock_arg: [u8; 20],
}

impl FromRaw for CodeCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<CodeCellLockArgs> {
        check_args_len(arg_raw_data.len(), CODE_LOCK_ARGS_LEN)?;

        let mut lock_arg = [0u8; CODE_LOCK_ARGS_LEN];
        lock_arg.copy_from_slice(&arg_raw_data);

        Some(CodeCellLockArgs { lock_arg })
    }
}
