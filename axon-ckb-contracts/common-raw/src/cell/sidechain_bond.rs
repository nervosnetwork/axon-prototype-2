use crate::{
    check_args_len,
    common::{BlockHeight, ChainId, PubKeyHash},
    FromRaw, PureSudtTokenCell, Serialize,
};

const SIDECHAIN_BOND_LOCK_ARGS_LEN: usize = 40;

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
    pub chain_id:                ChainId,
    pub collator_lock_arg:       PubKeyHash,
    pub unlock_sidechain_height: BlockHeight,
}

impl FromRaw for SidechainBondCellLockArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<SidechainBondCellLockArgs> {
        check_args_len(arg_raw_data.len(), SIDECHAIN_BOND_LOCK_ARGS_LEN)?;

        let chain_id = ChainId::from_raw(&arg_raw_data[0..4])?;

        let mut collator_lock_arg = PubKeyHash::default();
        collator_lock_arg.copy_from_slice(&arg_raw_data[4..24]);

        let unlock_sidechain_height = BlockHeight::from_raw(&arg_raw_data[24..40])?;

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

        buf[0..4].copy_from_slice(&self.chain_id.serialize());

        buf[4..24].copy_from_slice(&self.collator_lock_arg);

        buf[24..40].copy_from_slice(&self.unlock_sidechain_height.serialize());

        buf
    }
}
