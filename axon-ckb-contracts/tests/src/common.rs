use ckb_tool::ckb_types::{bytes::Bytes, packed, packed::*, prelude::*};

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
