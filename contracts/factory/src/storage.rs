use soroban_sdk::{
    contracttype, Address, BytesN, Env, Val, TryFromVal
};
use soroswap_factory_interface::{FactoryError};
use crate::pair::{Pair};



#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    FeeTo,      // Address. Instance storage
    FeeToSetter, // Address. Instance storage
    PairWasmHash, // BytesN<32>. Persistent storage
    FeesEnabled, // Bool. Instance storage
    TotalPairs, // Total pairs created by the Factory. u32, Instance storage
    PairAddressesNIndexed(u32), // Addresses of pairs created by the Factory. Persistent Storage
    PairAddressesByTokens(Pair)
}


const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

const PERSISTENT_BUMP_AMOUNT: u32 = 60 * DAY_IN_LEDGERS;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = PERSISTENT_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

/// Fetch an entry in persistent storage that has a default value if it doesn't exist
fn get_persistent_extend_or_error<V: TryFromVal<Env, Val>>(
    e: &Env,
    key: &DataKey,
    error: FactoryError
) -> Result<V, FactoryError> {
    if let Some(result) = e.storage().persistent().get(key) {
        e.storage()
            .persistent()
            .extend_ttl(key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        result
    } else {
        return Err(error);
    }
}

//// --- Storage helper functions ---

// TotalPairs
pub fn put_total_pairs(e: &Env, n: u32) {
    e.storage().instance().set(&DataKey::TotalPairs, &n);
}
pub fn get_total_pairs(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::TotalPairs).unwrap_or(0)
}
// Helper function in order to know if the contract has been initialized or not
pub fn has_total_pairs(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::TotalPairs)
}


// PairAddressesByTokens(Address, Address)
pub fn put_pair_address_by_token_pair(e: &Env, token_pair: Pair, pair_address: &Address) {
    e.storage()
        .persistent()
        .set(&DataKey::PairAddressesByTokens(token_pair), &pair_address)
}
pub fn get_pair_address_by_token_pair(e: &Env, token_pair: Pair) -> Result<Address, FactoryError> {
    let key = DataKey::PairAddressesByTokens(token_pair);
    get_persistent_extend_or_error(&e, &key, FactoryError::PairDoesNotExist)
}

pub fn get_pair_exists(e: &Env, token_pair: Pair) -> bool {
    let key:DataKey = DataKey::PairAddressesByTokens(token_pair);
    if e.storage().persistent().has(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        true
    } else {
        false
    }
    
}


pub fn get_fee_to(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::FeeTo).unwrap()
}


pub fn get_fees_enabled(e: &Env) -> bool {
    let key = DataKey::FeesEnabled;
    if let Some(state) = e.storage().instance().get(&key) {
        state
    } else {
        false // By default fees are not enabled
    }
}

pub fn get_fee_to_setter(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::FeeToSetter).unwrap()
}


pub fn get_pair_wasm_hash(e: &Env) -> Result<BytesN<32>, FactoryError>{
    let key = DataKey::PairWasmHash;
    get_persistent_extend_or_error(&e, &key, FactoryError::NotInitialized)
}

pub fn put_fee_to(e: &Env, to: Address) {
    e.storage().instance().set(&DataKey::FeeTo, &to);
}

pub fn put_fee_to_setter(e: &Env, setter: &Address) {
    e.storage().instance().set(&DataKey::FeeToSetter, setter);
}

pub fn put_fees_enabled(e: &Env, is_enabled: &bool) {
    e.storage().instance().set(&DataKey::FeesEnabled, is_enabled);
}

pub fn put_pair_wasm_hash(e: &Env, pair_wasm_hash: BytesN<32>) {
    e.storage().persistent().set(&DataKey::PairWasmHash, &pair_wasm_hash)
}

pub fn add_pair_to_all_pairs(e: &Env, pair_address: &Address) {
    // total_pairs is the total amount of pairs created by the Factory
    let mut total_pairs = get_total_pairs(e);
    // Because PairAddressesNIndexed is 0-indexed, we start with 0, default value of total_pairs

    e.storage().persistent().set(&DataKey::PairAddressesNIndexed(total_pairs), &pair_address);

    total_pairs = total_pairs.checked_add(1).unwrap();
    put_total_pairs(&e, total_pairs);
}

pub fn get_all_pairs(e: Env, n: u32) -> Result<Address, FactoryError> {
    let key = DataKey::PairAddressesNIndexed(n);
    get_persistent_extend_or_error(&e, &key, FactoryError::IndexDoesNotExist)
}