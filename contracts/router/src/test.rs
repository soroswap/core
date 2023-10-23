#![cfg(test)]
extern crate std;

use crate::{SoroswapRouter, SoroswapRouterClient};

use soroban_sdk::{Env, Address, testutils::Address as _};


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
                    

#[test]
fn test_initialize_and_get_factory() {
    let test = SoroswapRouterTest::setup();
    let factory = Address::random(&test.env);
    test.contract.initialize(&factory);
    assert_eq!(factory, test.contract.get_factory());
}

#[test]
#[should_panic(expected = "SoroswapRouter: not yet initialized")]
fn test_get_factory_not_yet_initialized() {
    let test = SoroswapRouterTest::setup();
    let factory = Address::random(&test.env);
    assert_eq!(factory, test.contract.get_factory());
}

#[test]
#[should_panic(expected = "SoroswapRouter: already initialized")]
fn test_initialize_twice() {
    let test = SoroswapRouterTest::setup();
    let factory = Address::random(&test.env);
    test.contract.initialize(&factory);
    let factory_another = Address::random(&test.env);
    test.contract.initialize(&factory_another);
}
