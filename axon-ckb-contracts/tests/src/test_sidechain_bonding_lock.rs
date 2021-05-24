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
use schemas::{
    common::basic_types::Uint64,
    sidechain_state_cell::{SSCBuilder, SSC},
};
use std::iter::Extend;

const MAX_CYCLES: u64 = 10_000_000;

fn load_script(
    builder: TransactionBuilder,
    context: &mut Context,
    mut pubkey_hash: Vec<u8>,
    chain_id: Vec<u8>,
    unlock_sidechain_height: Vec<u8>,
) -> (TransactionBuilder, Script) {
    let (builder, out_point) = load_contract(context, builder, "sidechain-bonding-lock");

    pubkey_hash.extend(chain_id);
    pubkey_hash.extend(unlock_sidechain_height);
    let lock_script = context
        .build_script(&out_point, pubkey_hash.to_vec().into())
        .expect("script");

    (builder, lock_script)
}

fn bootstrap(
    builder: TransactionBuilder,
    context: &mut Context,
    pubkey_hash: Vec<u8>,
    chain_id: Vec<u8>,
    unlock_sidechain_height: Vec<u8>,
    sidechain_state: SSC,
) -> TransactionView {
    let builder = with_secp256k1_cell_deps(builder, context);

    let (builder, lock_script) = load_script(
        builder,
        context,
        pubkey_hash,
        chain_id,
        unlock_sidechain_height,
    );

    // Prepare I/O cells
    let input_out_point = context.create_cell(new_cell_output(1000, &lock_script), Bytes::new());
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        new_cell_output(500, &lock_script),
        new_cell_output(500, &lock_script),
    ];

    // Prepare config dependency
    let config_out_point = context.deploy_cell(sidechain_state.as_bytes());
    let config_dep = CellDep::new_builder().out_point(config_out_point).build();

    let outputs_data = vec![Bytes::new(); 2];

    // build transaction
    let tx = builder
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(config_dep)
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

    let tx = bootstrap(
        TransactionBuilder::default(),
        &mut context,
        pubkey_hash,
        vec![0x00],
        vec![0; 8],
        SSCBuilder::default().build(),
    );
    let tx = sign_tx(tx, &wrong_privkey);

    context
        .verify_tx(&tx, MAX_CYCLES)
        .expect_err("pass verification");
}

#[test]
fn test_sign_with_correct_key_but_wrong_id() {
    // generate key pair
    let privkey = Generator::random_privkey();
    let pubkey = privkey.pubkey().expect("pubkey");
    let pubkey_hash = blake160(&pubkey.serialize()).to_vec();

    let mut context = Context::default();
    let tx = bootstrap(
        TransactionBuilder::default(),
        &mut context,
        pubkey_hash,
        vec![0x01],
        vec![0; 8],
        SSCBuilder::default().build(),
    );
    let tx = sign_tx(tx, &privkey);

    context
        .verify_tx(&tx, MAX_CYCLES)
        .expect_err("Unlocked with wrong chain id");
}

#[test]
fn test_sign_with_correct_key_and_id_but_height_not_passed() {
    // generate key pair
    let privkey = Generator::random_privkey();
    let pubkey = privkey.pubkey().expect("pubkey");
    let pubkey_hash = blake160(&pubkey.serialize()).to_vec();

    let mut context = Context::default();
    let tx = bootstrap(
        TransactionBuilder::default(),
        &mut context,
        pubkey_hash,
        vec![0x00],
        vec![0; 8],
        SSCBuilder::default()
            .confirmed_sidechain_height(
                Uint64::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]).unwrap(),
            )
            .build(),
    );
    let tx = sign_tx(tx, &privkey);

    context
        .verify_tx(&tx, MAX_CYCLES)
        .expect_err("Unlocked with wrong chain id");
}

#[test]
fn test_sign_with_correct_key() {
    // generate key pair
    let privkey = Generator::random_privkey();
    let pubkey = privkey.pubkey().expect("pubkey");
    let pubkey_hash = blake160(&pubkey.serialize()).to_vec();

    let mut context = Context::default();
    let tx = bootstrap(
        TransactionBuilder::default(),
        &mut context,
        pubkey_hash,
        vec![0x00],
        vec![0; 8],
        SSCBuilder::default().build(),
    );
    let tx = sign_tx(tx, &privkey);

    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}
