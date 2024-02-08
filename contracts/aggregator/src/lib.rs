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

use storage::{put_factory, has_factory, get_factory, extend_instance_ttl};
use models::DexDistribution;
pub use error::{SoroswapAggregatorError, CombinedAggregatorError};

mod dex_constants {
    pub const SOROSWAP: i32 = 0;
    pub const PHOENIX: i32 = 1;
}

pub fn check_nonnegative_amount(amount: i128) -> Result<(), CombinedAggregatorError> {
    if amount < 0 {
        Err(CombinedAggregatorError::RouterNegativeNotAllowed)
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
    if has_factory(e) {
        Ok(())
    } else {
        Err(CombinedAggregatorError::RouterNotInitialized)
    }
}

/*
    SOROSWAP AGGREGATOR SMART CONTRACT INTERFACE:
*/

pub trait SoroswapAggregatorTrait {

    /// Initializes the contract and sets the factory address
    // fn initialize(e: Env, factory: Address) -> Result<(), CombinedAggregatorError>;

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
    /// Initializes the contract and sets the factory address
    // fn initialize(e: Env, factory: Address) -> Result<(), CombinedAggregatorError> {
    //     if !has_factory(&e) {
    //         put_factory(&e, &factory);
    //         event::initialized(&e, factory);
    //         extend_instance_ttl(&e);
    //         Ok(())
    //     } else {
    //         Err(SoroswapAggregatorError::InitializeAlreadyInitialized.into())
    //     } 
        
    // }  

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
        // check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        check_nonnegative_amount(amount_out_min)?;
        extend_instance_ttl(&e);
        to.require_auth();
        ensure_deadline(&e, deadline)?;
       
        for dist in distribution {
            match dist.index {
                dex_constants::SOROSWAP => {
                    // Call function to handle swap via Soroswap
                    return Ok(0i128);
                },
                dex_constants::PHOENIX => {
                    // Call function to handle swap via Phoenix
                    return Ok(1i128);
                },
                _ => {
                    // Handle unknown index //Should return error
                    return Ok(100i128);
                }
            }
        }
        
        Ok(200i128)
    }

    /*  *** Read only functions: *** */

}
