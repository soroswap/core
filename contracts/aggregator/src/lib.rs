#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec, vec};

mod models;
mod dex_interfaces;
mod error;
mod event;
mod storage;
mod test;

use storage::{put_protocol_address, has_protocol_address, get_protocol_address, extend_instance_ttl, is_initialized, set_initialized, set_admin, get_admin};
use models::{DexDistribution, ProtocolAddressPair};
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
    if is_initialized(e) {
        Ok(())
    } else {
        Err(CombinedAggregatorError::AggregatorNotInitialized)
    }
}

fn is_valid_protocol(protocol_id: i32) -> bool {
    match protocol_id {
        dex_constants::SOROSWAP | dex_constants::PHOENIX => true,
        // Add additional protocols here as needed
        _ => false,
    }
}

/*
    SOROSWAP AGGREGATOR SMART CONTRACT INTERFACE:
*/

pub trait SoroswapAggregatorTrait {

    /// Initializes the contract and sets the soroswap_router address
    fn initialize(e: Env, admin: Address, protocol_addresses: Vec<ProtocolAddressPair>) -> Result<(), CombinedAggregatorError>;

    /// Updates the protocol addresses for the aggregator
    fn update_protocols(
        e: Env,
        protocol_addresses: Vec<ProtocolAddressPair>,
    ) -> Result<(), CombinedAggregatorError>;
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

    /// Returns the expected return amount for a given input amount and distribution
    // fn getExpectedReturn(
    //     e: Env,
    //     from_token: Address,
    //     dest_token: Address,
    //     amount: i128,
    //     parts: i128,
    // ) -> Result<i128, CombinedAggregatorError>;

    /*  *** Read only functions: *** */

    fn get_admin(e: &Env) -> Result<Address, CombinedAggregatorError>;
    fn get_protocols(e: &Env) -> Result<Vec<ProtocolAddressPair>, CombinedAggregatorError>;

}

#[contract]
struct SoroswapAggregator;

#[contractimpl]
impl SoroswapAggregatorTrait for SoroswapAggregator {
    /// Initializes the contract and sets the soroswap_router address
    fn initialize(
        e: Env,
        admin: Address,
        protocol_addresses: Vec<ProtocolAddressPair>,
    ) -> Result<(), CombinedAggregatorError> {
        if is_initialized(&e) {
            return Err(CombinedAggregatorError::AggregatorInitializeAlreadyInitialized.into());
        }
    
        for pair in protocol_addresses.iter() {
            put_protocol_address(&e, pair);
        }

        set_admin(&e, admin);
    
        // Mark the contract as initialized
        set_initialized(&e);
        event::initialized(&e, true);
        extend_instance_ttl(&e);
        Ok(())
    }
    
    fn update_protocols(
        e: Env,
        protocol_addresses: Vec<ProtocolAddressPair>,
    ) -> Result<(), CombinedAggregatorError> {
        check_initialized(&e)?;
        let admin: Address = get_admin(&e);
        admin.require_auth();
        // Check if the sender is the admin
        
        for pair in protocol_addresses.iter() {
            if !is_valid_protocol(pair.protocol_id) {
                // If the protocol_id is not recognized, return an error
                return Err(CombinedAggregatorError::AggregatorUnsupportedProtocol);
            }
            // Proceed to update the protocol address since the id is valid
            put_protocol_address(&e, pair);
        }
    
        // event::protocols_updated(&e);
        Ok(())
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
                    let swap_result = phoenix_interface::swap_with_phoenix(&e, &swap_amount, dist.path.clone(), to.clone())?;
                    total_swapped += swap_result;
                },
                _ => return Err(CombinedAggregatorError::AggregatorUnsupportedProtocol),
            }
        }
        
        Ok(total_swapped)
    }

    /*  *** Read only functions: *** */

    fn get_admin(e: &Env) -> Result<Address, CombinedAggregatorError> {
        check_initialized(&e)?;
        Ok(get_admin(&e))
    }

    fn get_protocols(e: &Env) -> Result<Vec<ProtocolAddressPair>, CombinedAggregatorError> {
        check_initialized(&e)?;
        let protocols = vec![
            e,
            dex_constants::SOROSWAP,
            dex_constants::PHOENIX,
        ];
    
        let mut addresses = Vec::new(e);
    
        for protocol_id in protocols.iter() {
            if has_protocol_address(e, protocol_id) {
                let address = get_protocol_address(e, protocol_id);
                addresses.push_back(ProtocolAddressPair { protocol_id, address });
            }
        }
    
        Ok(addresses)
    }    

}
