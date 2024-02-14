/// The `phoenix_interface` module defines the interface for interacting with the
/// Phoenix protocol. Similar to the `soroswap_interface`, it abstracts the details
/// of swap execution and other Phoenix-specific functionalities, facilitating their
/// integration and usage in the aggregator's broader swap strategy.
use soroban_sdk::{Env, Address, Vec};
use crate::storage::{get_protocol_address, has_protocol_address};
use crate::dex_interfaces::{dex_constants};
use crate::error::CombinedAggregatorError;

soroban_sdk::contractimport!(
    file = "./protocols/phoenix-contracts/target/wasm32-unknown-unknown/release/phoenix_multihop.optimized.wasm"
);
pub type PhoenixMultihopClient<'a> = Client<'a>;

fn convert_to_swaps(e: &Env, addresses: Vec<Address>) -> Vec<Swap> {
    let mut swaps = Vec::new(e);

    // Iterate through the addresses, creating a Swap for each pair
    // Skip the last address since it cannot be an offer_asset without a corresponding ask_asset
    for i in 0..(addresses.len() - 1) {
        let offer_asset = addresses.get(i).expect("Failed to get offer asset");
        let ask_asset = addresses.get(i + 1).expect("Failed to get ask asset");

        swaps.push_back(Swap {
            offer_asset: offer_asset.clone(),
            ask_asset: ask_asset.clone(),
            max_belief_price: None,
        });
    }

    swaps
}


pub fn swap_with_phoenix(
    e: &Env,
    amount: &i128,
    path: Vec<Address>,
    to: Address,
) -> Result<i128, CombinedAggregatorError> {
    if !has_protocol_address(e, dex_constants::PHOENIX) {
        return Err(CombinedAggregatorError::AggregatorProtocolAddressNotFound);
    }

    let phoenix_multihop_address = get_protocol_address(e, dex_constants::PHOENIX);
    let phoenix_multihop_client = PhoenixMultihopClient::new(e, &phoenix_multihop_address);

    let operations = convert_to_swaps(e, path);

    phoenix_multihop_client.swap(&to, &operations, &None, &amount);

    Ok(amount.clone())
}