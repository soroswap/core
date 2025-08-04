soroban_sdk::contractimport!(
    file = "../factory/target/wasm32v1-none/release/soroswap_factory.optimized.wasm"
);
pub type SoroswapFactoryClient<'a> = Client<'a>;