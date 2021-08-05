use crate::{check_args_len, FromRaw, PureSudtTokenCell, Serialize};

const CHECKER_BOND_LOCK_ARGS_LEN: usize = 52;

/**
    Checker Bond Cell
    Data:
    Type:
        codehash: sudt
        hashtype: type
        args: muse_token_admin
    Lock:
        codehash: checker bond cell lockscript
        hashtype: type
        args: checker public key | chain id bitmap
*/

// which is standard sudt
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct CheckerBondCell {
    pub amount: u128,
}

PureSudtTokenCell!(CheckerBondCell);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct CheckerBondCellLockArgs {
    pub checker_lock_arg: [u8; 20],
    pub chain_id_bitmap:  [u8; 32],
}

impl FromRaw for CheckerBondCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<CheckerBondCellLockArgs> {
        check_args_len(arg_raw_data.len(), CHECKER_BOND_LOCK_ARGS_LEN)?;

        let mut checker_lock_arg = [0u8; 20];
        checker_lock_arg.copy_from_slice(&arg_raw_data[0..20]);

        let mut chain_id_bitmap = [0u8; 32];
        chain_id_bitmap.copy_from_slice(&arg_raw_data[20..52]);

        Some(CheckerBondCellLockArgs {
            checker_lock_arg,
            chain_id_bitmap,
        })
    }
}

impl Serialize for CheckerBondCellLockArgs {
    type RawType = [u8; CHECKER_BOND_LOCK_ARGS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; CHECKER_BOND_LOCK_ARGS_LEN];

        buf[0..20].copy_from_slice(&self.checker_lock_arg);
        buf[20..52].copy_from_slice(&self.chain_id_bitmap);

        buf
    }
}
