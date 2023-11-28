// ONLY TO CHECK BUDGET
use crate::test::{SoroswapRouterTest, SoroswapPairClient, create_token_contract};
extern crate std;
use num_integer::Roots; 

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
fn budget_remove_liquidity_same_simulation_as_manual_testing() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    let token_0 = create_token_contract(&test.env, &test.admin);
    let token_1 = create_token_contract(&test.env, &test.admin);

    token_0.mint(&test.user, &25_000_000_0000000);
    token_1.mint(&test.user, &25_000_000_0000000);
    
    let amount_0: i128 = 1000_0000000;
    let amount_1: i128 = 1000_0000000;
    let desired_deadline = 9737055687;
    let created_pair_address = test.factory.create_pair(&token_0.address, &token_1.address);
    std::println!("created_pair_address: {:?}", created_pair_address);
    
    let (added_amount_0, added_amount_1, added_liquidity) = test.contract.add_liquidity(
        &token_0.address, //     token_a: Address,
        &token_1.address, //     token_b: Address,
        &amount_0, //     amount_a_desired: i128,
        &amount_1, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );

    std::println!("added_amount_0: {:?}", added_amount_0);
    std::println!("added_amount_1: {:?}", added_amount_1);
    std::println!("added_liquidity: {:?}", added_liquidity);

    // Check LP token balance
    let expected_total_liquidity: i128 = (amount_0 * amount_1).sqrt(); // sqrt(amount_0, amount_1);
    static MINIMUM_LIQUIDITY: i128 = 1000;
    let pair_address = test.factory.get_pair(&token_0.address, &token_1.address);
    assert_eq!(pair_address, created_pair_address);
    let pair_client = SoroswapPairClient::new(&test.env, &pair_address);
    let expected_liquidity: i128 = expected_total_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap();

    let lp_balance = pair_client.balance(&test.user);
    std::println!("token_0.balance(&test.user): {:?}", token_0.balance(&test.user));
    std::println!("token_1.balance(&test.user): {:?}", token_1.balance(&test.user));
    std::println!("pair_client.balance(&test.user): {:?}", pair_client.balance(&test.user));    

    // let expected_to_remove_0 = 0; //(amount_0*expected_liquidity) / expected_total_liquidity;
    // let expected_to_remove_1 = 0; //(amount_1*expected_liquidity) / expected_total_liquidity;
    
    test.env.budget().reset_unlimited();
    test.contract.remove_liquidity(
        &token_0.address, //     token_a: Address,
        &token_1.address, //     token_b: Address,
        &lp_balance, //     liquidity: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );
    std::println!("remove_liquidity - cpu_instruction_cost: {:?}", test.env.budget().cpu_instruction_cost());
    std::println!("remove_liquidity - test.env.budget(): {:?}", test.env.budget());
    test.env.budget().reset_unlimited();
   
    
}