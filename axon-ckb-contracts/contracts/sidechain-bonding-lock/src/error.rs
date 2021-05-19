use ckb_std::{debug, error::SysError};
use molecule::error::VerificationError;

/// Error
#[repr(i8)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    InvalidArgument,
    Secp256k1Error,
    MoleculeError,
    ChainIdMismatch,
    BlockHeightNotPassed,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}

impl From<VerificationError> for Error {
    fn from(err: VerificationError) -> Self {
        use VerificationError::*;
        match err {
            TotalSizeNotMatch(msg, a, b) => {
                debug!("TotalSizeNotMatch: {} {} {}", msg, a, b);
            }
            HeaderIsBroken(msg, a, b) => {
                debug!("HeaderIsBroken: {} {} {}", msg, a, b);
            }
            UnknownItem(msg, a, b) => {
                debug!("UnknownItem: {} {} {}", msg, a, b);
            }
            OffsetsNotMatch(msg) => {
                debug!("OffsetsNotMatch: {}", msg);
            }
            FieldCountNotMatch(msg, a, b) => {
                debug!("FieldCountNotMatch: {} {} {}", msg, a, b);
            }
        };
        Error::MoleculeError
    }
}
