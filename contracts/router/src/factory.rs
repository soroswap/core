soroban_sdk::contractimport!(
    file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory.optimized.wasm"
);
pub type SoroswapFactoryClient<'a> = Client<'a>;