use crate::common::*;
use crate::environment_builder::{AxonScripts, EnvironmentBuilder};
use crate::secp256k1::*;
use ckb_tool::bytes::Bytes;
use ckb_tool::ckb_crypto::secp::Generator;
use ckb_tool::ckb_types::packed::CellDep;
use ckb_tool::ckb_types::prelude::*;

use common_raw::cell::sidechain_config::{SidechainConfigCell, SidechainConfigCellTypeArgs, SidechainStatus};

use common_raw::cell::sidechain_state::{SidechainStateCell, SidechainStateCellTypeArgs};
use common_raw::witness::collator_shutdown_sidechain::CollatorShutdownSidechainWitness;

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

    //prepare scripts
    let config_type_args = SidechainConfigCellTypeArgs::default();
    let config_type_script = builder
        .context
        .build_script(&always_success_code, config_type_args.serialize())
        .expect("script");

    let state_type_args = SidechainStateCellTypeArgs::default();
    let state_type_script = builder
        .context
        .build_script(&always_success_code, state_type_args.serialize())
        .expect("script");

    //prepare deps
    let state_data_dep = SidechainStateCell::default();

    let state_dep_out_point = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &state_type_script),
        state_data_dep.serialize(),
    );
    let state_dep = CellDep::new_builder().out_point(state_dep_out_point).build();

    let mut builder = builder.cell_dep(state_dep);
    //prepare inputs
    let mut config_input_data = SidechainConfigCell::default();
    config_input_data.collator_lock_arg = pubkey_hash;

    let config_input_out_point = builder.create_input(
        new_type_cell_output(1000, &always_success, &config_type_script),
        config_input_data.serialize(),
    );

    let builder = builder.input(config_input_out_point);

    //prepare outputs
    let outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &config_type_script),
    ];

    let mut config_output_data = config_input_data.clone();
    config_output_data.sidechain_status = SidechainStatus::Shutdown;

    let outputs_data = vec![Bytes::new(), config_output_data.serialize()];

    let builder = builder.outputs(outputs).outputs_data(outputs_data.pack());

    let witness = CollatorShutdownSidechainWitness::default();
    let witnesses = [get_dummy_witness_builder().input_type(witness.serialize().pack_some()).as_bytes()];

    // build transaction
    let tx = builder.builder.build();
    let tx = tx
        .as_advanced_builder()
        .set_witnesses(sign_tx_with_witnesses(tx, witnesses.pack(), &privkey).unwrap())
        .build();

    // run
    builder.context.verify_tx(&tx, MAX_CYCLES).expect("pass verification");
}
