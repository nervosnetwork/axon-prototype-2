use core::convert::TryFrom;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum Pattern {
    AdminCreateSidechain = 0u8,

    CheckerBondWithdraw = 1u8,
    CheckerJoinSidechain,
    CheckerQuitSidechain,
    CheckerSubmitTask,
    CheckerPublishChallenge,
    CheckerSubmitChallenge,
    CheckerTakeBeneficiary,

    CollatorPublishTask = 8u8,
    CollatorSubmitTask,
    CollatorSubmitChallenge,
    CollatorRefreshTask,
    CollatorUnlockBond,
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
            6u8 => Self::CheckerSubmitChallenge,
            7u8 => Self::CheckerTakeBeneficiary,

            8u8 => Self::CollatorPublishTask,
            9u8 => Self::CollatorSubmitTask,
            10u8 => Self::CollatorSubmitChallenge,
            11u8 => Self::CollatorRefreshTask,
            12u8 => Self::CollatorUnlockBond,
            _ => return Err(()),
        });
    }
}
