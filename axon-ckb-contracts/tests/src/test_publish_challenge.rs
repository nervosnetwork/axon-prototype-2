use crate::common::*;
use crate::environment_builder::{AxonScripts, EnvironmentBuilder};
use crate::secp256k1::*;
use ckb_tool::{
    bytes::Bytes,
    ckb_crypto::secp::Generator,
    ckb_types::{packed::CellDep, prelude::*},
};
use common_raw::{
    cell::{
        checker_info::{CheckerInfoCellData, CheckerInfoCellMode, CheckerInfoCellTypeArgs},
        sidechain_config::{SidechainConfigCellData, SidechainConfigCellTypeArgs},
        task::{TaskCell, TaskCellTypeArgs, TaskMode},
    },
    witness::checker_publish_challenge::CheckerPublishChallengeWitness,
};

const MAX_CYCLES: u64 = 10_000_000;

#[test]
fn test_success() {
    // generate key pair
    let privkey = Generator::random_privkey();
    let pubkey = privkey.pubkey().expect("pubkey");
    let pubkey_hash = blake160(&pubkey.serialize());

    // deploy contract
    let (
        mut builder,
        AxonScripts {
            always_success_code,
            always_success_script: always_success,
            code_cell_script,
            ..
        },
    ) = EnvironmentBuilder::default().bootstrap(pubkey_hash.to_vec());

    // prepare scripts
    let config_type_args = SidechainConfigCellTypeArgs::default();
    let config_script = builder
        .context
        .build_script(&always_success_code, config_type_args.serialize())
        .expect("script");

    let task_type_args = TaskCellTypeArgs::default();
    let task_script = builder
        .context
        .build_script(&always_success_code, task_type_args.serialize())
        .expect("script");

    let mut checker_info_type_args = CheckerInfoCellTypeArgs::default();
    checker_info_type_args.checker_lock_arg.copy_from_slice(&pubkey_hash);

    let checker_info_script = builder
        .context
        .build_script(&always_success_code, checker_info_type_args.serialize())
        .expect("script");

    //prepare dep
    let mut config_dep_data = SidechainConfigCellData::default();
    config_dep_data.challenge_threshold = 2;
    config_dep_data.collator_lock_arg.copy_from_slice(&pubkey_hash);

    let config_dep_out_point = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &config_script),
        config_dep_data.serialize(),
    );
    let config_dep = CellDep::new_builder().out_point(config_dep_out_point).build();
    let mut builder = builder.cell_dep(config_dep);

    // prepare inputs
    let checker_info_input_data = CheckerInfoCellData::default();
    let checker_info_input = builder.create_input(
        new_type_cell_output(1000, &always_success, &checker_info_script),
        checker_info_input_data.serialize(),
    );

    let task_input_data = TaskCell::default();
    let task_input = builder.create_input(
        new_type_cell_output(1000, &always_success, &task_script),
        task_input_data.serialize(),
    );

    let builder = builder.input(checker_info_input).input(task_input);

    //prepare output
    let mut checker_info_output = checker_info_input_data.clone();
    checker_info_output.mode = CheckerInfoCellMode::ChallengePassed;

    let mut task_output_data = TaskCell::default();
    task_output_data.mode = TaskMode::Challenge;

    let outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &checker_info_script),
        new_type_cell_output(1000, &always_success, &task_script),
    ];
    let outputs_data = vec![Bytes::new(), checker_info_output.serialize(), task_output_data.serialize()];

    let mut witness = CheckerPublishChallengeWitness::default();
    witness.challenge_count = 2;
    witness.sidechain_config_dep_index = EnvironmentBuilder::BOOTSTRAP_CELL_DEPS_LENGTH;

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
