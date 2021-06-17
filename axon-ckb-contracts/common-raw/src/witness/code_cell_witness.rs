use core::convert::TryInto;

use crate::{pattern::Pattern, FromRaw};

const CODE_TYPE_WITNESS_LEN_MIN: usize = 1;

#[derive(Debug, Copy, Clone)]
pub struct CodeCellTypeWitness {
    pattern: Pattern,
}

impl FromRaw for CodeCellTypeWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CodeCellTypeWitness> {
        if witness_raw_data.len() < CODE_TYPE_WITNESS_LEN_MIN {
            return None;
        }

        let pattern = u8::from_raw(&witness_raw_data[0..1])?.try_into().ok()?;

        Some(CodeCellTypeWitness { pattern })
    }
}

impl CodeCellTypeWitness {
    pub fn pattern(&self) -> Pattern {
        self.pattern
    }
}
