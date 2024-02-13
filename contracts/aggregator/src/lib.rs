#![no_std]
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};
use soroswap_library::{SoroswapLibraryError};

mod models;
mod dex_interfaces;
mod error;
mod event;
mod storage;
mod test;

use storage::{put_soroswap_router, has_soroswap_router, get_soroswap_router, extend_instance_ttl};
use models::DexDistribution;
pub use error::{SoroswapAggregatorError, CombinedAggregatorError};
use crate::dex_interfaces::{dex_constants, soroswap_interface, phoenix_interface};

pub fn check_nonnegative_amount(amount: i128) -> Result<(), CombinedAggregatorError> {
    if amount < 0 {
        Err(CombinedAggregatorError::AggregatorNegativeNotAllowed)
    } else {
        Ok(())
    }
}

/// Panics if the specified deadline has passed.
///
/// # Arguments
/// * `e` - The runtime environment.
/// * `timestamp` - The deadline timestamp to compare against the current ledger timestamp.
fn ensure_deadline(e: &Env, timestamp: u64) -> Result<(), CombinedAggregatorError> {
    let ledger_timestamp = e.ledger().timestamp();
    if ledger_timestamp >= timestamp {
        Err(SoroswapAggregatorError::DeadlineExpired.into())
    } else {
        Ok(())
    }
}

fn check_initialized(e: &Env) -> Result<(), CombinedAggregatorError> {
    if has_soroswap_router(e) {
        Ok(())
    } else {
        Err(CombinedAggregatorError::AggregatorNotInitialized)
    }
}

/*
    SOROSWAP AGGREGATOR SMART CONTRACT INTERFACE:
*/

pub trait SoroswapAggregatorTrait {

    /// Initializes the contract and sets the soroswap_router address
    fn initialize(e: Env, soroswap_router: Address) -> Result<(), CombinedAggregatorError>;

    /// Executes a swap operation distributed across multiple decentralized exchanges (DEXes) as specified
    /// by the `distribution`. Each entry in the distribution details which DEX to use, the path of tokens
    /// for swap (if applicable), and the portion of the total `amount` to swap through that DEX. This 
    /// function aims to optimize the swap by leveraging different DEX protocols based on the distribution
    /// strategy to minimize slippage and maximize output.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    /// * `from_token` - The address of the input token to swap.
    /// * `dest_token` - The address of the destination token to receive.
    /// * `amount` - The total amount of `from_token` to be swapped.
    /// * `amount_out_min` - The minimum amount of `dest_token` expected to receive, ensuring the swap 
    ///   does not proceed under unfavorable conditions.
    /// * `distribution` - A vector of `DexDistribution` specifying how the total swap amount is distributed 
    ///   across different DEX protocols, including the swap path for each (if required by the DEX).
    /// * `to` - The recipient address for the `dest_token`.
    /// * `deadline` - A Unix timestamp marking the deadline by which the swap must be completed.
    ///
    /// # Returns
    /// The total amount of `dest_token` received from the swap if successful, encapsulated in a `Result`.
    /// On failure, returns a `CombinedAggregatorError` detailing the cause of the error.
    fn swap(
        e: Env,
        from_token: Address,
        dest_token: Address,
        amount: i128,
        amount_out_min: i128,
        distribution: Vec<DexDistribution>,
        to: Address,
        deadline: u64,
    ) -> Result<i128, CombinedAggregatorError>;

    /*  *** Read only functions: *** */

}

#[contract]
struct SoroswapAggregator;

#[contractimpl]
impl SoroswapAggregatorTrait for SoroswapAggregator {
    /// Initializes the contract and sets the soroswap_router address
    fn initialize(e: Env, soroswap_router: Address) -> Result<(), CombinedAggregatorError> {
        if !has_soroswap_router(&e) {
            put_soroswap_router(&e, &soroswap_router);
            event::initialized(&e, soroswap_router);
            extend_instance_ttl(&e);
            Ok(())
        } else {
            Err(SoroswapAggregatorError::InitializeAlreadyInitialized.into())
        } 
        
    }  

    fn swap(
        e: Env,
        from_token: Address,
        dest_token: Address,
        amount: i128,
        amount_out_min: i128,
        distribution: Vec<DexDistribution>,
        to: Address,
        deadline: u64,
    ) -> Result<i128, CombinedAggregatorError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        check_nonnegative_amount(amount_out_min)?;
        extend_instance_ttl(&e);
        to.require_auth();
        ensure_deadline(&e, deadline)?;

        let total_parts: i128 = distribution.iter().map(|dist| dist.parts).sum();

        let mut total_swapped: i128 = 0;
       
        for dist in distribution.iter() {
            let swap_amount = (amount * dist.parts) / total_parts;
            
            match dist.index {
                dex_constants::SOROSWAP => {
                    // Call function to handle swap via Soroswap
                    let swap_result = soroswap_interface::swap_with_soroswap(&e, &swap_amount, dist.path.clone(), to.clone(), deadline.clone())?;
                    total_swapped += swap_result;
                },
                dex_constants::PHOENIX => {
                    // Call function to handle swap via Phoenix
                    let swap_result = phoenix_interface::swap_with_phoenix(&e, &swap_amount, dist.path.clone())?;
                    total_swapped += swap_result;
                },
                _ => return Err(CombinedAggregatorError::AggregatorUnsupportedProtocol),
            }
        }
        
        Ok(total_swapped)
    }

    /*  *** Read only functions: *** */

}
