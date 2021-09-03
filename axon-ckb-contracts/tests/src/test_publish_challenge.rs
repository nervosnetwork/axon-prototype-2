use ckb_tool::{
    bytes::Bytes,
    ckb_crypto::secp::Generator,
    ckb_types::{packed::CellDep, prelude::*},
};

use common_raw::{
    cell::{
        checker_info::{CheckerInfoCell, CheckerInfoCellTypeArgs},
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs},
        sidechain_state::{CheckerLastAcceptTaskHeight, SidechainStateCell, SidechainStateCellTypeArgs},
        task::{TaskCell, TaskCellTypeArgs, TaskMode, TaskStatus},
    },
    witness::checker_publish_challenge::CheckerPublishChallengeWitness,
};

use crate::common::*;
use crate::environment_builder::{AxonScripts, EnvironmentBuilder};
use crate::secp256k1::*;

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

    let state_type_args = SidechainStateCellTypeArgs::default();
    let state_script = builder
        .context
        .build_script(&always_success_code, state_type_args.serialize())
        .expect("script");

    let mut checker_info_type_args = CheckerInfoCellTypeArgs::default();
    checker_info_type_args.checker_lock_arg = pubkey_hash;
    let checker_info_script = builder
        .context
        .build_script(&always_success_code, checker_info_type_args.serialize())
        .expect("script");

    let mut task_type_args = TaskCellTypeArgs::default();
    task_type_args.checker_lock_arg = pubkey_hash;
    let task_script = builder
        .context
        .build_script(&always_success_code, task_type_args.serialize())
        .expect("script");

    //prepare dep
    let mut config_dep_data = SidechainConfigCell::default();
    config_dep_data.challenge_threshold = 1;
    config_dep_data.collator_lock_arg.copy_from_slice(&pubkey_hash);

    let config_dep_out_point = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &config_script),
        config_dep_data.serialize(),
    );
    let config_dep = CellDep::new_builder().out_point(config_dep_out_point).build();
    let mut builder = builder.cell_dep(config_dep);

    // prepare inputs
    let mut state_input_date = SidechainStateCell::default();
    let mut info = CheckerLastAcceptTaskHeight::default();
    info.height = 2;
    state_input_date.checker_last_task_sidechain_heights.push(info);

    let state_input = builder.create_input(
        new_type_cell_output(1000, &always_success, &state_script),
        state_input_date.serialize(),
    );

    let checker_info_input_data = CheckerInfoCell::default();
    let checker_info_input = builder.create_input(
        new_type_cell_output(1000, &always_success, &checker_info_script),
        checker_info_input_data.serialize(),
    );

    let task_input_data = TaskCell::default();
    let task_input = builder.create_input(
        new_type_cell_output(1000, &always_success, &task_script),
        task_input_data.serialize(),
    );

    let builder = builder.input(state_input).input(checker_info_input).input(task_input);

    //prepare output
    let mut state_output_data = state_input_date.clone();
    state_output_data.random_offset += 1;

    let mut checker_info_output_data = checker_info_input_data.clone();
    checker_info_output_data.unpaid_fee = 0;

    let mut task_output_data = TaskCell::default();
    task_output_data.mode = TaskMode::Challenge;
    task_output_data.status = TaskStatus::ChallengePassed;

    let outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &state_script),
        new_type_cell_output(1000, &always_success, &checker_info_script),
        new_type_cell_output(1000, &always_success, &task_script),
    ];
    let outputs_data = vec![
        Bytes::new(),
        state_output_data.serialize(),
        checker_info_output_data.serialize(),
        task_output_data.serialize(),
    ];

    let mut witness = CheckerPublishChallengeWitness::default();
    witness.challenge_count = 1;
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
