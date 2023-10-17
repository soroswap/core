#![no_std]
mod test;
use soroban_sdk::{
    contract,
    contractimpl, Address, ConversionError, Env, Val, TryFromVal, Vec
};
use soroban_sdk::token::Client as TokenClient;
use soroswap_library;

mod factory {
    soroban_sdk::contractimport!(file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory_contract.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}

use factory::SoroswapFactoryClient;


mod pair {
    soroban_sdk::contractimport!(file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm");
    pub type SoroswapPairClient<'a> = Client<'a>;
}

use pair::SoroswapPairClient;


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

/// Panics if deadline has passed
// modifier ensure(uint deadline) {
//     require(deadline >= block.timestamp, 'UniswapV2Router: EXPIRED');
//     _;
// }
fn ensure_deadline(e: &Env, timestamp: u64) {
    let ledger_timestamp = e.ledger().timestamp();
    if ledger_timestamp >= timestamp{
        panic!("SoroswapRouter: expired")
    }
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
} 


// **** SWAP ****
// requires the initial amount to have already been sent to the first pair
// function _swap(uint[] memory amounts, address[] memory path, address _to) internal virtual {
fn swap(
    e: Env,
    amounts: Vec<i128>,
    path: Vec<Address>,
    _to: Address
){
    let factory_address = get_factory(&e);
    //     for (uint i; i < path.length - 1; i++) {
    for i in 0..path.len() - 1 { //  represents a half-open range, which includes the start value (0) but excludes the end value (path.len() - 1)
        // (address input, address output) = (path[i], path[i + 1]);
        let (input, output):(Address, Address) = (path.get(i).unwrap(), path.get(i+1).unwrap());

        // (address token0,) = UniswapV2Library.sortTokens(input, output);
        let (token_0, _token_1): (Address, Address) = soroswap_library::sort_tokens(input.clone(), output.clone());
        
        // uint amountOut = amounts[i + 1];
        let amount_out: i128 = amounts.get(i+1).unwrap();

        // (uint amount0Out, uint amount1Out) = input == token0 ? (uint(0), amountOut) : (amountOut, uint(0));
        let (amount_0_out, amount_1_out): (i128, i128) = if input == token_0 {
            (0, amount_out)
        } else {
            (amount_out, 0)
        };
        
        // before the end, "to" must be the next pair... "to" will be the user just at the end
        // address to = i < path.length - 2 ? UniswapV2Library.pairFor(factory, output, path[i + 2]) : _to;
        let to: Address = if i < path.len() - 2 {
            soroswap_library::pair_for(e.clone(), factory_address.clone(), output, path.get(i+2).unwrap())
        } else {
            _to.clone()
        };
        
        // TODO: Change swap function in pair
        // IUniswapV2Pair(UniswapV2Library.pairFor(factory, input, output)).swap(
        // amount0Out, amount1Out, to, new bytes(0)
        // );

        // fn swap(e: Env, to: Address, buy_a: bool, amount_out: i128, amount_in_max: i128);
        // SoroswapPairClient::new(&e, &soroswap_library::pair_for(
        //     e.clone(), factory_address.clone(), input, output))
        //     .swap(&to, );

        
    }
}

//     }
// }

pub trait SoroswapRouterTrait{

     /// Initializes the contract and sets the factory address
     fn initialize(e: Env, factory: Address);

    /// Add Liquidity to a Pool
    /// If a pool for the passed tokens does not exists, one is created automatically, 
    /// and exactly amountADesired/amountBDesired tokens are added.
     fn add_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address, 
        amount_a_desired: i128,
        amount_b_desired: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64
    ) -> (i128, i128, i128);

    /// Remove Liquidity to a Pool
    fn remove_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        liquidity: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64
    ) -> (i128, i128);

    fn swap_exact_tokens_for_tokens(
        e:Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64
    ) -> Vec<i128>;



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

    /// Add Liquidity to a pool
    /// If a pool for the passed tokens does not exists, one is created automatically, 
    /// and exactly amountADesired/amountBDesired tokens are added.
    // 
    // function addLiquidity(
    //     address tokenA,
    //     address tokenB,
    //     uint amountADesired,
    //     uint amountBDesired,
    //     uint amountAMin,
    //     uint amountBMin,
    //     address to,
    //     uint deadline
    fn add_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address, 
        amount_a_desired: i128,
        amount_b_desired: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64
    ) -> (i128, i128, i128) { // returns (uint amountA, uint amountB, uint liquidity)

        // In Soroban we don't need the user to have previously allowed, we can use to.require_auth();
        // and then take the tokens from the user
        to.require_auth();
        
        // ensure(deadline)
        ensure_deadline(&e, deadline);
        
        // (amountA, amountB) = _addLiquidity(tokenA, tokenB, amountADesired, amountBDesired, amountAMin, amountBMin);
        let (amount_a, amount_b) = add_liquidity_amounts(e.clone(), token_a.clone(), token_b.clone(), amount_a_desired, amount_b_desired, amount_a_min, amount_b_min);

        // address pair = UniswapV2Library.pairFor(factory, tokenA, tokenB);
        let pair: Address = soroswap_library::pair_for(e.clone(), get_factory(&e), token_a.clone(), token_b.clone());
        

        // TransferHelper.safeTransferFrom(tokenA, msg.sender, pair, amountA);
        // TransferHelper.safeTransferFrom(tokenB, msg.sender, pair, amountB);

        // TODO: Change Pair contracts so tokens are sent by the Router contract
        // In Soroban we will just make a simple transfer function because token contracts implement the require_auth()
        // transfer(&from, &to, &amount);
        // TokenClient::new(&e, &token_a).transfer(&to, &pair, &amount_a);
        // TokenClient::new(&e, &token_b).transfer(&to, &pair, &amount_b);
        // liquidity = IUniswapV2Pair(pair).mint(to);
        // let liquidity = SoroswapPairClient::new(&e, &pair).mint(&to);
        // For now we'll do:
        //  deposit(e: Env, to: Address, desired_a: i128, min_a: i128, desired_b: i128, min_b: i128);
        // TODO: Change in Pair so deposit returns the liquidity
        let liquidity = SoroswapPairClient::new(&e, &pair).deposit(&to, &amount_a, &amount_a, &amount_b, &amount_b);
        
        (amount_a,amount_b,liquidity)
    }

    /// Remove Liquidity to a Pool
    //  function removeLiquidity(
    // address tokenA,
    // address tokenB,
    // uint liquidity,
    // uint amountAMin,
    // uint amountBMin,
    // address to,
    // uint deadline
    fn remove_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        liquidity: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64
    ) -> (i128, i128) { // returns (uint amountA, uint amountB)
        // In Soroban we don't need the user to have previously allowed, we can use to.require_auth();
        // and then take the tokens from the user
        to.require_auth();
        
        // ensure(deadline)
        ensure_deadline(&e, deadline);
        
        // address pair = UniswapV2Library.pairFor(factory, tokenA, tokenB);
        let pair: Address = soroswap_library::pair_for(e.clone(), get_factory(&e), token_a.clone(), token_b.clone());
        
        // TODO: Change pair contract so tokens are being sent from the Router contract
        // IUniswapV2Pair(pair).transferFrom(msg.sender, pair, liquidity); // send liquidity to pair
        // (uint amount0, uint amount1) = IUniswapV2Pair(pair).burn(to);
        // For now we have:
        //fn withdraw(e: Env, to: Address, share_amount: i128, min_a: i128, min_b: i128) -> (i128, i128);
        let (amount_0, amount_1) = SoroswapPairClient::new(&e, &pair).withdraw(
            &to, &liquidity, &amount_a_min, &amount_b_min);
        
        // (address token0,) = UniswapV2Library.sortTokens(tokenA, tokenB);
        let (token_0,_token_1) = soroswap_library::sort_tokens(token_a.clone(), token_b.clone());

        // (amountA, amountB) = tokenA == token0 ? (amount0, amount1) : (amount1, amount0);
        let (amount_a, amount_b) = if token_a == token_0 {
            (amount_0, amount_1)
        } else{
            (amount_1, amount_0)
        };
        if amount_a < amount_a_min {panic!("SoroswapRouter: insufficient A amount")}
        if amount_b < amount_b_min {panic!("SoroswapRouter: insufficient B amount")}       
        
        
        (amount_a, amount_b)
    }
        

    fn swap_exact_tokens_for_tokens(
        e:Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64
    ) -> Vec<i128> {
        let mut amounts =  Vec::new(&e);
        amounts.push_back(amount_in);  
        amounts
    
    }
    // function swapExactTokensForTokens(
    //     uint amountIn,
    //     uint amountOutMin,
    //     address[] calldata path,
    //     address to,
    //     uint deadline
    // ) external virtual override ensure(deadline) returns (uint[] memory amounts) {
    //     amounts = UniswapV2Library.getAmountsOut(factory, amountIn, path);
    //     require(amounts[amounts.length - 1] >= amountOutMin, 'UniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNT');
    //     TransferHelper.safeTransferFrom(
    //         path[0], msg.sender, UniswapV2Library.pairFor(factory, path[0], path[1]), amounts[0]
    //     );
    //     _swap(amounts, path, to);
    // }



// }


    // }
    
}
