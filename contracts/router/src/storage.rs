use soroban_sdk::{contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]

enum DataKey {
    Factory, // Address of the Factory Contract. Instance Data Type
}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
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
