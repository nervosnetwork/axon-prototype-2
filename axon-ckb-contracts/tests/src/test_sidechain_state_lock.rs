use crate::common::*;
use crate::secp256k1::*;
use ckb_testtool::context::Context;
use ckb_tool::ckb_crypto::secp::Generator;
use ckb_tool::ckb_types::packed::Script;
use ckb_tool::ckb_types::{
    bytes::Bytes,
    core::{TransactionBuilder, TransactionView},
    packed::*,
    prelude::*,
};

const MAX_CYCLES: u64 = 10_000_000;

fn load_script(
    builder: TransactionBuilder,
    context: &mut Context,
    pubkey_hash: Vec<u8>,
) -> (TransactionBuilder, Script) {
    let (builder, out_point) = load_contract(context, builder, "sidechain-state-lock");

    let lock_script = context
        .build_script(&out_point, pubkey_hash.into())
        .expect("script");

    (builder, lock_script)
}

fn bootstrap(
    builder: TransactionBuilder,
    context: &mut Context,
    pubkey_hash: Vec<u8>,
) -> TransactionView {
    let builder = with_secp256k1_cell_deps(builder, context);

    let (builder, lock_script) = load_script(builder, context, pubkey_hash);

    // prepare cells
    let input_out_point = context.create_cell(new_cell_output(1000, &lock_script), Bytes::new());
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        new_cell_output(500, &lock_script),
        new_cell_output(500, &lock_script),
    ];

    let outputs_data = vec![Bytes::new(); 2];

    // build transaction
    let tx = builder
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();

    context.complete_tx(tx)
}

#[test]
fn test_sign_with_wrong_key() {
    // generate key pair
    let privkey = Generator::random_privkey();
    let pubkey = privkey.pubkey().expect("pubkey");
    let pubkey_hash = blake160(&pubkey.serialize()).to_vec();
    let wrong_privkey = Generator::random_privkey();

    let mut context = Context::default();

    let tx = bootstrap(TransactionBuilder::default(), &mut context, pubkey_hash);
    let tx = sign_tx(tx, &wrong_privkey);

    context
        .verify_tx(&tx, MAX_CYCLES)
        .expect_err("pass verification");
}

#[test]
fn test_sign_with_correct_key() {
    // generate key pair
    let privkey = Generator::random_privkey();
    let pubkey = privkey.pubkey().expect("pubkey");
    let pubkey_hash = blake160(&pubkey.serialize()).to_vec();

    let mut context = Context::default();
    let tx = bootstrap(TransactionBuilder::default(), &mut context, pubkey_hash);
    let tx = sign_tx(tx, &privkey);

    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}
