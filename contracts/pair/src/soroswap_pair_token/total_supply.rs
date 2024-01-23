use soroban_sdk::{Env};

use crate::soroswap_pair_token::storage_types::DataKey;

pub fn read_total_supply(e: &Env) -> i128 {
    let key = DataKey::TotalSupply;
    e.storage().instance().get(&key).unwrap_or(0)
}

pub fn write_total_supply(e: &Env, id: &i128) {
    let key = DataKey::TotalSupply;
    e.storage().instance().set(&key, id);
}
