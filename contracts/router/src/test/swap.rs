use core::mem;
use soroban_sdk::{
    Env,
    Address,
    Vec,
    testutils::{
        Address as _,
        Ledger,
    },
};
use num_integer::Roots; 

mod pair {
    soroban_sdk::contractimport!(file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair.wasm");
    pub type SoroswapPairClient<'a> = Client<'a>;
}
mod token {
    soroban_sdk::contractimport!(file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
mod factory {
    soroban_sdk::contractimport!(file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>; 
}
mod router {
    soroban_sdk::contractimport!(file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory.wasm");
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
    // pair: SoroswapPairClient<'a>,
    router: SoroswapRouterClient<'a>,
}

impl<'a> SoroswapRouterTest<'a> {
    fn new() -> Self {
        
        let env: Env = Default::default();
        // TODO: MockAuth implementation.
        // In the meanwhile we will be kickstarting with mock_all_auths and remove it gradually.
        // pair::test::operations has other related tests with Pair and Token initialization.
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
        // let pair = SoroswapPairClient::new(&env, &pair_address);
        let router = SoroswapRouterClient::new(&env, &env.register_contract(None, SoroswapRouter {}));
        router.initialize(factory_address);

        SoroswapRouterTest {
            env,
            alice,
            bob,
            factory,
            token_0,
            token_1,
            // pair,
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
    router_test.env.ledger().with_mut(|li| {
        li.timestamp = 0;
    });
    let deadline: u64 = router_test.env.ledger().timestamp() + 1000;
    router_test.token_0.mint(&router_test.alice, &1001);
    router_test.token_1.mint(&router_test.alice, &1001);    
    router_test
    .router
    // .mock_auths(
    //     &[
    //         MockAuth {
    //             address: &router_test.alice,
    //             invoke: &MockAuthInvoke {
    //                 contract: &router_test.router.address,
    //                 fn_name: "add_liquidity",
    //                 args: vec![
    //                     &router_test.env,
    //                     router_test.token_0.address.into_val(&router_test.env), //     token_a: Address,
    //                     router_test.token_1.address.into_val(&router_test.env), //     token_b: Address,
    //                     1001_i128.into_val(&router_test.env), //     amount_a_desired: i128,
    //                     1001_i128.into_val(&router_test.env), //     amount_b_desired: i128,
    //                     0_i128.into_val(&router_test.env), //     amount_a_min: i128,
    //                     0_i128.into_val(&router_test.env), //     amount_b_min: i128,
    //                     router_test.alice.into_val(&router_test.env), //     to: Address,
    //                     deadline.into_val(&router_test.env) //     deadline: u64,
    //                 ],
    //                 sub_invokes: &[],
    //             }
    //         }
    //     ]
    // )
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
#[should_panic]
pub fn mock_auth_add_liquidity_without_balance() {
    let router_test = SoroswapRouterTest::new();
    router_test.env.ledger().with_mut(|li| {
        li.timestamp = 0;
    });
    let deadline: u64 = router_test.env.ledger().timestamp() + 1000;    
    router_test.token_0.mint(&router_test.alice, &0);
    router_test.token_1.mint(&router_test.alice, &0);  
    router_test
    .router
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
#[should_panic]
pub fn mock_auth_add_liquidity_lt_minimum() {
    let router_test = SoroswapRouterTest::new();
    router_test.env.ledger().with_mut(|li| {
        li.timestamp = 0;
    });
    let deadline: u64 = router_test.env.ledger().timestamp() + 1000;    
    router_test.token_0.mint(&router_test.alice, &1001);
    router_test.token_1.mint(&router_test.alice, &1001);  
    router_test
    .router
    .add_liquidity(
        &router_test.token_0.address, //     token_a: Address,
        &router_test.token_1.address, //     token_b: Address,
        &1000_i128, //     amount_a_desired: i128,
        &1000_i128, //     amount_b_desired: i128,
        &0_i128, //     amount_a_min: i128,
        &0_i128, //     amount_b_min: i128,
        &router_test.alice, //     to: Address,
        &deadline//     deadline: u64,
    );
}

#[test]
#[should_panic]
pub fn mock_auth_add_liquidity_insuficcient_balance() {
    let router_test = SoroswapRouterTest::new();
    router_test.env.ledger().with_mut(|li| {
        li.timestamp = 0;
    });
    let deadline: u64 = router_test.env.ledger().timestamp() + 1000;    
    router_test.token_0.mint(&router_test.alice, &1000);
    router_test.token_1.mint(&router_test.alice, &1000);  
    router_test
    .router
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
pub fn swap_exact_tokens_for_tokens() {
    let router_test = SoroswapRouterTest::new();
    router_test.env.ledger().with_mut(|li| {
        li.timestamp = 0;
    });
    let deadline: u64 = router_test.env.ledger().timestamp() + 1000;
    router_test.token_0.mint(&router_test.alice, &10_000);
    router_test.token_1.mint(&router_test.alice, &10_000);
    let (a,b,l) = router_test
    .router
    .add_liquidity(
        &router_test.token_0.address, //     token_a: Address,
        &router_test.token_1.address, //     token_b: Address,
        &2002_i128, //     amount_a_desired: i128,
        &2003_i128, //     amount_b_desired: i128,
        &0_i128, //     amount_a_min: i128,
        &0_i128, //     amount_b_min: i128,
        &router_test.alice, //     to: Address,
        &deadline//     deadline: u64,
    );
    let mut path: Vec<Address> = Vec::new(&router_test.env);
    path.push_back(router_test.token_0.address.clone());
    path.push_back(router_test.token_1.address.clone());
    assert!(a == 2002);
    assert!(b == 2003);

    // TODO: Get rid of this hack?
    router_test.env.budget().reset_unlimited();


    let balance_0 = 2002_i128;// router_test.token_0.balance(&router_test.factory.address);
    let balance_1 = 2003_i128;// router_test.token_1.balance(&router_test.factory.address);
    let lqdt: i128 = (balance_0.checked_mul(balance_1).unwrap()).sqrt() - 1000;
    // TODO: Check liquidity.
    assert!(l == lqdt);
    router_test
    .router
    // .mock_auths(
    //     &[
    //         MockAuth {
    //             address: &router_test.alice,
    //             invoke: &MockAuthInvoke {
    //                 contract: &router_test.router.address,
    //                 fn_name: "swap_exact_tokens_for_tokens",
    //                 args: 
    //                 (
    //                     200_i128, //     amount_in: i128,
    //                     0_i128, //     amount_out_min: i128,
    //                     path.clone(), // path: Vec<Address>,
    //                     router_test.alice.clone(), //     to: Address,
    //                     deadline + 1000, //     deadline: u64,
    //                 ).into_val(&router_test.env),
    //                 sub_invokes: &[],
    //             }
    //         }
    //     ]
    // )
    .swap_exact_tokens_for_tokens(
        // router_test.env, // e: Env,
        &200, // amount_in: i128,
        &0, //  amount_out_min: i128,
        &path, // path: Vec<Address>,
        &router_test.alice, // to: Address,
        &(deadline + 1000)// deadline: u64,
    )
    ;
}



#[test]
pub fn swap_exact_tokens_for_tokens_two_agents() {
    let router_test = SoroswapRouterTest::new();
    router_test.env.ledger().with_mut(|li| {
        li.timestamp = 0;
    });
    let deadline: u64 = router_test.env.ledger().timestamp() + 1000;
    router_test.token_0.mint(&router_test.alice, &10_000);
    router_test.token_1.mint(&router_test.alice, &10_000);
    router_test.token_0.mint(&router_test.bob, &10_000);
    router_test.token_1.mint(&router_test.bob, &10_000);
    let (a_a,b_a,l_a) = router_test
    .router
    .add_liquidity(
        &router_test.token_0.address, //     token_a: Address,
        &router_test.token_1.address, //     token_b: Address,
        &2002_i128, //     amount_a_desired: i128,
        &2003_i128, //     amount_b_desired: i128,
        &0_i128, //     amount_a_min: i128,
        &0_i128, //     amount_b_min: i128,
        &router_test.alice, //     to: Address,
        &deadline//     deadline: u64,
    );
    // let (a_b,b_b,l_b) = router_test
    // .router
    // .add_liquidity(
    //     &router_test.token_0.address, //     token_a: Address,
    //     &router_test.token_1.address, //     token_b: Address,
    //     &2002_i128, //     amount_a_desired: i128,
    //     &2003_i128, //     amount_b_desired: i128,
    //     &0_i128, //     amount_a_min: i128,
    //     &0_i128, //     amount_b_min: i128,
    //     &router_test.bob, //     to: Address,
    //     &deadline//     deadline: u64,
    // );
    let mut path: Vec<Address> = Vec::new(&router_test.env);
    path.push_back(router_test.token_0.address.clone());
    path.push_back(router_test.token_1.address.clone());
    assert!(a_a == 2002);
    assert!(b_a == 2003);
    // assert!(a_b == 2002);
    // assert!(b_b == 2003);
    let balance_0 = 2002_i128;// router_test.token_0.balance(&router_test.factory.address);
    let balance_1 = 2003_i128;// router_test.token_1.balance(&router_test.factory.address);
    let lqdt: i128 = (balance_0.checked_mul(balance_1).unwrap()).sqrt() - 1000;
    // TODO: Check liquidity.
    assert!(l_a == lqdt);
    // assert!(l_b == lqdt);
    // router_test
    // .router
    // .mock_auths(
    //     &[
    //         MockAuth {
    //             address: &router_test.alice,
    //             invoke: &MockAuthInvoke {
    //                 contract: &router_test.router.address,
    //                 fn_name: "swap_exact_tokens_for_tokens",
    //                 args: 
    //                 (
    //                     200_i128, //     amount_in: i128,
    //                     0_i128, //     amount_out_min: i128,
    //                     path.clone(), // path: Vec<Address>,
    //                     router_test.alice.clone(), //     to: Address,
    //                     deadline + 1000, //     deadline: u64,
    //                 ).into_val(&router_test.env),
    //                 sub_invokes: &[],
    //             }
    //         }
    //     ]
    // )
    // .swap_exact_tokens_for_tokens(
    //     // router_test.env, // e: Env,
    //     &200, // amount_in: i128,
    //     &0, //  amount_out_min: i128,
    //     &path, // path: Vec<Address>,
    //     &router_test.alice, // to: Address,
    //     &(deadline + 1000)// deadline: u64,
    // )
    
}

// #[test]
pub fn mock_auth_add_liquidity_new_token() {
    let router_test = SoroswapRouterTest::new();
    // router_test.router.initialize(&router_test.factory.address);
    // let env = router_test.env;

    let mut token_2 = TokenClient::new(&router_test.env, &router_test.env.register_stellar_asset_contract(router_test.alice.clone()));
    let mut token_3 = TokenClient::new(&router_test.env, &router_test.env.register_stellar_asset_contract(router_test.alice.clone()));
    if &token_2.address.contract_id() < &token_3.address.contract_id() {
        mem::swap(&mut token_2, &mut token_3);
    } else 
    if &token_2.address.contract_id() == &token_3.address.contract_id() {
        panic!("token contract ids are equal");
    }
    // router_test.factory.create_pair(&token_2.address, &token_3.address);
    let get_factory = router_test.router.get_factory();
    let get_factory_client = factory::SoroswapFactoryClient::new(&router_test.env, &get_factory);
    get_factory_client.create_pair(&token_2.address, &token_3.address);
    let _pair_address = get_factory_client.get_pair(&token_2.address, &token_3.address);
    token_2.mint(&router_test.alice, &1001);
    token_3.mint(&router_test.alice, &1001);
    router_test.env.ledger().with_mut(|li| {
        li.timestamp = 0;
    });
    let deadline: u64 = router_test.env.ledger().timestamp() + 1000;
    router_test
    .router
    .add_liquidity(
        &token_2.address, //     token_a: Address,
        &token_3.address, //     token_b: Address,
        &1001_i128, //     amount_a_desired: i128,
        &1001_i128, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0, //     amount_b_min: i128,
        &router_test.alice, //     to: Address,
        &deadline//     deadline: u64,
    )
    ;
}