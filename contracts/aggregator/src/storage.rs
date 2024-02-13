use soroban_sdk::{contracttype, Env, Address};

#[derive(Clone)]
#[contracttype]

enum DataKey {
    ProtocolAddress(i32),
    Initialized,
}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn put_initialized(e: &Env) {
    e.storage().instance().set(&DataKey::Initialized, &true);
}

pub fn has_initialized(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Initialized)
}

pub fn put_protocol_address(e: &Env, protocol_id: i32, address: &Address) {
    e.storage().instance().set(&DataKey::ProtocolAddress(protocol_id), address);
}

pub fn has_protocol_address(e: &Env, protocol_id: i32) -> bool {
    e.storage().instance().has(&DataKey::ProtocolAddress(protocol_id))
}

pub fn get_protocol_address(e: &Env, protocol_id: i32) -> Address {
    e.storage().instance().get(&DataKey::ProtocolAddress(protocol_id)).unwrap()
}
