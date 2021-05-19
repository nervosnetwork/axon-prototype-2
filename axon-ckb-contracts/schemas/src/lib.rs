#![allow(dead_code)]
#![no_std]

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        extern crate std;
    }
}

pub mod common;

pub mod sidechain_state_cell;
