#![no_std]

// TODO: Implement the token interface in THIS contract
// TODO: Make Pair Trait
// TODO: Tell when token is a call of another contract (like tokenA), and when it should be this PairToken
// Own tokens functions to be imported: balance, mint, transfer, initialize
// Client token functions: transfer

mod test;
mod token;
mod create;

//use num_integer::Roots;
use soroban_sdk::{contractimpl, Address, Env}; //Bytes, BytesN, ConversionError, Env, RawVal, TryFromVal, token::Client as TokenClient};
//use token::{Token, TokenTrait};


//#[repr(u32)]

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

    // Returns the address of the pair for tokenA and tokenB, if it has been created, else address(0) 
    // function getPair(address tokenA, address tokenB) external view returns (address pair);
    fn get_pair(e: Env, tokenA: Address, tokenB: Address) -> Address;

    // Returns the address of the nth pair (0-indexed) created through the factory, or address(0) if not enough pairs have been created yet.
    // function allPairs(uint) external view returns (address pair);
    fn all_pairs(e: Env, n: i128) -> Address;

    /*  *** State-Changing Functions: *** */

    // function setFeeTo(address) external;
    fn set_fee_to(e: Env, to: Address);

    // function setFeeToSetter(address) external;
    fn set_fee_to_setter(e: Env, setter: Address);
    
    //Creates a pair for tokenA and tokenB if one doesn't exist already.
    // function createPair(address tokenA, address tokenB) external returns (address pair);
    fn create_pair(e: Env, tokenA: Address, tokenB: Address) -> Address;
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

    // Returns the address of the pair for tokenA and tokenB, if it has been created, else address(0) 
    // function getPair(address tokenA, address tokenB) external view returns (address pair);
    fn get_pair(e: Env, tokenA: Address, tokenB: Address) -> Address{
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
        // TODO: Implement
    }

    // function setFeeToSetter(address) external;
    fn set_fee_to_setter(e: Env, setter: Address){
        // TODO: Implement
    }
    
    //Creates a pair for tokenA and tokenB if one doesn't exist already.
    // function createPair(address tokenA, address tokenB) external returns (address pair);
    fn create_pair(e: Env, tokenA: Address, tokenB: Address) -> Address{
        // TODO: Implement
        e.current_contract_address()
    }
    

}
