use crate::{check_args_len, decode_u128, decode_u8, FromRaw, SUDT_DATA_LEN};

const SIDECHAIN_BOND_LOCK_ARGS_LEN: usize = 49;

/**
    Sidechain Bond Cell
    Data:
    Type:
        codehash: sudt
        hashtype: type
        args: custom sudt admin
    Lock:
        codehash: sidechain bond cell lockscript
        hashtype: type
        args: chain_id | collator_public_key | unlock_sidechain_height
*/

// which is standard sudt
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainBondCellData {
    pub amount: u128,
}

impl FromRaw for SidechainBondCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Option<SidechainBondCellData> {
        check_args_len(cell_raw_data.len(), SUDT_DATA_LEN)?;

        let sudt_amount = decode_u128(&cell_raw_data[0..16])?;

        Some(SidechainBondCellData { amount: sudt_amount })
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainBondCellLockArgs {
    pub chain_id:                u8,
    pub collator_public_key:     [u8; 32],
    pub unlock_sidechain_height: u128,
}

impl FromRaw for SidechainBondCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<SidechainBondCellLockArgs> {
        check_args_len(arg_raw_data.len(), SIDECHAIN_BOND_LOCK_ARGS_LEN)?;

        let chain_id = decode_u8(&arg_raw_data[0..1])?;

        let mut collator_public_key = [0u8; 32];
        collator_public_key.copy_from_slice(&arg_raw_data[1..33]);

        let unlock_sidechain_height = decode_u128(&arg_raw_data[33..49])?;

        Some(SidechainBondCellLockArgs {
            chain_id,
            collator_public_key,
            unlock_sidechain_height,
        })
    }
}
