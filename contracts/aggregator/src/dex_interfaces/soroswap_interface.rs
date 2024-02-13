/// The `soroswap_interface` module provides functions and types for interacting
/// with Soroswap protocol. It encapsulates all necessary logic for
/// executing swaps, and other interactions specific to Soroswap,
/// ensuring that these operations are easily accessible and modular within the aggregator.
use soroban_sdk::{contractimpl, Env, Address, Vec};
use soroban_sdk::token::Client as TokenClient;
use crate::models::DexDistribution;
use crate::storage::{get_soroswap_router};

soroban_sdk::contractimport!(
    file = "../router/target/wasm32-unknown-unknown/release/soroswap_router.optimized.wasm"
);
pub type SoroswapRouterClient<'a> = Client<'a>;

pub fn swap_with_soroswap(
    env: &Env,
    amount: &i128,
    path: Vec<Address>,
    to: Address,
    deadline: u64,
) -> Result<i128, crate::error::CombinedAggregatorError> {
    // Implementation specific to Soroswap
    let soroswap_router_address = get_soroswap_router(env);
    let soroswap_router_client = SoroswapRouterClient::new(env, &soroswap_router_address);

    soroswap_router_client.swap_exact_tokens_for_tokens(&amount, &0, &path, &to, &deadline);

    Ok(amount.clone())
}
