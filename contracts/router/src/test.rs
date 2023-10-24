#![cfg(test)]
extern crate std;
use crate::{SoroswapRouter, SoroswapRouterClient};
use soroban_sdk::{Env, BytesN, Address, testutils::Address as _};

// Token Contract
mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
use token::TokenClient;

fn create_token_contract<'a>(e: &Env, admin: & Address) -> TokenClient<'a> {
    TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
}

// Pair Contract
mod pair {
    soroban_sdk::contractimport!(file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm");
   pub type SoroswapPairClient<'a> = Client<'a>;
}
use pair::SoroswapPairClient;


fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

// SoroswapFactory Contract
mod factory {
    soroban_sdk::contractimport!(file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory_contract.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}
use factory::SoroswapFactoryClient;

fn create_soroswap_factory_contract<'a>(e: & Env, setter: & Address) -> SoroswapFactoryClient<'a> {
    let pair_hash = pair_contract_wasm(&e);  
    let factory_address = &e.register_contract_wasm(None, factory::WASM);
    let factory = SoroswapFactoryClient::new(e, factory_address); 
    factory.initialize(&setter, &pair_hash);
    factory
}

// SoroswapRouter Contract
fn create_soroswap_router_contract<'a>(e: &Env) -> SoroswapRouterClient<'a> {
    SoroswapRouterClient::new(e, &e.register_contract(None, SoroswapRouter {}))
}

// SoroswapRouter TEST

struct SoroswapRouterTest<'a> {
    env: Env,
    contract: SoroswapRouterClient<'a>,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    factory: SoroswapFactoryClient<'a>,
    admin: Address,
    user: Address,
}

impl<'a> SoroswapRouterTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();
        let contract = create_soroswap_router_contract(&env);

        let admin = Address::random(&env);
        let user = Address::random(&env);
        assert_ne!(admin, user);

        let mut token_0 = create_token_contract(&env, &admin);
        let mut token_1 = create_token_contract(&env, &admin);
        if &token_1.address.contract_id() < &token_0.address.contract_id() {
            std::mem::swap(&mut token_0, &mut token_1);
        }
        token_0.mint(&user, &10_000_000_000_000_000_000);
        token_1.mint(&user, &10_000_000_000_000_000_000);

        let factory = create_soroswap_factory_contract(&env, &admin);

        SoroswapRouterTest {
            env,
            contract,
            token_0,
            token_1,
            factory,
            admin,
            user
        }
    }
}

// Test mods:

pub mod initialize;
pub mod add_liquidity;
pub mod remove_liquidity;
pub mod library_functions;


