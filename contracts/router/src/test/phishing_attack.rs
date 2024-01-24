use soroban_sdk::{Env, Address, vec, Vec};

use crate::test::{SoroswapRouterTest};
use crate::test::add_liquidity::add_liquidity;
use crate::error::CombinedRouterError;

// Malicious Token Contract
mod token_malicious_contract {
    soroban_sdk::contractimport!(file = "../token-malicious/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
use token_malicious_contract::TokenClient as MaliciousTokenClient;

pub fn create_token_malicious_contract<'a>(e: &Env, admin: & Address) -> MaliciousTokenClient<'a> {
    MaliciousTokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
}




#[test]
fn phishing_attack() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;  
    let initial_user_balance = 10_000_000_000_000_000_000;

    // Malicious token setup
    let token_malicious = create_token_malicious_contract(&test.env, &test.admin);
    token_malicious.mint(&test.admin, &initial_user_balance);
    token_malicious.set_target_token_contract(&test.token_1.address.clone());
    // token_malicious


    // let amount_0: i128 = 1_000_000_000;
    // let amount_1: i128 = 4_000_000_000;

    // test.contract.add_liquidity(
    //     &test.token_0.address, //     token_a: Address,
    //     &test.token_1.address, //     token_b: Address,
    //     &amount_0, //     amount_a_desired: i128,
    //     &amount_1, //     amount_b_desired: i128,
    //     &0, //     amount_a_min: i128,
    //     &0 , //     amount_b_min: i128,
    //     &test.user, //     to: Address,
    //     &deadline//     deadline: u64,
    // );

    // let amount_2: i128 = 8_000_000_000;

    // test.contract.add_liquidity(
    //     &test.token_1.address, //     token_a: Address,
    //     &token_malicious.address, //     token_b: Address,
    //     &amount_1, //     amount_a_desired: i128,
    //     &amount_2, //     amount_b_desired: i128,
    //     &0, //     amount_a_min: i128,
    //     &0 , //     amount_b_min: i128,
    //     &test.user, //     to: Address,
    //     &deadline//     deadline: u64,
    // );
    
    
    // let mut path: Vec<Address> = Vec::new(&test.env);
    // path.push_back(test.token_0.address.clone());
    // path.push_back(test.token_1.address.clone());
    // path.push_back(token_malicious.address.clone());


    // let amount_in = 123_456_789;
    // // First out = (123456789*997*4000000000)/(1000000000*1000 + 997*123456789) = 438386277,6
    // let first_out = 438386277;
    // // Second out = (438386277*997*8000000000)/(4000000000*1000 + 997*438386277) = 788035362,1
    // let expected_amount_out = 788035362;

    // let executed_amounts = test.contract.swap_exact_tokens_for_tokens(
    //     &amount_in, //amount_in
    //     &0,  // amount_out_min
    //     &path, // path
    //     &test.user, // to
    //     &deadline); // deadline

    // assert_eq!(executed_amounts.get(0).unwrap(), amount_in);
    // assert_eq!(executed_amounts.get(1).unwrap(), first_out);
    // assert_eq!(executed_amounts.get(2).unwrap(), expected_amount_out);
    
    // assert_eq!(test.token_0.balance(&test.user), initial_user_balance - amount_0 - amount_in);
    // assert_eq!(test.token_1.balance(&test.user), initial_user_balance - amount_1*2);
    // assert_eq!(token_malicious.balance(&test.user), initial_user_balance -amount_2 + expected_amount_out);
}

