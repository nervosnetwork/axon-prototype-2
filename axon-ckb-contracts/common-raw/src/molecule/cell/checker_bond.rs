// Generated by Molecule 0.7.0

use super::super::common::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct CheckerBondCellLockArgs(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for CheckerBondCellLockArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for CheckerBondCellLockArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for CheckerBondCellLockArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "checker_lock_arg", self.checker_lock_arg())?;
        write!(f, ", {}: {}", "participated_chain_id", self.participated_chain_id())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for CheckerBondCellLockArgs {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            36, 0, 0, 0, 12, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        CheckerBondCellLockArgs::new_unchecked(v.into())
    }
}
impl CheckerBondCellLockArgs {
    pub const FIELD_COUNT: usize = 2;

    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }

    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }

    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }

    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }

    pub fn checker_lock_arg(&self) -> PubKeyHash {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        PubKeyHash::new_unchecked(self.0.slice(start..end))
    }

    pub fn participated_chain_id(&self) -> ChainIdList {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[12..]) as usize;
            ChainIdList::new_unchecked(self.0.slice(start..end))
        } else {
            ChainIdList::new_unchecked(self.0.slice(start..))
        }
    }

    pub fn as_reader<'r>(&'r self) -> CheckerBondCellLockArgsReader<'r> {
        CheckerBondCellLockArgsReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for CheckerBondCellLockArgs {
    type Builder = CheckerBondCellLockArgsBuilder;

    const NAME: &'static str = "CheckerBondCellLockArgs";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        CheckerBondCellLockArgs(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CheckerBondCellLockArgsReader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        CheckerBondCellLockArgsReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .checker_lock_arg(self.checker_lock_arg())
            .participated_chain_id(self.participated_chain_id())
    }
}
#[derive(Clone, Copy)]
pub struct CheckerBondCellLockArgsReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for CheckerBondCellLockArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for CheckerBondCellLockArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for CheckerBondCellLockArgsReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "checker_lock_arg", self.checker_lock_arg())?;
        write!(f, ", {}: {}", "participated_chain_id", self.participated_chain_id())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> CheckerBondCellLockArgsReader<'r> {
    pub const FIELD_COUNT: usize = 2;

    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }

    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }

    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }

    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }

    pub fn checker_lock_arg(&self) -> PubKeyHashReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        PubKeyHashReader::new_unchecked(&self.as_slice()[start..end])
    }

    pub fn participated_chain_id(&self) -> ChainIdListReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[12..]) as usize;
            ChainIdListReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            ChainIdListReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for CheckerBondCellLockArgsReader<'r> {
    type Entity = CheckerBondCellLockArgs;

    const NAME: &'static str = "CheckerBondCellLockArgsReader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        CheckerBondCellLockArgsReader(slice)
    }

    fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len == molecule::NUMBER_SIZE && Self::FIELD_COUNT == 0 {
            return Ok(());
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % molecule::NUMBER_SIZE != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        if slice_len < offset_first {
            return ve!(Self, HeaderIsBroken, offset_first, slice_len);
        }
        let field_count = offset_first / molecule::NUMBER_SIZE - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..offset_first]
            .chunks_exact(molecule::NUMBER_SIZE)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        PubKeyHashReader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        ChainIdListReader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct CheckerBondCellLockArgsBuilder {
    pub(crate) checker_lock_arg:      PubKeyHash,
    pub(crate) participated_chain_id: ChainIdList,
}
impl CheckerBondCellLockArgsBuilder {
    pub const FIELD_COUNT: usize = 2;

    pub fn checker_lock_arg(mut self, v: PubKeyHash) -> Self {
        self.checker_lock_arg = v;
        self
    }

    pub fn participated_chain_id(mut self, v: ChainIdList) -> Self {
        self.participated_chain_id = v;
        self
    }
}
impl molecule::prelude::Builder for CheckerBondCellLockArgsBuilder {
    type Entity = CheckerBondCellLockArgs;

    const NAME: &'static str = "CheckerBondCellLockArgsBuilder";

    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.checker_lock_arg.as_slice().len()
            + self.participated_chain_id.as_slice().len()
    }

    fn write<W: ::molecule::io::Write>(&self, writer: &mut W) -> ::molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.checker_lock_arg.as_slice().len();
        offsets.push(total_size);
        total_size += self.participated_chain_id.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.checker_lock_arg.as_slice())?;
        writer.write_all(self.participated_chain_id.as_slice())?;
        Ok(())
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        CheckerBondCellLockArgs::new_unchecked(inner.into())
    }
}
