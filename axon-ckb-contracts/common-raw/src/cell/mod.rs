pub mod checker_bond;
pub mod checker_info;
pub mod code;
pub mod global_config;
pub mod muse_token;
pub mod sidechain_bond;
pub mod sidechain_config;
pub mod sidechain_fee;
pub mod sidechain_state;
pub mod sudt_token;
pub mod task;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum CellType {
    Unknown,
    Sudt,
    MuseToken,
    CheckerBond,
    CheckerInfo,
    SidechainConfig,
    SidechainState,
    Task,
    SidechainFee,
    SidechainBond,
    Code,
}
