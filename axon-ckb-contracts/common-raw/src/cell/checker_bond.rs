use crate::{check_args_len, decode_u128, encode_u128, FromRaw, Serialize, SUDT_DATA_LEN};

const CHECKER_BOND_LOCK_ARGS_LEN: usize = 64;

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
pub struct CheckerBondCellData {
    pub amount: u128,
}

impl FromRaw for CheckerBondCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Option<CheckerBondCellData> {
        check_args_len(cell_raw_data.len(), SUDT_DATA_LEN)?;

        let sudt_amount = decode_u128(&cell_raw_data[0..16])?;

        Some(CheckerBondCellData { amount: sudt_amount })
    }
}

impl Serialize for CheckerBondCellData {
    type RawType = [u8; SUDT_DATA_LEN];

    fn serialize(&self) -> Self::RawType {
        encode_u128(self.amount)
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct CheckerBondCellLockArgs {
    pub checker_public_key: [u8; 32],
    pub chain_id_bitmap:    [u8; 32],
}

impl FromRaw for CheckerBondCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<CheckerBondCellLockArgs> {
        check_args_len(arg_raw_data.len(), CHECKER_BOND_LOCK_ARGS_LEN)?;

        let mut checker_address = [0u8; 32];
        checker_address.copy_from_slice(&arg_raw_data[0..32]);

        let mut chain_id_bitmap = [0u8; 32];
        chain_id_bitmap.copy_from_slice(&arg_raw_data[32..64]);

        Some(CheckerBondCellLockArgs {
            checker_public_key: checker_address,
            chain_id_bitmap,
        })
    }
}
