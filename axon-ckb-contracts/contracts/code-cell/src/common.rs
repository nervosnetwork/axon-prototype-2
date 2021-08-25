use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell_capacity, load_header};

use common_raw::cell::global_config::GlobalConfigCellData;
use common_raw::FromRaw;

use crate::{cell::CellOrigin, error::Error};

pub const CODE_INPUT: CellOrigin = CellOrigin(0, Source::Input);
pub const CODE_OUTPUT: CellOrigin = CellOrigin(0, Source::Output);

pub const CKB_HASH_PERSONALIZATION: &[u8] = b"ckb-default-hash";

pub fn is_cell_count_greater(n: usize, source: Source) -> bool {
    load_cell_capacity(n, source).is_ok()
}

pub fn is_cell_count_smaller(n: usize, source: Source) -> bool {
    load_cell_capacity(n - 1, source).is_err()
}

pub fn is_cell_count_not_equals(n: usize, source: Source) -> bool {
    is_cell_count_smaller(n, source) || is_cell_count_greater(n, source)
}

#[macro_export]
macro_rules! check_cells {
    ($global: expr, {$($type: ty: $origin: expr), * $(,)?} $(,)?) => {
        $(<$type>::check($origin, $global)?;)*
    }
}

pub fn check_global_cell() -> Result<GlobalConfigCellData, Error> {
    common::check_global_cell().ok_or(Error::GlobalConfigMissed)
}

pub fn require_header_dep() -> Result<u64, Error> {
    let header = load_header(0, Source::HeaderDep).map_err(|_| Error::MissingHeader)?;
    let raw_header = header.raw();

    u64::from_raw(raw_header.timestamp().as_reader().raw_data()).ok_or(Error::MissingHeader)
}

pub struct Blake2b {
    blake2b: blake2b_ref::Blake2b,
}

impl Default for Blake2b {
    fn default() -> Self {
        Self {
            blake2b: blake2b_ref::Blake2bBuilder::new(32).personal(CKB_HASH_PERSONALIZATION).build(),
        }
    }
}

impl Blake2b {
    pub fn update(&mut self, data: &[u8]) {
        self.blake2b.update(data);
    }

    pub fn finalize(self, dest: &mut [u8]) {
        self.blake2b.finalize(dest);
    }

    pub fn calculate(data: &[u8]) -> [u8; 32] {
        let mut blake2b = Self::default();
        blake2b.update(data);

        let mut result = [0u8; 32];
        blake2b.finalize(&mut result);

        result
    }
}
