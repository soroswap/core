#![no_std]
mod test;
use soroban_sdk::{
    contract,
    contractimpl, Address, ConversionError, Env, Val, TryFromVal,
};
use soroswap_library;

mod factory {
    soroban_sdk::contractimport!(file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory_contract.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}

use factory::SoroswapFactoryClient;


#[derive(Clone, Copy)]
pub enum DataKey {
    Factory = 0,        // address public factory;
}

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

fn get_factory(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Factory).unwrap()
}


pub trait SoroswapRouterTrait{

     /// Initializes the contract and sets the factory address
     fn initialize(e: Env, factory: Address);

}

/// Internal add_liquidityt function
//  function _addLiquidity(
//     address tokenA,
//     address tokenB,
//     uint amountADesired,
//     uint amountBDesired,
//     uint amountAMin,
//     uint amountBMin
// ) internal virtual returns (uint amountA, uint amountB) {
fn add_liquidity_amounts(
    e: Env,
    token_a: Address,
    token_b: Address, 
    amount_a_desired: i128,
    amount_b_desired: i128,
    amount_a_min: i128,
    amount_b_min: i128
) -> (i128, i128) { // returns (uint amountA, uint amountB)
    // create the pair if it doesn't exist yet
    // if (IUniswapV2Factory(factory).getPair(tokenA, tokenB) == address(0)) {
    //     IUniswapV2Factory(factory).createPair(tokenA, tokenB);
    // }
    let factory_address = get_factory(&e);
    let factory = SoroswapFactoryClient::new(&e, &factory_address);
    if !factory.pair_exists(&token_a, &token_b) {
        factory.create_pair(&token_a, &token_b);
    }
    
    //  (uint reserveA, uint reserveB) = UniswapV2Library.getReserves(factory, tokenA, tokenB);
    //  TODO: Check if we can borrow the values instead
    let (reserve_a, reserve_b) = soroswap_library::get_reserves(e.clone(), factory_address.clone(), token_a.clone(), token_b.clone());
    
    //     if (reserveA == 0 && reserveB == 0) {
    if reserve_a == 0 && reserve_b == 0 {
        // (amountA, amountB) = (amountADesired, amountBDesired);
        (amount_a_desired, amount_b_desired)
    } else {
        // We try first with the amount a desired:
        // uint amountBOptimal = UniswapV2Library.quote(amountADesired, reserveA, reserveB);
        let amount_b_optimal = soroswap_library::quote(amount_a_desired.clone(), reserve_a.clone(), reserve_b.clone());
        // if (amountBOptimal <= amountBDesired) {
        if amount_b_optimal <= amount_b_desired{
            // require(amountBOptimal >= amountBMin, 'UniswapV2Router: INSUFFICIENT_B_AMOUNT');
            if amount_b_optimal < amount_b_min {
                panic!("SoroswapRouter: insufficient b amount")
            }
            // (amountA, amountB) = (amountADesired, amountBOptimal);
            (amount_a_desired, amount_b_optimal)
        }

        // If not, we can try with the amount b desired
        else {
            // uint amountAOptimal = UniswapV2Library.quote(amountBDesired, reserveB, reserveA);
            let amount_a_optimal = soroswap_library::quote(amount_b_desired, reserve_b, reserve_a);


            // assert(amountAOptimal <= amountADesired);
            // This should happen anyway. Because if we where not able to fulfill with our amount_b_desired  for our amount_a_desired
            // It is to expect that the amount_a_optimal for that lower amount_b_desired to be lower than the amount_a_desired
            assert!(amount_a_optimal <= amount_a_desired);

            // require(amountAOptimal >= amountAMin, 'UniswapV2Router: INSUFFICIENT_A_AMOUNT');
            if amount_a_optimal < amount_a_min {
                panic!("SoroswapRouter: insufficient a amount")
            }

            // (amountA, amountB) = (amountAOptimal, amountBDesired);
            (amount_a_optimal, amount_b_desired)
        }
    }
    
    


//     }
// }

} 



#[contract]
struct SoroswapRouter;

#[contractimpl]
impl SoroswapRouterTrait for SoroswapRouter {

    /// Initializes the contract and sets the factory address
    fn initialize(e: Env, factory: Address) {
        // constructor(address _factory, address _WETH) public {
        //     factory = _factory;
        //     WETH = _WETH;
        // }
        
        assert!(!has_factory(&e), "SoroswapRouter: already initialized");
        put_factory(&e, &factory);
    }
    
}
