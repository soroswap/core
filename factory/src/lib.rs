#![no_std]

// TODO: Implement the token interface in THIS contract
// TODO: Make Pair Trait
// TODO: Tell when token is a call of another contract (like token_a), and when it should be this PairToken
// Own tokens functions to be imported: balance, mint, transfer, initialize
// Client token functions: transfer

mod test;
mod token;
mod create;

//use num_integer::Roots;
use soroban_sdk::{contractimpl, Address, Env, TryFromVal, RawVal, ConversionError}; //Bytes, BytesN, ConversionError, Env, RawVal, TryFromVal, token::Client as TokenClient};
//use token::{Token, TokenTrait};

#[derive(Clone, Copy)]
#[repr(u32)]

pub enum DataKey {
    FeeTo = 0, // address public feeTo;
    FeeToSetter = 1, // address public feeToSetter;
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

fn get_fee_to_setter(e: &Env) -> Address {
    e.storage().get_unchecked(&DataKey::FeeToSetter).unwrap()
}

fn put_fee_to(e: &Env, to: Address) {
    e.storage().set(&DataKey::FeeTo, &to);
}

fn put_fee_to_setter(e: &Env, setter: Address) {
    e.storage().set(&DataKey::FeeToSetter, &setter);
}


pub trait SoroswapFactoryTrait{
    /*  *** Read only functions: *** */

    // feeTo is the recipient of the charge.
    // function feeTo() external view returns (address);
    fn fee_to(e: Env) -> Address;

    // The address allowed to change feeTo.
    // function feeToSetter() external view returns (address);
    fn fee_to_setter(e: Env) -> Address;

    // Returns the total number of pairs created through the factory so far.
    // function allPairsLength() external view returns (uint);  
    fn all_pairs_length(e: Env) -> i128;

    // Returns the address of the pair for token_a and token_b, if it has been created, else address(0) 
    // function getPair(address token_a, address token_b) external view returns (address pair);
    fn get_pair(e: Env, token_a: Address, token_b: Address) -> Address;

    // Returns the address of the nth pair (0-indexed) created through the factory, or address(0) if not enough pairs have been created yet.
    // function allPairs(uint) external view returns (address pair);
    fn all_pairs(e: Env, n: i128) -> Address;

    /*  *** State-Changing Functions: *** */

    // function setFeeTo(address) external;
    fn set_fee_to(e: Env, to: Address);

    // function setFeeToSetter(address) external;
    fn set_fee_to_setter(e: Env, setter: Address);
    
    //Creates a pair for token_a and token_b if one doesn't exist already.
    // function createPair(address token_a, address token_b) external returns (address pair);
    fn create_pair(e: Env, token_a: Address, token_b: Address) -> Address;
}

struct SoroswapFactory;

#[contractimpl]
impl SoroswapFactoryTrait for SoroswapFactory {

    /*  *** Read only functions: *** */

    // feeTo is the recipient of the charge.
    // function feeTo() external view returns (address);
    fn fee_to(e: Env) -> Address {
        // TODO: Implement
        e.current_contract_address()
    }

    // The address allowed to change feeTo.
    // function feeToSetter() external view returns (address);
    fn fee_to_setter(e: Env) -> Address {
        // TODO: Implement
        e.current_contract_address()
    }

    // Returns the total number of pairs created through the factory so far.
    // function allPairsLength() external view returns (uint);  
    fn all_pairs_length(e: Env) -> i128{
        // TODO: Implement
        1
    }

    // Returns the address of the pair for token_a and token_b, if it has been created, else address(0) 
    // function getPair(address token_a, address token_b) external view returns (address pair);
    fn get_pair(e: Env, token_a: Address, token_b: Address) -> Address{
        // TODO: Implement
        e.current_contract_address()
    }

    // Returns the address of the nth pair (0-indexed) created through the factory, or address(0) if not enough pairs have been created yet.
    // function allPairs(uint) external view returns (address pair);
    fn all_pairs(e: Env, n: i128) -> Address{
        // TODO: Implement
        e.current_contract_address()
    }

    /*  *** State-Changing Functions: *** */

    // function setFeeTo(address) external;
    fn set_fee_to(e: Env, to: Address){
        // TODO: Implement restriction
        // require(msg.sender == feeToSetter, 'UniswapV2: FORBIDDEN');
        
        put_fee_to(&e, to);
    }

    // function setFeeToSetter(address) external;
    fn set_fee_to_setter(e: Env, setter: Address){
        // TODO: Implement restriction
        // require(msg.sender == feeToSetter, 'UniswapV2: FORBIDDEN');
        
        put_fee_to(&e, setter);
    }
    
    //Creates a pair for token_a and token_b if one doesn't exist already.
    // function createPair(address token_a, address token_b) external returns (address pair);
    fn create_pair(e: Env, token_a: Address, token_b: Address) -> Address{
        // TODO: Implement
        e.current_contract_address()
    }
    

}
