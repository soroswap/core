#![cfg(test)]
extern crate std;
use soroban_sdk::{
    Env, 
    vec,
    Vec,
    BytesN, 
    Address, 
    testutils::{
        Address as _,
        Ledger,
    },
};
use crate::{SoroswapAggregator, SoroswapAggregatorClient, dex_constants};
use crate::models::{DexDistribution, ProtocolAddressPair};

// Token Contract
mod token {
    soroban_sdk::contractimport!(file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
use token::TokenClient;

pub fn create_token_contract<'a>(e: &Env, admin: & Address) -> TokenClient<'a> {
    TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
}

// Pair Contract
mod pair {
    soroban_sdk::contractimport!(file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair.wasm");
   pub type SoroswapPairClient<'a> = Client<'a>;
}
use pair::SoroswapPairClient;


fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

// SoroswapFactory Contract
mod factory {
    soroban_sdk::contractimport!(file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}
use factory::SoroswapFactoryClient;

fn create_soroswap_factory<'a>(e: & Env, setter: & Address) -> SoroswapFactoryClient<'a> {
    let pair_hash = pair_contract_wasm(&e);  
    let factory_address = &e.register_contract_wasm(None, factory::WASM);
    let factory = SoroswapFactoryClient::new(e, factory_address); 
    factory.initialize(&setter, &pair_hash);
    factory
}

// SoroswapRouter Contract
mod router {
    soroban_sdk::contractimport!(file = "../router/target/wasm32-unknown-unknown/release/soroswap_router.wasm");
    pub type SoroswapRouterClient<'a> = Client<'a>;
}
use router::SoroswapRouterClient;

// SoroswapRouter Contract
fn create_soroswap_router<'a>(e: &Env) -> SoroswapRouterClient<'a> {
    let router_address = &e.register_contract_wasm(None, router::WASM);
    let router = SoroswapRouterClient::new(e, router_address); 
    router
}

// SoroswapAggregator Contract
fn create_soroswap_aggregator<'a>(e: &Env) -> SoroswapAggregatorClient<'a> {
    SoroswapAggregatorClient::new(e, &e.register_contract(None, SoroswapAggregator {}))
}

// Helper function to create a simple distribution vector for testing
fn create_test_distribution(test: &SoroswapAggregatorTest) -> Vec<DexDistribution> {
    vec![&test.env,
        DexDistribution {
            index: dex_constants::SOROSWAP,
            path: vec![&test.env, test.token_0.address.clone(), test.token_1.address.clone(), test.token_2.address.clone()],
            parts: 3,
        },
        DexDistribution {
            index: dex_constants::SOROSWAP,
            path: vec![&test.env, test.token_0.address.clone(), test.token_2.address.clone()],
            parts: 2,
        },
    ]
}

// Helper function to initialize / update soroswap aggregator protocols
pub fn create_protocols_addresses(test: &SoroswapAggregatorTest) -> Vec<ProtocolAddressPair> {
    vec![&test.env,
        ProtocolAddressPair {
            protocol_id: dex_constants::SOROSWAP,
            address: test.router_contract.address.clone(),
        },
    ]
}

pub struct SoroswapAggregatorTest<'a> {
    env: Env,
    aggregator_contract: SoroswapAggregatorClient<'a>,
    router_contract: SoroswapRouterClient<'a>,
    factory_contract: SoroswapFactoryClient<'a>,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    token_2: TokenClient<'a>,
    user: Address,
    admin: Address
}

impl<'a> SoroswapAggregatorTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        let aggregator_contract = create_soroswap_aggregator(&env);
        let router_contract = create_soroswap_router(&env);

        let initial_user_balance = 10_000_000_000_000_000_000;

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        assert_ne!(admin, user);

        let mut token_0 = create_token_contract(&env, &admin);
        let mut token_1 = create_token_contract(&env, &admin);
        let mut token_2 = create_token_contract(&env, &admin);
        if &token_1.address < &token_0.address {
            std::mem::swap(&mut token_0, &mut token_1);
        }
        token_0.mint(&user, &initial_user_balance);
        token_1.mint(&user, &initial_user_balance);
        token_2.mint(&user, &initial_user_balance);

        let factory_contract = create_soroswap_factory(&env, &admin);
        env.budget().reset_unlimited();

        let ledger_timestamp = 100;
        let desired_deadline = 1000;
    
        assert!(desired_deadline > ledger_timestamp);
    
        env.ledger().with_mut(|li| {
            li.timestamp = ledger_timestamp;
        });
    
        let amount_0: i128 = 1_000_000_000_000_000_000;
        let amount_1: i128 = 4_000_000_000_000_000_000;
        let expected_liquidity: i128 = 2_000_000_000_000_000_000;
    
        // Check initial user value of every token:
        assert_eq!(token_0.balance(&user), initial_user_balance);
        assert_eq!(token_1.balance(&user), initial_user_balance);
        assert_eq!(token_2.balance(&user), initial_user_balance);
    
        router_contract.initialize(&factory_contract.address);

        assert_eq!(factory_contract.pair_exists(&token_0.address, &token_1.address), false);
        let (added_token_0, added_token_1, added_liquidity) = router_contract.add_liquidity(
            &token_0.address, //     token_a: Address,
            &token_1.address, //     token_b: Address,
            &amount_0, //     amount_a_desired: i128,
            &amount_1, //     amount_b_desired: i128,
            &0, //     amount_a_min: i128,
            &0 , //     amount_b_min: i128,
            &user, //     to: Address,
            &desired_deadline//     deadline: u64,
        );

        let (added_token_2, added_token_3, added_liquidity_2) = router_contract.add_liquidity(
            &token_1.address, //     token_a: Address,
            &token_2.address, //     token_b: Address,
            &amount_1, //     amount_a_desired: i128,
            &amount_0, //     amount_b_desired: i128,
            &0, //     amount_a_min: i128,
            &0 , //     amount_b_min: i128,
            &user, //     to: Address,
            &desired_deadline//     deadline: u64,
        );

        let (added_token_2, added_token_3, added_liquidity_2) = router_contract.add_liquidity(
            &token_0.address, //     token_a: Address,
            &token_2.address, //     token_b: Address,
            &amount_0, //     amount_a_desired: i128,
            &amount_1, //     amount_b_desired: i128,
            &0, //     amount_a_min: i128,
            &0 , //     amount_b_min: i128,
            &user, //     to: Address,
            &desired_deadline//     deadline: u64,
        );

        static MINIMUM_LIQUIDITY: i128 = 1000;
    
        assert_eq!(added_token_0, amount_0);
        assert_eq!(added_token_1, amount_1);
        assert_eq!(added_token_2, amount_0);
        assert_eq!(added_token_3, amount_1);
        assert_eq!(added_liquidity, expected_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap());
        assert_eq!(added_liquidity_2, expected_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap());
    
        assert_eq!(token_0.balance(&user), 8_000_000_000_000_000_000);
        assert_eq!(token_1.balance(&user), 2_000_000_000_000_000_000);
        assert_eq!(token_2.balance(&user), 5_000_000_000_000_000_000);

        SoroswapAggregatorTest {
            env,
            aggregator_contract,
            router_contract,
            factory_contract,
            token_0,
            token_1,
            token_2,
            user,
            admin
        }
    }
}

#[test]
fn test_swap() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let from_token = &test.token_0.address;
    let dest_token = &test.token_1.address;
    let amount = 500_000_000_000_000_000;
    let amount_out_min = 90i128;
    let distribution = create_test_distribution(&test);
    let to = &test.user;
    let deadline = test.env.ledger().timestamp() + 100; // Deadline in the future

    let result = test.aggregator_contract.swap(&from_token, &dest_token, &amount, &amount_out_min, &distribution, &to, &deadline);

    assert_eq!(test.token_0.balance(&test.user), 7_500_000_000_000_000_000);
    assert_eq!(test.token_1.balance(&test.user), 2_000_000_000_000_000_000);
    assert_eq!(test.token_2.balance(&test.user), 5_851_690_580_469_525_867);
}

pub mod initialize;