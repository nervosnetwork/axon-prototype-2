use ckb_tool::bytes::Bytes;
use ckb_tool::ckb_crypto::secp::Generator;
use ckb_tool::ckb_types::packed::{CellDep, CellInput};
use ckb_tool::ckb_types::prelude::*;

use common_raw::cell::muse_token::MuseTokenCell;
use common_raw::cell::sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs};
use common_raw::cell::sidechain_fee::{SidechainFeeCell, SidechainFeeCellLockArgs};
use common_raw::cell::sidechain_state::CheckerLastAcceptTaskHeight;
use common_raw::cell::task::{TaskCell, TaskCellTypeArgs};
use common_raw::cell::{
    sidechain_bond::{SidechainBondCell, SidechainBondCellLockArgs},
    sidechain_state::{SidechainStateCell, SidechainStateCellTypeArgs},
};
use common_raw::common::BlockSlice;
use common_raw::witness::collator_publish_task::CollatorPublishTaskWitness;

use crate::common::*;
use crate::environment_builder::{AxonScripts, EnvironmentBuilder};
use crate::secp256k1::*;

const MAX_CYCLES: u64 = 10_000_000;
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

    let sidechain_state_type_args = SidechainStateCellTypeArgs::default();
    let sidechain_state_type_script = builder
        .context
        .build_script(&always_success_code, sidechain_state_type_args.serialize())
        .expect("script");

    let mut sidechain_bond_lock_args_dep = SidechainBondCellLockArgs::default();
    sidechain_bond_lock_args_dep.collator_lock_arg.copy_from_slice(&pubkey_hash);
    sidechain_bond_lock_args_dep.unlock_sidechain_height = SIDECHAIN_BOND_UNLOCK_HEIGHT;
    let sidechain_bond_lock_script_dep = builder
        .context
        .build_script(&always_success_code, sidechain_bond_lock_args_dep.serialize())
        .expect("script");

    let sidechian_fee_lock_args = SidechainFeeCellLockArgs::default();
    let sidechian_fee_lock_script = builder
        .context
        .build_script(&always_success_code, sidechian_fee_lock_args.serialize())
        .expect("script");

    //prepare dep
    let mut sidechain_config_data_dep = SidechainConfigCell::default();
    sidechain_config_data_dep.checker_total_count = 1;
    sidechain_config_data_dep.checker_normal_count = 1;
    sidechain_config_data_dep.activated_checkers.push([0; 20]);
    sidechain_config_data_dep.challenge_threshold = 1;
    sidechain_config_data_dep.commit_threshold = 1;
    sidechain_config_data_dep.check_fee_rate = 1;
    sidechain_config_data_dep.check_data_size_limit = 2;
    sidechain_config_data_dep.collator_lock_arg.copy_from_slice(&pubkey_hash);
    let sidechain_config_dep_out_point = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &config_type_script),
        sidechain_config_data_dep.serialize(),
    );
    let sidechain_config_dep = CellDep::new_builder().out_point(sidechain_config_dep_out_point).build();

    let sidechain_bond_data_dep = SidechainBondCell::default();
    let sidechain_bond_dep_out_point = builder.context.create_cell(
        new_type_cell_output(1000, &sidechain_bond_lock_script_dep, &always_success),
        sidechain_bond_data_dep.serialize(),
    );
    let sidechain_bond_dep = CellDep::new_builder().out_point(sidechain_bond_dep_out_point).build();
    let builder = builder.cell_dep(sidechain_config_dep);
    let mut builder = builder.cell_dep(sidechain_bond_dep);

    //prepare input
    let mut sidechain_state_data_input = SidechainStateCell::default();
    let mut info = CheckerLastAcceptTaskHeight::default();
    info.height = 2;
    sidechain_state_data_input.checker_last_task_sidechain_heights.push(info);

    let output = new_type_cell_output(1000, &always_success, &sidechain_state_type_script);
    let sidechain_state_input_outpoint = builder.context.create_cell(output, sidechain_state_data_input.serialize());
    let sidechain_state_input = CellInput::new_builder()
        .previous_output(sidechain_state_input_outpoint.clone())
        .build();
    let mut builder = builder.input(sidechain_state_input);

    let sidechain_fee_data_input = SidechainFeeCell::default();
    let output = new_type_cell_output(1000, &sidechian_fee_lock_script, &always_success);
    let sidechain_fee_input_outpoint = builder.context.create_cell(output, sidechain_fee_data_input.serialize());
    let sidechain_fee_input = CellInput::new_builder()
        .previous_output(sidechain_fee_input_outpoint.clone())
        .build();
    let mut builder = builder.input(sidechain_fee_input);

    let mut muse_token_data_input = MuseTokenCell::default();
    muse_token_data_input.amount = 1;
    let output = new_type_cell_output(1000, &always_success, &always_success);
    let muse_token_input_outpoint = builder.context.create_cell(output, muse_token_data_input.serialize());
    let muse_token_input = CellInput::new_builder().previous_output(muse_token_input_outpoint.clone()).build();
    let builder = builder.input(muse_token_input);

    //prepare output
    let mut outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &sidechain_state_type_script),
        new_type_cell_output(1000, &sidechian_fee_lock_script, &always_success),
    ];

    let mut sidechain_state_data_output = sidechain_state_data_input.clone();
    sidechain_state_data_output.random_offset += 1;
    sidechain_state_data_output.waiting_jobs.push(BlockSlice { from: 1, to: 2 });

    let mut sidechain_fee_data_output = SidechainFeeCell::default();
    sidechain_fee_data_output.amount = 1;
    let mut outputs_data = vec![
        Bytes::new(),
        sidechain_state_data_output.serialize(),
        sidechain_fee_data_output.serialize(),
    ];

    let mut task_data_output = TaskCell::default();
    task_data_output.sidechain_block_height_from = 1;
    task_data_output.sidechain_block_height_to = 2;
    let output = new_type_cell_output(1000, &always_success, &task_type_script);
    outputs.push(output);
    outputs_data.push(task_data_output.serialize());

    let mut witness = CollatorPublishTaskWitness::default();
    witness.from_height = 1;
    witness.to_height = 2;
    witness.check_data_size = 1;
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
