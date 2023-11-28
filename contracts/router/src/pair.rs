soroban_sdk::contractimport!(
    file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair.wasm"
);
pub type SoroswapPairClient<'a> = Client<'a>;