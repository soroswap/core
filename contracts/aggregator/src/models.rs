use soroban_sdk::{contracttype, Vec, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DexDistribution {
    pub index: i32,
    pub path: Vec<Address>,
    pub parts: i128,
}