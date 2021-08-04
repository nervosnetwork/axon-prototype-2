use crate::common::*;
use crate::environment_builder::{AxonScripts, EnvironmentBuilder};
use crate::secp256k1::*;
use ckb_tool::ckb_crypto::secp::Generator;
use ckb_tool::ckb_types::{bytes::Bytes, prelude::*};

use common_raw::{
    cell::{
        checker_bond::{CheckerBondCellData, CheckerBondCellLockArgs},
        sudt_token::SudtTokenData,
    },
    witness::checker_bond_withdraw::CheckerBondWithdrawWitness,
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
    println!("{:?}", always_success.as_reader().code_hash().as_slice());
    println!("{:?}", always_success.calc_script_hash().as_slice());
    // prepare scripts
    let mut checker_bond_lock_args = CheckerBondCellLockArgs::default();
    checker_bond_lock_args.checker_lock_arg.copy_from_slice(&pubkey_hash);

    let checker_bond_lock_input_script = builder
        .context
        .build_script(&always_success_code, checker_bond_lock_args.serialize())
        .expect("script");

    // prepare inputs
    let mut checker_bond_input_data = CheckerBondCellData::default();
    checker_bond_input_data.amount = 100;

    let checker_bond_input = builder.create_input(
        new_type_cell_output(1000, &checker_bond_lock_input_script, &always_success),
        checker_bond_input_data.serialize(),
    );

    let builder = builder.input(checker_bond_input);

    // prepare outputs
    let mut sudt_output = SudtTokenData::default();
    sudt_output.amount = 100;

    let outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &always_success),
    ];
    let outputs_data: Vec<Bytes> = vec![Bytes::new(), sudt_output.serialize()];

    let witness = CheckerBondWithdrawWitness::default();
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
