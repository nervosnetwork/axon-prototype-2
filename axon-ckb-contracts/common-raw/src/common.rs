pub type BlockHeader = [u8; 32];
pub type BlockHeight = u128;
pub type CodeHash = [u8; 32];

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[repr(u8)]
pub enum HashType {
    Data,
    Type,
}

pub type MerkleHash = [u8; 32];
pub type PubKeyHash = [u8; 20];
pub type ScriptHash = [u8; 32];

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
struct BlockSlice {
    from: BlockHeight,
    to:   BlockHeight,
}

pub type ChainId = u32;

pub type RandomSeed = [u8; 32];
pub type CommittedHash = [u8; 32];
