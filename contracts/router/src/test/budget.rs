// ONLY TO CHECK BUDGET
use crate::test::{SoroswapRouterTest};
extern crate std;
use num_integer::Roots; 
use crate::test::add_liquidity::add_liquidity;



use soroban_sdk::{
    testutils::{
        Ledger},};

#[test]
fn budget_add_liquidity() {
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



#[test]
fn budget_remove_liquidity_equal_amount_exact_minimum_out() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    
    let amount_0: i128 = 10_000_000_000;
    let amount_1: i128 = 10_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);

    // Check LP token balance
    let expected_total_liquidity: i128 = (amount_0 * amount_1).sqrt(); // sqrt(amount_0, amount_1);
    static MINIMUM_LIQUIDITY: i128 = 1000;
    let expected_liquidity: i128 = expected_total_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap();

    let ledger_timestamp = 200;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    let expected_to_remove_0 = (amount_0*expected_liquidity) / expected_total_liquidity;
    let expected_to_remove_1 = (amount_1*expected_liquidity) / expected_total_liquidity;
    
    test.env.budget().reset_unlimited();
    test.contract.remove_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &expected_liquidity, //     liquidity: i128,
        &expected_to_remove_0, //     amount_a_min: i128,
        &expected_to_remove_1 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );
    std::println!("remove_liquidity / cpu_instruction_cost: {:?}", test.env.budget().cpu_instruction_cost());
    std::println!("remove_liquidity / test.env.budget(): {:?}", test.env.budget());
    test.env.budget().reset_unlimited();
   
    
}