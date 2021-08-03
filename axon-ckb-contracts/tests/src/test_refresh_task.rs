use crate::common::*;
use crate::environment_builder::{AxonScripts, EnvironmentBuilder};
use crate::secp256k1::*;
use ckb_tool::ckb_crypto::secp::Generator;
use ckb_tool::ckb_types::{bytes::Bytes, core, packed::*, prelude::*};
use common_raw::cell::{
    sidechain_config::{SidechainConfigCellData, SidechainConfigCellTypeArgs},
    task::{TaskCell, TaskCellTypeArgs},
};

const MAX_CYCLES: u64 = 10_000_000;

fn with_number_header(mut builder: EnvironmentBuilder, number: u64) -> (EnvironmentBuilder, core::HeaderView) {
    let header = core::HeaderBuilder::default().number(number.pack()).build();
    builder.context.insert_header(header.clone());

    let builder = builder.header_dep(header.hash());

    (builder, header)
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

    // prepare headers
    let (builder, config_header) = with_number_header(builder, 1000);
    let (mut builder, _) = with_number_header(builder, 1010);

    // prepare scripts
    let config_type_args = SidechainConfigCellTypeArgs::default();
    let config_script = builder
        .context
        .build_script(&always_success_code, config_type_args.serialize())
        .expect("script");

    let task_type_args = TaskCellTypeArgs::default();
    let task_script = builder
        .context
        .build_script(&always_success_code, task_type_args.serialize())
        .expect("script");

    // prepare celldep
    let scc_data = SidechainConfigCellData::default();
    let scc_dep = CellDepBuilder::default()
        .out_point(builder.context.create_cell(
            new_type_cell_output(1000, &always_success, &config_script),
            Bytes::copy_from_slice(&scc_data.serialize()),
        ))
        .build();
    let mut builder = builder.cell_dep(scc_dep);

    // prepare inputs
    let task_cell_data = TaskCell::default();
    let task_cell_outpoint = builder.context.create_cell(
        new_type_cell_output(1000, &always_success, &task_script),
        task_cell_data.serialize(),
    );
    let task_cell_input = CellInput::new_builder().previous_output(task_cell_outpoint.clone()).build();

    builder
        .context
        .link_cell_with_block(task_cell_outpoint.clone(), config_header.hash(), 0);

    let builder = builder.input(task_cell_input);

    // prepare outputs
    let task_cell_data = TaskCell::default();

    let outputs = vec![
        new_type_cell_output(1000, &always_success, &code_cell_script),
        new_type_cell_output(1000, &always_success, &task_script),
    ];
    let outputs_data: Vec<Bytes> = vec![Bytes::new(), task_cell_data.serialize()];

    let witnesses = [get_dummy_witness_builder()
        .input_type(Bytes::copy_from_slice(&[11, 0]).pack_some())
        .as_bytes()];

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
