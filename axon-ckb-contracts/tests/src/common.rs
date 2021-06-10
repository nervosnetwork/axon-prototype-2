use super::Loader;
use ckb_testtool::context::Context;
use ckb_tool::ckb_types::{bytes::Bytes, core::TransactionBuilder, packed, packed::*, prelude::*};

pub fn new_cell_output(capacity: u64, script: &Script) -> CellOutput {
    CellOutput::new_builder().capacity(capacity.pack()).lock(script.clone()).build()
}

pub fn new_type_cell_output(capacity: u64, lock: &Script, type_: &Script) -> CellOutput {
    CellOutput::new_builder()
        .capacity(capacity.pack())
        .lock(lock.clone())
        .type_(type_.clone().pack_some())
        .build()
}

pub fn create_input(context: &mut Context, output: CellOutput, data: Bytes) -> CellInput {
    CellInput::new_builder().previous_output(context.create_cell(output, data)).build()
}

pub fn create_dep(context: &mut Context, output: CellOutput, data: Bytes) -> CellDep {
    CellDep::new_builder().out_point(context.create_cell(output, data)).build()
}

pub fn load_contract(context: &mut Context, builder: TransactionBuilder, name: &str) -> (TransactionBuilder, OutPoint) {
    let contract_bin: Bytes = Loader::default().load_binary(name);
    let out_point = context.deploy_cell(contract_bin);

    (
        builder.cell_dep(CellDep::new_builder().out_point(out_point.clone()).build()),
        out_point,
    )
}

pub fn load_script(context: &mut Context, builder: TransactionBuilder, name: &str) -> (TransactionBuilder, Script) {
    let (builder, code) = load_contract(context, builder, name);
    let script = context.build_script(&code, Bytes::new()).expect("script");

    (builder, script)
}

pub trait SerializableRef {
    fn serialize(&self) -> Bytes;
}

impl<T: AsRef<[u8]>> SerializableRef for T {
    fn serialize(&self) -> Bytes {
        self.as_ref().to_vec().into()
    }
}

pub trait SerializableSerialize {
    fn serialize(&self) -> Bytes;
}

impl<T: common_raw::Serialize> SerializableSerialize for T {
    fn serialize(&self) -> Bytes {
        self.serialize().serialize()
    }
}

pub trait IntoOpt<T> {
    fn pack_some(self) -> T;
}

impl<T, N> IntoOpt<T> for N
where
    T: Entity,
    Option<N>: Pack<T>,
{
    fn pack_some(self) -> T {
        Some(self).pack()
    }
}

pub trait PackableEntity {
    fn pack(&self) -> packed::Bytes;
}

impl<T: Entity> PackableEntity for T {
    fn pack(&self) -> packed::Bytes {
        self.as_bytes().pack()
    }
}

pub trait PackableBuilder {
    fn pack(&self) -> packed::Bytes;
}

impl<T: Builder> PackableBuilder for T {
    fn pack(&self) -> packed::Bytes {
        self.build().pack()
    }
}

pub trait AsBytesBuilder {
    fn as_bytes(&self) -> Bytes;
}

impl<T: Builder> AsBytesBuilder for T {
    fn as_bytes(&self) -> Bytes {
        self.build().as_bytes()
    }
}
