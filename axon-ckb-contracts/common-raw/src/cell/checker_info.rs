use core::convert::{TryFrom, TryInto};
use core::result::Result;

use crate::{check_args_len, FromRaw, Serialize};

const CHECKER_INFO_DATA_LEN: usize = 563;
const CHECKER_INFO_TYPE_ARGS_LEN: usize = 33;

/**
    Checker Info Cell
    Data:
    Type:
        codehash: typeId
        hashtype: type
        args: chain_id | public_key
    Lock:
        codehash: A.S.
        hashtype: type
        args: null
*/
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[repr(u8)]
pub enum CheckerInfoCellMode {
    Idle = 0u8,
    TaskPassed,
    ChallengePassed,
    ChallengeRejected,
}

impl TryFrom<u8> for CheckerInfoCellMode {
    type Error = ();

    fn try_from(mode: u8) -> Result<Self, Self::Error> {
        match mode {
            0u8 => Ok(Self::Idle),
            1u8 => Ok(Self::TaskPassed),
            2u8 => Ok(Self::ChallengePassed),
            3u8 => Ok(Self::ChallengeRejected),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct CheckerInfoCellData {
    pub chain_id:                u8,
    pub checker_id:              u8,
    pub unpaid_fee:              u128,
    pub rpc_url:                 [u8; 512],
    pub checker_public_key_hash: [u8; 20],
    pub mode:                    CheckerInfoCellMode,
}

impl Default for CheckerInfoCellData {
    fn default() -> Self {
        CheckerInfoCellData {
            chain_id:                0u8,
            checker_id:              0u8,
            unpaid_fee:              0u128,
            rpc_url:                 [0u8; 512],
            checker_public_key_hash: [0u8; 20],
            mode:                    CheckerInfoCellMode::Idle,
        }
    }
}

impl FromRaw for CheckerInfoCellData {
    fn from_raw(cell_raw_data: &[u8]) -> Option<CheckerInfoCellData> {
        check_args_len(cell_raw_data.len(), CHECKER_INFO_DATA_LEN)?;

        let chain_id = u8::from_raw(&cell_raw_data[0..1])?;
        let checker_id = u8::from_raw(&cell_raw_data[1..2])?;
        let unpaid_fee = u128::from_raw(&cell_raw_data[2..18])?;

        let mut rpc_url = [0u8; 512];
        rpc_url.copy_from_slice(&cell_raw_data[18..530]);

        let mut checker_public_key_hash = [0u8; 20];
        checker_public_key_hash.copy_from_slice(&cell_raw_data[530..550]);

        let mode_u8 = u8::from_raw(&cell_raw_data[550..551])?;
        let mode: CheckerInfoCellMode = mode_u8.try_into().ok()?;

        Some(CheckerInfoCellData {
            chain_id,
            checker_id,
            unpaid_fee,
            rpc_url,
            checker_public_key_hash,
            mode,
        })
    }
}

impl Serialize for CheckerInfoCellData {
    type RawType = [u8; CHECKER_INFO_DATA_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; CHECKER_INFO_DATA_LEN];

        buf[0..1].copy_from_slice(&self.chain_id.serialize());
        buf[1..2].copy_from_slice(&self.checker_id.serialize());
        buf[2..18].copy_from_slice(&self.unpaid_fee.serialize());

        buf[18..530].copy_from_slice(&self.rpc_url);
        buf[530..550].copy_from_slice(&self.checker_public_key_hash);

        buf[550..551].copy_from_slice(&(self.mode as u8).serialize());

        buf
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CheckerInfoCellTypeArgs {
    pub chain_id:           u8,
    pub checker_public_key: [u8; 32],
}

impl FromRaw for CheckerInfoCellTypeArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<CheckerInfoCellTypeArgs> {
        check_args_len(arg_raw_data.len(), CHECKER_INFO_TYPE_ARGS_LEN)?;

        let chain_id = u8::from_raw(&arg_raw_data[0..1])?;

        let mut checker_public_key = [0u8; 32];
        checker_public_key.copy_from_slice(&arg_raw_data[1..33]);

        Some(CheckerInfoCellTypeArgs {
            chain_id,
            checker_public_key,
        })
    }
}
