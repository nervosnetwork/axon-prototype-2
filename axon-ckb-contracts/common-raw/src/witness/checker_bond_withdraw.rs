use crate::{decode_u8, FromRaw};

#[derive(Debug)]
pub struct CheckerBondWithdrawWitness {
    pub pattern: u8,
}

impl FromRaw for CheckerBondWithdrawWitness {
    fn from_raw(witness_raw_data: &[u8]) -> Option<CheckerBondWithdrawWitness> {
        if witness_raw_data.len() < 2 {
            return None;
        }

        let pattern = decode_u8(&witness_raw_data[0..1])?;

        Some(CheckerBondWithdrawWitness { pattern })
    }
}
