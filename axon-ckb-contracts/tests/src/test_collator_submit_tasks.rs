use crate::common::*;
use crate::environment_builder::{AxonScripts, EnvironmentBuilder};
use crate::secp256k1::*;
use ckb_tool::ckb_types::prelude::*;
use ckb_tool::{bytes::Bytes, ckb_crypto::secp::Generator, ckb_types::packed::CellInput};
use common_raw::{
    cell::{
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs},
        sidechain_fee::{SidechainFeeCell, SidechainFeeCellLockArgs},
        sidechain_state::{CommittedCheckerInfo, SidechainStateCell, SidechainStateCellTypeArgs},
        task::{TaskCell, TaskCellTypeArgs, TaskMode, TaskStatus},
    },
    common::*,
    witness::{collator_submit_tasks::CollatorSubmitTasksWitness, common_submit_jobs::ExistedCommittedCheckerInfo},
};
const MAX_CYCLES: u64 = 10_000_000;

const COMMIT_THRESHOLD: u32 = 4;
const CHALLENGE_THRESHOLD: u32 = 2;
const TASK_NUMBER: u32 = 5; // 4 - once challenge + 2
const CHECKED_SIZE: u128 = 10;
const FEE_RATE: u32 = 1;

const BLANK_HASH: [u8; 32] = [
    38, 108, 236, 151, 203, 237, 226, 207, 188, 231, 54, 102, 240, 141, 238, 217, 86, 11, 223, 120, 65, 167, 165, 165, 27, 58, 63, 9, 218,
    36, 158, 33,
]; // Blake2b hash for [0u8; 32]

const VALID_CHECKER_LOCK_ARG: PubKeyHash = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const NEW_CHECKER_LOCK_ARG: PubKeyHash = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const INVALID_CHECKER_LOCK_ARG: PubKeyHash = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const VALID_CHALLENGE_CHECKER_LOCK_ARG: PubKeyHash = [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const INVALID_CHALLENGE_CHECKER_LOCK_ARG: PubKeyHash = [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

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

    let sidechain_state_type_args = SidechainStateCellTypeArgs::default();
    let sidechain_state_type_script = builder
        .context
        .build_script(&always_success_code, sidechain_state_type_args.serialize())
        .expect("script");

    let mut sidechain_fee_lock_args = SidechainFeeCellLockArgs::default();
    let sidechain_fee_output_lock_script = builder
        .context
        .build_script(&always_success_code, sidechain_fee_lock_args.serialize())
        .expect("script");

    sidechain_fee_lock_args.surplus = FEE_RATE as u128 * CHECKED_SIZE * TASK_NUMBER as u128;
    let sidechain_fee_input_lock_script = builder
        .context
        .build_script(&always_success_code, sidechain_fee_lock_args.serialize())
        .expect("script");

    let mut task_type_args = TaskCellTypeArgs::default();
    let existed_task_type_script = builder
        .context
        .build_script(&always_success_code, task_type_args.serialize())
        .expect("script");

    task_type_args.checker_lock_arg = NEW_CHECKER_LOCK_ARG;
    let new_task_type_script = builder
        .context
        .build_script(&always_success_code, task_type_args.serialize())
        .expect("script");

    task_type_args.checker_lock_arg = INVALID_CHECKER_LOCK_ARG;
    let invalid_task_type_script = builder
        .context
        .build_script(&always_success_code, task_type_args.serialize())
        .expect("script");

    task_type_args.checker_lock_arg = VALID_CHALLENGE_CHECKER_LOCK_ARG;
    let valid_challenge_type_script = builder
        .context
        .build_script(&always_success_code, task_type_args.serialize())
        .expect("script");

    task_type_args.checker_lock_arg = INVALID_CHALLENGE_CHECKER_LOCK_ARG;
    let invalid_challenge_type_script = builder
        .context
        .build_script(&always_success_code, task_type_args.serialize())
        .expect("script");

    //prepare inputs
    let mut sidechain_config_input_data = SidechainConfigCell::default();
    sidechain_config_input_data.commit_threshold = COMMIT_THRESHOLD;
    sidechain_config_input_data.challenge_threshold = CHALLENGE_THRESHOLD;
    sidechain_config_input_data.collator_lock_arg.copy_from_slice(&pubkey_hash);
    sidechain_config_input_data.check_fee_rate = FEE_RATE;
    sidechain_config_input_data.activated_checkers = vec![
        VALID_CHECKER_LOCK_ARG,
        NEW_CHECKER_LOCK_ARG,
        INVALID_CHECKER_LOCK_ARG,
        VALID_CHALLENGE_CHECKER_LOCK_ARG,
        INVALID_CHECKER_LOCK_ARG,
    ];

    let sidechain_config_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &sidechain_config_type_script),
        sidechain_config_input_data.serialize(),
    );
    let sidechain_config_input = CellInput::new_builder()
        .previous_output(sidechain_config_input_outpoint.clone())
        .build();
    let mut builder = builder.input(sidechain_config_input);

    let mut sidechain_state_input_data = SidechainStateCell::default();

    let existed_checker_info = CommittedCheckerInfo {
        checker_lock_arg: VALID_CHECKER_LOCK_ARG,
        committed_hash:   BLANK_HASH,
    };
    let invalid_checker_info = CommittedCheckerInfo {
        checker_lock_arg: INVALID_CHECKER_LOCK_ARG,
        committed_hash:   BLANK_HASH,
    };
    let valid_challenge_checker_info = CommittedCheckerInfo {
        checker_lock_arg: VALID_CHALLENGE_CHECKER_LOCK_ARG,
        committed_hash:   BLANK_HASH,
    };
    let invalid_challenge_checker_info = CommittedCheckerInfo {
        checker_lock_arg: INVALID_CHALLENGE_CHECKER_LOCK_ARG,
        committed_hash:   BLANK_HASH,
    };

    sidechain_state_input_data.random_commit = vec![
        existed_checker_info,
        invalid_checker_info,
        valid_challenge_checker_info,
        invalid_challenge_checker_info,
    ];

    let sidechain_state_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &sidechain_state_type_script),
        sidechain_state_input_data.serialize(),
    );
    let sidechain_state_input = CellInput::new_builder()
        .previous_output(sidechain_state_input_outpoint.clone())
        .build();
    let mut builder = builder.input(sidechain_state_input);

    let sidechain_fee_input_data = SidechainFeeCell::default();
    let sidechain_fee_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &sidechain_fee_input_lock_script, &always_success),
        sidechain_fee_input_data.serialize(),
    );
    let sidechain_fee_input = CellInput::new_builder()
        .previous_output(sidechain_fee_input_outpoint.clone())
        .build();
    let mut builder = builder.input(sidechain_fee_input);

    let mut task_input_data = TaskCell::default();
    task_input_data.mode = TaskMode::Task;
    task_input_data.status = TaskStatus::TaskPassed;
    task_input_data.commit = BLANK_HASH;

    let existed_task_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &existed_task_type_script),
        task_input_data.serialize(),
    );
    let existed_task_input = CellInput::new_builder()
        .previous_output(existed_task_input_outpoint.clone())
        .build();
    let mut builder = builder.input(existed_task_input);

    let new_task_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &new_task_type_script),
        task_input_data.serialize(),
    );
    let new_task_input = CellInput::new_builder().previous_output(new_task_input_outpoint.clone()).build();
    let mut builder = builder.input(new_task_input);

    task_input_data.reveal[0] = 1;

    let invalid_task_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &invalid_task_type_script),
        task_input_data.serialize(),
    );
    let invalid_task_input = CellInput::new_builder()
        .previous_output(invalid_task_input_outpoint.clone())
        .build();
    let mut builder = builder.input(invalid_task_input);

    task_input_data.reveal = RandomSeed::default();
    task_input_data.mode = TaskMode::Challenge;
    task_input_data.status = TaskStatus::ChallengeRejected;

    let valid_challenge_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &valid_challenge_type_script),
        task_input_data.serialize(),
    );
    let valid_challenge_input = CellInput::new_builder()
        .previous_output(valid_challenge_input_outpoint.clone())
        .build();
    let mut builder = builder.input(valid_challenge_input);

    task_input_data.status = TaskStatus::ChallengePassed;

    let invalid_challenge_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &invalid_challenge_type_script),
        task_input_data.serialize(),
    );
    let invalid_challenge_input = CellInput::new_builder()
        .previous_output(invalid_challenge_input_outpoint.clone())
        .build();
    let builder = builder.input(invalid_challenge_input);

    //prepare outputs
    let mut sidechain_config_output_data = sidechain_config_input_data.clone();
    sidechain_config_output_data.activated_checkers = vec![VALID_CHECKER_LOCK_ARG, NEW_CHECKER_LOCK_ARG, VALID_CHALLENGE_CHECKER_LOCK_ARG];
    sidechain_config_output_data.jailed_checkers = vec![INVALID_CHECKER_LOCK_ARG, INVALID_CHALLENGE_CHECKER_LOCK_ARG];

    let mut sidechain_state_data_output = sidechain_state_input_data.clone();
    sidechain_state_data_output.random_seed = [
        221, 69, 216, 101, 62, 143, 232, 10, 142, 65, 192, 13, 1, 143, 107, 149, 92, 153, 26, 231, 162, 9, 76, 81, 63, 187, 104, 92, 156,
        86, 150, 48,
    ];

    let new_checker_info = CommittedCheckerInfo {
        checker_lock_arg: NEW_CHECKER_LOCK_ARG,
        committed_hash:   BLANK_HASH,
    };

    sidechain_state_data_output.random_commit = vec![existed_checker_info, valid_challenge_checker_info, new_checker_info];

    let sidechain_fee_data_output = SidechainFeeCell::default();

    let outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &sidechain_config_type_script),
        new_type_cell_output(1000, &always_success, &sidechain_state_type_script),
        new_type_cell_output(1000, &sidechain_fee_output_lock_script, &always_success),
    ];
    let outputs_data = vec![
        Bytes::new(),
        sidechain_config_output_data.serialize(),
        sidechain_state_data_output.serialize(),
        sidechain_fee_data_output.serialize(),
    ];

    let mut witness = CollatorSubmitTasksWitness::default();

    let existed_checker_info = ExistedCommittedCheckerInfo {
        index:                 Some(0),
        checker_lock_arg:      VALID_CHECKER_LOCK_ARG,
        origin_committed_hash: Some(BLANK_HASH),
        new_committed_hash:    Some(BLANK_HASH),
    };
    let new_checker_info = ExistedCommittedCheckerInfo {
        index:                 None,
        checker_lock_arg:      NEW_CHECKER_LOCK_ARG,
        origin_committed_hash: None,
        new_committed_hash:    Some(BLANK_HASH),
    };
    let invalid_checker_info = ExistedCommittedCheckerInfo {
        index:                 Some(1),
        checker_lock_arg:      INVALID_CHECKER_LOCK_ARG,
        origin_committed_hash: Some(BLANK_HASH),
        new_committed_hash:    None,
    };
    let valid_challenge_checker_info = ExistedCommittedCheckerInfo {
        index:                 Some(2),
        checker_lock_arg:      VALID_CHALLENGE_CHECKER_LOCK_ARG,
        origin_committed_hash: Some(BLANK_HASH),
        new_committed_hash:    Some(BLANK_HASH),
    };
    let invalid_challenge_checker_info = ExistedCommittedCheckerInfo {
        index:                 Some(3),
        checker_lock_arg:      INVALID_CHALLENGE_CHECKER_LOCK_ARG,
        origin_committed_hash: Some(BLANK_HASH),
        new_committed_hash:    None,
    };
    witness.common.commit = vec![
        existed_checker_info,
        new_checker_info,
        invalid_checker_info,
        valid_challenge_checker_info,
        invalid_challenge_checker_info,
    ];

    witness.common.challenge_times = 1;
    witness.common.fee_per_checker = FEE_RATE as u128 * CHECKED_SIZE;
    witness.common.fee = FEE_RATE as u128 * CHECKED_SIZE * TASK_NUMBER as u128;
    witness
        .common
        .new_random_seed
        .copy_from_slice(&sidechain_state_data_output.random_seed);

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
