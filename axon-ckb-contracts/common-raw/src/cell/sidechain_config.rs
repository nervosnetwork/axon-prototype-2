use molecule::prelude::*;

use crate::{
    common::*,
    molecule::{
        cell::sidechain_config::{
            CheckerInfoListBuilder, CheckerStatusReader, SidechainConfigCellBuilder, SidechainConfigCellReader,
            SidechainConfigCellTypeArgsBuilder, SidechainConfigCellTypeArgsReader, SidechainStatusReader,
        },
        common::{
            BlockHeightReader, ChainIdReader, CodeHashReader, HashTypeReader, PubKeyHashReader, Uint128Reader, Uint32Reader, Uint8Reader,
        },
    },
    FromRaw, Serialize,
};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[repr(u8)]
pub enum SidechainStatus {
    Relaying,
    Shutdown,
}

impl SidechainStatus {
    fn from_reader(reader: SidechainStatusReader) -> Option<Self> {
        let status = u8::from_raw(reader.raw_data())?;
        match status {
            0u8 => Some(Self::Relaying),
            1u8 => Some(Self::Shutdown),
            _ => None,
        }
    }
}

impl Default for SidechainStatus {
    fn default() -> Self {
        Self::Relaying
    }
}

impl FromRaw for SidechainStatus {
    fn from_raw(raw: &[u8]) -> Option<Self> {
        let reader = SidechainStatusReader::from_slice(raw).ok()?;
        Self::from_reader(reader)
    }
}

impl Serialize for SidechainStatus {
    type RawType = [u8; 1];

    fn serialize(&self) -> Self::RawType {
        (*self as u8).serialize()
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[repr(u8)]
pub enum CheckerStatus {
    Normal,
    Jailed,
}

impl CheckerStatus {
    fn from_reader(reader: CheckerStatusReader) -> Option<Self> {
        let status = u8::from_raw(reader.raw_data())?;
        match status {
            0u8 => Some(Self::Normal),
            1u8 => Some(Self::Jailed),
            _ => None,
        }
    }
}

impl Default for CheckerStatus {
    fn default() -> Self {
        Self::Normal
    }
}

impl FromRaw for CheckerStatus {
    fn from_raw(raw: &[u8]) -> Option<Self> {
        let reader = CheckerStatusReader::from_slice(raw).ok()?;
        Self::from_reader(reader)
    }
}

impl Serialize for CheckerStatus {
    type RawType = [u8; 1];

    fn serialize(&self) -> Self::RawType {
        (*self as u8).serialize()
    }
}

/**
    Sidechain Config Cell
    Data:
    Type:
        codehash: typeId
        hashtype: type
        args: chain_id(for lumos)
    Lock:
        codehash: A.S
        hashtype: data
        args: null
*/
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainConfigCell {
    pub sidechain_status: SidechainStatus,

    pub commit_threshold:    u32,
    pub challenge_threshold: u32,

    pub checker_normal_count: u32,
    pub checker_threshold:    u32,
    pub checker_total_count:  u32,
    pub activated_checkers:   Vec<PubKeyHash>,
    pub jailed_checkers:      Vec<PubKeyHash>,

    pub refresh_punish_points:             u32,
    pub refresh_punish_release_points:     u32,
    pub refresh_punish_threshold:          u32,
    pub refresh_sidechain_height_interval: BlockHeight,

    pub check_data_size_limit: u128,
    pub check_fee_rate: u32,
    pub minimal_bond: u128,
    pub parallel_job_upper_bond: u8,
    pub parallel_job_maximal_height_range: BlockHeight,

    pub admin_lock_arg:    PubKeyHash,
    pub collator_lock_arg: PubKeyHash,

    pub bond_sudt_typescript_codehash: CodeHash,
    pub bond_sudt_typescript_hashtype: HashType,
}

impl FromRaw for SidechainConfigCell {
    fn from_raw(cell_raw_data: &[u8]) -> Option<SidechainConfigCell> {
        let reader = SidechainConfigCellReader::from_slice(cell_raw_data).ok()?;

        let sidechain_status = SidechainStatus::from_reader(reader.sidechain_status())?;

        let commit_threshold = u32::from_raw(reader.commit_threshold().raw_data())?;
        let challenge_threshold = u32::from_raw(reader.challenge_threshold().raw_data())?;

        let checker_normal_count = u32::from_raw(reader.checker_normal_count().raw_data())?;
        let checker_threshold = u32::from_raw(reader.checker_threshold().raw_data())?;
        let checker_total_count = u32::from_raw(reader.checker_total_count().raw_data())?;

        let activated_checkers_reader = reader.activated_checkers();
        let activated_checkers_len = activated_checkers_reader.len();
        let mut activated_checkers = Vec::with_capacity(activated_checkers_len);

        for i in 0..activated_checkers_len {
            let result = PubKeyHash::from_raw(activated_checkers_reader.get_unchecked(i).raw_data())?;
            activated_checkers.push(result);
        }

        let jailed_checkers_reader = reader.activated_checkers();
        let jailed_checkers_len = jailed_checkers_reader.len();
        let mut jailed_checkers = Vec::with_capacity(jailed_checkers_len);

        for i in 0..jailed_checkers_len {
            let result = PubKeyHash::from_raw(jailed_checkers_reader.get_unchecked(i).raw_data())?;
            jailed_checkers.push(result);
        }

        let refresh_punish_points = u32::from_raw(reader.refresh_punish_points().raw_data())?;
        let refresh_punish_release_points = u32::from_raw(reader.refresh_punish_release_points().raw_data())?;
        let refresh_punish_threshold = u32::from_raw(reader.refresh_punish_threshold().raw_data())?;
        let refresh_sidechain_height_interval = BlockHeight::from_raw(reader.refresh_sidechain_height_interval().raw_data())?;

        let check_data_size_limit = u128::from_raw(reader.check_data_size_limit().raw_data())?;
        let check_fee_rate = u32::from_raw(reader.check_fee_rate().raw_data())?;
        let minimal_bond = u128::from_raw(reader.minimal_bond().raw_data())?;
        let parallel_job_upper_bond = u8::from_raw(reader.parallel_job_upper_bond().raw_data())?;
        let parallel_job_maximal_height_range = BlockHeight::from_raw(reader.parallel_job_maximal_height_range().raw_data())?;

        let mut admin_lock_arg: PubKeyHash = [0u8; 20];
        admin_lock_arg.copy_from_slice(reader.admin_lock_arg().raw_data());

        let mut collator_lock_arg: PubKeyHash = [0u8; 20];
        collator_lock_arg.copy_from_slice(reader.collator_lock_arg().raw_data());

        let mut bond_sudt_typescript_codehash: CodeHash = [0u8; 32];
        bond_sudt_typescript_codehash.copy_from_slice(reader.bond_sudt_typescript_codehash().raw_data());

        let bond_sudt_typescript_hashtype = HashType::from_raw(reader.bond_sudt_typescript_hashtype().raw_data())?;

        Some(SidechainConfigCell {
            sidechain_status,

            commit_threshold,
            challenge_threshold,

            checker_normal_count,
            checker_threshold,
            checker_total_count,
            activated_checkers,
            jailed_checkers,
            refresh_punish_points,
            refresh_punish_release_points,
            refresh_punish_threshold,
            refresh_sidechain_height_interval,

            check_data_size_limit,
            check_fee_rate,
            minimal_bond,
            parallel_job_upper_bond,
            parallel_job_maximal_height_range,

            admin_lock_arg,
            collator_lock_arg,

            bond_sudt_typescript_codehash,
            bond_sudt_typescript_hashtype,
        })
    }
}

impl Serialize for SidechainConfigCell {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let sidechain_status = SidechainStatusReader::new_unchecked(&self.sidechain_status.serialize()).to_entity();

        let commit_threshold = Uint32Reader::new_unchecked(&self.commit_threshold.serialize()).to_entity();
        let challenge_threshold = Uint32Reader::new_unchecked(&self.challenge_threshold.serialize()).to_entity();

        let checker_normal_count = Uint32Reader::new_unchecked(&self.checker_normal_count.serialize()).to_entity();
        let checker_threshold = Uint32Reader::new_unchecked(&self.checker_threshold.serialize()).to_entity();
        let checker_total_count = Uint32Reader::new_unchecked(&self.checker_total_count.serialize()).to_entity();

        let mut activated_checkers = CheckerInfoListBuilder::default();
        for checker in &self.activated_checkers {
            activated_checkers = activated_checkers.push(PubKeyHashReader::new_unchecked(checker).to_entity());
        }

        let mut jailed_checkers = CheckerInfoListBuilder::default();
        for checker in &self.jailed_checkers {
            jailed_checkers = jailed_checkers.push(PubKeyHashReader::new_unchecked(checker).to_entity());
        }

        let refresh_punish_points = Uint32Reader::new_unchecked(&self.refresh_punish_points.serialize()).to_entity();
        let refresh_punish_release_points = Uint32Reader::new_unchecked(&self.refresh_punish_release_points.serialize()).to_entity();
        let refresh_punish_threshold = Uint32Reader::new_unchecked(&self.refresh_punish_threshold.serialize()).to_entity();
        let refresh_sidechain_height_interval =
            BlockHeightReader::new_unchecked(&self.refresh_sidechain_height_interval.serialize()).to_entity();

        let check_data_size_limit = Uint128Reader::new_unchecked(&self.check_data_size_limit.serialize()).to_entity();
        let check_fee_rate = Uint32Reader::new_unchecked(&self.check_fee_rate.serialize()).to_entity();
        let minimal_bond = Uint128Reader::new_unchecked(&self.minimal_bond.serialize()).to_entity();
        let parallel_job_upper_bond = Uint8Reader::new_unchecked(&self.parallel_job_upper_bond.serialize()).to_entity();
        let parallel_job_maximal_height_range =
            BlockHeightReader::new_unchecked(&self.parallel_job_maximal_height_range.serialize()).to_entity();

        let admin_lock_arg = PubKeyHashReader::new_unchecked(&self.admin_lock_arg).to_entity();
        let collator_lock_arg = PubKeyHashReader::new_unchecked(&self.collator_lock_arg).to_entity();

        let bond_sudt_typescript_codehash = CodeHashReader::new_unchecked(&self.bond_sudt_typescript_codehash).to_entity();
        let bond_sudt_typescript_hashtype = HashTypeReader::new_unchecked(&self.bond_sudt_typescript_hashtype.serialize()).to_entity();

        let builder = SidechainConfigCellBuilder::default()
            .sidechain_status(sidechain_status)
            .commit_threshold(commit_threshold)
            .challenge_threshold(challenge_threshold)
            .checker_normal_count(checker_normal_count)
            .checker_threshold(checker_threshold)
            .checker_total_count(checker_total_count)
            .activated_checkers(activated_checkers.build())
            .jailed_checkers(jailed_checkers.build())
            .refresh_punish_points(refresh_punish_points)
            .refresh_punish_release_points(refresh_punish_release_points)
            .refresh_punish_threshold(refresh_punish_threshold)
            .refresh_sidechain_height_interval(refresh_sidechain_height_interval)
            .check_data_size_limit(check_data_size_limit)
            .check_fee_rate(check_fee_rate)
            .minimal_bond(minimal_bond)
            .parallel_job_upper_bond(parallel_job_upper_bond)
            .parallel_job_maximal_height_range(parallel_job_maximal_height_range)
            .admin_lock_arg(admin_lock_arg)
            .collator_lock_arg(collator_lock_arg)
            .bond_sudt_typescript_codehash(bond_sudt_typescript_codehash)
            .bond_sudt_typescript_hashtype(bond_sudt_typescript_hashtype);

        let mut buf = Vec::new();
        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing SidechainConfigCell");
        buf
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SidechainConfigCellTypeArgs {
    pub chain_id: u8, // TODO: Change to ChainId.
}

impl FromRaw for SidechainConfigCellTypeArgs {
    fn from_raw(arg_raw_data: &[u8]) -> Option<SidechainConfigCellTypeArgs> {
        let reader = SidechainConfigCellTypeArgsReader::from_slice(arg_raw_data).ok()?;

        let chain_id = ChainId::from_raw(reader.chain_id().raw_data())? as u8;

        Some(SidechainConfigCellTypeArgs { chain_id })
    }
}

impl Serialize for SidechainConfigCellTypeArgs {
    type RawType = Vec<u8>;

    fn serialize(&self) -> Self::RawType {
        let chain_id = ChainIdReader::new_unchecked(&(self.chain_id as ChainId).serialize()).to_entity();

        let builder = SidechainConfigCellTypeArgsBuilder::default().chain_id(chain_id);

        let mut buf = Vec::new();
        builder
            .write(&mut buf)
            .expect("Unable to write buffer while serializing SidechainConfigCellTypeArgs");
        buf
    }
}
