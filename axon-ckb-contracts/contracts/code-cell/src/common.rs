use crate::{cell::CellOrigin, error::Error};

use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell_capacity, load_header, QueryIter};

use common_raw::{
    cell::{global_config::GlobalConfigCellData, sidechain_config::SidechainConfigCell},
    FromRaw,
};

use bit_vec::*;
use core::convert::TryInto;

pub const CODE_INPUT: CellOrigin = CellOrigin(0, Source::Input);
pub const CODE_OUTPUT: CellOrigin = CellOrigin(0, Source::Output);

pub const EMPTY_BIT_MAP: [u8; 32] = [0; 32];

pub fn is_cell_count_greater(n: usize, source: Source) -> bool {
    load_cell_capacity(n, source).is_ok()
}

pub fn is_cell_count_smaller(n: usize, source: Source) -> bool {
    load_cell_capacity(n - 1, source).is_err()
}

pub fn is_cell_count_not_equals(n: usize, source: Source) -> bool {
    is_cell_count_smaller(n, source) || is_cell_count_greater(n, source)
}

pub fn has_task_passed_update_interval(config: &SidechainConfigCell, origin: CellOrigin) -> Result<(), Error> {
    let CellOrigin(index, source) = origin;
    let config_number = u64::from_raw(load_header(index, source)?.as_reader().raw().number().raw_data()).unwrap();
    let number_proof = QueryIter::new(load_header, Source::HeaderDep).find(|header| {
        let number = u64::from_raw(header.as_reader().raw().number().raw_data()).unwrap();
        u128::from(number - config_number) >= config.refresh_sidechain_height_interval
    });

    if number_proof.is_none() {
        return Err(Error::SidechainConfigMismatch);
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

pub fn bit_map_count(input: [u8; 32]) -> Option<u8> {
    let input = BitVec::from_bytes(&input);
    input.into_iter().filter(|&x| x).count().try_into().ok()
}

pub fn check_global_cell() -> Result<GlobalConfigCellData, Error> {
    common::check_global_cell().ok_or(Error::GlobalConfigMissed)
}
