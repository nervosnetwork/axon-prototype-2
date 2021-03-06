// Generated by Molecule 0.7.0

use super::super::common::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct GlobalConfigCell(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for GlobalConfigCell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for GlobalConfigCell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for GlobalConfigCell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "admin_lock_arg", self.admin_lock_arg())?;
        write!(
            f,
            ", {}: {}",
            "checker_info_cell_type_codehash",
            self.checker_info_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "checker_info_cell_type_hashtype",
            self.checker_info_cell_type_hashtype()
        )?;
        write!(
            f,
            ", {}: {}",
            "checker_bond_cell_type_codehash",
            self.checker_bond_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "checker_bond_cell_type_hashtype",
            self.checker_bond_cell_type_hashtype()
        )?;
        write!(f, ", {}: {}", "code_cell_type_codehash", self.code_cell_type_codehash())?;
        write!(f, ", {}: {}", "code_cell_type_hashtype", self.code_cell_type_hashtype())?;
        write!(
            f,
            ", {}: {}",
            "sidechain_bond_cell_type_codehash",
            self.sidechain_bond_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_bond_cell_type_hashtype",
            self.sidechain_bond_cell_type_hashtype()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_config_cell_type_codehash",
            self.sidechain_config_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_config_cell_type_hashtype",
            self.sidechain_config_cell_type_hashtype()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_fee_cell_type_codehash",
            self.sidechain_fee_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_fee_cell_type_hashtype",
            self.sidechain_fee_cell_type_hashtype()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_registry_cell_type_codehash",
            self.sidechain_registry_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_registry_cell_type_hashtype",
            self.sidechain_registry_cell_type_hashtype()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_state_cell_type_codehash",
            self.sidechain_state_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_state_cell_type_hashtype",
            self.sidechain_state_cell_type_hashtype()
        )?;
        write!(f, ", {}: {}", "task_cell_type_codehash", self.task_cell_type_codehash())?;
        write!(f, ", {}: {}", "task_cell_type_hashtype", self.task_cell_type_hashtype())?;
        write!(f, " }}")
    }
}
impl ::core::default::Default for GlobalConfigCell {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        GlobalConfigCell::new_unchecked(v.into())
    }
}
impl GlobalConfigCell {
    pub const FIELD_COUNT: usize = 19;
    pub const FIELD_SIZES: [usize; 19] = [20, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1];
    pub const TOTAL_SIZE: usize = 317;

    pub fn admin_lock_arg(&self) -> PubKeyHash {
        PubKeyHash::new_unchecked(self.0.slice(0..20))
    }

    pub fn checker_info_cell_type_codehash(&self) -> CodeHash {
        CodeHash::new_unchecked(self.0.slice(20..52))
    }

    pub fn checker_info_cell_type_hashtype(&self) -> HashType {
        HashType::new_unchecked(self.0.slice(52..53))
    }

    pub fn checker_bond_cell_type_codehash(&self) -> CodeHash {
        CodeHash::new_unchecked(self.0.slice(53..85))
    }

    pub fn checker_bond_cell_type_hashtype(&self) -> HashType {
        HashType::new_unchecked(self.0.slice(85..86))
    }

    pub fn code_cell_type_codehash(&self) -> CodeHash {
        CodeHash::new_unchecked(self.0.slice(86..118))
    }

    pub fn code_cell_type_hashtype(&self) -> HashType {
        HashType::new_unchecked(self.0.slice(118..119))
    }

    pub fn sidechain_bond_cell_type_codehash(&self) -> CodeHash {
        CodeHash::new_unchecked(self.0.slice(119..151))
    }

    pub fn sidechain_bond_cell_type_hashtype(&self) -> HashType {
        HashType::new_unchecked(self.0.slice(151..152))
    }

    pub fn sidechain_config_cell_type_codehash(&self) -> CodeHash {
        CodeHash::new_unchecked(self.0.slice(152..184))
    }

    pub fn sidechain_config_cell_type_hashtype(&self) -> HashType {
        HashType::new_unchecked(self.0.slice(184..185))
    }

    pub fn sidechain_fee_cell_type_codehash(&self) -> CodeHash {
        CodeHash::new_unchecked(self.0.slice(185..217))
    }

    pub fn sidechain_fee_cell_type_hashtype(&self) -> HashType {
        HashType::new_unchecked(self.0.slice(217..218))
    }

    pub fn sidechain_registry_cell_type_codehash(&self) -> CodeHash {
        CodeHash::new_unchecked(self.0.slice(218..250))
    }

    pub fn sidechain_registry_cell_type_hashtype(&self) -> HashType {
        HashType::new_unchecked(self.0.slice(250..251))
    }

    pub fn sidechain_state_cell_type_codehash(&self) -> CodeHash {
        CodeHash::new_unchecked(self.0.slice(251..283))
    }

    pub fn sidechain_state_cell_type_hashtype(&self) -> HashType {
        HashType::new_unchecked(self.0.slice(283..284))
    }

    pub fn task_cell_type_codehash(&self) -> CodeHash {
        CodeHash::new_unchecked(self.0.slice(284..316))
    }

    pub fn task_cell_type_hashtype(&self) -> HashType {
        HashType::new_unchecked(self.0.slice(316..317))
    }

    pub fn as_reader<'r>(&'r self) -> GlobalConfigCellReader<'r> {
        GlobalConfigCellReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for GlobalConfigCell {
    type Builder = GlobalConfigCellBuilder;

    const NAME: &'static str = "GlobalConfigCell";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        GlobalConfigCell(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        GlobalConfigCellReader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        GlobalConfigCellReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .admin_lock_arg(self.admin_lock_arg())
            .checker_info_cell_type_codehash(self.checker_info_cell_type_codehash())
            .checker_info_cell_type_hashtype(self.checker_info_cell_type_hashtype())
            .checker_bond_cell_type_codehash(self.checker_bond_cell_type_codehash())
            .checker_bond_cell_type_hashtype(self.checker_bond_cell_type_hashtype())
            .code_cell_type_codehash(self.code_cell_type_codehash())
            .code_cell_type_hashtype(self.code_cell_type_hashtype())
            .sidechain_bond_cell_type_codehash(self.sidechain_bond_cell_type_codehash())
            .sidechain_bond_cell_type_hashtype(self.sidechain_bond_cell_type_hashtype())
            .sidechain_config_cell_type_codehash(self.sidechain_config_cell_type_codehash())
            .sidechain_config_cell_type_hashtype(self.sidechain_config_cell_type_hashtype())
            .sidechain_fee_cell_type_codehash(self.sidechain_fee_cell_type_codehash())
            .sidechain_fee_cell_type_hashtype(self.sidechain_fee_cell_type_hashtype())
            .sidechain_registry_cell_type_codehash(self.sidechain_registry_cell_type_codehash())
            .sidechain_registry_cell_type_hashtype(self.sidechain_registry_cell_type_hashtype())
            .sidechain_state_cell_type_codehash(self.sidechain_state_cell_type_codehash())
            .sidechain_state_cell_type_hashtype(self.sidechain_state_cell_type_hashtype())
            .task_cell_type_codehash(self.task_cell_type_codehash())
            .task_cell_type_hashtype(self.task_cell_type_hashtype())
    }
}
#[derive(Clone, Copy)]
pub struct GlobalConfigCellReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for GlobalConfigCellReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for GlobalConfigCellReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for GlobalConfigCellReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "admin_lock_arg", self.admin_lock_arg())?;
        write!(
            f,
            ", {}: {}",
            "checker_info_cell_type_codehash",
            self.checker_info_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "checker_info_cell_type_hashtype",
            self.checker_info_cell_type_hashtype()
        )?;
        write!(
            f,
            ", {}: {}",
            "checker_bond_cell_type_codehash",
            self.checker_bond_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "checker_bond_cell_type_hashtype",
            self.checker_bond_cell_type_hashtype()
        )?;
        write!(f, ", {}: {}", "code_cell_type_codehash", self.code_cell_type_codehash())?;
        write!(f, ", {}: {}", "code_cell_type_hashtype", self.code_cell_type_hashtype())?;
        write!(
            f,
            ", {}: {}",
            "sidechain_bond_cell_type_codehash",
            self.sidechain_bond_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_bond_cell_type_hashtype",
            self.sidechain_bond_cell_type_hashtype()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_config_cell_type_codehash",
            self.sidechain_config_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_config_cell_type_hashtype",
            self.sidechain_config_cell_type_hashtype()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_fee_cell_type_codehash",
            self.sidechain_fee_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_fee_cell_type_hashtype",
            self.sidechain_fee_cell_type_hashtype()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_registry_cell_type_codehash",
            self.sidechain_registry_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_registry_cell_type_hashtype",
            self.sidechain_registry_cell_type_hashtype()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_state_cell_type_codehash",
            self.sidechain_state_cell_type_codehash()
        )?;
        write!(
            f,
            ", {}: {}",
            "sidechain_state_cell_type_hashtype",
            self.sidechain_state_cell_type_hashtype()
        )?;
        write!(f, ", {}: {}", "task_cell_type_codehash", self.task_cell_type_codehash())?;
        write!(f, ", {}: {}", "task_cell_type_hashtype", self.task_cell_type_hashtype())?;
        write!(f, " }}")
    }
}
impl<'r> GlobalConfigCellReader<'r> {
    pub const FIELD_COUNT: usize = 19;
    pub const FIELD_SIZES: [usize; 19] = [20, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1];
    pub const TOTAL_SIZE: usize = 317;

    pub fn admin_lock_arg(&self) -> PubKeyHashReader<'r> {
        PubKeyHashReader::new_unchecked(&self.as_slice()[0..20])
    }

    pub fn checker_info_cell_type_codehash(&self) -> CodeHashReader<'r> {
        CodeHashReader::new_unchecked(&self.as_slice()[20..52])
    }

    pub fn checker_info_cell_type_hashtype(&self) -> HashTypeReader<'r> {
        HashTypeReader::new_unchecked(&self.as_slice()[52..53])
    }

    pub fn checker_bond_cell_type_codehash(&self) -> CodeHashReader<'r> {
        CodeHashReader::new_unchecked(&self.as_slice()[53..85])
    }

    pub fn checker_bond_cell_type_hashtype(&self) -> HashTypeReader<'r> {
        HashTypeReader::new_unchecked(&self.as_slice()[85..86])
    }

    pub fn code_cell_type_codehash(&self) -> CodeHashReader<'r> {
        CodeHashReader::new_unchecked(&self.as_slice()[86..118])
    }

    pub fn code_cell_type_hashtype(&self) -> HashTypeReader<'r> {
        HashTypeReader::new_unchecked(&self.as_slice()[118..119])
    }

    pub fn sidechain_bond_cell_type_codehash(&self) -> CodeHashReader<'r> {
        CodeHashReader::new_unchecked(&self.as_slice()[119..151])
    }

    pub fn sidechain_bond_cell_type_hashtype(&self) -> HashTypeReader<'r> {
        HashTypeReader::new_unchecked(&self.as_slice()[151..152])
    }

    pub fn sidechain_config_cell_type_codehash(&self) -> CodeHashReader<'r> {
        CodeHashReader::new_unchecked(&self.as_slice()[152..184])
    }

    pub fn sidechain_config_cell_type_hashtype(&self) -> HashTypeReader<'r> {
        HashTypeReader::new_unchecked(&self.as_slice()[184..185])
    }

    pub fn sidechain_fee_cell_type_codehash(&self) -> CodeHashReader<'r> {
        CodeHashReader::new_unchecked(&self.as_slice()[185..217])
    }

    pub fn sidechain_fee_cell_type_hashtype(&self) -> HashTypeReader<'r> {
        HashTypeReader::new_unchecked(&self.as_slice()[217..218])
    }

    pub fn sidechain_registry_cell_type_codehash(&self) -> CodeHashReader<'r> {
        CodeHashReader::new_unchecked(&self.as_slice()[218..250])
    }

    pub fn sidechain_registry_cell_type_hashtype(&self) -> HashTypeReader<'r> {
        HashTypeReader::new_unchecked(&self.as_slice()[250..251])
    }

    pub fn sidechain_state_cell_type_codehash(&self) -> CodeHashReader<'r> {
        CodeHashReader::new_unchecked(&self.as_slice()[251..283])
    }

    pub fn sidechain_state_cell_type_hashtype(&self) -> HashTypeReader<'r> {
        HashTypeReader::new_unchecked(&self.as_slice()[283..284])
    }

    pub fn task_cell_type_codehash(&self) -> CodeHashReader<'r> {
        CodeHashReader::new_unchecked(&self.as_slice()[284..316])
    }

    pub fn task_cell_type_hashtype(&self) -> HashTypeReader<'r> {
        HashTypeReader::new_unchecked(&self.as_slice()[316..317])
    }
}
impl<'r> molecule::prelude::Reader<'r> for GlobalConfigCellReader<'r> {
    type Entity = GlobalConfigCell;

    const NAME: &'static str = "GlobalConfigCellReader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        GlobalConfigCellReader(slice)
    }

    fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len != Self::TOTAL_SIZE {
            return ve!(Self, TotalSizeNotMatch, Self::TOTAL_SIZE, slice_len);
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct GlobalConfigCellBuilder {
    pub(crate) admin_lock_arg: PubKeyHash,
    pub(crate) checker_info_cell_type_codehash: CodeHash,
    pub(crate) checker_info_cell_type_hashtype: HashType,
    pub(crate) checker_bond_cell_type_codehash: CodeHash,
    pub(crate) checker_bond_cell_type_hashtype: HashType,
    pub(crate) code_cell_type_codehash: CodeHash,
    pub(crate) code_cell_type_hashtype: HashType,
    pub(crate) sidechain_bond_cell_type_codehash: CodeHash,
    pub(crate) sidechain_bond_cell_type_hashtype: HashType,
    pub(crate) sidechain_config_cell_type_codehash: CodeHash,
    pub(crate) sidechain_config_cell_type_hashtype: HashType,
    pub(crate) sidechain_fee_cell_type_codehash: CodeHash,
    pub(crate) sidechain_fee_cell_type_hashtype: HashType,
    pub(crate) sidechain_registry_cell_type_codehash: CodeHash,
    pub(crate) sidechain_registry_cell_type_hashtype: HashType,
    pub(crate) sidechain_state_cell_type_codehash: CodeHash,
    pub(crate) sidechain_state_cell_type_hashtype: HashType,
    pub(crate) task_cell_type_codehash: CodeHash,
    pub(crate) task_cell_type_hashtype: HashType,
}
impl GlobalConfigCellBuilder {
    pub const FIELD_COUNT: usize = 19;
    pub const FIELD_SIZES: [usize; 19] = [20, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1];
    pub const TOTAL_SIZE: usize = 317;

    pub fn admin_lock_arg(mut self, v: PubKeyHash) -> Self {
        self.admin_lock_arg = v;
        self
    }

    pub fn checker_info_cell_type_codehash(mut self, v: CodeHash) -> Self {
        self.checker_info_cell_type_codehash = v;
        self
    }

    pub fn checker_info_cell_type_hashtype(mut self, v: HashType) -> Self {
        self.checker_info_cell_type_hashtype = v;
        self
    }

    pub fn checker_bond_cell_type_codehash(mut self, v: CodeHash) -> Self {
        self.checker_bond_cell_type_codehash = v;
        self
    }

    pub fn checker_bond_cell_type_hashtype(mut self, v: HashType) -> Self {
        self.checker_bond_cell_type_hashtype = v;
        self
    }

    pub fn code_cell_type_codehash(mut self, v: CodeHash) -> Self {
        self.code_cell_type_codehash = v;
        self
    }

    pub fn code_cell_type_hashtype(mut self, v: HashType) -> Self {
        self.code_cell_type_hashtype = v;
        self
    }

    pub fn sidechain_bond_cell_type_codehash(mut self, v: CodeHash) -> Self {
        self.sidechain_bond_cell_type_codehash = v;
        self
    }

    pub fn sidechain_bond_cell_type_hashtype(mut self, v: HashType) -> Self {
        self.sidechain_bond_cell_type_hashtype = v;
        self
    }

    pub fn sidechain_config_cell_type_codehash(mut self, v: CodeHash) -> Self {
        self.sidechain_config_cell_type_codehash = v;
        self
    }

    pub fn sidechain_config_cell_type_hashtype(mut self, v: HashType) -> Self {
        self.sidechain_config_cell_type_hashtype = v;
        self
    }

    pub fn sidechain_fee_cell_type_codehash(mut self, v: CodeHash) -> Self {
        self.sidechain_fee_cell_type_codehash = v;
        self
    }

    pub fn sidechain_fee_cell_type_hashtype(mut self, v: HashType) -> Self {
        self.sidechain_fee_cell_type_hashtype = v;
        self
    }

    pub fn sidechain_registry_cell_type_codehash(mut self, v: CodeHash) -> Self {
        self.sidechain_registry_cell_type_codehash = v;
        self
    }

    pub fn sidechain_registry_cell_type_hashtype(mut self, v: HashType) -> Self {
        self.sidechain_registry_cell_type_hashtype = v;
        self
    }

    pub fn sidechain_state_cell_type_codehash(mut self, v: CodeHash) -> Self {
        self.sidechain_state_cell_type_codehash = v;
        self
    }

    pub fn sidechain_state_cell_type_hashtype(mut self, v: HashType) -> Self {
        self.sidechain_state_cell_type_hashtype = v;
        self
    }

    pub fn task_cell_type_codehash(mut self, v: CodeHash) -> Self {
        self.task_cell_type_codehash = v;
        self
    }

    pub fn task_cell_type_hashtype(mut self, v: HashType) -> Self {
        self.task_cell_type_hashtype = v;
        self
    }
}
impl molecule::prelude::Builder for GlobalConfigCellBuilder {
    type Entity = GlobalConfigCell;

    const NAME: &'static str = "GlobalConfigCellBuilder";

    fn expected_length(&self) -> usize {
        Self::TOTAL_SIZE
    }

    fn write<W: ::molecule::io::Write>(&self, writer: &mut W) -> ::molecule::io::Result<()> {
        writer.write_all(self.admin_lock_arg.as_slice())?;
        writer.write_all(self.checker_info_cell_type_codehash.as_slice())?;
        writer.write_all(self.checker_info_cell_type_hashtype.as_slice())?;
        writer.write_all(self.checker_bond_cell_type_codehash.as_slice())?;
        writer.write_all(self.checker_bond_cell_type_hashtype.as_slice())?;
        writer.write_all(self.code_cell_type_codehash.as_slice())?;
        writer.write_all(self.code_cell_type_hashtype.as_slice())?;
        writer.write_all(self.sidechain_bond_cell_type_codehash.as_slice())?;
        writer.write_all(self.sidechain_bond_cell_type_hashtype.as_slice())?;
        writer.write_all(self.sidechain_config_cell_type_codehash.as_slice())?;
        writer.write_all(self.sidechain_config_cell_type_hashtype.as_slice())?;
        writer.write_all(self.sidechain_fee_cell_type_codehash.as_slice())?;
        writer.write_all(self.sidechain_fee_cell_type_hashtype.as_slice())?;
        writer.write_all(self.sidechain_registry_cell_type_codehash.as_slice())?;
        writer.write_all(self.sidechain_registry_cell_type_hashtype.as_slice())?;
        writer.write_all(self.sidechain_state_cell_type_codehash.as_slice())?;
        writer.write_all(self.sidechain_state_cell_type_hashtype.as_slice())?;
        writer.write_all(self.task_cell_type_codehash.as_slice())?;
        writer.write_all(self.task_cell_type_hashtype.as_slice())?;
        Ok(())
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        GlobalConfigCell::new_unchecked(inner.into())
    }
}
