use core::mem;
use soroban_sdk::{
    Env,
    Address,
    vec,
    testutils::{
        Address as _,
        MockAuth,
        MockAuthInvoke,
        Ledger,
    },
    IntoVal
};

mod pair {
    soroban_sdk::contractimport!(file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm");
    pub type SoroswapPairClient<'a> = Client<'a>;
}
mod token {
    soroban_sdk::contractimport!(file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
mod factory {
    soroban_sdk::contractimport!(file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory_contract.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>; 
}
mod router {
    soroban_sdk::contractimport!(file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory_contract.wasm");
    pub type _SoroswapFactoryClient<'a> = Client<'a>; 
}
use pair::SoroswapPairClient;
use token::TokenClient;
use factory::SoroswapFactoryClient;
use crate::{ 
    SoroswapRouter, 
    SoroswapRouterClient
};

struct SoroswapRouterTest<'a> {
    env: Env,
    alice: Address,
    bob: Address,
    factory: SoroswapFactoryClient<'a>,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    pair: SoroswapPairClient<'a>,
    router: SoroswapRouterClient<'a>,
}

impl<'a> SoroswapRouterTest<'a> {
    fn new() -> Self {
        
        let env: Env = Default::default();
        env.mock_all_auths();
        let alice = Address::random(&env);
        let bob = Address::random(&env);
        let mut token_0: TokenClient<'a> = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
        let mut token_1: TokenClient<'a> = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
        if &token_1.address.contract_id() < &token_0.address.contract_id() {
            mem::swap(&mut token_0, &mut token_1);
        } else 
        if &token_1.address.contract_id() == &token_0.address.contract_id() {
            panic!("token contract ids are equal");
        }
        // The other form for registering the contract with the environment
        // interface is directly calling the WASM code:
        // 
        // let factory_address = &env.register_contract_wasm(None, factory::WASM);
        //
        let factory_address = &env.register_contract_wasm(None, factory::WASM);
        let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
        let factory = SoroswapFactoryClient::new(&env, &factory_address);
        factory.initialize(&alice, &pair_hash);
        factory.create_pair(&token_0.address, &token_1.address);
        let pair_address = factory.get_pair(&token_0.address, &token_1.address);
        let pair = SoroswapPairClient::new(&env, &pair_address);
        let router = SoroswapRouterClient::new(&env, &env.register_contract(None, SoroswapRouter {}));
        router.initialize(factory_address);

        SoroswapRouterTest {
            env,
            alice,
            bob,
            factory,
            token_0,
            token_1,
            pair,
            router
        }
    }
}

#[test]
pub fn initialize() {
    let router_test = SoroswapRouterTest::new();
    assert_eq!(router_test.factory.address, router_test.router.get_factory());
}

#[test]
pub fn deposit() {
    let router_test = SoroswapRouterTest::new();
    assert_eq!(router_test.factory.address, router_test.router.get_factory());
}

#[test]
pub fn mock_auth_add_liquidity() {
    let router_test = SoroswapRouterTest::new();
    // router_test.router.initialize(&router_test.factory.address);
    router_test.env.ledger().with_mut(|li| {
        li.timestamp = 0;
    });
    let deadline: u64 = router_test.env.ledger().timestamp() + 1000;
    router_test.token_0.mint(&router_test.alice, &10_000_000_000_000_000_000);
    router_test.token_1.mint(&router_test.alice, &10_000_000_000_000_000_000);    
    router_test
    .router
    // .mock_auths(&[MockAuth {
    //     address: &router_test.alice,
    //     invoke: &MockAuthInvoke {
    //         contract: &router_test.router.address,
    //         fn_name: "add_liquidity",
    //         args: vec![
    //             &router_test.router.env,
    //             router_test.token_0.address.into_val(&router_test.env), //     token_a: Address,
    //             router_test.token_1.address.into_val(&router_test.env), //     token_b: Address,
    //             10000_i128.into_val(&router_test.env), //     amount_a_desired: i128,
    //             10000_i128.into_val(&router_test.env), //     amount_b_desired: i128,
    //             0.into_val(&router_test.env), //     amount_a_min: i128,
    //             0.into_val(&router_test.env) , //     amount_b_min: i128,
    //             (&router_test.alice,).into_val(&router_test.env), //     to: Address,
    //             deadline.into_val(&router_test.env)//     deadline: u64,
    //             ],
    //         sub_invokes: &[],
    //     },
    // }])
    .add_liquidity(
        &router_test.token_0.address, //     token_a: Address,
        &router_test.token_1.address, //     token_b: Address,
        &1001_i128, //     amount_a_desired: i128,
        &1001_i128, //     amount_b_desired: i128,
        &0_i128, //     amount_a_min: i128,
        &0_i128, //     amount_b_min: i128,
        &router_test.alice, //     to: Address,
        &deadline//     deadline: u64,
    );
}

#[test]
pub fn mock_auth_add_liquidity_new_token() {
    let router_test = SoroswapRouterTest::new();
    // router_test.router.initialize(&router_test.factory.address);
    // let env = router_test.env;
    let deadline: u64 = router_test.env.ledger().timestamp() + 120;
    let mut token_2 = TokenClient::new(&router_test.env, &router_test.env.register_stellar_asset_contract(router_test.alice.clone()));
    let mut token_3 = TokenClient::new(&router_test.env, &router_test.env.register_stellar_asset_contract(router_test.alice.clone()));
    token_2.mint(&router_test.bob, &10_000_000_000_000_000_000);
    token_3.mint(&router_test.bob, &10_000_000_000_000_000_000);
    router_test
    .router
    .mock_auths(&[MockAuth {
        address: &router_test.alice,
        invoke: &MockAuthInvoke {
            contract: &router_test.router.address,
            fn_name: "add_liquidity",
            args: vec![
                &router_test.router.env,
                token_2.address.into_val(&router_test.env), //     token_a: Address,
                token_3.address.into_val(&router_test.env), //     token_b: Address,
                10.into_val(&router_test.env), //     amount_a_desired: i128,
                10.into_val(&router_test.env), //     amount_b_desired: i128,
                0.into_val(&router_test.env), //     amount_a_min: i128,
                0.into_val(&router_test.env) , //     amount_b_min: i128,
                (&router_test.alice,).into_val(&router_test.env), //     to: Address,
                deadline.into_val(&router_test.env)//     deadline: u64,
                ],
            sub_invokes: &[],
        },
    }])
    // .add_liquidity(
    //     &token_2.address, //     token_a: Address,
    //     &token_3.address, //     token_b: Address,
    //     &10, //     amount_a_desired: i128,
    //     &10, //     amount_b_desired: i128,
    //     &0, //     amount_a_min: i128,
    //     &0, //     amount_b_min: i128,
    //     &router_test.alice, //     to: Address,
    //     &deadline//     deadline: u64,
    // )
    ;
}