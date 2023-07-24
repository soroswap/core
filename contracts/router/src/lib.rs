#![no_std]

use soroban_sdk::{contractimpl, Address, Bytes, ConversionError, Env, RawVal, TryFromVal};



pub trait SoroswapRouterTrait{

    fn my_function(e: Env) -> i128;
}

struct SoroswapRouter;

#[contractimpl]
impl SoroswapRouterTrait for SoroswapRouter {
  
    fn my_function(e: Env) -> i128{
        3
    }
}
