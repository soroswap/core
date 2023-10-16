#![no_std]
use soroban_sdk::{contract, contractimpl};


pub fn my_increment(count: u32) -> u32 {
    count + 1
}

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(count: u32) -> u32 {
        my_increment(count)
    }
}

mod test;
