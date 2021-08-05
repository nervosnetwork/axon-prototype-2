use crate::{check_args_len, FromRaw, PureSudtTokenCell, Serialize};

const SIDECHAIN_BOND_LOCK_ARGS_LEN: usize = 37;

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
pub struct SidechainBondCell {
    pub amount: u128,
}

PureSudtTokenCell!(SidechainBondCell);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainBondCellLockArgs {
    pub chain_id:                u8,
    pub collator_lock_arg:       [u8; 20],
    pub unlock_sidechain_height: u128,
}

impl FromRaw for SidechainBondCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<SidechainBondCellLockArgs> {
        check_args_len(arg_raw_data.len(), SIDECHAIN_BOND_LOCK_ARGS_LEN)?;

        let chain_id = u8::from_raw(&arg_raw_data[0..1])?;

        let mut collator_lock_arg = [0u8; 20];
        collator_lock_arg.copy_from_slice(&arg_raw_data[1..21]);

        let unlock_sidechain_height = u128::from_raw(&arg_raw_data[21..37])?;

        Some(SidechainBondCellLockArgs {
            chain_id,
            collator_lock_arg,
            unlock_sidechain_height,
        })
    }
}

impl Serialize for SidechainBondCellLockArgs {
    type RawType = [u8; SIDECHAIN_BOND_LOCK_ARGS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; SIDECHAIN_BOND_LOCK_ARGS_LEN];

        buf[0..1].copy_from_slice(&self.chain_id.serialize());

        buf[1..21].copy_from_slice(&self.collator_lock_arg);

        buf[21..37].copy_from_slice(&self.unlock_sidechain_height.serialize());

        buf
    }
}
