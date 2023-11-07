use crate::test::{SoroswapRouterTest};
use crate::test::add_liquidity::add_liquidity;

use soroban_sdk::{
    Address,
    testutils::{
        
        Ledger},
    vec, Vec};

#[test]
#[should_panic(expected = "SoroswapRouter: not yet initialized")] 
fn swap_tokens_for_exact_tokens_not_initialized() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    let path: Vec<Address> = Vec::new(&test.env);
    test.contract.swap_tokens_for_exact_tokens(
        &0, //amount_out
        &0,  // amount_in_max
        &path, // path
        &test.user, // to
        &0); // deadline
}

#[test]
#[should_panic(expected = "SoroswapRouter: expired")]
fn swap_tokens_for_exact_tokens_expired() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let path: Vec<Address> = Vec::new(&test.env);
    test.contract.swap_tokens_for_exact_tokens(
        &0, //amount_out
        &0,  // amount_in_max
        &path, // path
        &test.user, // to
        &0); // deadline
}


#[test]
#[should_panic(expected = "SoroswapLibrary: invalid path")]
fn swap_tokens_for_exact_tokens_invalid_path() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;    
    let path: Vec<Address> =  vec![&test.env, test.token_0.address.clone()];

    test.contract.swap_tokens_for_exact_tokens(
        &0, //amount_out
        &0,  // amount_in_max
        &path, // path
        &test.user, // to
        &deadline); // deadline
}


// #[test]
// #[should_panic(expected = "SoroswapLibrary: insufficient output amount")]
// fn swap_tokens_for_exact_tokens_insufficient_output_amount() {
//     let test = SoroswapRouterTest::setup();
//     test.contract.initialize(&test.factory.address);
//     let deadline: u64 = test.env.ledger().timestamp() + 1000;  

//     let mut path: Vec<Address> = Vec::new(&test.env);
//     path.push_back(test.token_0.address.clone());
//     path.push_back(test.token_1.address.clone());

//     test.contract.swap_tokens_for_exact_tokens(
//         &0, //amount_out
//         &0,  // amount_in_max
//         &path, // path
//         &test.user, // to
//         &deadline); // deadline
// }

#[test]
fn swap_tokens_for_exact_tokens() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);

    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let expected_amount_out = 5_000_000;
    // For a 1 swap, get_amounts_in returns [input, output]
    let amount_in_should = test.contract.router_get_amounts_in(&expected_amount_out, &path).get(0).unwrap();

    let ledger_timestamp = 100;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    // test.contract.swap_tokens_for_exact_tokens(
    //     &expected_amount_out, //amount_out
    //     &(amount_in_should + 1_000_000_000_000_000_000),  // amount_in_max
    //     &path, // path
    //     &test.user, // to
    //     &desired_deadline); // deadline

    // fn swap_tokens_for_exact_tokens(
    //     e: Env,
    //     amount_out: i128,
    //     amount_in_max: i128,
    //     path: Vec<Address>,
    //     to: Address,
    //     deadline: u64,
    // ) -> Vec<i128> {
}

