#![no_std]

mod event;
mod pair;
mod test;

use pair::{create_contract, Pair};
use soroban_sdk::{
    contract,
    contractimpl, Address, BytesN, ConversionError, Map, Env, Val, TryFromVal, Vec,
};
use soroswap_factory_interface::SoroswapFactoryTrait;


#[derive(Clone, Copy)]
#[repr(u32)]

pub enum DataKey {
    FeeTo = 0,        // address public feeTo;
    FeeToSetter = 1,  // address public feeToSetter;
    AllPairs = 2,     //  address[] public allPairs;
    PairsMapping = 3, // Map of pairs
    PairWasmHash = 4,
    FeesEnabled = 5, // bool is taking fees?
}

impl TryFromVal<Env, DataKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
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

fn get_all_pairs(e: &Env) -> Vec<Address> {
    e.storage()
        .instance()
        .get(&DataKey::AllPairs)
        .unwrap_or(Vec::new(&e))
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

fn _put_all_pairs(e: &Env, all_pairs: Vec<Address>) {
    e.storage().instance().set(&DataKey::AllPairs, &all_pairs);
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
    // Get the current `allPairs` vector from storage
    let mut all_pairs = get_all_pairs(e);
    // Push the new `pair_address` onto the vector
    all_pairs.push_back(pair_address.clone());
    // Save the updated `allPairs` vector to storage
    e.storage().instance().set(&DataKey::AllPairs, &all_pairs);
}

#[contract]
struct SoroswapFactory;

#[contractimpl]
impl SoroswapFactoryTrait for SoroswapFactory {
    /*  *** Read only functions: *** */

    /// Returns the recipient of the fee.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// 
    /// # Panics
    /// 
    /// Panics if the Factory is not yet initialized.
    fn fee_to(e: Env) -> Address {
        assert!(has_pair_wasm_hash(&e), "SoroswapFactory: not yet initialized");
        get_fee_to(&e)
    }

    /// Returns the address allowed to change the fee recipient.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// 
    /// # Panics
    /// 
    /// Panics if the Factory is not yet initialized.
    fn fee_to_setter(e: Env) -> Address {
        assert!(has_pair_wasm_hash(&e), "SoroswapFactory: not yet initialized");
        get_fee_to_setter(&e)
    }

    /// Checks if fees are enabled.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// 
    /// # Panics
    /// 
    /// Panics if the Factory is not yet initialized.
    fn fees_enabled(e: Env) -> bool {
        assert!(has_pair_wasm_hash(&e), "SoroswapFactory: not yet initialized");
        get_fees_enabled(&e)
    }

    /// Returns the total number of pairs created through the factory so far.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// 
    /// # Panics
    /// 
    /// Panics if the Factory is not yet initialized.
    fn all_pairs_length(e: Env) -> u32 {
        assert!(has_pair_wasm_hash(&e), "SoroswapFactory: not yet initialized");
        get_all_pairs(&e).len() as u32
    }


    /// Returns the address of the pair for `token_a` and `token_b`, if it has been created.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `token_a` - The address of the first token in the pair.
    /// * `token_b` - The address of the second token in the pair.
    /// 
    /// # Panics
    /// 
    /// Panics if the Factory is not yet initialized.
    /// TODO: Implement error if the pair does not exist
    fn get_pair(e: Env, token_a: Address, token_b: Address) -> Address {
        assert!(has_pair_wasm_hash(&e), "SoroswapFactory: not yet initialized");
        
        // Get the mapping of pairs from storage in the current environment.
        let pairs_mapping = get_pairs_mapping(&e);
        
        // Create a tuple of (Address, Address) using the two input addresses to use as the key.
        let pair_key = Pair::new(token_a, token_b);
        
        // Get the value from the pairs mapping using the pair_key as the key.
        let pair_address = pairs_mapping.get(pair_key).unwrap();
        
        // Return the pair address.
        pair_address
    }

    /// Returns the address of the nth pair (0-indexed) created through the factory, or address(0) if not enough pairs have been created yet.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `n` - The index of the pair to retrieve.
    /// 
    /// # Panics
    /// 
    /// Panics if the Factory is not yet initialized.
    /// TODO: Implement error if index `n` does not exist
    fn all_pairs(e: Env, n: u32) -> Address {
        assert!(has_pair_wasm_hash(&e), "SoroswapFactory: not yet initialized");
        
        // TODO: Implement error if `n` does not exist
        get_all_pairs(&e).get(n).unwrap()
    }

    /// Checks if a pair exists for the given `token_a` and `token_b`.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `token_a` - The address of the first token in the pair.
    /// * `token_b` - The address of the second token in the pair.
    /// 
    /// # Panics
    /// 
    /// Panics if the Factory is not yet initialized.
    fn pair_exists(e: Env, token_a: Address, token_b: Address) -> bool {
        assert!(has_pair_wasm_hash(&e), "SoroswapFactory: not yet initialized");
        get_pair_exists(&e, &Pair::new(token_a, token_b))
    }


    /*  *** State-Changing Functions: *** */

    /// Sets the `fee_to_setter` address and initializes the factory.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `setter` - The address to set as the current `fee_to_setter`.
    /// * `pair_wasm_hash` - The Wasm hash of the pair.
    /// 
    /// # Panics
    /// 
    /// Panics if the Factory is already initialized.
    fn initialize(e: Env, setter: Address, pair_wasm_hash: BytesN<32>) {
        assert!(!has_pair_wasm_hash(&e), "SoroswapFactory: already initialized");
        put_fee_to_setter(&e, &setter);
        put_fee_to(&e, setter.clone());
        put_pair_wasm_hash(&e, pair_wasm_hash);
        event::initialized(&e, setter);
    }

    /// Sets the `fee_to` address.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `to` - The address to set as the `fee_to`.
    /// 
    /// # Panics
    /// 
    /// Panics if the Factory is not yet initialized.
    /// Panics if the caller is not the current `fee_to_setter`.
    fn set_fee_to(e: Env, to: Address) {
        assert!(has_pair_wasm_hash(&e), "SoroswapFactory: not yet initialized");
        
        let setter = get_fee_to_setter(&e);
        
        // Panics if the caller is not the current `fee_to_setter`.
        setter.require_auth();
        
        let old = get_fee_to(&e);
        put_fee_to(&e, to.clone());
        event::new_fee_to(&e, setter, old, to);
    }

    /// Sets the `fee_to_setter` address.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `new_setter` - The address to set as the new `fee_to_setter`.
    /// 
    /// # Panics
    /// 
    /// Panics if the Factory is not yet initialized.
    /// Panics if the caller is not the existing `fee_to_setter`.
    fn set_fee_to_setter(e: Env, new_setter: Address) {
        assert!(has_pair_wasm_hash(&e), "SoroswapFactory: not yet initialized");
        
        let setter = get_fee_to_setter(&e);
        
        // Panics if the caller is not the existing `fee_to_setter`.
        setter.require_auth();
        
        put_fee_to_setter(&e, &new_setter);
        event::new_setter(&e, setter, new_setter);
    }

    /// Sets whether fees are enabled or disabled.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `is_enabled` - A boolean indicating whether fees are enabled or disabled.
    /// 
    /// # Panics
    /// 
    /// Panics if the Factory is not yet initialized.
    /// Panics if the caller is not the current `fee_to_setter`.
    fn set_fees_enabled(e: Env, is_enabled: bool) {
        assert!(has_pair_wasm_hash(&e), "SoroswapFactory: not yet initialized");
        
        let setter = get_fee_to_setter(&e);
        
        // Panics if the caller is not the current `fee_to_setter`.
        setter.require_auth();
        
        put_fees_enabled(&e, &is_enabled);
        event::new_fees_enabled(&e, is_enabled);
    }


    /// Creates a pair for `token_a` and `token_b` if one doesn't exist already.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `token_a` - The address of the first token in the pair.
    /// * `token_b` - The address of the second token in the pair.
    /// 
    /// # Panics
    /// 
    /// Panics if the pair is not yet initialized.
    /// Panics if `token_a` and `token_b` have identical addresses.
    /// Panics if the pair already exists between `token_a` and `token_b`.
    fn create_pair(e: Env, token_a: Address, token_b: Address) -> Address {
        assert!(has_pair_wasm_hash(&e), "SoroswapFactory: not yet initialized");
        
        // Check if token_a and token_b have identical addresses
        if token_a == token_b {
            panic!("SoroswapFactory: token_a and token_b have identical addresses");
        }

        // token0 is guaranteed to be strictly less than token1 by sort order.
        let token_pair = Pair::new(token_a, token_b);

        // Check if the pair already exists between token_a and token_b
        if get_pair_exists(&e, &token_pair) {
            panic!("SoroswapFactory: pair already exists between token_a and token_b");
            
        }

        // Implementation of contract creation
        let pair_wasm_hash = get_pair_wasm_hash(&e);
        let pair = create_contract(&e, pair_wasm_hash, &token_pair);

        // Initialize the created pair
        pair::Client::new(&e, &pair).initialize_pair(
            &e.current_contract_address(),
            &token_pair.token_0(), 
            &token_pair.token_1()
        );

        // Add the pair to the mapping and all pairs list
        add_pair_to_mapping(&e, &token_pair, &pair);
        add_pair_to_all_pairs(&e, &pair);

        // Emit new_pair event
        event::new_pair(&e, token_pair.token_0().clone(), token_pair.token_1().clone(), pair.clone(), get_all_pairs(&e).len());

        pair
    }

}
