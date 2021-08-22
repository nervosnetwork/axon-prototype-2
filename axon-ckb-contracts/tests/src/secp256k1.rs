use ckb_system_scripts::BUNDLED_CELL;
use ckb_tool::ckb_crypto::secp::Privkey;
use ckb_tool::ckb_hash::{blake2b_256, new_blake2b};
use ckb_tool::ckb_types::{bytes::Bytes, core::TransactionView, packed, packed::*, prelude::*, H256};

use crate::common::*;
use crate::environment_builder::EnvironmentBuilder;

const SIGNATURE_SIZE: usize = 65;

pub fn blake160(data: &[u8]) -> [u8; 20] {
    let mut buf = [0u8; 20];
    let hash = blake2b_256(data);
    buf.clone_from_slice(&hash[..20]);
    buf
}

pub fn sign_tx(tx: TransactionView, key: &Privkey) -> TransactionView {
    let witnesses = BytesVecBuilder::default().push(get_dummy_witness_builder().pack()).build();
    tx.as_advanced_builder()
        .set_witnesses(sign_tx_with_witnesses(tx, witnesses, key).unwrap())
        .build()
}

pub fn get_dummy_witness_builder() -> WitnessArgsBuilder {
    let zero_lock: Bytes = {
        let buf = [0u8; SIGNATURE_SIZE];
        buf.serialize()
    };

    WitnessArgsBuilder::default().lock(zero_lock.pack_some())
}

pub fn sign_tx_with_witnesses(tx: TransactionView, witnesses: BytesVec, key: &Privkey) -> Option<Vec<packed::Bytes>> {
    let witnesses_len = witnesses.len();

    let mut blake2b = new_blake2b();
    blake2b.update(&tx.hash().raw_data());

    // digest the first witness
    let signature_witness = witnesses.get(0)?.raw_data();
    WitnessArgsReader::verify(&signature_witness, false).ok()?;

    let signature_witness_len = signature_witness.len() as u64;
    blake2b.update(&signature_witness_len.to_le_bytes());
    blake2b.update(&signature_witness);

    // digest rest witnesses
    (1..witnesses_len).for_each(|n| {
        let witness = witnesses.get(n).unwrap().raw_data();
        let witness_len = witness.len() as u64;
        blake2b.update(&witness_len.to_le_bytes());
        blake2b.update(&witness);
    });

    let mut message = [0u8; 32];
    blake2b.finalize(&mut message);
    let message = H256::from(message);

    let sig = key.sign_recoverable(&message).expect("sign");

    let mut signed_witnesses: Vec<packed::Bytes> = Vec::new();
    signed_witnesses.push(
        WitnessArgs::new_unchecked(signature_witness)
            .as_builder()
            .lock(Bytes::from(sig.serialize()).pack_some())
            .pack(),
    );
    for i in 1..witnesses_len {
        signed_witnesses.push(witnesses.get(i)?);
    }

    Some(signed_witnesses)
}

pub fn with_secp256k1_cell_deps(mut builder: EnvironmentBuilder) -> (EnvironmentBuilder, OutPoint) {
    let secp256k1_bin = BUNDLED_CELL.get("specs/cells/secp256k1_blake160_sighash_all").unwrap();
    let secp256k1_out_point = builder.context.deploy_cell(secp256k1_bin.serialize());
    let secp256k1_dep = CellDep::new_builder().out_point(secp256k1_out_point.clone()).build();

    let secp256k1_data_bin = BUNDLED_CELL.get("specs/cells/secp256k1_data").unwrap();
    let secp256k1_data_out_point = builder.context.deploy_cell(secp256k1_data_bin.serialize());
    let secp256k1_data_dep = CellDep::new_builder().out_point(secp256k1_data_out_point).build();

    (builder.cell_dep(secp256k1_dep).cell_dep(secp256k1_data_dep), secp256k1_out_point)
}
