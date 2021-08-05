use crate::common::*;
use crate::environment_builder::{AxonScripts, EnvironmentBuilder};
use crate::secp256k1::*;
use ckb_tool::bytes::Bytes;
use ckb_tool::ckb_crypto::secp::Generator;
use ckb_tool::ckb_types::packed::{CellDep, CellInput};
use ckb_tool::ckb_types::prelude::*;

use common_raw::cell::sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs};
use common_raw::cell::task::{TaskCell, TaskCellTypeArgs};
use common_raw::cell::{
    sidechain_bond::{SidechainBondCell, SidechainBondCellLockArgs},
    sidechain_state::{SidechainStateCell, SidechainStateCellTypeArgs},
};
use common_raw::witness::collator_publish_task::CollatorPublishTaskWitness;

const MAX_CYCLES: u64 = 10_000_000;

const CHECKER_COUNT: u32 = 20;
const MIN_BOND: u128 = 10;
const SIDECHAIN_BOND_AMOUNT: u128 = MIN_BOND + 1;
const TASK_NUMBER: u32 = 3;
const SIDECHAIN_BOND_UNLOCK_HEIGHT: u128 = 1000;
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
    let config_type_script = builder
        .context
        .build_script(&always_success_code, config_type_args.serialize())
        .expect("script");

    let task_type_args = TaskCellTypeArgs::default();
    let task_type_script = builder
        .context
        .build_script(&always_success_code, task_type_args.serialize())
        .expect("script");

    let sidechain_state_type_args_input_output = SidechainStateCellTypeArgs::default();
    let sidechain_state_type_script_input_output = builder
        .context
        .build_script(&always_success_code, sidechain_state_type_args_input_output.serialize())
        .expect("script");

    let mut sidechain_bond_lock_args_dep = SidechainBondCellLockArgs::default();
    sidechain_bond_lock_args_dep.collator_lock_arg.copy_from_slice(&pubkey_hash);
    sidechain_bond_lock_args_dep.unlock_sidechain_height = SIDECHAIN_BOND_UNLOCK_HEIGHT;
    let sidechain_bond_lock_script_dep = builder
        .context
        .build_script(&always_success_code, sidechain_bond_lock_args_dep.serialize())
        .expect("script");

    //prepare dep
    let mut sidechain_config_data_dep = SidechainConfigCell::default();
    sidechain_config_data_dep.checker_total_count = CHECKER_COUNT;
    sidechain_config_data_dep.commit_threshold = TASK_NUMBER;
    sidechain_config_data_dep.minimal_bond = MIN_BOND;
    sidechain_config_data_dep.collator_lock_arg.copy_from_slice(&pubkey_hash);
    let sidechain_config_dep_out_point = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &config_type_script),
        sidechain_config_data_dep.serialize(),
    );
    let sidechain_config_dep = CellDep::new_builder().out_point(sidechain_config_dep_out_point).build();

    let mut sidechain_bond_data_dep = SidechainBondCell::default();
    sidechain_bond_data_dep.amount = SIDECHAIN_BOND_AMOUNT;
    let sidechain_bond_dep_out_point = builder.context.create_cell(
        new_type_cell_output(1000, &sidechain_bond_lock_script_dep, &always_success),
        sidechain_bond_data_dep.serialize(),
    );
    let sidechain_bond_dep = CellDep::new_builder().out_point(sidechain_bond_dep_out_point).build();
    let builder = builder.cell_dep(sidechain_config_dep);
    let mut builder = builder.cell_dep(sidechain_bond_dep);

    //prepare input
    let sidechain_state_data_input = SidechainStateCell::default();
    let output = new_type_cell_output(1000, &always_success, &sidechain_state_type_script_input_output);
    let sidechain_state_input_outpoint = builder.context.create_cell(output, sidechain_state_data_input.serialize());
    let sidechain_state_input = CellInput::new_builder()
        .previous_output(sidechain_state_input_outpoint.clone())
        .build();
    let builder = builder.input(sidechain_state_input);

    //prepare output
    let mut outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &sidechain_state_type_script_input_output),
    ];

    let sidechain_state_data_output = SidechainStateCell::default();

    let mut sidechain_bond_data_output = SidechainBondCell::default();
    sidechain_bond_data_output.amount = SIDECHAIN_BOND_AMOUNT;

    let mut outputs_data = vec![Bytes::new(), sidechain_state_data_output.serialize()];

    for _ in 0..TASK_NUMBER {
        let mut task_data_output = TaskCell::default();
        task_data_output.sidechain_block_height_from = 1;
        task_data_output.sidechain_block_height_to = 2;
        let output = new_type_cell_output(1000, &always_success, &task_type_script);
        outputs.push(output);
        outputs_data.push(task_data_output.serialize());
    }

    let witness = CollatorPublishTaskWitness::default();
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
