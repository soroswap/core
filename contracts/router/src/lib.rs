#![no_std]
mod test;
use soroban_sdk::{
    contract,
    contractimpl, Address, ConversionError, Env, Val, TryFromVal,
};

// #[derive(Clone, Copy)]
// #[repr(u32)]

#[derive(Clone, Copy)]
pub enum DataKey {
    Factory = 0,        // address public factory;
}


// use SoroswapLibraryTrait;
//use fixed_point_math;
use dummy_contract::is_true;

impl TryFromVal<Env, DataKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}


fn put_factory(e: &Env, factory: &Address) {
    e.storage().instance().set(&DataKey::Factory, &factory);
}

fn has_factory(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Factory)
}

pub trait SoroswapRouterTrait{

     // Initializes the contract and sets the factory address
     fn initialize(e: Env, factory: Address);
}



#[contract]
struct SoroswapRouter;

#[contractimpl]
impl SoroswapRouterTrait for SoroswapRouter {

    // Initializes the contract and sets the factory address
    fn initialize(e: Env, factory: Address) {
        // constructor(address _factory, address _WETH) public {
        //     factory = _factory;
        //     WETH = _WETH;
        // }
        
        assert!(!has_factory(&e), "SoroswapRouter: already initialized");
        put_factory(&e, &factory);
    }
    
}
