use molecule::prelude::*;

use crate::{
    common::ChainId,
    molecule::{
        cell::sidechain_registry::{SidechainRegistryCellBuilder, SidechainRegistryCellReader},
        common::{ChainIdListBuilder, ChainIdReader},
    },
    FromRaw, Serialize,
};

#[derive(Debug, Clone, Default)]
struct SidechainRegistryCell {
    chain_ids: Vec<ChainId>,
}

impl FromRaw for SidechainRegistryCell {
    fn from_raw(cell_raw_data: &[u8]) -> Option<Self> {
        let reader = SidechainRegistryCellReader::from_slice(cell_raw_data).ok()?;
        let mut chain_ids = Vec::new();
        for chain_id_reader in reader.chain_ids().iter() {
            let chain_id = ChainId::from_raw(chain_id_reader.raw_data())?;
            chain_ids.push(chain_id);
        }
        Some(Self { chain_ids })
    }
}

impl Serialize for SidechainRegistryCell {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let mut builder = SidechainRegistryCellBuilder::default();

        let chain_id_list = self
            .chain_ids
            .iter()
            .fold(ChainIdListBuilder::default(), |chain_id_list, chain_id| {
                chain_id_list.push(ChainIdReader::new_unchecked(&chain_id.serialize()).to_entity())
            })
            .build();

        builder = builder.chain_ids(chain_id_list);
        let mut buf = Vec::new();
        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing SidechainRegistryCell");
        buf
    }
}
