use crate::common::*;
use crate::environment_builder::{AxonScripts, EnvironmentBuilder};
use crate::secp256k1::*;
use ckb_tool::ckb_types::prelude::*;
use ckb_tool::{
    bytes::Bytes,
    ckb_crypto::secp::Generator,
    ckb_types::packed::{CellDep, CellInput},
};
use common_raw::{
    cell::{
        checker_info::{CheckerInfoCell, CheckerInfoCellTypeArgs},
        muse_token::MuseTokenData,
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs},
        sidechain_fee::{SidechainFeeCell, SidechainFeeCellLockArgs},
        sidechain_state::{SidechainStateCell, SidechainStateCellTypeArgs},
    },
    witness::collator_submit_task::CollatorSubmitTaskWitness,
};
const MAX_CYCLES: u64 = 10_000_000;

const TASK_NUMBER: u8 = 1;
const CHECKED_SIZE: u128 = 10;
const FEE_RATE: u32 = 1;

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
    let sidechain_config_type_args = SidechainConfigCellTypeArgs::default();
    let sidechain_config_type_script = builder
        .context
        .build_script(&always_success_code, sidechain_config_type_args.serialize())
        .expect("script");

    let sidechain_state_type_args_input_output = SidechainStateCellTypeArgs::default();
    let sidechain_state_type_script_input_output = builder
        .context
        .build_script(&always_success_code, sidechain_state_type_args_input_output.serialize())
        .expect("script");

    let sidechain_fee_lock_args_input_output = SidechainFeeCellLockArgs::default();
    let sidechain_fee_lcok_script_input_output = builder
        .context
        .build_script(&always_success_code, sidechain_fee_lock_args_input_output.serialize())
        .expect("script");

    let checker_info_type_args_input_output = CheckerInfoCellTypeArgs::default();
    let checker_info_type_script_input_output = builder
        .context
        .build_script(&always_success_code, checker_info_type_args_input_output.serialize())
        .expect("script");

    // prepare deps
    let mut sidechain_config_data_dep = SidechainConfigCell::default();
    sidechain_config_data_dep.commit_threshold = TASK_NUMBER;
    sidechain_config_data_dep.collator_lock_arg.copy_from_slice(&pubkey_hash);
    sidechain_config_data_dep.check_fee_rate = FEE_RATE;

    let sidechain_config_dep_out_point = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &sidechain_config_type_script),
        sidechain_config_data_dep.serialize(),
    );

    let sidechain_config_dep = CellDep::new_builder().out_point(sidechain_config_dep_out_point).build();
    let mut builder = builder.cell_dep(sidechain_config_dep);

    //prepare inputs
    let mut sidechain_state_data_input = SidechainStateCell::default();
    sidechain_state_data_input.latest_block_height = 1000;
    let output = new_type_cell_output(1000, &always_success, &sidechain_state_type_script_input_output);
    let sidechain_state_input_outpoint = builder.context.create_cell(output, sidechain_state_data_input.serialize());
    let sidechain_state_input = CellInput::new_builder()
        .previous_output(sidechain_state_input_outpoint.clone())
        .build();
    let mut builder = builder.input(sidechain_state_input);

    let sidechain_fee_data_input = SidechainFeeCell::default();
    let output = new_type_cell_output(1000, &sidechain_fee_lcok_script_input_output, &always_success);
    let sidechain_fee_input_outpoint = builder.context.create_cell(output, sidechain_fee_data_input.serialize());
    let sidechain_fee_input = CellInput::new_builder()
        .previous_output(sidechain_fee_input_outpoint.clone())
        .build();
    let mut builder = builder.input(sidechain_fee_input);

    let mut muse_token_data_input = MuseTokenData::default();
    muse_token_data_input.amount = FEE_RATE as u128 * CHECKED_SIZE * TASK_NUMBER as u128;
    let output = new_type_cell_output(1000, &always_success, &always_success);
    let muse_token_input_outpoint = builder.context.create_cell(output, muse_token_data_input.serialize());
    let muse_token_input = CellInput::new_builder().previous_output(muse_token_input_outpoint.clone()).build();
    let mut builder = builder.input(muse_token_input);

    let mut checker_info_data_input = CheckerInfoCell::default();

    let output = new_type_cell_output(1000, &always_success, &checker_info_type_script_input_output);
    let checker_info_input_outpoint = builder.context.create_cell(output, checker_info_data_input.serialize());
    let checker_info_input = CellInput::new_builder()
        .previous_output(checker_info_input_outpoint.clone())
        .build();
    let builder = builder.input(checker_info_input);

    //prepare outputs
    let outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &sidechain_state_type_script_input_output),
        new_type_cell_output(1000, &sidechain_fee_lcok_script_input_output, &always_success),
        new_type_cell_output(1000, &always_success, &checker_info_type_script_input_output),
    ];
    let mut sidechain_state_data_output = SidechainStateCellData::default();

    let mut sidechain_fee_data_output = SidechainFeeCell::default();
    sidechain_fee_data_output.amount = FEE_RATE as u128 * CHECKED_SIZE * TASK_NUMBER as u128;

    let mut checker_info_data_output = CheckerInfoCell::default();

    let outputs_data = vec![
        Bytes::new(),
        sidechain_state_data_output.serialize(),
        sidechain_fee_data_output.serialize(),
        checker_info_data_output.serialize(),
    ];

    let mut witness = CollatorSubmitTaskWitness::default();
    witness.fee_per_checker = FEE_RATE as u128 * CHECKED_SIZE;
    witness.fee = FEE_RATE as u128 * CHECKED_SIZE * TASK_NUMBER as u128;
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
