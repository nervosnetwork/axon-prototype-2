use core::result::Result;

use crate::error::Error;

use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell_data, load_cell_lock, load_cell_type, load_header, QueryIter};
use common_raw::{
    cell::{
        checker_bond::{CheckerBondCellData, CheckerBondCellLockArgs},
        checker_info::{CheckerInfoCellData, CheckerInfoCellTypeArgs},
        code::{CodeCellLockArgs, CodeCellTypeArgs},
        global_config::GlobalConfigCellData,
        muse_token::MuseTokenData,
        sidechain_bond::{SidechainBondCellData, SidechainBondCellLockArgs},
        sidechain_config::{SidechainConfigCellData, SidechainConfigCellTypeArgs},
        sidechain_fee::{SidechainFeeCellData, SidechainFeeCellLockArgs},
        sidechain_state::{SidechainStateCellData, SidechainStateCellTypeArgs},
        sudt_token::SudtTokenData,
        task::{TaskCellData, TaskCellTypeArgs},
    },
    decode_u64, FromRaw,
};

pub struct CellOrigin(pub usize, pub Source);

pub trait LoadableCell {
    fn load(origin: CellOrigin) -> Result<Self, Error>
    where
        Self: Sized + FromRaw,
    {
        let CellOrigin(index, source) = origin;
        let data = load_cell_data(index, source)?;
        Self::from_raw(&data).ok_or(Error::Encoding)
    }
}

impl LoadableCell for CheckerBondCellData {}
impl LoadableCell for CheckerInfoCellData {}
impl LoadableCell for GlobalConfigCellData {}
impl LoadableCell for MuseTokenData {}
impl LoadableCell for SidechainBondCellData {}
impl LoadableCell for SidechainConfigCellData {}
impl LoadableCell for SidechainFeeCellData {}
impl LoadableCell for SidechainStateCellData {}
impl LoadableCell for SudtTokenData {}
impl LoadableCell for TaskCellData {}

pub trait LoadableLockArgs {
    fn load(origin: CellOrigin) -> Result<Self, Error>
    where
        Self: Sized + FromRaw,
    {
        let CellOrigin(index, source) = origin;
        let data = load_cell_lock(index, source)?.args();
        Self::from_raw(data.as_reader().raw_data()).ok_or(Error::Encoding)
    }
}

impl LoadableLockArgs for CheckerBondCellLockArgs {}
impl LoadableLockArgs for CodeCellLockArgs {}
impl LoadableLockArgs for SidechainBondCellLockArgs {}
impl LoadableLockArgs for SidechainFeeCellLockArgs {}

pub trait LoadableTypeArgs {
    fn load(origin: CellOrigin) -> Result<Self, Error>
    where
        Self: Sized + FromRaw,
    {
        let CellOrigin(index, source) = origin;
        let data = load_cell_type(index, source)?.ok_or(Error::TypeScriptMissed)?.args();
        Self::from_raw(data.as_reader().raw_data()).ok_or(Error::Encoding)
    }
}

impl LoadableLockArgs for CheckerInfoCellTypeArgs {}
impl LoadableTypeArgs for CodeCellTypeArgs {}
impl LoadableTypeArgs for SidechainConfigCellTypeArgs {}
impl LoadableTypeArgs for SidechainStateCellTypeArgs {}
impl LoadableTypeArgs for TaskCellTypeArgs {}

#[macro_export]
macro_rules! load_entities {
    ($($type: ty: $origin: expr), * $(,)?) => {
        (
            $(<$type>::load($origin)?,)*
        )
    }
}

pub fn has_sidechain_config_passed_update_interval(config: SidechainConfigCellData, origin: CellOrigin) -> Result<(), Error> {
    if config.checker_total_count >= config.checker_threshold {
        let CellOrigin(index, source) = origin;
        let config_timestamp = decode_u64(load_header(index, source)?.as_reader().raw().timestamp().raw_data()).unwrap();

        let time_proof = QueryIter::new(load_header, Source::HeaderDep).find(|header| {
            let timestamp = decode_u64(header.as_reader().raw().timestamp().raw_data()).unwrap();
            timestamp - config_timestamp >= config.update_interval.into()
        });

        if time_proof.is_none() {
            return Err(Error::SidechainConfigMismatch);
        }
    }

    Ok(())
}
