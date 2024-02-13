use soroban_sdk::{contracttype, Env, Address};

#[derive(Clone)]
#[contracttype]

enum DataKey {
    Soroswap, // Address of the Soroswap Contract. Instance Data Type
}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn put_soroswap_router(e: &Env, soroswap_router: &Address) {
    e.storage().instance().set(&DataKey::Soroswap, &soroswap_router);
}

pub fn has_soroswap_router(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Soroswap)
}

pub fn get_soroswap_router(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Soroswap).unwrap()
}
