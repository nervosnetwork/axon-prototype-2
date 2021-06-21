use crate::{check_args_len, FromRaw, Serialize};

const SIDECHAIN_STATE_DATA_LEN: usize = 98;
const SIDECHAIN_STATE_TYPE_ARGS_LEN: usize = 1;
/**
    Sidechain State Cell
    Data:
    Type:
        codehash: typeId
        hashtype: type
        args: chain_id
    Lock:
        codehash: A.S.
        hashtype: type
        args: null
*/
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainStateCellData {
    pub chain_id:               u8,
    pub version:                u8,
    pub latest_block_height:    u128,
    pub latest_block_hash:      [u8; 32],
    pub committed_block_height: u128,
    pub committed_block_hash:   [u8; 32],
}

impl FromRaw for SidechainStateCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Option<SidechainStateCellData> {
        check_args_len(cell_raw_data.len(), SIDECHAIN_STATE_DATA_LEN)?;

        let chain_id = u8::from_raw(&cell_raw_data[0..1])?;
        let version = u8::from_raw(&cell_raw_data[1..2])?;

        let latest_block_height = u128::from_raw(&cell_raw_data[2..18])?;
        let mut latest_block_hash = [0u8; 32];
        latest_block_hash.copy_from_slice(&cell_raw_data[18..50]);

        let committed_block_height = u128::from_raw(&cell_raw_data[50..66])?;
        let mut committed_block_hash = [0u8; 32];
        committed_block_hash.copy_from_slice(&cell_raw_data[66..98]);

        Some(SidechainStateCellData {
            chain_id,
            version,
            latest_block_height,
            latest_block_hash,
            committed_block_height,
            committed_block_hash,
        })
    }
}

impl Serialize for SidechainStateCellData {
    type RawType = [u8; SIDECHAIN_STATE_DATA_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; SIDECHAIN_STATE_DATA_LEN];

        buf[0..1].copy_from_slice(&self.chain_id.serialize());
        buf[1..2].copy_from_slice(&self.version.serialize());

        buf[2..18].copy_from_slice(&self.latest_block_height.serialize());
        buf[18..50].copy_from_slice(&self.latest_block_hash);

        buf[50..66].copy_from_slice(&self.committed_block_height.serialize());
        buf[66..98].copy_from_slice(&self.committed_block_hash);

        buf
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainStateCellTypeArgs {
    pub chain_id: u8,
}

impl FromRaw for SidechainStateCellTypeArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<SidechainStateCellTypeArgs> {
        check_args_len(arg_raw_data.len(), SIDECHAIN_STATE_TYPE_ARGS_LEN)?;

        let chain_id = u8::from_raw(&arg_raw_data[0..1])?;

        Some(SidechainStateCellTypeArgs { chain_id })
    }
}

impl Serialize for SidechainStateCellTypeArgs {
    type RawType = [u8; SIDECHAIN_STATE_TYPE_ARGS_LEN];

    fn serialize(&self) -> Self::RawType {
        self.chain_id.serialize()
    }
}
