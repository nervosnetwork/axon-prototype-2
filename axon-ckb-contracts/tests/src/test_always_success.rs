use super::*;
use ckb_testtool::context::Context;
use ckb_tool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};

const MAX_CYCLES: u64 = 10_000_000;

#[test]
fn test_success() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("always-success");
    let out_point = context.deploy_cell(contract_bin);

    // prepare scripts
    context
        .build_script(&out_point, Bytes::from(vec![]))
        .expect("script");
    let lock_script_dep = CellDep::new_builder().out_point(out_point).build();

    // build transaction
    let tx = TransactionBuilder::default()
        .cell_dep(lock_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}
