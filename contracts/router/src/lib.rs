#![no_std]
mod test;

use  soroswap_library;
use soroswap_library::SoroswapLibrary as SoroswapLibraryTrait;
use soroswap_library::{__sort_tokens};
// use dummy_increment_contract;
use soroban_sdk::{
    contract, contractimpl, Address};

// use SoroswapLibraryTrait;
//use fixed_point_math;
use dummy_contract::is_true;

pub trait SoroswapRouterTrait{

    // **** LIBRARY FUNCTIONS ****
    
    // given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    // fn quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> i128;
    fn my_bool() -> bool;

    // returns sorted token addresses, used to handle return values from pairs sorted in this order
    fn my_sort_tokens(token_a: Address, token_b: Address) -> (Address, Address);
}

#[contract]
struct SoroswapRouter;

#[contractimpl]
impl SoroswapRouterTrait for SoroswapRouter {
  
   
    // given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    // fn quote(amount_a: i128, reserve_a: i128, reserve_b: i128)  -> i128 {
    //     // function quote(uint amountA, uint reserveA, uint reserveB) public pure virtual override returns (uint amountB) {
    //     //     return UniswapV2Library.quote(amountA, reserveA, reserveB);
    //     // }
    //     //quote(amount_a, reserve_a, reserve_b)
    //     0
    // }

    fn my_bool() -> bool {
        soroswap_library::is_true()
        
    }

    // returns sorted token addresses, used to handle return values from pairs sorted in this order
    fn my_sort_tokens(token_a: Address, token_b: Address) -> (Address, Address){
        sort_tokens(token_a, token_b)
    }
}
