use crate::{FromRaw, Serialize};
use core::convert::TryFrom;

const PATTERN_LEN: usize = 1;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum Pattern {
    AdminCreateSidechain = 0u8,

    CheckerBondWithdraw = 1u8,
    CheckerJoinSidechain,
    CheckerQuitSidechain,
    CheckerSubmitTask,
    CheckerPublishChallenge,
    CheckerTakeBeneficiary,

    CollatorPublishTask = 7u8,
    CollatorSubmitTask,
    CollatorSubmitFaildChallenge,
    CollatorRefreshTask,
    CollatorUnlockBond,
    CollatorSubmitSuccessChallenge,
}

impl TryFrom<u8> for Pattern {
    type Error = ();

    fn try_from(input: u8) -> Result<Self, Self::Error> {
        return Ok(match input {
            0u8 => Self::AdminCreateSidechain,

            1u8 => Self::CheckerBondWithdraw,
            2u8 => Self::CheckerJoinSidechain,
            3u8 => Self::CheckerQuitSidechain,
            4u8 => Self::CheckerSubmitTask,
            5u8 => Self::CheckerPublishChallenge,
            6u8 => Self::CheckerTakeBeneficiary,

            7u8 => Self::CollatorPublishTask,
            8u8 => Self::CollatorSubmitTask,
            9u8 => Self::CollatorSubmitFaildChallenge,
            10u8 => Self::CollatorRefreshTask,
            11u8 => Self::CollatorUnlockBond,
            12u8 => Self::CollatorSubmitSuccessChallenge,
            _ => return Err(()),
        });
    }
}

impl FromRaw for Pattern {
    fn from_raw(raw: &[u8]) -> Option<Self> {
        Pattern::try_from(u8::from_raw(raw)?).ok()
    }
}

impl Serialize for Pattern {
    type RawType = [u8; PATTERN_LEN];

    fn serialize(&self) -> Self::RawType {
        (self.clone() as u8).serialize()
    }
}
