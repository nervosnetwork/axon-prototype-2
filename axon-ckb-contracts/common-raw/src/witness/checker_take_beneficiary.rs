use crate::{common::ChainId, pattern::Pattern, FromRaw, Serialize};

const CHECKER_TAKE_BENEFICIARY_WITNESS_LEN: usize = 22;
#[derive(Debug)]
pub struct CheckerTakeBeneficiaryWitness {
    pattern:        Pattern,
    pub chain_id:   ChainId,
    pub checker_id: u8,
    pub fee:        u128,
}

impl Default for CheckerTakeBeneficiaryWitness {
    fn default() -> Self {
        Self {
            pattern:    Pattern::CheckerTakeBeneficiary,
            chain_id:   ChainId::default(),
            checker_id: 0,
            fee:        0,
        }
    }
}

impl FromRaw for CheckerTakeBeneficiaryWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerTakeBeneficiaryWitness> {
        if witness_raw_data.len() != CHECKER_TAKE_BENEFICIARY_WITNESS_LEN {
            return None;
        }

        let pattern = Pattern::from_raw(&witness_raw_data[0..1])?;
        let chain_id = ChainId::from_raw(&witness_raw_data[1..5])?;
        let checker_id = u8::from_raw(&witness_raw_data[5..6])?;
        let fee = u128::from_raw(&witness_raw_data[6..22])?;

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
        buf[1..5].copy_from_slice(&self.chain_id.serialize());
        buf[5..6].copy_from_slice(&self.checker_id.serialize());
        buf[6..22].copy_from_slice(&self.fee.serialize());

        buf
    }
}
