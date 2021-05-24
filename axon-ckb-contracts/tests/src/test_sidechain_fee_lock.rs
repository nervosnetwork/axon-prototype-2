use crate::common::*;
use ckb_testtool::context::Context;
use ckb_tool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};

use schemas::{
    checker_info_cell::CICBuilder, common::basic_types::Uint128, sidechain_fee_cell::SFCBuilder,
};

const MAX_CYCLES: u64 = 10_000_000;

fn bootstrap(
    context: &mut Context,
    builder: TransactionBuilder,
) -> (TransactionBuilder, OutPoint, Script) {
    let (builder, lock_out_point) = load_contract(context, builder, "sidechain-fee-lock");
    let (builder, always_success_out_point) = load_contract(context, builder, "always-success");

    let always_success_script = context
        .build_script(&always_success_out_point, Bytes::new())
        .expect("script");

    (builder, lock_out_point, always_success_script)
}

#[test]
fn test_io_amount_mismatch() {
    // deploy contract
    let mut context = Context::default();
    let builder = TransactionBuilder::default();

    let (builder, lock_out_point, always_success_script) = bootstrap(&mut context, builder);

    // prepare inputs
    let lock_script = &context
        .build_script(&lock_out_point, vec![0x00].into())
        .expect("script");
    let input_outpoint = context.create_cell(
        new_cell_output(1000, lock_script),
        SFCBuilder::default()
            .amount(Uint128::from_slice(&10u128.to_le_bytes()).unwrap())
            .build()
            .as_bytes(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_outpoint)
        .build();

    let cic_outpoint = context.create_cell(
        new_cell_output(1000, &always_success_script),
        CICBuilder::default()
            .unpaid_income(Uint128::from_slice(&10u128.to_le_bytes()).unwrap())
            .build()
            .as_bytes(),
    );
    let cic = CellInput::new_builder()
        .previous_output(cic_outpoint)
        .build();

    // prepare outputs
    let outputs = vec![
        new_cell_output(1000, &lock_script),
        new_cell_output(1000, &always_success_script),
    ];
    let outputs_data = vec![
        SFCBuilder::default()
            .amount(Uint128::from_slice(&0u128.to_le_bytes()).unwrap())
            .build()
            .as_bytes(),
        CICBuilder::default()
            .unpaid_income(Uint128::from_slice(&10u128.to_le_bytes()).unwrap())
            .build()
            .as_bytes(),
    ];

    // build transaction
    let tx = builder
        .input(input)
        .input(cic)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();
    let tx = context.complete_tx(tx);

    // run
    context
        .verify_tx(&tx, MAX_CYCLES)
        .expect_err("pass verification");
}

#[test]
fn test_io_amount_match_but_different_chain() {
    // deploy contract
    let mut context = Context::default();
    let builder = TransactionBuilder::default();

    let (builder, lock_out_point, always_success_script) = bootstrap(&mut context, builder);

    // prepare inputs
    let lock_script = &context
        .build_script(&lock_out_point, vec![0x10].into())
        .expect("script");
    let input_outpoint = context.create_cell(
        new_cell_output(1000, lock_script),
        SFCBuilder::default()
            .amount(Uint128::from_slice(&10u128.to_le_bytes()).unwrap())
            .build()
            .as_bytes(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_outpoint)
        .build();

    let cic_outpoint = context.create_cell(
        new_cell_output(1000, &always_success_script),
        CICBuilder::default()
            .unpaid_income(Uint128::from_slice(&10u128.to_le_bytes()).unwrap())
            .build()
            .as_bytes(),
    );
    let cic = CellInput::new_builder()
        .previous_output(cic_outpoint)
        .build();

    // prepare outputs
    let outputs = vec![
        new_cell_output(1000, &lock_script),
        new_cell_output(1000, &always_success_script),
    ];
    let outputs_data = vec![
        SFCBuilder::default()
            .amount(Uint128::from_slice(&0u128.to_le_bytes()).unwrap())
            .build()
            .as_bytes(),
        CICBuilder::default()
            .unpaid_income(Uint128::from_slice(&0u128.to_le_bytes()).unwrap())
            .build()
            .as_bytes(),
    ];

    // build transaction
    let tx = builder
        .input(input)
        .input(cic)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();
    let tx = context.complete_tx(tx);

    // run
    context
        .verify_tx(&tx, MAX_CYCLES)
        .expect_err("pass verification");
}

#[test]
fn test_success() {
    // deploy contract
    let mut context = Context::default();
    let builder = TransactionBuilder::default();

    let (builder, lock_out_point, always_success_script) = bootstrap(&mut context, builder);

    // prepare inputs
    let input_script = &context
        .build_script(&lock_out_point, vec![0x00].into())
        .expect("script");
    let input_outpoint = context.create_cell(
        new_cell_output(1000, input_script),
        SFCBuilder::default()
            .amount(Uint128::from_slice(&10u128.to_le_bytes()).unwrap())
            .build()
            .as_bytes(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_outpoint)
        .build();

    let cic_outpoint = context.create_cell(
        new_cell_output(1000, &always_success_script),
        CICBuilder::default()
            .unpaid_income(Uint128::from_slice(&10u128.to_le_bytes()).unwrap())
            .build()
            .as_bytes(),
    );
    let cic = CellInput::new_builder()
        .previous_output(cic_outpoint)
        .build();

    // prepare outputs
    let outputs = vec![
        new_cell_output(1000, &always_success_script),
        new_cell_output(1000, &always_success_script),
    ];
    let outputs_data = vec![
        SFCBuilder::default()
            .amount(Uint128::from_slice(&0u128.to_le_bytes()).unwrap())
            .build()
            .as_bytes(),
        CICBuilder::default()
            .unpaid_income(Uint128::from_slice(&0u128.to_le_bytes()).unwrap())
            .build()
            .as_bytes(),
    ];

    // build transaction
    let tx = builder
        .input(input)
        .input(cic)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}
