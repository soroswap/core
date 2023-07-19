use crate::token::storage_types::DataKey;
use soroban_sdk::{Address, Env};

pub fn read_balance(e: &Env, addr: &Address) -> i128 {
    let key = DataKey::Balance(addr.clone());
    if let Some(balance) = e.storage().get(&key) {
        balance.unwrap()
    } else {
        0
    }
}

fn write_balance(e: &Env, addr: &Address, amount: i128) {
    let key = DataKey::Balance(addr.clone());
    e.storage().set(&key, &amount);
}

pub fn receive_balance(e: &Env, addr: &Address, amount: i128) {
    let balance = read_balance(e, addr);
    if !is_authorized(e, addr) {
        panic!("can't receive when deauthorized");
    }
    write_balance(e, addr, balance + amount);
}

pub fn spend_balance(e: &Env, addr: &Address, amount: i128) {
    let balance = read_balance(e, addr);
    if !is_authorized(e, addr) {
        panic!("can't spend when deauthorized");
    }
    if balance < amount {
        panic!("insufficient balance");
    }
    write_balance(e, addr, balance - amount);
}

pub fn is_authorized(e: &Env, addr: &Address) -> bool {
    let key = DataKey::State(addr.clone());
    if let Some(state) = e.storage().get(&key) {
        state.unwrap()
    } else {
        true
    }
}

pub fn write_authorization(e: &Env, addr: &Address, is_authorized: bool) {
    let key = DataKey::State(addr.clone());
    e.storage().set(&key, &is_authorized);
}
