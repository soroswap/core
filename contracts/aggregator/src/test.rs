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
    },
};
use crate::{SoroswapAggregator, SoroswapAggregatorClient, dex_constants};
use crate::models::DexDistribution;

// Token Contract
mod token {
    soroban_sdk::contractimport!(file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
use token::TokenClient;

pub fn create_token_contract<'a>(e: &Env, admin: & Address) -> TokenClient<'a> {
    TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
}

// SoroswapAggregator Contract
fn create_aggregator_router<'a>(e: &Env) -> SoroswapAggregatorClient<'a> {
    SoroswapAggregatorClient::new(e, &e.register_contract(None, SoroswapAggregator {}))
}

// Helper function to create a simple distribution vector for testing
fn create_test_distribution(test: &SoroswapAggregatorTest) -> Vec<DexDistribution> {
    vec![&test.env,
        DexDistribution {
            index: dex_constants::SOROSWAP,
            path: vec![&test.env, test.token_0.address.clone()],
            parts: 3,
        },
        DexDistribution {
            index: dex_constants::PHOENIX,
            path: vec![&test.env, test.token_0.address.clone()],
            parts: 2,
        },
    ]
}

pub struct SoroswapAggregatorTest<'a> {
    env: Env,
    contract: SoroswapAggregatorClient<'a>,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    user: Address,
    admin: Address
}

impl<'a> SoroswapAggregatorTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        let contract = create_aggregator_router(&env);

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        assert_ne!(admin, user);

        let mut token_0 = create_token_contract(&env, &admin);
        let mut token_1 = create_token_contract(&env, &admin);
        if &token_1.address < &token_0.address {
            std::mem::swap(&mut token_0, &mut token_1);
        }
        token_0.mint(&user, &10_000_000_000_000_000_000);
        token_1.mint(&user, &10_000_000_000_000_000_000);

        env.budget().reset_unlimited();

        SoroswapAggregatorTest {
            env,
            contract,
            token_0,
            token_1,
            user,
            admin
        }
    }
}

#[test]
fn test_swap() {
    let test = SoroswapAggregatorTest::setup();

    let from_token = &test.token_0.address;
    let dest_token = &test.token_1.address;
    let amount = 100i128;
    let amount_out_min = 90i128;
    let distribution = create_test_distribution(&test);
    let to = &test.user;
    let deadline = test.env.ledger().timestamp() + 100; // Deadline in the future

    let result = test.contract.swap(&from_token, &dest_token, &amount, &amount_out_min, &distribution, &to, &deadline);
    assert_eq!(result, amount, "Expected the swap result to be 0");
}

// #[test]
// fn test_swap_failure_due_to_deadline() {
//     let mut e = MockEnv::default();
//     mock_initialization(&mut e);

//     let aggregator = SoroswapAggregator {};

//     // Setup similar to `test_swap` but with a past deadline
//     let deadline = e.ledger().timestamp() - 1; // Deadline in the past

//     // Similar call to `aggregator.swap` as in `test_swap` but with updated deadline
//     // Assert that result is an error and matches `DeadlineExpired` error
// }

// Additional tests here for handling incorrect inputs, and other edge cases
