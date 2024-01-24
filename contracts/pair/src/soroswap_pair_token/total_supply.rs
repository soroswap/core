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

pub fn increase_total_supply(e: &Env, amount: i128) {
    let total_supply = read_total_supply(&e);
    let new_total_supply = total_supply.checked_add(amount)
        .expect("Integer overflow occurred while increasing total supply.");
    write_total_supply(&e, &new_total_supply);
}

pub fn decrease_total_supply(e: &Env, amount: i128) {
    let total_supply = read_total_supply(&e);
    if total_supply < amount {
        panic!("insufficient total supply");
    }
    let new_total_supply = total_supply.checked_sub(amount)
        .expect("Integer underflow occurred while decreasing total supply.");
    write_total_supply(&e, &new_total_supply);
}