use crate::PureSudtTokenCell;

// which is standard sudt
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Default)]
pub struct SudtTokenCell {
    pub amount: u128,
}

PureSudtTokenCell!(SudtTokenCell);
