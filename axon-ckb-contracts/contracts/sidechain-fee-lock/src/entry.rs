use alloc::vec::Vec;
use core::result::Result;

use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    high_level::{load_cell_data, load_script, QueryIter},
};

use crate::error::Error;

use schemas::{
    checker_info_cell::{CICReader, CIC},
    sidechain_fee_cell::{SFCReader, SFC},
};

const UDT_LEN: usize = 16;

pub fn main() -> Result<(), Error> {
    // TODO: Skip checking if SSC exist (Confirming checking task / challenge task)

    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    // Chain id: 1 Byte
    if args.len() != 1 {
        return Err(Error::InvalidArgument);
    }

    let chain_id = Vec::<u8>::from(args);

    let input_amount = get_amount(QueryIter::new(load_cell_data, Source::GroupInput));
    let output_amount = get_amount(QueryIter::new(load_cell_data, Source::GroupOutput));

    let input_unpaid_income =
        get_unpaid_income(&chain_id, QueryIter::new(load_cell_data, Source::Input));
    let output_unpaid_income =
        get_unpaid_income(&chain_id, QueryIter::new(load_cell_data, Source::Output));

    if output_amount >= input_amount
        || output_unpaid_income >= input_unpaid_income
        || input_amount - output_amount != input_unpaid_income - output_unpaid_income
    {
        return Err(Error::NotBalancedAmount);
    }

    Ok(())
}

fn get_amount<T: Iterator<Item = Vec<u8>>>(iter: T) -> u128 {
    let mut buf = [0u8; UDT_LEN];
    iter.map(|data| {
        SFCReader::verify(data.as_slice(), false).expect("sfc encoding");
        let sidechain_fee = SFC::new_unchecked(data.into());

        buf.copy_from_slice(sidechain_fee.amount().as_slice());
        u128::from_le_bytes(buf)
    })
    .sum::<u128>()
}

fn get_unpaid_income<T: Iterator<Item = Vec<u8>>>(chain_id: &[u8], iter: T) -> u128 {
    let mut buf = [0u8; UDT_LEN];
    iter.map(|data| {
        // TODO: Find CIC by type script
        if CICReader::verify(data.as_slice(), false).is_err() {
            return 0;
        }
        let checker_info = CIC::new_unchecked(data.into());

        if chain_id != checker_info.chain_id().as_slice() {
            return 0;
        }

        buf.copy_from_slice(checker_info.unpaid_income().as_slice());
        u128::from_le_bytes(buf)
    })
    .sum::<u128>()
}
