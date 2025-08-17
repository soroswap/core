#![no_std]
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};
use soroswap_library::SoroswapLibraryError;

mod error;
mod event;
mod factory;
mod pair;
mod storage;
mod test;

pub use error::{CombinedRouterError, SoroswapRouterError};
use factory::SoroswapFactoryClient;
use pair::SoroswapPairClient;
use storage::{extend_instance_ttl, get_factory, has_factory, put_factory};

pub fn check_nonnegative_amount(amount: i128) -> Result<(), CombinedRouterError> {
    if amount < 0 {
        Err(CombinedRouterError::RouterNegativeNotAllowed)
    } else {
        Ok(())
    }
}

/// Panics if the specified deadline has passed.
///
/// # Arguments
/// * `e` - The runtime environment.
/// * `timestamp` - The deadline timestamp to compare against the current ledger timestamp.
fn ensure_deadline(e: &Env, timestamp: u64) -> Result<(), CombinedRouterError> {
    let ledger_timestamp = e.ledger().timestamp();
    if ledger_timestamp >= timestamp {
        Err(SoroswapRouterError::DeadlineExpired.into())
    } else {
        Ok(())
    }
}

fn check_initialized(e: &Env) -> Result<(), CombinedRouterError> {
    if has_factory(e) {
        Ok(())
    } else {
        Err(CombinedRouterError::RouterNotInitialized)
    }
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
) -> Result<(i128, i128), CombinedRouterError> {
    // checks if the pair exists; otherwise, creates the pair
    let factory_client = SoroswapFactoryClient::new(&e, &factory);
    if !factory_client.pair_exists(&token_a, &token_b) {
        factory_client.create_pair(&token_a, &token_b);
    }

    let (reserve_a, reserve_b) = soroswap_library::get_reserves_with_factory(
        e.clone(),
        factory.clone(),
        token_a.clone(),
        token_b.clone(),
    )?;

    // When there is no liquidity (first deposit)
    if reserve_a == 0 && reserve_b == 0 {
        Ok((amount_a_desired, amount_b_desired))
    } else {
        // We try first with the amount a desired:
        let amount_b_optimal = soroswap_library::quote(
            amount_a_desired.clone(),
            reserve_a.clone(),
            reserve_b.clone(),
        )?;

        if amount_b_optimal <= amount_b_desired {
            if amount_b_optimal < amount_b_min {
                return Err(SoroswapRouterError::InsufficientBAmount.into());
            }
            Ok((amount_a_desired, amount_b_optimal))
        }
        // If not, we can try with the amount b desired
        else {
            let amount_a_optimal = soroswap_library::quote(amount_b_desired, reserve_b, reserve_a)
                .map_err(SoroswapLibraryError::from)?;

            // This should happen anyway. Because if we were not able to fulfill with our amount_b_desired for our amount_a_desired
            // It is to expect that the amount_a_optimal for that lower amount_b_desired to be lower than the amount_a_desired
            assert!(amount_a_optimal <= amount_a_desired);

            if amount_a_optimal < amount_a_min {
                return Err(SoroswapRouterError::InsufficientAAmount.into());
            }
            Ok((amount_a_optimal, amount_b_desired))
        }
    }
}

/// Executes a series of token swaps along the provided trading route.
/// Requires that the initial amount has already been sent to the first pair in the route.
///
/// # Arguments
/// * `e` - The runtime environment.
/// * `factory_address` - The address of the Soroswap factory contract.
/// * `amounts` - A vector containing the output amounts for each step of the trading route.
/// * `path` - A vector representing the trading route, where each element is a token address.
/// * `_to` - The final destination address for the swapped tokens.
fn swap(
    e: &Env,
    factory_address: &Address,
    amounts: &Vec<i128>,
    path: &Vec<Address>,
    _to: &Address,
) -> Result<(), CombinedRouterError> {
    for i in 0..path.len() - 1 {
        //  represents a half-open range, which includes the start value (0) but excludes the end value (path.len() - 1)
        let (input, output): (Address, Address) = (path.get(i).unwrap(), path.get(i + 1).unwrap());

        let (token_0, _token_1): (Address, Address) =
            (soroswap_library::sort_tokens(input.clone(), output.clone()))?;

        let amount_out: i128 = amounts.get(i + 1).unwrap();

        let (amount_0_out, amount_1_out): (i128, i128) = if input == token_0 {
            (0, amount_out)
        } else {
            (amount_out, 0)
        };

        // before the end, "to" must be the next pair... "to" will be the user only at the end
        let to: Address = if i < path.len() - 2 {
            soroswap_library::pair_for(
                e.clone(),
                factory_address.clone(),
                output.clone(),
                path.get(i + 2).unwrap(),
            )?
        } else {
            _to.clone()
        };

        SoroswapPairClient::new(
            &e,
            &soroswap_library::pair_for(e.clone(), factory_address.clone(), input, output)?,
        )
        .swap(&amount_0_out, &amount_1_out, &to);
    }

    Ok(())
}

/*
    SOROSWAP ROUTER SMART CONTRACT INTERFACE:
*/

pub trait SoroswapRouterTrait {
    /// Initializes the contract and sets the factory address
    fn initialize(e: Env, factory: Address) -> Result<(), CombinedRouterError>;

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
    ) -> Result<(i128, i128, i128), CombinedRouterError>;

    /// Removes liquidity from a token pair's pool.
    ///
    /// This function facilitates the removal of liquidity from a Soroswap Liquidity Pool by burning a specified amount
    /// of Liquidity Pool tokens (`liquidity`) owned by the caller. In return, it transfers back the corresponding
    /// amounts of the paired tokens (`token_a` and `token_b`) to the caller's specified address (`to`).
    ///
    /// # Arguments
    /// * `token_a` - The address of the first token in the Liquidity Pool.
    /// * `token_b` - The address of the second token in the Liquidity Pool.
    /// * `liquidity` - The desired amount of Liquidity Pool tokens to be burned.
    /// * `amount_a_min` - The minimum required amount of the first token to receive.
    /// * `amount_b_min` - The minimum required amount of the second token to receive.
    /// * `to` - The address where the paired tokens will be sent to, and from where the LP tokens will be taken.
    /// * `deadline` - The deadline for executing the operation.
    ///
    /// # Returns
    /// A tuple containing the amounts of `token_a` and `token_b` withdrawn from the pool.  
    fn remove_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        liquidity: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64,
    ) -> Result<(i128, i128), CombinedRouterError>;

    /// Swaps an exact amount of input tokens for as many output tokens as possible
    /// along the specified trading route. The route is determined by the `path` vector,
    /// where the first element is the input token, the last is the output token,
    /// and any intermediate elements represent pairs to trade through if a direct pair does not exist.
    ///
    /// # Arguments
    /// * `amount_in` - The exact amount of input tokens to be swapped.
    /// * `amount_out_min` - The minimum required amount of output tokens to receive.
    /// * `path` - A vector representing the trading route, where the first element is the input token
    ///            and the last is the output token. Intermediate elements represent pairs to trade through.
    /// * `to` - The address where the output tokens will be sent to.
    /// * `deadline` - The deadline for executing the operation.
    ///
    /// # Returns
    /// A vector containing the amounts of tokens received at each step of the trading route.
    fn swap_exact_tokens_for_tokens(
        e: Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, CombinedRouterError>;

    /// Swaps tokens for an exact amount of output token, following the specified trading route.
    /// The route is determined by the `path` vector, where the first element is the input token,
    /// the last is the output token, and any intermediate elements represent pairs to trade through.
    ///
    /// # Arguments
    /// * `amount_out` - The exact amount of output token to be received.
    /// * `amount_in_max` - The maximum allowed amount of input tokens to be swapped.
    /// * `path` - A vector representing the trading route, where the first element is the input token
    ///            and the last is the output token. Intermediate elements represent pairs to trade through.
    /// * `to` - The address where the output tokens will be sent to.
    /// * `deadline` - The deadline for executing the operation.
    ///
    /// # Returns
    /// A vector containing the amounts of tokens used at each step of the trading route.
    fn swap_tokens_for_exact_tokens(
        e: Env,
        amount_out: i128,
        amount_in_max: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, CombinedRouterError>;

    /*  *** Read only functions: *** */

    /// This function retrieves the factory contract's address associated with the provided environment.
    /// It also checks if the factory has been initialized and raises an assertion error if not.
    /// If the factory is not initialized, this code will raise an assertion error with the message "SoroswapRouter: not yet initialized".
    ///
    /// # Arguments
    /// * `e` - The contract environment (`Env`) in which the contract is executing.
    fn get_factory(e: Env) -> Result<Address, CombinedRouterError>;

    /*
    LIBRARY FUNCTIONS:
    */

    /// Calculates the deterministic address for a pair without making any external calls.
    /// check <https://github.com/paltalabs/deterministic-address-soroban>
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `token_a` - The address of the first token.
    /// * `token_b` - The address of the second token.
    ///
    /// # Returns
    ///
    /// Returns `Result<Address, SoroswapLibraryError>` where `Ok` contains the deterministic address for the pair, and `Err` indicates an error such as identical tokens or an issue with sorting.
    fn router_pair_for(
        e: Env,
        token_a: Address,
        token_b: Address,
    ) -> Result<Address, CombinedRouterError>;

    /// Given some amount of an asset and pair reserves, returns an equivalent amount of the other asset.
    ///
    /// # Arguments
    ///
    /// * `amount_a` - The amount of the first asset.
    /// * `reserve_a` - Reserves of the first asset in the pair.
    /// * `reserve_b` - Reserves of the second asset in the pair.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the calculated equivalent amount, and `Err` indicates an error such as insufficient amount or liquidity
    fn router_quote(
        amount_a: i128,
        reserve_a: i128,
        reserve_b: i128,
    ) -> Result<i128, CombinedRouterError>;

    /// Given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset.
    ///
    /// # Arguments
    ///
    /// * `amount_in` - The input amount of the asset.
    /// * `reserve_in` - Reserves of the input asset in the pair.
    /// * `reserve_out` - Reserves of the output asset in the pair.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the calculated maximum output amount, and `Err` indicates an error such as insufficient input amount or liquidity.
    fn router_get_amount_out(
        amount_in: i128,
        reserve_in: i128,
        reserve_out: i128,
    ) -> Result<i128, CombinedRouterError>;

    /// Given an output amount of an asset and pair reserves, returns a required input amount of the other asset.
    ///
    /// # Arguments
    ///
    /// * `amount_out` - The output amount of the asset.
    /// * `reserve_in` - Reserves of the input asset in the pair.
    /// * `reserve_out` - Reserves of the output asset in the pair.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the required input amount, and `Err` indicates an error such as insufficient output amount or liquidity.
    fn router_get_amount_in(
        amount_out: i128,
        reserve_in: i128,
        reserve_out: i128,
    ) -> Result<i128, CombinedRouterError>;

    /// Performs chained get_amount_out calculations on any number of pairs.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `amount_in` - The input amount.
    /// * `path` - Vector of token addresses representing the path.
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<i128>, SoroswapLibraryError>` where `Ok` contains a vector of calculated amounts, and `Err` indicates an error such as an invalid path.
    fn router_get_amounts_out(
        e: Env,
        amount_in: i128,
        path: Vec<Address>,
    ) -> Result<Vec<i128>, CombinedRouterError>;

    /// Performs chained get_amount_in calculations on any number of pairs.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `amount_out` - The output amount.
    /// * `path` - Vector of token addresses representing the path.
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<i128>, SoroswapLibraryError>` where `Ok` contains a vector of calculated amounts, and `Err` indicates an error such as an invalid path.
    fn router_get_amounts_in(
        e: Env,
        amount_out: i128,
        path: Vec<Address>,
    ) -> Result<Vec<i128>, CombinedRouterError>;
}

#[contract]
struct SoroswapRouter;

#[contractimpl]
impl SoroswapRouterTrait for SoroswapRouter {
    /// Initializes the contract and sets the factory address
    fn initialize(e: Env, factory: Address) -> Result<(), CombinedRouterError> {
        if !has_factory(&e) {
            put_factory(&e, &factory);
            event::initialized(&e, factory);
            extend_instance_ttl(&e);
            Ok(())
        } else {
            Err(SoroswapRouterError::InitializeAlreadyInitialized.into())
        }
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
    ) -> Result<(i128, i128, i128), CombinedRouterError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount_a_desired)?;
        check_nonnegative_amount(amount_b_desired)?;
        check_nonnegative_amount(amount_a_min)?;
        check_nonnegative_amount(amount_b_min)?;
        extend_instance_ttl(&e);
        to.require_auth();
        ensure_deadline(&e, deadline)?;

        let factory = get_factory(&e);

        let (amount_a, amount_b) = add_liquidity_amounts(
            e.clone(),
            factory.clone(),
            token_a.clone(),
            token_b.clone(),
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
        )?;

        let pair: Address =
            soroswap_library::pair_for(e.clone(), factory, token_a.clone(), token_b.clone())
                .map_err(SoroswapLibraryError::from)?;

        TokenClient::new(&e, &token_a).transfer(&to, &pair, &amount_a);
        TokenClient::new(&e, &token_b).transfer(&to, &pair, &amount_b);

        let liquidity = SoroswapPairClient::new(&e, &pair).deposit(&to);

        event::add_liquidity(
            &e, token_a, token_b, pair, amount_a, amount_b, liquidity, to,
        );

        Ok((amount_a, amount_b, liquidity))
    }

    /// Removes liquidity from a token pair's pool.
    ///
    /// This function facilitates the removal of liquidity from a Soroswap Liquidity Pool by burning a specified amount
    /// of Liquidity Pool tokens (`liquidity`) owned by the caller. In return, it transfers back the corresponding
    /// amounts of the paired tokens (`token_a` and `token_b`) to the caller's specified address (`to`).
    ///
    /// # Arguments
    /// * `token_a` - The address of the first token in the Liquidity Pool.
    /// * `token_b` - The address of the second token in the Liquidity Pool.
    /// * `liquidity` - The desired amount of Liquidity Pool tokens to be burned.
    /// * `amount_a_min` - The minimum required amount of the first token to receive.
    /// * `amount_b_min` - The minimum required amount of the second token to receive.
    /// * `to` - The address where the paired tokens will be sent to, and from where the LP tokens will be taken.
    /// * `deadline` - The deadline for executing the operation.
    ///
    /// # Returns
    /// A tuple containing the amounts of `token_a` and `token_b` withdrawn from the pool.
    fn remove_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        liquidity: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64,
    ) -> Result<(i128, i128), CombinedRouterError> {
        check_initialized(&e)?;
        check_nonnegative_amount(liquidity)?;
        check_nonnegative_amount(amount_a_min)?;
        check_nonnegative_amount(amount_b_min)?;
        extend_instance_ttl(&e);
        to.require_auth();
        ensure_deadline(&e, deadline)?;

        // Ensure that the pair exists in the Soroswap factory
        let factory_address = get_factory(&e);
        let factory = SoroswapFactoryClient::new(&e, &factory_address);

        if !factory.pair_exists(&token_a, &token_b) {
            return Err(SoroswapRouterError::PairDoesNotExist.into());
        }

        // Retrieve the pair's contract address using the Soroswap library
        let pair: Address = soroswap_library::pair_for(
            e.clone(),
            get_factory(&e),
            token_a.clone(),
            token_b.clone(),
        )?;

        // Transfer LP tokens from the caller to the pair contract
        TokenClient::new(&e, &pair).transfer(&to, &pair, &liquidity);

        // Withdraw paired tokens from the pool
        let (amount_0, amount_1) = SoroswapPairClient::new(&e, &pair).withdraw(&to);

        // Sort tokens to match the expected order
        let (token_0, _token_1) = soroswap_library::sort_tokens(token_a.clone(), token_b.clone())?;
        let (amount_a, amount_b) = if token_a == token_0 {
            (amount_0, amount_1)
        } else {
            (amount_1, amount_0)
        };

        // Check if the received amounts meet the minimum requirements
        if amount_a < amount_a_min {
            return Err(SoroswapRouterError::InsufficientAAmount.into());
        }
        if amount_b < amount_b_min {
            return Err(SoroswapRouterError::InsufficientBAmount.into());
        }

        event::remove_liquidity(
            &e, token_a, token_b, pair, amount_a, amount_b, liquidity, to,
        );

        // Return the amounts of paired tokens withdrawn
        Ok((amount_a, amount_b))
    }

    /// Swaps an exact amount of input tokens for as many output tokens as possible
    /// along the specified trading route. The route is determined by the `path` vector,
    /// where the first element is the input token, the last is the output token,
    /// and any intermediate elements represent pairs to trade through if a direct pair does not exist.
    ///
    /// # Arguments
    /// * `amount_in` - The exact amount of input tokens to be swapped.
    /// * `amount_out_min` - The minimum required amount of output tokens to receive.
    /// * `path` - A vector representing the trading route, where the first element is the input token
    ///            and the last is the output token. Intermediate elements represent pairs to trade through.
    /// * `to` - The address where the output tokens will be sent to.
    /// * `deadline` - The deadline for executing the operation.
    ///
    /// # Returns
    /// A vector containing the amounts of tokens received at each step of the trading route.
    fn swap_exact_tokens_for_tokens(
        e: Env,
        amount_in: i128,
        amount_out_min: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, CombinedRouterError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount_in)?;
        check_nonnegative_amount(amount_out_min)?;
        extend_instance_ttl(&e);
        to.require_auth();
        ensure_deadline(&e, deadline)?;

        // Get the expected output amounts for each step of the trading route
        let factory_address = get_factory(&e);
        let amounts = soroswap_library::get_amounts_out(
            e.clone(),
            factory_address.clone(),
            amount_in,
            path.clone(),
        )?;

        // Ensure that the final output amount meets the minimum requirement
        if amounts.get(amounts.len() - 1).unwrap() < amount_out_min {
            return Err(SoroswapRouterError::InsufficientOutputAmount.into());
        }

        // Determine the pair contract address for the first step of the trading route
        let pair = soroswap_library::pair_for(
            e.clone(),
            factory_address.clone(),
            path.get(0).unwrap(),
            path.get(1).unwrap(),
        )?;

        // Transfer input tokens to the pair contract
        // If the pair does not exist, this will fail here: Should be implement factory.pair_exists?
        // If we implement, we will include an additional cross-contract call...
        TokenClient::new(&e, &path.get(0).unwrap()).transfer(&to, &pair, &amounts.get(0).unwrap());

        // Execute the tokens swap
        swap(&e, &factory_address, &amounts, &path, &to)?;

        event::swap(&e, path, amounts.clone(), to);

        // Return the amounts of tokens received at each step of the trading route
        Ok(amounts)
    }

    /// Swaps tokens for an exact amount of output token, following the specified trading route.
    /// The route is determined by the `path` vector, where the first element is the input token,
    /// the last is the output token, and any intermediate elements represent pairs to trade through.
    ///
    /// # Arguments
    /// * `amount_out` - The exact amount of output token to be received.
    /// * `amount_in_max` - The maximum allowed amount of input tokens to be swapped.
    /// * `path` - A vector representing the trading route, where the first element is the input token
    ///            and the last is the output token. Intermediate elements represent pairs to trade through.
    /// * `to` - The address where the output tokens will be sent to.
    /// * `deadline` - The deadline for executing the operation.
    ///
    /// # Returns
    /// A vector containing the amounts of tokens used at each step of the trading route.
    fn swap_tokens_for_exact_tokens(
        e: Env,
        amount_out: i128,
        amount_in_max: i128,
        path: Vec<Address>,
        to: Address,
        deadline: u64,
    ) -> Result<Vec<i128>, CombinedRouterError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount_out)?;
        check_nonnegative_amount(amount_in_max)?;
        extend_instance_ttl(&e);
        to.require_auth();
        ensure_deadline(&e, deadline)?;

        // Get the expected input amounts for each step of the trading route
        let factory_address = get_factory(&e);
        let amounts = soroswap_library::get_amounts_in(
            e.clone(),
            factory_address.clone(),
            amount_out,
            path.clone(),
        )?;

        // Ensure that the input amount does not exceed the maximum allowed
        if amounts.get(0).unwrap() > amount_in_max {
            return Err(SoroswapRouterError::ExcessiveInputAmount.into());
        }

        // Determine the pair contract address for the first step of the trading route
        let pair = soroswap_library::pair_for(
            e.clone(),
            factory_address.clone(),
            path.get(0).unwrap(),
            path.get(1).unwrap(),
        )?;
        // Transfer input tokens to the pair contract
        // If the pair does not exist, this will fail here: Should be implement factory.pair_exists?
        // If we implement, we will include an additional cross-contract call...
        TokenClient::new(&e, &path.get(0).unwrap()).transfer(&to, &pair, &amounts.get(0).unwrap());

        // Execute the token swap
        swap(&e, &factory_address, &amounts, &path, &to)?;

        event::swap(&e, path, amounts.clone(), to);

        // Return the amounts of tokens used at each step of the trading route
        Ok(amounts)
    }

    /*  *** Read only functions: *** */

    /// This function retrieves the factory contract's address associated with the provided environment.
    /// It also checks if the factory has been initialized and raises an assertion error if not.
    /// If the factory is not initialized, this code will raise an assertion error with the message "SoroswapRouter: not yet initialized".
    ///https://github.com/benjaminsalon/malicious_sorochat
    /// # Arguments
    /// * `e` - The contract environment (`Env`) in which the contract is executing.
    fn get_factory(e: Env) -> Result<Address, CombinedRouterError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        let factory_address = get_factory(&e);
        Ok(factory_address)
    }

    /// Calculates the deterministic address for a pair without making any external calls.
    /// check <https://github.com/paltalabs/deterministic-address-soroban>
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `token_a` - The address of the first token.
    /// * `token_b` - The address of the second token.
    ///
    /// # Returns
    ///
    /// Returns `Result<Address, SoroswapLibraryError>` where `Ok` contains the deterministic address for the pair, and `Err` indicates an error such as identical tokens or an issue with sorting.
    fn router_pair_for(
        e: Env,
        token_a: Address,
        token_b: Address,
    ) -> Result<Address, CombinedRouterError> {
        extend_instance_ttl(&e);
        Ok(soroswap_library::pair_for(
            e.clone(),
            get_factory(&e),
            token_a.clone(),
            token_b.clone(),
        )?)
    }

    /// Given some amount of an asset and pair reserves, returns an equivalent amount of the other asset.
    ///
    /// # Arguments
    ///
    /// * `amount_a` - The amount of the first asset.
    /// * `reserve_a` - Reserves of the first asset in the pair.
    /// * `reserve_b` - Reserves of the second asset in the pair.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the calculated equivalent amount, and `Err` indicates an error such as insufficient amount or liquidity
    fn router_quote(
        amount_a: i128,
        reserve_a: i128,
        reserve_b: i128,
    ) -> Result<i128, CombinedRouterError> {
        Ok(soroswap_library::quote(amount_a, reserve_a, reserve_b)?)
    }

    /// Given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset.
    ///
    /// # Arguments
    ///
    /// * `amount_in` - The input amount of the asset.
    /// * `reserve_in` - Reserves of the input asset in the pair.
    /// * `reserve_out` - Reserves of the output asset in the pair.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the calculated maximum output amount, and `Err` indicates an error such as insufficient input amount or liquidity.
    fn router_get_amount_out(
        amount_in: i128,
        reserve_in: i128,
        reserve_out: i128,
    ) -> Result<i128, CombinedRouterError> {
        Ok(soroswap_library::get_amount_out(
            amount_in,
            reserve_in,
            reserve_out,
        )?)
    }

    /// Given an output amount of an asset and pair reserves, returns a required input amount of the other asset.
    ///
    /// # Arguments
    ///
    /// * `amount_out` - The output amount of the asset.
    /// * `reserve_in` - Reserves of the input asset in the pair.
    /// * `reserve_out` - Reserves of the output asset in the pair.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the required input amount, and `Err` indicates an error such as insufficient output amount or liquidity.
    fn router_get_amount_in(
        amount_out: i128,
        reserve_in: i128,
        reserve_out: i128,
    ) -> Result<i128, CombinedRouterError> {
        Ok(soroswap_library::get_amount_in(
            amount_out,
            reserve_in,
            reserve_out,
        )?)
    }

    /// Performs chained get_amount_out calculations on any number of pairs.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `amount_in` - The input amount.
    /// * `path` - Vector of token addresses representing the path.
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<i128>, SoroswapLibraryError>` where `Ok` contains a vector of calculated amounts, and `Err` indicates an error such as an invalid path.
    fn router_get_amounts_out(
        e: Env,
        amount_in: i128,
        path: Vec<Address>,
    ) -> Result<Vec<i128>, CombinedRouterError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        let factory = get_factory(&e);
        Ok(soroswap_library::get_amounts_out(
            e, factory, amount_in, path,
        )?)
    }

    /// Performs chained get_amount_in calculations on any number of pairs.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `amount_out` - The output amount.
    /// * `path` - Vector of token addresses representing the path.
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<i128>, SoroswapLibraryError>` where `Ok` contains a vector of calculated amounts, and `Err` indicates an error such as an invalid path.
    fn router_get_amounts_in(
        e: Env,
        amount_out: i128,
        path: Vec<Address>,
    ) -> Result<Vec<i128>, CombinedRouterError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        let factory = get_factory(&e);
        Ok(soroswap_library::get_amounts_in(
            e, factory, amount_out, path,
        )?)
    }
}
