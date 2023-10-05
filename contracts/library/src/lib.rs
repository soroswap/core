#![no_std]

mod test;

use soroban_sdk::{
    contract,
    contractimpl, Address, BytesN, ConversionError, Map, Env, Val, TryFromVal, Vec,
};

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


pub trait SoroswapLibraryTrait {
    
    // returns sorted token addresses, used to handle return values from pairs sorted in this order
    fn sortTokens(token_a: Address, token_b: Address) -> (Address, Address);

   
}

#[contract]
struct SoroswapLibrary;

#[contractimpl]
impl SoroswapLibraryTrait for SoroswapLibrary {

    // returns sorted token addresses, used to handle return values from pairs sorted in this order
    // function sortTokens(address tokenA, address tokenB) internal pure returns (address token0, address token1) {
    fn sortTokens(token_a: Address, token_b: Address) -> (Address, Address) {
        //     require(tokenA != tokenB, 'UniswapV2Library: IDENTICAL_ADDRESSES');
        if token_a == token_b {
            panic!("SoroswapFactory: token_a and token_b have identical addresses");
        }
        
        //     (token0, token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        if token_a < token_b {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        }
        
        //     require(token0 != address(0), 'UniswapV2Library: ZERO_ADDRESS');
        // In Soroban we don't have the concept of ZERO_ADDRESS
    }

    


}
