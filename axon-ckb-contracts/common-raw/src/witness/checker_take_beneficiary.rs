use crate::{pattern::Pattern, FromRaw, Serialize};

const CHECKER_TAKE_BENEFICIARY_WITNESS_LEN: usize = 19;
#[derive(Debug)]
pub struct CheckerTakeBeneficiaryWitness {
    pattern:        Pattern,
    pub chain_id:   u8,
    pub checker_id: u8,
    pub fee:        u128,
}

impl Default for CheckerTakeBeneficiaryWitness {
    fn default() -> Self {
        Self {
            pattern:    Pattern::CheckerTakeBeneficiary,
            chain_id:   0,
            checker_id: 0,
            fee:        0,
        }
    }
}

impl FromRaw for CheckerTakeBeneficiaryWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerTakeBeneficiaryWitness> {
        if witness_raw_data.len() < 4 {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = u8::from_raw(&witness_raw_data[1..2])?;
        let checker_id = u8::from_raw(&witness_raw_data[2..3])?;
        let fee = u128::from_raw(&witness_raw_data[3..19])?;

        Some(CheckerTakeBeneficiaryWitness {
            pattern,
            chain_id,
            checker_id,
            fee,
        })
    }
}

impl Serialize for CheckerTakeBeneficiaryWitness {
    type RawType = [u8; CHECKER_TAKE_BENEFICIARY_WITNESS_LEN];

    fn serialize(&self) -> Self::RawType {
        let mut buf = [0u8; CHECKER_TAKE_BENEFICIARY_WITNESS_LEN];

        buf[0..1].copy_from_slice(&self.pattern.serialize());
        buf[1..2].copy_from_slice(&self.chain_id.serialize());
        buf[2..3].copy_from_slice(&self.checker_id.serialize());
        buf[3..19].copy_from_slice(&self.fee.serialize());

        buf
    }
}
