use ckb_std::error::SysError;

#[repr(i8)]
#[derive(Debug)]
pub enum CommonError {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    MissingTypeScript = 5,
    MissingCell,
    CodeHashMismatch,
    HashTypeMismatch,
    UnknownCellType,
    CellNumberMismatch = 10,
    LoadTypeHash,
    GlobalConfigCellDep,
    BitOperator,
}

impl From<SysError> for CommonError {
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
