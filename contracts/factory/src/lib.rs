#![no_std]

mod event;
mod pair;
mod test;

use pair::{create_contract, Pair};
use soroban_sdk::{
    contract,
    contractimpl,
    contracttype, Address, BytesN, Map, Env,
};
use soroswap_factory_interface::{SoroswapFactoryTrait, FactoryError};


#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    FeeTo,      // Address. Instance storage
    FeeToSetter, // Address. Instance storage
    PairsMapping, // TODO: Not use Mapping
    PairWasmHash, // BytesN<32>. Instance storage
    FeesEnabled, // Bool. Instance storage
    TotalPairs, // Total pairs created by the Factory. u32, Instance storage
    PairAddresses(u32) // Addresses of pairs created by the Factory. Persistent Storage
}


// #[derive(Clone)]
// pub enum DataKey {
//     PairAddresses(u32)
// }


// Storage helper functions
fn get_total_pairs(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::TotalPairs).unwrap_or(0)
}
fn put_total_pairs(e: &Env, n: u32) {
    e.storage().instance().set(&DataKey::TotalPairs, &n);
}


fn get_fee_to(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::FeeTo).unwrap()
}


fn get_fees_enabled(e: &Env) -> bool {
    let key = DataKey::FeesEnabled;
    if let Some(state) = e.storage().instance().get(&key) {
        state
    } else {
        false // By default fees are not enabled
    }
}

fn get_fee_to_setter(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::FeeToSetter).unwrap()
}

fn get_pairs_mapping(e: &Env) -> Map<Pair, Address> {
    // Note: Using unwrap_or_else() can be more efficient because it only evaluates the closure when it is necessary, whereas unwrap_or() always evaluates the default value expression.
    e.storage()
        .instance()
        .get(&DataKey::PairsMapping)
        .unwrap_or_else(|| Map::new(&e))
}

fn get_pair_exists(e: &Env, pair_key: &Pair) -> bool {
    // Get the pairs mapping
    let pairs_mapping = get_pairs_mapping(&e);

    // Check if the pair exists with the first key:
    pairs_mapping.contains_key(pair_key.clone())
}

fn get_pair_wasm_hash(e: &Env) -> BytesN<32> {
    e.storage().instance().get(&DataKey::PairWasmHash).unwrap()
}

fn put_fee_to(e: &Env, to: Address) {
    e.storage().instance().set(&DataKey::FeeTo, &to);
}

fn put_fee_to_setter(e: &Env, setter: &Address) {
    e.storage().instance().set(&DataKey::FeeToSetter, setter);
}

fn put_fees_enabled(e: &Env, is_enabled: &bool) {
    e.storage().instance().set(&DataKey::FeesEnabled, is_enabled);
}

fn put_pairs_mapping(e: &Env, pairs_mapping: Map<Pair, Address>) {
    e.storage().instance().set(&DataKey::PairsMapping, &pairs_mapping)
}

fn put_pair_wasm_hash(e: &Env, pair_wasm_hash: BytesN<32>) {
    e.storage().instance().set(&DataKey::PairWasmHash, &pair_wasm_hash)
}


// Helper function in order to know if the contract has been initialized or not
pub fn has_pair_wasm_hash(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::PairWasmHash)
}


fn add_pair_to_mapping(e: &Env, token_pair: &Pair, pair: &Address) {
    // Get the pairs mapping
    let mut pairs_mapping = get_pairs_mapping(e);
    
    // Insert the pair address for both keys into the pairs mapping
    pairs_mapping.set(token_pair.clone(), pair.clone());
    // pairs_mapping.set(pair_key_b, pair);
    // Update the pairs mapping in storage
    put_pairs_mapping(e, pairs_mapping);
}

fn add_pair_to_all_pairs(e: &Env, pair_address: &Address) {
    // total_pairs is the total amount of pairs created by the Factory
    let mut total_pairs = get_total_pairs(e);
    // Because PairAddresses is 0-indexed, we start with 0, default value of total_pairs
    e.storage().persistent().set(&DataKey::PairAddresses(total_pairs), &pair_address);
    total_pairs = total_pairs.checked_add(1).unwrap();
    put_total_pairs(&e, total_pairs);
}

#[contract]
struct SoroswapFactory;

#[contractimpl]
impl SoroswapFactoryTrait for SoroswapFactory {

/* *** Read-only functions: *** */

/// Returns the recipient of the fee.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// 
/// # Errors
/// 
/// Returns an error if the Factory is not yet initialized.
fn fee_to(e: Env) -> Result<Address, FactoryError> {
    if !has_pair_wasm_hash(&e) {
        return Err(FactoryError::NotInitialized);
    }
    Ok(get_fee_to(&e))
}

/// Returns the address allowed to change the fee recipient.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// 
/// # Errors
/// 
/// Returns an error if the Factory is not yet initialized.
fn fee_to_setter(e: Env) -> Result<Address, FactoryError> {
    if !has_pair_wasm_hash(&e) {
        return Err(FactoryError::NotInitialized);
    }
    Ok(get_fee_to_setter(&e))
}

/// Checks if fees are enabled.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// 
/// # Errors
/// 
/// Returns an error if the Factory is not yet initialized.
fn fees_enabled(e: Env) -> Result<bool, FactoryError> {
    if !has_pair_wasm_hash(&e) {
        return Err(FactoryError::NotInitialized);
    }
    Ok(get_fees_enabled(&e))
}

/// Returns the total number of pairs created through the factory so far.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// 
/// # Errors
/// 
/// Returns an error if the Factory is not yet initialized.
fn all_pairs_length(e: Env) -> Result<u32, FactoryError> {
    if !has_pair_wasm_hash(&e) {
        return Err(FactoryError::NotInitialized);
    }
    Ok(get_total_pairs(&e))
}

/// Returns the address of the pair for `token_a` and `token_b`, if it has been created.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// * `token_a` - The address of the first token in the pair.
/// * `token_b` - The address of the second token in the pair.
/// 
/// # Errors
/// 
/// Returns an error if the Factory is not yet initialized or if the pair does not exist
fn get_pair(e: Env, token_a: Address, token_b: Address) -> Result<Address, FactoryError> {
    if !has_pair_wasm_hash(&e) {
        return Err(FactoryError::NotInitialized);
    }
    
    let pairs_mapping = get_pairs_mapping(&e);
    let pair_key = Pair::new(token_a, token_b);
    pairs_mapping.get(pair_key).ok_or(FactoryError::PairDoesNotExist)
}

/// Returns the address of the nth pair (0-indexed) created through the factory, or address(0) if not enough pairs have been created yet.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// * `n` - The index of the pair to retrieve.
/// 
/// # Errors
/// 
/// Returns an error if the Factory is not yet initialized or if index `n` does not exist.
fn all_pairs(e: Env, n: u32) -> Result<Address, FactoryError> {
    if !has_pair_wasm_hash(&e) {
        return Err(FactoryError::NotInitialized);
    }
    e.storage().persistent().get(&DataKey::PairAddresses(n)).ok_or(FactoryError::IndexDoesNotExist)
}

/// Checks if a pair exists for the given `token_a` and `token_b`.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// * `token_a` - The address of the first token in the pair.
/// * `token_b` - The address of the second token in the pair.
/// 
/// # Errors
/// 
/// Returns an error if the Factory is not yet initialized.
fn pair_exists(e: Env, token_a: Address, token_b: Address) -> Result<bool, FactoryError> {
    if !has_pair_wasm_hash(&e) {
        return Err(FactoryError::NotInitialized);
    }
    Ok(get_pair_exists(&e, &Pair::new(token_a, token_b)))
}


/* *** State-Changing Functions: *** */

/// Sets the `fee_to_setter` address and initializes the factory.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// * `setter` - The address to set as the current `fee_to_setter`.
/// * `pair_wasm_hash` - The Wasm hash of the pair.
/// 
/// # Errors
/// 
/// Returns an error if the Factory is already initialized.
fn initialize(e: Env, setter: Address, pair_wasm_hash: BytesN<32>) -> Result<(), FactoryError> {
    if has_pair_wasm_hash(&e) {
        return Err(FactoryError::InitializeAlreadyInitialized);
    }
    put_fee_to_setter(&e, &setter);
    put_fee_to(&e, setter.clone());
    put_pair_wasm_hash(&e, pair_wasm_hash);
    event::initialized(&e, setter);
    Ok(())
}

/// Sets the `fee_to` address.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// * `to` - The address to set as the `fee_to`.
/// 
/// # Errors
/// 
/// Returns an error if the Factory is not yet initialized or if the caller is not the current `fee_to_setter`.
fn set_fee_to(e: Env, to: Address) -> Result<(), FactoryError> {
    if !has_pair_wasm_hash(&e) {
        return Err(FactoryError::NotInitialized);
    }
    
    let setter = get_fee_to_setter(&e);
    setter.require_auth();

    let old = get_fee_to(&e);
    put_fee_to(&e, to.clone());
    event::new_fee_to(&e, setter, old, to);
    Ok(())
}

/// Sets the `fee_to_setter` address.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// * `new_setter` - The address to set as the new `fee_to_setter`.
/// 
/// # Errors
/// 
/// Returns an error if the Factory is not yet initialized or if the caller is not the existing `fee_to_setter`.
fn set_fee_to_setter(e: Env, new_setter: Address) -> Result<(), FactoryError> {
    if !has_pair_wasm_hash(&e) {
        return Err(FactoryError::NotInitialized);
    }

    let setter = get_fee_to_setter(&e);
    setter.require_auth();

    put_fee_to_setter(&e, &new_setter);
    event::new_setter(&e, setter, new_setter);
    Ok(())
}

/// Sets whether fees are enabled or disabled.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// * `is_enabled` - A boolean indicating whether fees are enabled or disabled.
/// 
/// # Errors
/// 
/// Returns an error if the Factory is not yet initialized or if the caller is not the current `fee_to_setter`.
fn set_fees_enabled(e: Env, is_enabled: bool) -> Result<(), FactoryError> {
    if !has_pair_wasm_hash(&e) {
        return Err(FactoryError::NotInitialized);
    }

    let setter = get_fee_to_setter(&e);
    setter.require_auth();

    put_fees_enabled(&e, &is_enabled);
    event::new_fees_enabled(&e, is_enabled);
    Ok(())
}

/// Creates a pair for `token_a` and `token_b` if one doesn't exist already.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// * `token_a` - The address of the first token in the pair.
/// * `token_b` - The address of the second token in the pair.
/// 
/// # Errors
/// 
/// Returns an error if the pair is not yet initialized, if `token_a` and `token_b` have identical addresses, or if the pair already exists between `token_a` and `token_b`.
fn create_pair(e: Env, token_a: Address, token_b: Address) -> Result<Address, FactoryError> {
    if !has_pair_wasm_hash(&e) {
        return Err(FactoryError::NotInitialized);
    }

    if token_a == token_b {
        return Err(FactoryError::CreatePairIdenticalTokens);
    }

    let token_pair = Pair::new(token_a, token_b);

    if get_pair_exists(&e, &token_pair) {
        return Err(FactoryError::CreatePairAlreadyExists);
    }

    let pair_wasm_hash = get_pair_wasm_hash(&e);
    let pair = create_contract(&e, pair_wasm_hash, &token_pair);

    pair::Client::new(&e, &pair).initialize_pair(
        &e.current_contract_address(),
        &token_pair.token_0(), 
        &token_pair.token_1()
    );

    add_pair_to_mapping(&e, &token_pair, &pair);
    add_pair_to_all_pairs(&e, &pair);

    event::new_pair(&e, token_pair.token_0().clone(), token_pair.token_1().clone(), pair.clone(), get_total_pairs(&e));

    Ok(pair)
}


}
