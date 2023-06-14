#![no_std]

mod test;
mod pair;
mod event;
 

use soroban_sdk::{  contractimpl, 
                    Env, 
                    TryFromVal, 
                    RawVal, 
                    ConversionError, 
                    Vec, 
                    Map, 
                    BytesN,
                    Address};
use pair::create_contract;

#[derive(Clone, Copy)]
#[repr(u32)]

pub enum DataKey {
    FeeTo = 0, // address public feeTo;
    FeeToSetter = 1, // address public feeToSetter;
    AllPairs = 2, //  address[] public allPairs;
    PairsMapping = 3, // Map of pairs
    PairWasmHash =4,
    FeesEnabled = 5, // bool is taking fees?

}

impl TryFromVal<Env, DataKey> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}


fn get_fee_to(e: &Env) -> Address {
    e.storage().get_unchecked(&DataKey::FeeTo).unwrap()
}

fn get_fees_enabled(e: &Env) -> bool {
    let key = DataKey::FeesEnabled;
    if let Some(state) = e.storage().get(&key) {
        state.unwrap()
    } else {
        false // By default fees are not enabled
    }
}

fn get_fee_to_setter(e: &Env) -> Address {
    e.storage().get_unchecked(&DataKey::FeeToSetter).unwrap()
}

fn get_all_pairs(e: &Env) -> Vec<Address> {
    e.storage().get(&DataKey::AllPairs).unwrap_or(Ok(Vec::new(&e))).unwrap()
}
fn get_pairs_mapping(e: &Env) -> Map<(Address, Address), Address> {
    // Note: Using unwrap_or_else() can be more efficient because it only evaluates the closure when it is necessary, whereas unwrap_or() always evaluates the default value expression.
    e.storage()
        .get(&DataKey::PairsMapping)
        .unwrap_or_else(|| Ok(Map::new(&e)))
        .unwrap()
}

fn get_pair_exists(e: &Env, token_a: Address, token_b: Address) -> bool {
    // Get the pairs mapping
    let pairs_mapping = get_pairs_mapping(&e);

    // Create a tuple of (Address, Address) to use as the key
    let pair_key = (token_a.clone(), token_b.clone());

    // Check if the pair exists with the first key:
    if pairs_mapping.contains_key(pair_key) {
        // If it does, return true
        return true;
    }

    // Check the other way around:
    let otherway_key = (token_b.clone(), token_a.clone());
    if pairs_mapping.contains_key(otherway_key) {
        // If it does, return true
        return true;
    }

    // If neither key exists, return false
    false
}

fn get_pair_wasm_hash(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(&DataKey::PairWasmHash).unwrap()
}

fn put_fee_to(e: &Env, to: Address) {
    e.storage().set(&DataKey::FeeTo, &to);
}

fn put_fee_to_setter(e: &Env,   setter: &Address) {
    e.storage().set(&DataKey::FeeToSetter, setter);
}

fn put_fees_enabled(e: &Env,   is_enabled: &bool) {
    e.storage().set(&DataKey::FeesEnabled, is_enabled);
}

fn _put_all_pairs(e: &Env, all_pairs: Vec<Address>) {
    e.storage().set(&DataKey::AllPairs, &all_pairs);
}

fn put_pairs_mapping(e: &Env, pairs_mapping: Map<(Address, Address), Address>) {
    e.storage().set(&DataKey::PairsMapping, &pairs_mapping)
}

fn put_pair_wasm_hash(e: &Env, pair_wasm_hash: BytesN<32>) {
    e.storage().set(&DataKey::PairWasmHash, &pair_wasm_hash)
}

fn add_pair_to_mapping(
    e: &Env,
    token_a: Address,
    token_b: Address,
    pair: Address,
) {
    // Get the pairs mapping
    let mut pairs_mapping = get_pairs_mapping(e);
    // Create a tuple of (Address, Address) for the first pair key
    let pair_key_a = (token_a.clone(), token_b.clone());
    // Create a tuple of (Address, Address) for the second pair key
    let pair_key_b = (token_b, token_a);
    // Insert the pair address for both keys into the pairs mapping
    pairs_mapping.set(pair_key_a, pair.clone());
    pairs_mapping.set(pair_key_b, pair);
    // Update the pairs mapping in storage
    put_pairs_mapping(e, pairs_mapping);
}

fn add_pair_to_all_pairs(e: &Env, pair_address: Address) {
    // Get the current `allPairs` vector from storage
    let mut all_pairs = get_all_pairs(e);
    // Push the new `pair_address` onto the vector
    all_pairs.push_back(pair_address);
    // Save the updated `allPairs` vector to storage
    e.storage().set(&DataKey::AllPairs, &all_pairs);
}


pub trait SoroswapFactoryTrait{
    // Sets the fee_to_setter address and sets the pair_wasm_hash to create new pair contracts
    fn initialize(e: Env, setter: Address, pair_wasm_hash: BytesN<32>);

    /*  *** Read only functions: *** */

    // feeTo is the recipient of the charge.
    // function feeTo() external view returns (address);
    fn fee_to(e: Env) -> Address;

    // The address allowed to change feeTo.
    // function feeToSetter() external view returns (address);
    fn fee_to_setter(e: Env) -> Address;

    fn fees_enabled(e: Env) -> bool;

    // Returns the total number of pairs created through the factory so far.
    // function allPairsLength() external view returns (uint);  
    fn all_pairs_length(e: Env) -> u32;

    // Returns the address of the pair for token_a and token_b, if it has been created, else address(0) 
    // function getPair(address token_a, address token_b) external view returns (address pair);
    fn get_pair(e: Env, token_a: Address, token_b: Address) -> Address ;

    // Returns the address of the nth pair (0-indexed) created through the factory, or address(0) if not enough pairs have been created yet.
    // function allPairs(uint) external view returns (address pair);
    fn all_pairs(e: Env, n: u32) -> Address;

    // Returns a bool if a pair exists;
    fn pair_exists(e: Env, token_a: Address, token_b: Address) -> bool;

    /*  *** State-Changing Functions: *** */

    // function setFeeTo(address) external;
    fn set_fee_to(e: Env, to: Address);

    // function setFeeToSetter(address) external;
    fn set_fee_to_setter(e: Env, new_setter: Address);

    fn set_fees_enabled(e: Env, is_enabled: bool);
    
    //Creates a pair for token_a and token_b if one doesn't exist already.
    // function createPair(address token_a, address token_b) external returns (address pair);
    fn create_pair(e: Env, token_a: Address, token_b: Address) -> Address;
}

struct SoroswapFactory;

#[contractimpl] 
impl SoroswapFactoryTrait for SoroswapFactory {
    // Sets the fee_to_setter address
    fn initialize(e: Env, setter: Address, pair_wasm_hash: BytesN<32>){
        // TODO: This should be called only once, and by the contract creator
        // if has_administrator(&e) {
        //     panic!("already initialized")
        // }
        // write_administrator(&e, &admin);
        put_fee_to_setter(&e, &setter);
        put_pair_wasm_hash(&e, pair_wasm_hash);
    }

    /*  *** Read only functions: *** */

    // feeTo is the recipient of the charge.
    // function feeTo() external view returns (address);
    fn fee_to(e: Env) -> Address {
        get_fee_to(&e)
    }

    // The address allowed to change feeTo.
    // function feeToSetter() external view returns (address);
    fn fee_to_setter(e: Env) -> Address {
        get_fee_to_setter(&e)
    }

    fn fees_enabled(e: Env) -> bool {
        get_fees_enabled(&e)
    }

    // Returns the total number of pairs created through the factory so far.
    // function allPairsLength() external view returns (uint);  
    fn all_pairs_length(e: Env) -> u32{
        get_all_pairs(&e).len()
    }

    // Returns the address of the pair for token_a and token_b, if it has been created, else Panics
    fn get_pair(e: Env, token_a: Address, token_b: Address) -> Address {
        // Get the mapping of pairs from storage in the current environment.
        let pairs_mapping = get_pairs_mapping(&e);
        // Create a tuple of (Address, Address) using the two input addresses to use as the key.
        let pair_key = (token_a.clone(), token_b.clone());
        // Get the value from the pairs mapping using the pair_key as the key.
        // Unwrap the result of the get() method twice to get the actual value of the pair_address.
        let pair_address = pairs_mapping.get(pair_key).unwrap().unwrap();
        // Return the pair address.
        pair_address
    }


    // Returns the address of the nth pair (0-indexed) created through the factory, or address(0) if not enough pairs have been created yet.
    // function allPairs(uint) external view returns (address pair);
    fn all_pairs(e: Env, n: u32) -> Address{
        // TODO: Implement error if n does not exist
        get_all_pairs(&e).get_unchecked(n).unwrap()
    }

    fn pair_exists(e: Env, token_a: Address, token_b: Address) -> bool {
        get_pair_exists(&e, token_a, token_b)
    }

    /*  *** State-Changing Functions: *** */

    // function setFeeTo(address) external;
    fn set_fee_to(e: Env, to: Address){
        let setter = get_fee_to_setter(&e);
        setter.require_auth();
        put_fee_to(&e, to);
    }

    // function setFeeToSetter(address) external;
    fn set_fee_to_setter(e: Env, new_setter: Address){
        let setter = get_fee_to_setter(&e);
        setter.require_auth();
        put_fee_to_setter(&e, &new_setter);
    }

    fn set_fees_enabled(e: Env, is_enabled: bool) {
        let setter = get_fee_to_setter(&e);
        setter.require_auth();
        put_fees_enabled(&e,&is_enabled)
    }
    
    //Creates a pair for token_a and token_b if one doesn't exist already.
    // function createPair(address token_a, address token_b) external returns (address pair);
    // token0 is guaranteed to be strictly less than token1 by sort order.
    fn create_pair(e: Env, token_a: Address, token_b: Address) -> Address{
        //require(tokenA != tokenB, 'UniswapV2: IDENTICAL_ADDRESSES');
        if token_a == token_b {
            panic!("SoroswapFactory: token_a and token_b have identical addresses");
        }

        // token0 is guaranteed to be strictly less than token1 by sort order.
        //(address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        let token_0;
        let token_1;
        if token_a < token_b {
            token_0 = token_a;
            token_1 = token_b;
        }
        else {
            token_0 = token_b;
            token_1 = token_a;
        }

        // TODO: Implement restriction of any kind of zero address
        //require(token0 != address(0), 'UniswapV2: ZERO_ADDRESS');

        //require(getPair[token0][token1] == address(0), 'UniswapV2: PAIR_EXISTS'); // single check is sufficient
        if get_pair_exists(&e, token_0.clone(), token_1.clone()){
            panic!("SoroswapFactory: pair already exist between token_0 and token_1");
        }

        /* 
        // Creation of the contract:
        // Code in Solidity

        bytes memory bytecode = type(UniswapV2Pair).creationCode;
            bytes32 salt = keccak256(abi.encodePacked(token0, token1));
            assembly {
                pair := create2(0, add(bytecode, 32), mload(bytecode), salt)
            }
            IUniswapV2Pair(pair).initialize(token0, token1);
        
        */
        let pair_wasm_hash = get_pair_wasm_hash(&e);
        let pair = create_contract(&e, &pair_wasm_hash, &token_0.clone(), &token_1.clone());
        // TODO: Implement name of the pair depending on the token names
        pair::Client::new(&e, &pair).initialize_pair(
            &e.current_contract_address(),
            &token_0,
            &token_1);
        
        // getPair[token0][token1] = pair;
        // getPair[token1][token0] = pair; // populate mapping in the reverse direction
        add_pair_to_mapping(&e, token_0.clone(), token_1.clone(), pair.clone());
        
        // allPairs.push(pair);
        add_pair_to_all_pairs(&e, pair.clone());

        // emit PairCreated(token0, token1, pair, allPairs.length);
        event::pair_created(&e, token_0, token_1, pair.clone(), get_all_pairs(&e).len());
        pair


    }
    

}
