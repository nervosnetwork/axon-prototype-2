use crate::common::*;
use crate::environment_builder::{AxonScripts, EnvironmentBuilder};
use crate::secp256k1::*;
use ckb_tool::bytes::Bytes;
use ckb_tool::ckb_crypto::secp::Generator;
use ckb_tool::ckb_types::packed::CellInput;
use ckb_tool::ckb_types::prelude::*;
use common_raw::cell::checker_info::{CheckerInfoCellData, CheckerInfoCellMode, CheckerInfoCellTypeArgs};
use common_raw::cell::sidechain_bond::{SidechainBondCellData, SidechainBondCellLockArgs};
use common_raw::cell::sidechain_config::{SidechainConfigCellData, SidechainConfigCellTypeArgs};
use common_raw::cell::sidechain_fee::{SidechainFeeCellData, SidechainFeeCellLockArgs};
use common_raw::pattern::Pattern;
use common_raw::witness::collator_submit_challenge::CollatorSubmitChallengeWitness;
use core::convert::TryFrom;

const MAX_CYCLES: u64 = 10_000_000;
const COMMIT_THRESHOLD: u8 = 3;
const REJECT_TASK_COUNT: u8 = 2;
const CHALLENGE_THRESHOLD: u8 = 1;
const FEE_RATE: u128 = 1;
const CHECKER_COUNT: u8 = 10;
const CHECKE_SIZE: u128 = 10;

const SIDECHAIN_BOND_AMOUNT: u128 = 10;
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

    //prepare scripts
    let sidechain_config_type_args_input_output = SidechainConfigCellTypeArgs::default();
    let sidechain_config_type_script_input_output = builder
        .context
        .build_script(&always_success_code, sidechain_config_type_args_input_output.serialize())
        .expect("script");

    let sidechain_fee_lock_args_input_output = SidechainFeeCellLockArgs::default();
    let sidechain_fee_lock_script_input_output = builder
        .context
        .build_script(&always_success_code, sidechain_fee_lock_args_input_output.serialize())
        .expect("script");

    let sidechain_bond_lock_args_input = SidechainBondCellLockArgs::default();
    let sidechain_bond_lock_script_input = builder
        .context
        .build_script(&always_success_code, sidechain_bond_lock_args_input.serialize())
        .expect("script");

    let checker_info_type_args_input_output = CheckerInfoCellTypeArgs::default();
    let checker_info_type_script_input_output = builder
        .context
        .build_script(&always_success_code, checker_info_type_args_input_output.serialize())
        .expect("script");
    //prepare inputs
    let mut sidechain_config_data_input = SidechainConfigCellData::default();
    sidechain_config_data_input.commit_threshold = COMMIT_THRESHOLD;
    sidechain_config_data_input.challenge_threshold = CHALLENGE_THRESHOLD;
    sidechain_config_data_input.checker_total_count = CHECKER_COUNT;
    sidechain_config_data_input.check_fee_rate = u32::try_from(FEE_RATE).expect("convert");

    sidechain_config_data_input.checker_bitmap = [
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    let sidechain_config_input_out_point = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &sidechain_config_type_script_input_output),
        sidechain_config_data_input.serialize(),
    );
    let sidechain_config_input = CellInput::new_builder().previous_output(sidechain_config_input_out_point).build();
    let mut builder = builder.input(sidechain_config_input);

    let sidechain_fee_data_input = SidechainFeeCellData::default();
    let sidechain_fee_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &sidechain_fee_lock_script_input_output, &always_success),
        sidechain_fee_data_input.serialize(),
    );
    let sidechain_fee_input = CellInput::new_builder().previous_output(sidechain_fee_input_outpoint).build();
    let mut builder = builder.input(sidechain_fee_input);

    let mut sidechain_bond_data_input = SidechainBondCellData::default();
    sidechain_bond_data_input.amount = SIDECHAIN_BOND_AMOUNT;
    let sidechain_bond_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &sidechain_bond_lock_script_input, &always_success),
        sidechain_bond_data_input.serialize(),
    );
    let sidechain_bond_input = CellInput::new_builder().previous_output(sidechain_bond_input_outpoint).build();
    let mut builder = builder.input(sidechain_bond_input);

    for _ in 0..REJECT_TASK_COUNT * CHALLENGE_THRESHOLD {
        let mut checker_info_data_input = CheckerInfoCellData::default();
        checker_info_data_input.mode = CheckerInfoCellMode::ChallengeRejected;
        checker_info_data_input.unpaid_check_data_size = CHECKE_SIZE;
        let checker_info_input_outpoint = builder.context.create_cell(
            new_type_cell_output(1000, &always_success, &checker_info_type_script_input_output),
            checker_info_data_input.serialize(),
        );
        let checker_info_input = CellInput::new_builder().previous_output(checker_info_input_outpoint).build();
        builder = builder.input(checker_info_input);
    }

    let mut checker_info_data_input = CheckerInfoCellData::default();
    checker_info_data_input.mode = CheckerInfoCellMode::TaskPassed;
    checker_info_data_input.unpaid_check_data_size = CHECKE_SIZE;
    checker_info_data_input.checker_id = 1;
    let checker_info_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &checker_info_type_script_input_output),
        checker_info_data_input.serialize(),
    );
    let checker_info_input = CellInput::new_builder().previous_output(checker_info_input_outpoint).build();
    let builder = builder.input(checker_info_input);

    //prepare outputs
    let outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &sidechain_config_type_script_input_output),
        new_type_cell_output(1000, &sidechain_fee_lock_script_input_output, &always_success),
        new_type_cell_output(1000, &always_success, &checker_info_type_script_input_output),
        new_type_cell_output(1000, &always_success, &checker_info_type_script_input_output),
    ];

    let mut sidechain_config_data_output = SidechainConfigCellData::default();
    sidechain_config_data_output.commit_threshold = COMMIT_THRESHOLD;
    sidechain_config_data_output.challenge_threshold = CHALLENGE_THRESHOLD;
    sidechain_config_data_output.checker_total_count = CHECKER_COUNT - 1;
    sidechain_config_data_output.check_fee_rate = u32::try_from(FEE_RATE).expect("convert");

    let mut sidechain_fee_data_output = SidechainFeeCellData::default();
    sidechain_fee_data_output.amount = SIDECHAIN_BOND_AMOUNT;

    let mut outputs_data = vec![
        Bytes::new(),
        sidechain_config_data_output.serialize(),
        sidechain_fee_data_output.serialize(),
    ];

    for _ in 0..REJECT_TASK_COUNT * CHALLENGE_THRESHOLD {
        let mut checker_info_data_output = CheckerInfoCellData::default();
        checker_info_data_output.mode = CheckerInfoCellMode::Idle;
        checker_info_data_output.unpaid_check_data_size = CHECKE_SIZE;
        checker_info_data_output.unpaid_fee = CHECKE_SIZE * FEE_RATE;
        outputs_data.push(checker_info_data_output.serialize());
    }

    let mut witness = CollatorSubmitChallengeWitness::default();
    witness.fee = u128::from(REJECT_TASK_COUNT) * CHECKE_SIZE * FEE_RATE;
    witness.fee_per_checker = CHECKE_SIZE * FEE_RATE;
    witness.valid_challenge_count = 2;
    witness.task_count = 1;
    witness.punish_checker_bitmap = [0; 32];
    witness.punish_checker_bitmap[0] = 0x40;
    witness.pattern = Pattern::CollatorSubmitSuccessChallenge;

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
