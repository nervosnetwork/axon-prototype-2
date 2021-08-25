use ckb_tool::ckb_crypto::secp::Generator;
use ckb_tool::ckb_types::{bytes::Bytes, core, packed::*, prelude::*};

use common_raw::cell::sidechain_state::{CheckerLastAcceptTaskHeight, PunishedChecker, SidechainStateCell, SidechainStateCellTypeArgs};
use common_raw::common::PubKeyHash;
use common_raw::{
    cell::{
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs},
        task::{TaskCell, TaskCellTypeArgs},
    },
    witness::anyone_refresh_task::AnyoneRefreshTaskWitness,
};

use crate::common::*;
use crate::environment_builder::{AxonScripts, EnvironmentBuilder};
use crate::secp256k1::*;

const MAX_CYCLES: u64 = 10_000_000;
const PUNISH_THREAD: u32 = 1000;

fn with_time_header(mut builder: EnvironmentBuilder, timestamp: u64) -> EnvironmentBuilder {
    let header = core::HeaderBuilder::default().timestamp(timestamp.pack()).build();
    builder.context.insert_header(header.clone());
    let builder = builder.header_dep(header.hash());
    builder
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
    //prepare headers
    let mut builder = with_time_header(builder, 1100);

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

    let task_input_type_args = TaskCellTypeArgs::default();
    let task_input_script = builder
        .context
        .build_script(&always_success_code, task_input_type_args.serialize())
        .expect("script");

    let task_output_type_args = TaskCellTypeArgs::default();
    let task_output_script = builder
        .context
        .build_script(&always_success_code, task_output_type_args.serialize())
        .expect("script");

    // prepare inputs
    let mut config_cell_data = SidechainConfigCell::default();
    config_cell_data.activated_checkers.push(PubKeyHash::default());
    config_cell_data.refresh_punish_threshold = PUNISH_THREAD;
    config_cell_data.refresh_interval = 90;
    let config_cell_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &config_script),
        config_cell_data.serialize(),
    );
    let config_cell_input = CellInput::new_builder().previous_output(config_cell_outpoint).build();
    let mut builder = builder.input(config_cell_input);

    let state_cell_input_data = SidechainStateCell::default();
    let state_cell_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &state_script),
        state_cell_input_data.serialize(),
    );
    let state_cell_input = CellInput::new_builder().previous_output(state_cell_outpoint).build();
    let mut builder = builder.input(state_cell_input);

    let task_cell_data = TaskCell::default();
    let task_cell_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &task_input_script),
        task_cell_data.serialize(),
    );
    let task_cell_input = CellInput::new_builder().previous_output(task_cell_outpoint).build();
    let builder = builder.input(task_cell_input);

    // prepare outputs
    let mut task_cell_data = TaskCell::default();
    task_cell_data.refresh_timestamp = 1190;
    let mut config_cell_data = SidechainConfigCell::default();
    config_cell_data.activated_checkers.push(PubKeyHash::default());
    config_cell_data.refresh_punish_threshold = PUNISH_THREAD;
    config_cell_data.refresh_interval = 90;

    let mut state_cell_output_data = SidechainStateCell::default();
    state_cell_output_data.punish_checkers.push(PunishedChecker::default());
    state_cell_output_data
        .checker_last_task_sidechain_heights
        .push(CheckerLastAcceptTaskHeight::default());
    state_cell_output_data.random_offset = 1;

    let outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &config_script),
        new_type_cell_output(1000, &always_success, &state_script),
        new_type_cell_output(1000, &always_success, &task_output_script),
    ];
    let outputs_data: Vec<Bytes> = vec![
        Bytes::new(),
        config_cell_data.serialize(),
        state_cell_output_data.serialize(),
        task_cell_data.serialize(),
    ];

    let witness = AnyoneRefreshTaskWitness::default();
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
