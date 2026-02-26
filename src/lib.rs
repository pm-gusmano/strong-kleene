#![no_std]
#![doc = include_str!("../README.md")]
pub mod trit;

pub use trit::{Trit, UnknownToBoolError};
