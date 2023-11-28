#![no_std]
mod pair;
mod factory;
mod test;
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};
use factory::SoroswapFactoryClient;
use pair::SoroswapPairClient;


#[derive(Clone)]
#[contracttype]

pub enum DataKey {
    Factory, // Address of the Factory Contract
}


fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("SoroswapRouter: negative amount is not allowed: {}", amount)
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
    if ledger_timestamp >= timestamp {
        panic!("SoroswapRouter: expired")
    }
}

/// Transfer tokens from an account to another (requires require.auth)
fn transfer_from(e: &Env, token: &Address, from: &Address, to: &Address, value: &i128) {
    let token_client = TokenClient::new(&e, &token);
    token_client.transfer(&from, &to, &value);
}

/// Given a pair of tokens, a desired and minimum amount of tokens to provide as liquidity, this function calculates
/// the correct amounts of tokens to add to the pool. If the pool doesn't exist, it creates one.
///
/// It considers the desired and minimum amounts for both tokens and calculates the optimal distribution to
/// satisfy these requirements while taking into account the current reserves in the pool.
///
/// # Arguments
/// * `e` - The contract environment (`Env`) in which the contract is executing.
/// * `token_a` - The address of the first token in the pair.
/// * `token_b` - The address of the second token in the pair.
/// * `amount_a_desired` - The desired amount of the first token to add.
/// * `amount_b_desired` - The desired amount of the second token to add.
/// * `amount_a_min` - The minimum required amount of the first token to add.
/// * `amount_b_min` - The minimum required amount of the second token to add.
///
/// # Returns
/// A tuple containing the calculated amounts of token A and B to be added to the pool.
fn add_liquidity_amounts(
    e: Env,
    factory: Address,
    token_a: Address,
    token_b: Address,
    amount_a_desired: i128,
    amount_b_desired: i128,
    amount_a_min: i128,
    amount_b_min: i128,
) -> (i128, i128) {
    // checks if the pair exist, otherwise, creates the pair
    let factory_client = SoroswapFactoryClient::new(&e, &factory);
    if !factory_client.pair_exists(&token_a, &token_b) {
        factory_client.create_pair(&token_a, &token_b);
    }

    let (reserve_a, reserve_b) = soroswap_library::get_reserves(
        e.clone(),
        factory.clone(),
        token_a.clone(),
        token_b.clone(),
    );

    // When there is no liquidity (first deposit)
    if reserve_a == 0 && reserve_b == 0 {
        (amount_a_desired, amount_b_desired)
    } else {
        // We try first with the amount a desired:
        let amount_b_optimal = soroswap_library::quote(
            amount_a_desired.clone(),
            reserve_a.clone(),
            reserve_b.clone(),
        );

        if amount_b_optimal <= amount_b_desired {
            if amount_b_optimal < amount_b_min {
                panic!("SoroswapRouter: insufficient b amount")
            }
            (amount_a_desired, amount_b_optimal)
        }
        // If not, we can try with the amount b desired
        else {
            let amount_a_optimal = soroswap_library::quote(amount_b_desired, reserve_b, reserve_a);

            // This should happen anyway. Because if we where not able to fulfill with our amount_b_desired  for our amount_a_desired
            // It is to expect that the amount_a_optimal for that lower amount_b_desired to be lower than the amount_a_desired
            assert!(amount_a_optimal <= amount_a_desired);

            if amount_a_optimal < amount_a_min {
                panic!("SoroswapRouter: insufficient a amount")
            }
            (amount_a_optimal, amount_b_desired)
        }
    }
}

// **** SWAP ****
// requires the initial amount to have already been sent to the first pair
// function _swap(uint[] memory amounts, address[] memory path, address _to) internal virtual {
fn swap(e: &Env, amounts: &Vec<i128>, path: &Vec<Address>, _to: &Address) {
    let factory_address = get_factory(&e);
    //     for (uint i; i < path.length - 1; i++) {
    for i in 0..path.len() - 1 {
        //  represents a half-open range, which includes the start value (0) but excludes the end value (path.len() - 1)
        // (address input, address output) = (path[i], path[i + 1]);
        let (input, output): (Address, Address) = (path.get(i).unwrap(), path.get(i + 1).unwrap());

        // (address token0,) = UniswapV2Library.sortTokens(input, output);
        let (token_0, _token_1): (Address, Address) =
            soroswap_library::sort_tokens(input.clone(), output.clone());

        // uint amountOut = amounts[i + 1];
        let amount_out: i128 = amounts.get(i + 1).unwrap();

        // (uint amount0Out, uint amount1Out) = input == token0 ? (uint(0), amountOut) : (amountOut, uint(0));
        let (amount_0_out, amount_1_out): (i128, i128) = if input == token_0 {
            (0, amount_out)
        } else {
            (amount_out, 0)
        };

        // before the end, "to" must be the next pair... "to" will be the user just at the end
        // address to = i < path.length - 2 ? UniswapV2Library.pairFor(factory, output, path[i + 2]) : _to;
        let to: Address = if i < path.len() - 2 {
            soroswap_library::pair_for(
                e.clone(),
                factory_address.clone(),
                output.clone(),
                path.get(i + 2).unwrap(),
            )
        } else {
            _to.clone()
        };

        // IUniswapV2Pair(UniswapV2Library.pairFor(factory, input, output)).swap(
        // amount0Out, amount1Out, to, new bytes(0)
        // );
        // We dont use the bytes part of it in Soroswap

        //fn swap(e: Env, amount_0_out: i128, amount_1_out: i128, to: Address) {
        SoroswapPairClient::new(
            &e,
            &soroswap_library::pair_for(e.clone(), factory_address.clone(), input, output),
        )
        .swap(&amount_0_out, &amount_1_out, &to);
    }
}


/*
    SOROSWAP ROUTER SMART CONTRACT INTERFACE:
*/

pub trait SoroswapRouterTrait {

    /// Initializes the contract and sets the factory address
    fn initialize(e: Env, factory: Address);

    /// This function retrieves the factory contract's address associated with the provided environment.
    /// It also checks if the factory has been initialized and raises an assertion error if not.
    /// If the factory is not initialized, this code will raise an assertion error with the message "SoroswapRouter: not yet initialized".
    ///
    /// # Arguments
    /// * `e` - The contract environment (`Env`) in which the contract is executing.
    fn get_factory(e: Env) -> Address;

    /// Adds liquidity to a token pair's pool, creating it if it doesn't exist. Ensures that exactly the desired amounts
    /// of both tokens are added, subject to minimum requirements.
    ///
    /// This function is responsible for transferring tokens from the user to the pool and minting liquidity tokens in return.
    ///
    /// # Arguments
    /// * `e` - The contract environment (`Env`) in which the contract is executing.
    /// * `token_a` - The address of the first token to add liquidity for.
    /// * `token_b` - The address of the second token to add liquidity for.
    /// * `amount_a_desired` - The desired amount of the first token to add.
    /// * `amount_b_desired` - The desired amount of the second token to add.
    /// * `amount_a_min` - The minimum required amount of the first token to add.
    /// * `amount_b_min` - The minimum required amount of the second token to add.
    /// * `to` - The address where the liquidity tokens will be minted and sent.
    /// * `deadline` - The deadline for executing the operation.
    ///
    /// # Returns
    /// A tuple containing the actual amounts of token A and B added to the pool, as well as the amount of liquidity tokens minted.
    fn add_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        amount_a_desired: i128,
        amount_b_desired: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64,
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
        deadline: u64,
    ) -> (i128, i128);

    /// Swaps an exact amount of input tokens for as many output tokens as possible,
    /// along the route determined by the path. The first element of path is the input token,
    /// the last is the output token, and any intermediate elements represent intermediate
    /// pairs to trade through (if, for example, a direct pair does not exist).
    fn swap_exact_tokens_for_tokens(
        e: Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Vec<i128>;

    fn swap_tokens_for_exact_tokens(
        e: Env,
        amount_out: i128,
        amount_in_max: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Vec<i128>;

    /*
    LIBRARY FUNCTIONS:
    */

    /// given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    fn router_quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> i128;

    /// given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
    fn router_get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128;

    /// given an output amount of an asset and pair reserves, returns a required input amount of the other asset
    fn router_get_amount_in(amount_out: i128, reserve_in: i128, reserve_out: i128) -> i128;

    /// performs chained getAmountOut calculations on any number of pairs
    fn router_get_amounts_out(
        e: Env,
        amount_in: i128,
        path: Vec<Address>,
    ) -> Vec<i128>;

    /// performs chained getAmountIn calculations on any number of pairs
    fn router_get_amounts_in(
        e: Env,
        amount_out: i128,
        path: Vec<Address>,
    ) -> Vec<i128>;

    /// calculates a determinisic pair address
    fn router_pair_for(
        e: Env,
        token_a: Address,
        token_b: Address) -> Address;

}

#[contract]
struct SoroswapRouter;

#[contractimpl]
impl SoroswapRouterTrait for SoroswapRouter {
    /// Initializes the contract and sets the factory address
    fn initialize(e: Env, factory: Address) {
        assert!(!has_factory(&e), "SoroswapRouter: already initialized");
        put_factory(&e, &factory);
    } 

    /// This function retrieves the factory contract's address associated with the provided environment.
    /// It also checks if the factory has been initialized and raises an assertion error if not.
    /// If the factory is not initialized, this code will raise an assertion error with the message "SoroswapRouter: not yet initialized".
    ///
    /// # Arguments
    /// * `e` - The contract environment (`Env`) in which the contract is executing.
    fn get_factory(e: Env) -> Address {
        assert!(has_factory(&e), "SoroswapRouter: not yet initialized"); 
        let factory_address = get_factory(&e);
        factory_address
    }

    /// Adds liquidity to a token pair's pool, creating it if it doesn't exist. Ensures that exactly the desired amounts
    /// of both tokens are added, subject to minimum requirements.
    /// This function is responsible for transferring tokens from the user to the pool and minting liquidity tokens in return.
    /// # Arguments
    /// * `token_a` - The address of the first token to add liquidity for.
    /// * `token_b` - The address of the second token to add liquidity for.
    /// * `amount_a_desired` - The desired amount of the first token to add.
    /// * `amount_b_desired` - The desired amount of the second token to add.
    /// * `amount_a_min` - The minimum required amount of the first token to add.
    /// * `amount_b_min` - The minimum required amount of the second token to add.
    /// * `to` - The address where the liquidity tokens will be minted and sent.
    /// * `deadline` - The deadline for executing the operation.
    /// # Returns
    /// A tuple containing: amounts of token A and B added to the pool.
    /// plus the amount of liquidity tokens minted.
    fn add_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        amount_a_desired: i128,
        amount_b_desired: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64,
    ) -> (i128, i128, i128) {
        assert!(has_factory(&e), "SoroswapRouter: not yet initialized");
        check_nonnegative_amount(amount_a_desired);
        check_nonnegative_amount(amount_b_desired);
        check_nonnegative_amount(amount_a_min);
        check_nonnegative_amount(amount_b_min);

        to.require_auth();
        ensure_deadline(&e, deadline);

        let factory = get_factory(&e);

        // (amountA, amountB) = _addLiquidity(tokenA, tokenB, amountADesired, amountBDesired, amountAMin, amountBMin);
        let (amount_a, amount_b) = add_liquidity_amounts(
            e.clone(),
            factory.clone(),
            token_a.clone(),
            token_b.clone(),
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
        );

        // address pair = UniswapV2Library.pairFor(factory, tokenA, tokenB);
        let pair: Address = soroswap_library::pair_for(
            e.clone(),
            factory,
            token_a.clone(),
            token_b.clone(),
        );

        // TransferHelper.safeTransferFrom(tokenA, msg.sender, pair, amountA);
        // TransferHelper.safeTransferFrom(tokenB, msg.sender, pair, amountB);
        TokenClient::new(&e, &token_a).transfer(&to, &pair, &amount_a);
        TokenClient::new(&e, &token_b).transfer(&to, &pair, &amount_b);

        // liquidity = IUniswapV2Pair(pair).mint(to);
        let liquidity = SoroswapPairClient::new(&e, &pair).deposit(&to);

        (amount_a, amount_b, liquidity)
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
        deadline: u64,
    ) -> (i128, i128) {
        assert!(has_factory(&e), "SoroswapRouter: not yet initialized");
        check_nonnegative_amount(liquidity);
        check_nonnegative_amount(amount_a_min);
        check_nonnegative_amount(amount_b_min);
        // returns (uint amountA, uint amountB)
        // In Soroban we don't need the user to have previously allowed, we can use to.require_auth();
        // and then take the tokens from the user
        to.require_auth();

        // ensure(deadline)
        ensure_deadline(&e, deadline);

        //Require that the pair exist
        let factory_address = get_factory(&e);
        let factory = SoroswapFactoryClient::new(&e, &factory_address);
        assert!(factory.pair_exists(&token_a, &token_b), "SoroswapRouter: pair does not exist");

        // address pair = UniswapV2Library.pairFor(factory, tokenA, tokenB);
        let pair: Address = soroswap_library::pair_for(
            e.clone(),
            get_factory(&e),
            token_a.clone(),
            token_b.clone(),
        );

        // IUniswapV2Pair(pair).transferFrom(msg.sender, pair, liquidity); // send liquidity to pair
        transfer_from(&e, &pair, &to, &pair, &liquidity);

        // (uint amount0, uint amount1) = IUniswapV2Pair(pair).burn(to);

        let (amount_0, amount_1) = SoroswapPairClient::new(&e, &pair).withdraw(&to);

        // (address token0,) = UniswapV2Library.sortTokens(tokenA, tokenB);
        let (token_0, _token_1) = soroswap_library::sort_tokens(token_a.clone(), token_b.clone());

        // (amountA, amountB) = tokenA == token0 ? (amount0, amount1) : (amount1, amount0);
        let (amount_a, amount_b) = if token_a == token_0 {
            (amount_0, amount_1)
        } else {
            (amount_1, amount_0)
        };
        if amount_a < amount_a_min {
            panic!("SoroswapRouter: insufficient A amount")
        }
        if amount_b < amount_b_min {
            panic!("SoroswapRouter: insufficient B amount")
        }

        (amount_a, amount_b)
    }

    /// Swaps an exact amount of input tokens for as many output tokens as possible,
    /// along the route determined by the path. The first element of path is the input token,
    /// the last is the output token, and any intermediate elements represent intermediate
    /// pairs to trade through (if, for example, a direct pair does not exist).
    // function swapExactTokensForTokens(
    //     uint amountIn,
    //     uint amountOutMin,
    //     address[] calldata path,
    //     address to,
    //     uint deadline
    fn swap_exact_tokens_for_tokens(
        e: Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Vec<i128> {
        assert!(has_factory(&e), "SoroswapRouter: not yet initialized");
        check_nonnegative_amount(amount_in);
        check_nonnegative_amount(amount_out_min);
        
        // In Soroban we don't need the user to have previously allowed, we can use to.require_auth();
        // and then take the tokens from the user
        to.require_auth();

        // returns (uint[] memory amounts)

        // ensure(deadline)
        ensure_deadline(&e, deadline);

        // amounts = UniswapV2Library.getAmountsOut(factory, amountIn, path);
        let factory_address = get_factory(&e);
        let amounts = soroswap_library::get_amounts_out(
            e.clone(),
            factory_address.clone(),
            amount_in,
            path.clone(),
        );

        // require(amounts[amounts.length - 1] >= amountOutMin, 'UniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNT');
        if amounts.get(amounts.len() - 1).unwrap() < amount_out_min {
            panic!("SoroswapRouter: insufficient output amount")
        }

        //     TransferHelper.safeTransferFrom(
        //      path[0],
        // msg.sender,
        // UniswapV2Library.pairFor(factory, path[0], path[1]),
        // amounts[0]
        //     );
        // function safeTransferFrom(
        //     address token,
        //     address from,
        //     address to,
        //     uint256 value
        // )
        let pair = soroswap_library::pair_for(
            e.clone(),
            factory_address,
            path.get(0).unwrap(),
            path.get(1).unwrap(),
        );
        TokenClient::new(&e, &path.get(0).unwrap()).transfer(&to, &pair, &amounts.get(0).unwrap());

        // _swap(amounts, path, to);
        swap(&e, &amounts, &path, &to);

        // returns (uint[] memory amounts)
        amounts
    }

    fn swap_tokens_for_exact_tokens(
        e: Env,
        amount_out: i128,
        amount_in_max: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Vec<i128> {
        assert!(has_factory(&e), "SoroswapRouter: not yet initialized");
        check_nonnegative_amount(amount_out);
        check_nonnegative_amount(amount_in_max);


        // In Soroban we don't need the user to have previously allowed, we can use to.require_auth();
        // and then take the tokens from the user
        to.require_auth(); 

        // returns (uint[] memory amounts)
        // ensure(deadline)
        ensure_deadline(&e, deadline);

        // amounts = UniswapV2Library.getAmountsIn(factory, amountOut, path);
        let factory_address = get_factory(&e);
        let amounts = soroswap_library::get_amounts_in(
            e.clone(),
            factory_address.clone(),
            amount_out,
            path.clone(),
        );

        // require(amounts[0] <= amountInMax, 'UniswapV2Router: EXCESSIVE_INPUT_AMOUNT');
        if amounts.get(0).unwrap() > amount_in_max {
            panic!("SoroswapRouter: excessive input amount")
        }

        // TransferHelper.safeTransferFrom(
        //     path[0], // token
        //     msg.sender, // from
        //     UniswapV2Library.pairFor(factory, path[0], path[1]), // to
        //     amounts[0] // value
        // );
        let pair = soroswap_library::pair_for(
            e.clone(),
            factory_address,
            path.get(0).unwrap(),
            path.get(1).unwrap(),
        );
        transfer_from(
            &e,
            &path.get(0).unwrap(),
            &to,
            &pair,
            &amounts.get(0).unwrap(),
        );

        // _swap(amounts, path, to);
        swap(&e, &amounts, &path, &to);

        // returns (uint[] memory amounts)
        amounts
    }

    /// given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    // function quote(uint amountA, uint reserveA, uint reserveB) internal pure returns (uint amountB) {
    fn router_quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> i128 {
        soroswap_library::quote(amount_a, reserve_a, reserve_b)
    }

    /// given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
    // function getAmountOut(uint amountIn, uint reserveIn, uint reserveOut) internal pure returns (uint amountOut) {
    fn router_get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128 {
        soroswap_library::get_amount_out(amount_in, reserve_in, reserve_out)
    }

    /// given an output amount of an asset and pair reserves, returns a required input amount of the other asset
    // function getAmountIn(uint amountOut, uint reserveIn, uint reserveOut) internal pure returns (uint amountIn) {
    fn router_get_amount_in(amount_out: i128, reserve_in: i128, reserve_out: i128) -> i128 {
        soroswap_library::get_amount_in(amount_out, reserve_in, reserve_out)
    }

    /// performs chained getAmountOut calculations on any number of pairs
    // function getAmountsOut(address factory, uint amountIn, address[] memory path) internal view returns (uint[] memory amounts) {
    fn router_get_amounts_out(
        e: Env,
        amount_in: i128,
        path: Vec<Address>,
    ) -> Vec<i128> {
        assert!(has_factory(&e), "SoroswapRouter: not yet initialized");
        let factory = get_factory(&e);
        soroswap_library::get_amounts_out(e, factory, amount_in, path)
    }

    /// performs chained getAmountIn calculations on any number of pairs
    // function getAmountsIn(address factory, uint amountOut, address[] memory path) internal view returns (uint[] memory amounts) {
    fn router_get_amounts_in(
        e: Env,
        amount_out: i128,
        path: Vec<Address>,
    ) -> Vec<i128> {
        assert!(has_factory(&e), "SoroswapRouter: not yet initialized");
        let factory = get_factory(&e);
        soroswap_library::get_amounts_in(e, factory, amount_out, path)
    }

     /// calculates a determinisic pair address
     fn router_pair_for(
        e: Env,
        token_a: Address,
        token_b: Address) -> Address{
            soroswap_library::pair_for(
                e.clone(),
                get_factory(&e),
                token_a.clone(),
                token_b.clone(),
            )

        }

}
