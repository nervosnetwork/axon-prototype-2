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

#[test]
fn test_io_amount_mismatch() {
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
    let config_input_data = SidechainConfigCellData::default();
    let config_input = create_input(
        &mut context,
        new_type_cell_output(1000, &always_success, &always_success),
        config_input_data.serialize(),
    );

    let checker_bond_input_data = CheckerBondCellData::default();
    let checker_bond_input = create_input(
        &mut context,
        new_type_cell_output(1000, &checker_bond_lock_input_script, &always_success),
        checker_bond_input_data.serialize(),
    );

    let builder = builder.input(config_input).input(checker_bond_input);

    // prepare outputs
    let mut config_output = SidechainConfigCellData::default();
    config_output.checker_total_count = 1;
    config_output.checker_bitmap = [
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    let checker_bond_output = CheckerBondCellData::default();
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
