use core::convert::TryFrom;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum Pattern {
    CreateCodeCell = 0u8,

    AdminCreateSidechain = 1u8,

    CheckerBondDeposit = 2u8,
    CheckerBondWithdraw,
    CheckerJoinSidechain,
    CheckerQuitSidechain,
    CheckerSubmitTask,
    CheckerPublishChallenge,
    CheckerSubmitChallenge,
    CheckerTakeBeneficiary,

    CollatorPublishTask = 10u8,
    CollatorSubmitTask,
    CollatorSubmitChallenge,
    CollatorRefreshTask,
    CollatorUnlockBond,
}

impl TryFrom<u8> for Pattern {
    type Error = ();

    fn try_from(input: u8) -> Result<Self, Self::Error> {
        return Ok(match input {
            0u8 => Self::CreateCodeCell,
            1u8 => Self::AdminCreateSidechain,

            2u8 => Self::CheckerBondDeposit,
            3u8 => Self::CheckerBondWithdraw,
            4u8 => Self::CheckerJoinSidechain,
            5u8 => Self::CheckerQuitSidechain,
            6u8 => Self::CheckerSubmitTask,
            7u8 => Self::CheckerPublishChallenge,
            8u8 => Self::CheckerSubmitChallenge,
            9u8 => Self::CheckerTakeBeneficiary,

            10u8 => Self::CollatorPublishTask,
            11u8 => Self::CollatorSubmitTask,
            12u8 => Self::CollatorSubmitChallenge,
            13u8 => Self::CollatorRefreshTask,
            14u8 => Self::CollatorUnlockBond,
            _ => return Err(()),
        });
    }
}
