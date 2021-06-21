use ckb_std::error::SysError;

/// Error
#[repr(i8)]
#[derive(Debug)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,

    MissingTypeScript,
    CodeHashMismatch,
    HashTypeMismatch,
    CellNumberMismatch,

    GlobalConfigMissed,
    // PatternWitnessTypeMissing,
    // Secp256k1Error,
    // BusyChecker,
    // MissingSignature,
    MissingWitness,
    // PatternCollision,
    // PatternInvalid,
    // ChainIdBitMapMismatch,
    // CheckerInfoMode,
    // CheckerUnpaidFee,
    #[allow(dead_code)]
    TypeScriptMissed,
    Wrong,
    SidechainConfigMismatch,
    SidechainFeeMismatch,
    CheckerBondMismatch,
    CheckerInfoMismatch,
    MuseTokenMismatch,
    TaskMismatch,
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
