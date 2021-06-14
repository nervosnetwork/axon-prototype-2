use crate::common::*;
use crate::secp256k1::*;
use ckb_testtool::context::Context;
use ckb_tool::ckb_crypto::secp::Generator;
use ckb_tool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed::*, prelude::*};

use common_raw::{
    cell::{
        checker_bond::{CheckerBondCellData, CheckerBondCellLockArgs},
        checker_info::CheckerInfoCellData,
        global_config::GlobalConfigCellData,
        sidechain_config::SidechainConfigCellData,
    },
    witness::checker_join_sidechain::CheckerJoinSidechainWitness,
};

const MAX_CYCLES: u64 = 10_000_000;

fn bootstrap(builder: TransactionBuilder, context: &mut Context, lock_args: &[u8]) -> (TransactionBuilder, Script, Script) {
    let (builder, secp256k1_code) = with_secp256k1_cell_deps(builder, context);
    let secp256k1_script = context.build_script(&secp256k1_code, lock_args.to_vec().into()).expect("script");

    let (builder, code_cell_script) = load_script(context, builder, "code-cell");

    let code_cell_input = create_input(
        context,
        new_type_cell_output(1000, &secp256k1_script, &code_cell_script),
        Bytes::new(),
    );

    (builder.input(code_cell_input), code_cell_script, secp256k1_script)
}

fn with_time_header(
    builder: TransactionBuilder,
    context: &mut Context,
    timestamp: u64,
) -> (TransactionBuilder, ckb_tool::ckb_types::core::HeaderView) {
    let header = ckb_tool::ckb_types::core::HeaderBuilder::default()
        .timestamp(timestamp.pack())
        .build();
    context.insert_header(header.clone());

    (builder.header_dep(header.hash()), header)
}

#[test]
fn test_success() {
    // generate key pair
    let privkey = Generator::random_privkey();
    let pubkey = privkey.pubkey().expect("pubkey");
    let pubkey_hash = blake160(&pubkey.serialize());

    // deploy contract
    let mut context = Context::default();

    let (builder, code_cell_script, secp256k1_script) = bootstrap(TransactionBuilder::default(), &mut context, &pubkey_hash);

    let (builder, always_success_code) = load_contract(&mut context, builder, "always-success");
    let always_success = context.build_script(&always_success_code, Bytes::new()).expect("script");
    let a_s_codehash = always_success.as_reader().code_hash().raw_data();

    // prepare cell_deps
    let mut global_config = GlobalConfigCellData::default();

    global_config
        .code_cell_type_codehash
        .copy_from_slice(code_cell_script.as_reader().code_hash().raw_data());
    global_config.checker_bond_cell_lock_codehash.copy_from_slice(a_s_codehash);
    global_config.checker_info_cell_type_codehash.copy_from_slice(a_s_codehash);
    global_config.sidechain_config_cell_type_codehash.copy_from_slice(a_s_codehash);

    let global_config_dep = create_dep(
        &mut context,
        new_type_cell_output(1000, &always_success, &always_success),
        global_config.serialize(),
    );

    let builder = builder.cell_dep(global_config_dep);

    // prepare headers
    let (builder, config_header) = with_time_header(builder, &mut context, 1000);
    let (builder, _) = with_time_header(builder, &mut context, 1100);

    // prepare scripts
    let mut checker_bond_lock_args = CheckerBondCellLockArgs::default();
    let checker_bond_lock_input_script = context
        .build_script(&always_success_code, checker_bond_lock_args.serialize())
        .expect("script");

    checker_bond_lock_args.chain_id_bitmap = [
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let checker_bond_lock_output_script = context
        .build_script(&always_success_code, checker_bond_lock_args.serialize())
        .expect("script");

    // prepare inputs
    let mut config_input_data = SidechainConfigCellData::default();
    config_input_data.minimal_bond = 100;
    config_input_data.update_interval = 100;

    let config_input_out_point = context.create_cell(
        new_type_cell_output(1000, &always_success, &always_success),
        config_input_data.serialize(),
    );
    let config_input = CellInput::new_builder().previous_output(config_input_out_point.clone()).build();

    let mut checker_bond_input_data = CheckerBondCellData::default();
    checker_bond_input_data.amount = 100;

    let checker_bond_input = create_input(
        &mut context,
        new_type_cell_output(1000, &checker_bond_lock_input_script, &always_success),
        checker_bond_input_data.serialize(),
    );

    context.link_cell_with_block(config_input_out_point.clone(), config_header.hash(), 0);
    let builder = builder.input(config_input).input(checker_bond_input);

    // prepare outputs
    let mut config_output = config_input_data.clone();
    config_output.checker_total_count = 1;
    config_output.checker_bitmap = [
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    let checker_bond_output = checker_bond_input_data.clone();
    let mut checker_info_output = CheckerInfoCellData::default();
    checker_info_output.checker_public_key_hash.copy_from_slice(&pubkey_hash);

    let outputs = vec![
        new_type_cell_output(1000, &secp256k1_script, &code_cell_script),
        new_type_cell_output(1000, &always_success, &always_success),
        new_type_cell_output(1000, &checker_bond_lock_output_script, &always_success),
        new_type_cell_output(1000, &always_success, &always_success),
    ];
    let outputs_data: Vec<Bytes> = vec![
        Bytes::new(),
        config_output.serialize(),
        checker_bond_output.serialize(),
        checker_info_output.serialize(),
    ];

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
