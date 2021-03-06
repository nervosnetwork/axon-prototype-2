use ckb_testtool::context::Context;
use ckb_tool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};

use crate::common::*;

use super::Loader;

const MAX_CYCLES: u64 = 10_000_000;

#[test]
fn test_success() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("always-success");
    let out_point = context.deploy_cell(contract_bin);

    // prepare scripts
    let lock_script = context.build_script(&out_point, Bytes::from(vec![])).expect("script");
    let lock_script_dep = CellDep::new_builder().out_point(out_point).build();

    // prepare cells
    let input_outpoint = context.create_cell(new_cell_output(1000, &lock_script), Bytes::new());
    let input = CellInput::new_builder().previous_output(input_outpoint).build();

    // build transaction
    let tx = TransactionBuilder::default().input(input).cell_dep(lock_script_dep).build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
    println!("consume cycles: {}", cycles);
}
