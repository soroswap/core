use soroban_sdk::{contracttype, Env, Address};
use crate::models::{ProtocolAddressPair};

#[derive(Clone)]
#[contracttype]

enum DataKey {
    ProtocolAddress(i32),
    Initialized,
    Admin,
}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn set_initialized(e: &Env) {
    e.storage().instance().set(&DataKey::Initialized, &true);
}

pub fn is_initialized(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Initialized)
}

pub fn put_protocol_address(e: &Env, pair: ProtocolAddressPair) {
    e.storage().instance().set(&DataKey::ProtocolAddress(pair.protocol_id), &pair.address);
}

pub fn has_protocol_address(e: &Env, protocol_id: i32) -> bool {
    e.storage().instance().has(&DataKey::ProtocolAddress(protocol_id))
}

pub fn get_protocol_address(e: &Env, protocol_id: i32) -> Address {
    e.storage().instance().get(&DataKey::ProtocolAddress(protocol_id)).unwrap()
}

pub fn set_admin(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::Admin, &address)
}

pub fn get_admin(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Admin).unwrap()
}