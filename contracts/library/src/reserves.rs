use soroban_sdk::{Address, Env};
use crate::tokens::{sort_tokens, pair_for};
use crate::error::LibraryError;

mod pair {
    soroban_sdk::contractimport!(
        file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair.wasm"
    );
}
use pair::Client as SoroswapPairClient;


/// Fetches and sorts the reserves for a pair of tokens.
///
/// # Arguments
///
/// * `e` - The environment.
/// * `factory` - The factory address.
/// * `token_a` - The address of the first token.
/// * `token_b` - The address of the second token.
///
/// # Returns
///
/// Returns `Result<(i128, i128), LibraryError>` where `Ok` contains a tuple of sorted reserves, and `Err` indicates an error such as identical tokens or an issue with sorting.
pub fn get_reserves(e: Env,factory: Address, token_a: Address, token_b: Address) -> Result<(i128,i128), LibraryError>{
    let (token_0,token_1) = sort_tokens(token_a.clone(), token_b.clone())?;
    let pair_address = pair_for(e.clone(), factory, token_0.clone(), token_1.clone())?;
    let pair_client = SoroswapPairClient::new(&e, &pair_address);
    let (reserve_0, reserve_1, _block_timestamp_last) = pair_client.get_reserves();
    
    let (reserve_a, reseve_b) =
        if token_a == token_0 {
            (reserve_0, reserve_1) 
        } else {
            (reserve_1, reserve_0) };

    Ok((reserve_a, reseve_b))
}