use super::Loader;
use ckb_testtool::context::Context;
use ckb_tool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};

pub fn new_cell_output(capacity: u64, script: &Script) -> CellOutput {
    CellOutput::new_builder().capacity(capacity.pack()).lock(script.clone()).build()
}

pub fn load_contract(context: &mut Context, builder: TransactionBuilder, name: &str) -> (TransactionBuilder, OutPoint) {
    let contract_bin: Bytes = Loader::default().load_binary(name);
    let out_point = context.deploy_cell(contract_bin);

    (
        builder.cell_dep(CellDep::new_builder().out_point(out_point.clone()).build()),
        out_point,
    )
}
