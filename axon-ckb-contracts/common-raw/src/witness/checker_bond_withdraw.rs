use core::convert::TryInto;

use crate::{pattern::Pattern, FromRaw, Serialize};

const CHECKER_BOND_WITHDRAW_WITNESS_LEN: usize = 1;

#[derive(Debug)]
pub struct CheckerBondWithdrawWitness {
    pattern: Pattern,
}

impl Default for CheckerBondWithdrawWitness {
    fn default() -> Self {
        Self {
            pattern: Pattern::CheckerBondWithdraw,
        }
    }
}

impl FromRaw for CheckerBondWithdrawWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerBondWithdrawWitness> {
        if witness_raw_data.len() < CHECKER_BOND_WITHDRAW_WITNESS_LEN {
            return None;
        }

        let pattern = u8::from_raw(&witness_raw_data[0..1])?.try_into().ok()?;

        Some(CheckerBondWithdrawWitness { pattern })
    }
}

impl Serialize for CheckerBondWithdrawWitness {
    type RawType = [u8; CHECKER_BOND_WITHDRAW_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        (self.pattern as u8).serialize()
    }
}
