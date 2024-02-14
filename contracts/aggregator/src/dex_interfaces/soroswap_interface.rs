/// The `soroswap_interface` module provides functions and types for interacting
/// with Soroswap protocol. It encapsulates all necessary logic for
/// executing swaps, and other interactions specific to Soroswap,
/// ensuring that these operations are easily accessible and modular within the aggregator.
use soroban_sdk::{Env, Address, Vec};
use crate::storage::{get_protocol_address, has_protocol_address};
use crate::dex_interfaces::{dex_constants};
use crate::error::CombinedAggregatorError;

soroban_sdk::contractimport!(
    file = "../router/target/wasm32-unknown-unknown/release/soroswap_router.optimized.wasm"
);
pub type SoroswapRouterClient<'a> = Client<'a>;

pub fn swap_with_soroswap(
    e: &Env,
    amount: &i128,
    path: Vec<Address>,
    to: Address,
    deadline: u64,
) -> Result<i128, CombinedAggregatorError> {
    if !has_protocol_address(e, dex_constants::SOROSWAP) {
        return Err(CombinedAggregatorError::AggregatorProtocolAddressNotFound);
    }
    // Implementation specific to Soroswap
    let soroswap_router_address = get_protocol_address(e, dex_constants::SOROSWAP);
    let soroswap_router_client = SoroswapRouterClient::new(e, &soroswap_router_address);

    soroswap_router_client.swap_exact_tokens_for_tokens(&amount, &0, &path, &to, &deadline);

    Ok(amount.clone())
}
