use crate::{check_args_len, decode_u128, decode_u8, FromRaw};

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

        let chain_id = decode_u8(&cell_raw_data[0..1])?;
        let version = decode_u8(&cell_raw_data[1..2])?;

        let latest_block_height = decode_u128(&cell_raw_data[2..18])?;
        let mut latest_block_hash = [0u8; 32];
        latest_block_hash.copy_from_slice(&cell_raw_data[18..50]);

        let committed_block_height = decode_u128(&cell_raw_data[50..66])?;
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

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainStateCellTypeArgs {
    pub chain_id: u8,
}

impl FromRaw for SidechainStateCellTypeArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<SidechainStateCellTypeArgs> {
        check_args_len(arg_raw_data.len(), SIDECHAIN_STATE_TYPE_ARGS_LEN)?;

        let chain_id = decode_u8(&arg_raw_data[0..1])?;

        Some(SidechainStateCellTypeArgs { chain_id })
    }
}
