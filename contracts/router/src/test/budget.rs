// ONLY TO CHECK BUDGET
use crate::test::{SoroswapRouterTest};
extern crate std;

use soroban_sdk::{
    testutils::{
        Ledger},};

#[test]
fn add_liquidity() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    
    let ledger_timestamp = 100;
    let desired_deadline = 1000;

    assert!(desired_deadline > ledger_timestamp);

    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;

    test.env.budget().reset_unlimited();
    test.contract.add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &amount_0, //     amount_a_desired: i128,
        &amount_1, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );
    std::println!("add_liquidity / cpu_instruction_cost: {:?}", test.env.budget().cpu_instruction_cost());
    std::println!("add_liquidity / test.env.budget(): {:?}", test.env.budget());
    test.env.budget().reset_unlimited();
}