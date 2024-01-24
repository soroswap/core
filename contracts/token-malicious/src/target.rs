use crate::storage_types::DataKey;
use soroban_sdk::{Address, Env};

pub fn read_target_token_contract(e: &Env) -> Address {
    let key = DataKey::TargetTokenContract;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_target_token_contract(e: &Env, id: &Address) {
    let key = DataKey::TargetTokenContract;
    e.storage().instance().set(&key, id);
}

pub fn read_target_user(e: &Env) -> Address {
    let key = DataKey::TargetUser;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_target_user(e: &Env, id: &Address) {
    let key = DataKey::TargetUser;
    e.storage().instance().set(&key, id);
}
