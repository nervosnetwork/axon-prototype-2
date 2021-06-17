use crate::{cell::CellOrigin, error::Error};

use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell, load_header, QueryIter};

use common_raw::{
    cell::{global_config::GlobalConfigCellData, sidechain_config::SidechainConfigCellData},
    FromRaw,
};

use bit_vec::*;

pub const CODE_INPUT: CellOrigin = CellOrigin(0, Source::Input);
pub const CODE_OUTPUT: CellOrigin = CellOrigin(0, Source::Output);

pub const EMPTY_BIT_MAP: [u8; 32] = [0; 32];

pub fn get_input_cell_count() -> usize {
    QueryIter::new(load_cell, Source::Input).count()
}

pub fn get_output_cell_count() -> usize {
    QueryIter::new(load_cell, Source::Output).count()
}

pub fn has_sidechain_config_passed_update_interval(config: SidechainConfigCellData, origin: CellOrigin) -> Result<(), Error> {
    if config.checker_total_count >= config.checker_threshold {
        let CellOrigin(index, source) = origin;
        let config_timestamp = u64::from_raw(load_header(index, source)?.as_reader().raw().timestamp().raw_data()).unwrap();

        let time_proof = QueryIter::new(load_header, Source::HeaderDep).find(|header| {
            let timestamp = u64::from_raw(header.as_reader().raw().timestamp().raw_data()).unwrap();
            timestamp - config_timestamp >= config.update_interval.into()
        });

        if time_proof.is_none() {
            return Err(Error::SidechainConfigMismatch);
        }
    }

    Ok(())
}

#[macro_export]
macro_rules! check_cells {
    ($global: expr, {$($type: ty: $origin: expr), * $(,)?} $(,)?) => {
        $(<$type>::check($origin, $global)?;)*
    }
}

pub fn bit_map_add(input: &[u8; 32], checker_id: u8) -> Option<[u8; 32]> {
    let mut input = BitVec::from_bytes(&input[..]);

    //should be false
    if input.get(checker_id as usize)? {
        return None;
    }

    input.set(checker_id as usize, true);

    let mut ret = [0u8; 32];
    ret.copy_from_slice(input.to_bytes().as_slice());

    Some(ret)
}

pub fn bit_map_remove(input: [u8; 32], checker_id: u8) -> Option<[u8; 32]> {
    let mut input = BitVec::from_bytes(&input[..]);

    //should be true
    if !input.get(checker_id as usize)? {
        return None;
    }

    input.set(checker_id as usize, false);

    let mut ret = [0u8; 32];
    ret.copy_from_slice(&input.to_bytes().as_slice()[0..32]);

    Some(ret)
}

//check if given number is bit-marked in input array
pub fn bit_map_marked(input: [u8; 32], checker_id: u8) -> Option<bool> {
    let input = BitVec::from_bytes(&input[..]);

    Some(input.get(checker_id as usize)?)
}

pub fn check_global_cell() -> Result<GlobalConfigCellData, Error> {
    common::check_global_cell().ok_or(Error::GlobalConfigMissed)
}
