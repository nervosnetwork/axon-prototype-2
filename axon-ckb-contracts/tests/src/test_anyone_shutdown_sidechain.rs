use crate::{
    common::*,
    environment_builder::{AxonScripts, EnvironmentBuilder},
    secp256k1::*,
};
use ckb_tool::{
    bytes::Bytes,
    ckb_crypto::secp::Generator,
    ckb_types::packed::{CellInput, OutPoint, Script},
    ckb_types::prelude::*,
};
use common_raw::{
    cell::{
        sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs, SidechainStatus},
        sidechain_fee::{SidechainFeeCell, SidechainFeeCellLockArgs},
        task::{TaskCell, TaskCellTypeArgs, TaskMode, TaskStatus},
    },
    common::*,
    witness::anyone_shutdown_sidechain::AnyoneShutdownSidechainWitness,
};

const MAX_CYCLES: u64 = 10_000_000;

const COMMIT_THRESHOLD: u32 = 3;
const CHALLENGE_THRESHOLD: u32 = 2;

const FEE_RATE: u32 = 1;
const CHECKED_SIZE: u128 = 1;

const TOTAL_FEE: u128 = 3 * FEE_RATE as u128 * CHECKED_SIZE; // 2 checkers will be jailed, so 3 checkers left

const GOOD_CHECKER_1: PubKeyHash = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const GOOD_CHECKER_2: PubKeyHash = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const GOOD_CHECKER_3: PubKeyHash = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

const REJECT_CHALLENGE_CHECKER: PubKeyHash = [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const PASS_TASK_CHECKER: PubKeyHash = [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

fn get_task_type_script(
    builder: &mut EnvironmentBuilder,
    always_success_code: &OutPoint,
    type_args: &mut TaskCellTypeArgs,
    lock_arg: &PubKeyHash,
) -> Script {
    type_args.checker_lock_arg.copy_from_slice(lock_arg);

    builder
        .context
        .build_script(&always_success_code, type_args.serialize())
        .expect("script")
}

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
    let sidechain_config_type_args = SidechainConfigCellTypeArgs::default();
    let sidechain_config_type_script = builder
        .context
        .build_script(&always_success_code, sidechain_config_type_args.serialize())
        .expect("script");

    let mut sidechain_fee_lock_args = SidechainFeeCellLockArgs::default();

    let sidechain_fee_output_lock_script = builder
        .context
        .build_script(&always_success_code, sidechain_fee_lock_args.serialize())
        .expect("script");

    sidechain_fee_lock_args.surplus = TOTAL_FEE;
    let sidechain_fee_input_lock_script = builder
        .context
        .build_script(&always_success_code, sidechain_fee_lock_args.serialize())
        .expect("script");

    let mut task_type_args = TaskCellTypeArgs::default();

    let good_checker_1_task_type_script = get_task_type_script(&mut builder, &always_success_code, &mut task_type_args, &GOOD_CHECKER_1);
    let good_checker_2_task_type_script = get_task_type_script(&mut builder, &always_success_code, &mut task_type_args, &GOOD_CHECKER_2);
    let good_checker_3_task_type_script = get_task_type_script(&mut builder, &always_success_code, &mut task_type_args, &GOOD_CHECKER_3);
    let reject_challenge_task_type_script =
        get_task_type_script(&mut builder, &always_success_code, &mut task_type_args, &REJECT_CHALLENGE_CHECKER);
    let pass_task_task_type_script = get_task_type_script(&mut builder, &always_success_code, &mut task_type_args, &PASS_TASK_CHECKER);

    //prepare inputs
    let mut sidechain_config_input_data = SidechainConfigCell::default();
    sidechain_config_input_data.commit_threshold = COMMIT_THRESHOLD;
    sidechain_config_input_data.challenge_threshold = CHALLENGE_THRESHOLD;
    sidechain_config_input_data.check_fee_rate = FEE_RATE;
    sidechain_config_input_data.checker_total_count = 5;
    sidechain_config_input_data.checker_normal_count = 5;
    sidechain_config_input_data.activated_checkers = vec![
        GOOD_CHECKER_1,
        GOOD_CHECKER_2,
        GOOD_CHECKER_3,
        REJECT_CHALLENGE_CHECKER,
        PASS_TASK_CHECKER,
    ];

    let sidechain_config_input_out_point = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &sidechain_config_type_script),
        sidechain_config_input_data.serialize(),
    );
    let sidechain_config_input = CellInput::new_builder().previous_output(sidechain_config_input_out_point).build();
    let mut builder = builder.input(sidechain_config_input);

    let sidechain_fee_input_data = SidechainFeeCell::default();
    let sidechain_fee_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &sidechain_fee_input_lock_script, &always_success),
        sidechain_fee_input_data.serialize(),
    );
    let sidechain_fee_input = CellInput::new_builder().previous_output(sidechain_fee_input_outpoint).build();
    let mut builder = builder.input(sidechain_fee_input);

    let mut task_input_data = TaskCell::default();
    task_input_data.mode = TaskMode::Challenge;
    task_input_data.status = TaskStatus::ChallengePassed;
    task_input_data.check_data_size = CHECKED_SIZE;

    // 3 passed challenge -- good checkers
    for type_script in vec![
        good_checker_1_task_type_script,
        good_checker_2_task_type_script,
        good_checker_3_task_type_script,
    ]
    .into_iter()
    {
        let task_input_outpoint = builder.context.create_cell(
            new_type_cell_output(1000, &always_success, &type_script),
            task_input_data.serialize(),
        );
        let task_input = CellInput::new_builder().previous_output(task_input_outpoint).build();
        builder = builder.input(task_input);
    }

    // 1 rejected challenge -- bad checker
    task_input_data.status = TaskStatus::ChallengeRejected;

    let task_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &reject_challenge_task_type_script),
        task_input_data.serialize(),
    );
    let task_input = CellInput::new_builder().previous_output(task_input_outpoint).build();

    let mut builder = builder.input(task_input);

    // 1 passed task -- bad checker
    task_input_data.mode = TaskMode::Task;
    task_input_data.status = TaskStatus::TaskPassed;

    let task_input_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &pass_task_task_type_script),
        task_input_data.serialize(),
    );
    let task_input = CellInput::new_builder().previous_output(task_input_outpoint).build();

    let builder = builder.input(task_input);

    //prepare outputs
    let mut sidechain_config_output_data = sidechain_config_input_data.clone();
    sidechain_config_output_data.sidechain_status = SidechainStatus::Shutdown;
    sidechain_config_output_data.checker_normal_count = 3;
    sidechain_config_output_data.activated_checkers = vec![GOOD_CHECKER_1, GOOD_CHECKER_2, GOOD_CHECKER_3];
    sidechain_config_output_data.jailed_checkers = vec![REJECT_CHALLENGE_CHECKER, PASS_TASK_CHECKER];

    let sidechain_fee_output_data = SidechainFeeCell::default();

    let outputs_data = vec![
        Bytes::new(),
        sidechain_config_output_data.serialize(),
        sidechain_fee_output_data.serialize(),
    ];

    let outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &sidechain_config_type_script),
        new_type_cell_output(1000, &sidechain_fee_output_lock_script, &always_success),
    ];

    let mut witness = AnyoneShutdownSidechainWitness::default();
    witness.check_data_size = CHECKED_SIZE;
    witness.challenge_times = 2;
    witness.jailed_checkers = vec![REJECT_CHALLENGE_CHECKER, PASS_TASK_CHECKER];

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
