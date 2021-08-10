use crate::common::*;
use crate::environment_builder::{AxonScripts, EnvironmentBuilder};
use crate::secp256k1::*;
use ckb_tool::ckb_crypto::secp::Generator;
use ckb_tool::ckb_types::{bytes::Bytes, core, packed::*, prelude::*};

use common_raw::{
    cell::{
        checker_bond::{CheckerBondCell, CheckerBondCellLockArgs},
        checker_info::{CheckerInfoCell, CheckerInfoCellTypeArgs},
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs},
    },
    witness::checker_quit_sidechain::CheckerQuitSidechainWitness,
};

const MAX_CYCLES: u64 = 10_000_000;

fn with_time_header(mut builder: EnvironmentBuilder, timestamp: u64) -> (EnvironmentBuilder, core::HeaderView) {
    let header = core::HeaderBuilder::default().timestamp(timestamp.pack()).build();
    builder.context.insert_header(header.clone());

    let builder = builder.header_dep(header.hash());

    (builder, header)
}

#[test]
fn test_success() {
    // generate key pair
    let privkey = Generator::random_privkey();
    let pubkey = privkey.pubkey().expect("pubkey");
    let pubkey_hash = blake160(&pubkey.serialize());

    // deploy contract
    let (
        builder,
        AxonScripts {
            always_success_code,
            always_success_script: always_success,
            code_cell_script,
            ..
        },
    ) = EnvironmentBuilder::default().bootstrap(pubkey_hash.to_vec());

    // prepare headers
    let (builder, config_header) = with_time_header(builder, 1000);
    let (mut builder, _) = with_time_header(builder, 1100);

    // prepare scripts
    let mut checker_bond_lock_args = CheckerBondCellLockArgs::default();
    checker_bond_lock_args.checker_lock_arg.copy_from_slice(&pubkey_hash);

    let checker_bond_lock_output_script = builder
        .context
        .build_script(&always_success_code, checker_bond_lock_args.serialize())
        .expect("script");

    let checker_bond_lock_input_script = builder
        .context
        .build_script(&always_success_code, checker_bond_lock_args.serialize())
        .expect("script");

    let mut checker_info_type_args = CheckerInfoCellTypeArgs::default();
    checker_info_type_args.checker_lock_arg.copy_from_slice(&pubkey_hash);

    let checker_info_script = builder
        .context
        .build_script(&always_success_code, checker_info_type_args.serialize())
        .expect("script");

    let config_type_args = SidechainConfigCellTypeArgs::default();
    let config_script = builder
        .context
        .build_script(&always_success_code, config_type_args.serialize())
        .expect("script");

    // prepare inputs
    let mut config_input_data = SidechainConfigCell::default();
    config_input_data.checker_total_count = 1;
    config_input_data.minimal_bond = 100;

    let config_input_out_point = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &config_script),
        config_input_data.serialize(),
    );
    let config_input = CellInput::new_builder().previous_output(config_input_out_point.clone()).build();

    let mut checker_bond_input_data = CheckerBondCell::default();
    checker_bond_input_data.amount = 100;

    let checker_bond_input = builder.create_input(
        new_type_cell_output(1000, &checker_bond_lock_input_script, &always_success),
        checker_bond_input_data.serialize(),
    );

    let checker_info_input_data = CheckerInfoCell::default();
    let checker_info_input = builder.create_input(
        new_type_cell_output(1000, &always_success, &checker_info_script),
        checker_info_input_data.serialize(),
    );

    builder
        .context
        .link_cell_with_block(config_input_out_point.clone(), config_header.hash(), 0);
    let builder = builder.input(config_input).input(checker_bond_input).input(checker_info_input);

    // prepare outputs
    let mut config_output = config_input_data.clone();
    config_output.checker_total_count = 0;

    let checker_bond_output = checker_bond_input_data.clone();

    let outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &always_success),
        new_type_cell_output(1000, &checker_bond_lock_output_script, &always_success),
    ];
    let outputs_data: Vec<Bytes> = vec![Bytes::new(), config_output.serialize(), checker_bond_output.serialize()];

    let witness = CheckerQuitSidechainWitness::default();
    let witnesses = [get_dummy_witness_builder().input_type(witness.serialize().pack_some()).as_bytes()];

    // build transaction
    let builder = builder.outputs(outputs).outputs_data(outputs_data.pack());
    let tx = builder.builder.build();
    let tx = tx
        .as_advanced_builder()
        .set_witnesses(sign_tx_with_witnesses(tx, witnesses.pack(), &privkey).unwrap())
        .build();

    // run
    builder.context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
}
