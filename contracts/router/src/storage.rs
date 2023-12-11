use soroban_sdk::{contracttype, Env, Address};

#[derive(Clone)]
#[contracttype]

enum DataKey {
    Factory, // Address of the Factory Contract
}

pub fn put_factory(e: &Env, factory: &Address) {
    e.storage().instance().set(&DataKey::Factory, &factory);
}

pub fn has_factory(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Factory)
}

pub fn get_factory(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Factory).unwrap()
}
