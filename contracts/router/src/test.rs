#![cfg(test)]
extern crate std;

use crate::{SoroswapRouter, SoroswapRouterClient};

use soroban_sdk::{Env,
    Address,
    testutils::{
        Address as _, 
        MockAuth,
        MockAuthInvoke,
    },
    vec,
    IntoVal};


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

#[test]
#[should_panic(expected = "Unauthorized function call for address")]
fn test_add_liquidity_not_authorized() {
    let test = SoroswapRouterTest::setup();
    let alice = Address::random(&test.env);
    let bob = Address::random(&test.env);
    // alice is not equal to bob
    assert_ne!(alice, bob);
    let token_a = Address::random(&test.env);
    let token_b = Address::random(&test.env);

    /*
        Here we test the add_liquidity function "to.require_auth();" requirement
        
        fn add_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        amount_a_desired: i128,
        amount_b_desired: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        to: Address,
        deadline: u64)

        So if alice calls the function but sets "bob" in the "to" argument, this should fail
    */
    test.contract
        .mock_auths(&[MockAuth {
            address: &alice,
            invoke: &MockAuthInvoke {
                contract: &test.contract.address,
                fn_name: "add_liquidity",
                args: vec![
                    &test.env,
                    token_a.into_val(&test.env), //     token_a: Address,
                    token_b.into_val(&test.env), //     token_b: Address,
                    0.into_val(&test.env), //     amount_a_desired: i128,
                    0.into_val(&test.env), //     amount_b_desired: i128,
                    0.into_val(&test.env), //     amount_a_min: i128,
                    0.into_val(&test.env) , //     amount_b_min: i128,
                    (&bob,).into_val(&test.env), //     to: Address,
                    0.into_val(&test.env)//     deadline: u64,
                    ],
                sub_invokes: &[],
            },
        }])
        .add_liquidity(
            &token_a, //     token_a: Address,
            &token_b, //     token_b: Address,
            &0, //     amount_a_desired: i128,
            &0, //     amount_b_desired: i128,
            &0, //     amount_a_min: i128,
            &0 , //     amount_b_min: i128,
            &bob, //     to: Address,
            &0//     deadline: u64,
        );

}