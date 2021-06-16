use crate::{check_args_len, FromRaw};

const SUDT_DATA_LEN: usize = 16; // u128

// which is standard sudt
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct MuseTokenData {
    pub amount: u128,
}

impl FromRaw for MuseTokenData {
    fn from_raw(cell_raw_data: &[u8]) -> Option<MuseTokenData> {
        check_args_len(cell_raw_data.len(), SUDT_DATA_LEN)?;

        let sudt_amount = u128::from_raw(&cell_raw_data[..16])?;

        Some(MuseTokenData { amount: sudt_amount })
    }
}
