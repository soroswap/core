#![cfg(test)]
extern crate std;
use crate::{SoroswapRouter, SoroswapRouterClient};
use soroban_sdk::{Env};


fn create_soroswap_router_contract<'a>(e: &Env) -> SoroswapRouterClient<'a> {
    SoroswapRouterClient::new(e, &e.register_contract(None, SoroswapRouter {}))
}

struct SoroswapRouterTest<'a> {
    env: Env,
    contract: SoroswapRouterClient<'a>,
}

impl<'a> SoroswapRouterTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();
        let contract = create_soroswap_router_contract(&env);

        SoroswapRouterTest {
            env,
            contract,
        }
    }
}

// Test mods:

pub mod initialize;
pub mod add_liquidity;

