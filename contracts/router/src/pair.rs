soroban_sdk::contractimport!(
    file = "../pair/target/wasm32v1-none/release/soroswap_pair.wasm"
);
pub type SoroswapPairClient<'a> = Client<'a>;