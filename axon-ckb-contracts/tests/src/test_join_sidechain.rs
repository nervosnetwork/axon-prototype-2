use crate::common::*;
use crate::secp256k1::*;
use ckb_testtool::context::Context;
use ckb_tool::ckb_crypto::secp::Generator;
use ckb_tool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};

use common_raw::witness::checker_join_sidechain::CheckerJoinSidechainWitness;

const MAX_CYCLES: u64 = 10_000_000;

fn bootstrap(builder: TransactionBuilder, context: &mut Context, lock_args: Vec<u8>) -> (TransactionBuilder, ScriptOpt, Script) {
    let (builder, secp256k1_code) = with_secp256k1_cell_deps(builder, context);
    let secp256k1_script = context.build_script(&secp256k1_code, lock_args.into()).expect("script");

    let (builder, code_cell_code) = load_contract(context, builder, "code-cell");
    let code_cell_script = context.build_script(&code_cell_code, Bytes::new()).expect("script");
    let code_cell_script = code_cell_script.pack_some();

    let code_cell = CellOutput::new_builder()
        .capacity(1000.pack())
        .lock(secp256k1_script.clone())
        .type_(code_cell_script.clone())
        .build();

    let code_cell_outpoint = context.create_cell(code_cell, Bytes::new());
    let code_cell_input = CellInput::new_builder().previous_output(code_cell_outpoint).build();

    (builder.input(code_cell_input), code_cell_script, secp256k1_script)
}

#[test]
fn test_io_amount_mismatch() {
    // generate key pair
    let privkey = Generator::random_privkey();
    let pubkey = privkey.pubkey().expect("pubkey");
    let pubkey_hash = blake160(&pubkey.serialize()).to_vec();

    // deploy contract
    let mut context = Context::default();

    let (builder, code_cell_script, secp256k1_script) = bootstrap(TransactionBuilder::default(), &mut context, pubkey_hash);

    // prepare outputs
    let outputs = vec![CellOutput::new_builder()
        .capacity(1000.pack())
        .lock(secp256k1_script.clone())
        .type_(code_cell_script.clone())
        .build()];
    let outputs_data: Vec<Bytes> = vec![Bytes::new()];

    let mut witness = CheckerJoinSidechainWitness::default();
    witness.pattern = 5u8;

    let witnesses = [get_dummy_witness_builder().input_type(witness.serialize().pack_some()).as_bytes()];

    // build transaction
    let tx = builder.outputs(outputs).outputs_data(outputs_data.pack()).build();
    let tx = tx
        .as_advanced_builder()
        .set_witnesses(sign_tx_with_witnesses(tx, witnesses.pack(), &privkey).unwrap())
        .build();

    // run
    context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
}
