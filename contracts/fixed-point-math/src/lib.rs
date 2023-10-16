#![no_std]

pub const STROOP: u64 = 1_0000000;

pub mod i128;
pub mod u64;

mod fixed_point;
pub use fixed_point::FixedPoint;
