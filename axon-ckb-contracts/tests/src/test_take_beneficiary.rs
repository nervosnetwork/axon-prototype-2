use crate::common::*;
use crate::environment_builder::{AxonScripts, EnvironmentBuilder};
use crate::secp256k1::*;
use ckb_tool::ckb_crypto::secp::Generator;
use ckb_tool::ckb_types::{bytes::Bytes, prelude::*};

use common_raw::{
    cell::{
        checker_info::{CheckerInfoCell, CheckerInfoCellTypeArgs},
        muse_token::MuseTokenCell,
        sidechain_fee::{SidechainFeeCell, SidechainFeeCellLockArgs},
    },
    witness::checker_take_beneficiary::CheckerTakeBeneficiaryWitness,
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
    let sidechain_fee_lock_args = SidechainFeeCellLockArgs::default();
    let sidechain_fee_script = builder
        .context
        .build_script(&always_success_code, sidechain_fee_lock_args.serialize())
        .expect("script");

    let mut checker_info_type_args = CheckerInfoCellTypeArgs::default();
    checker_info_type_args.checker_lock_arg.copy_from_slice(&pubkey_hash);

    let checker_info_script = builder
        .context
        .build_script(&always_success_code, checker_info_type_args.serialize())
        .expect("script");

    // prepare inputs
    let mut checker_info_input_data = CheckerInfoCell::default();
    checker_info_input_data.unpaid_fee = 100;

    let checker_info_input = builder.create_input(
        new_type_cell_output(1000, &always_success, &checker_info_script),
        checker_info_input_data.serialize(),
    );

    let mut sidechain_fee_input_data = SidechainFeeCell::default();
    sidechain_fee_input_data.amount = 100;

    let sidechain_fee_input = builder.create_input(
        new_type_cell_output(1000, &sidechain_fee_script, &always_success),
        sidechain_fee_input_data.serialize(),
    );

    let muse_input_data = MuseTokenCell::default();
    let muse_input = builder.create_input(
        new_type_cell_output(1000, &always_success, &always_success),
        muse_input_data.serialize(),
    );

    let builder = builder.input(checker_info_input).input(sidechain_fee_input).input(muse_input);

    // prepare outputs
    let mut checker_info_output = checker_info_input_data.clone();
    checker_info_output.unpaid_fee = 0;

    let mut sidechain_fee_output = sidechain_fee_input_data.clone();
    sidechain_fee_output.amount = 0;

    let mut muse_output = muse_input_data.clone();
    muse_output.amount = 100;

    let outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &checker_info_script),
        new_type_cell_output(1000, &sidechain_fee_script, &always_success),
        new_type_cell_output(1000, &always_success, &always_success),
    ];
    let outputs_data: Vec<Bytes> = vec![
        Bytes::new(),
        checker_info_output.serialize(),
        sidechain_fee_output.serialize(),
        muse_output.serialize(),
    ];

    let mut witness = CheckerTakeBeneficiaryWitness::default();
    witness.fee = 100;

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
