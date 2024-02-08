use soroban_sdk::{contracttype, Vec, Address};

#[contracttype]
pub struct DexDistribution {
    pub index: i32,
    pub path: Option<Vec<Address>>,
}