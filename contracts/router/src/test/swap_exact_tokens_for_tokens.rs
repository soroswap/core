use soroban_sdk::{Address, vec, Vec};

use crate::test::{SoroswapRouterTest, create_token_contract};
use crate::test::add_liquidity::add_liquidity;
use crate::error::CombinedRouterError;

#[test]
fn swap_exact_tokens_for_tokens_not_initialized() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.contract.try_swap_exact_tokens_for_tokens(
        &0,            // amount_in
        &0,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
    );

    assert_eq!(
        result,
        Err(Ok(CombinedRouterError::RouterNotInitialized))
    );
}

#[test]
fn swap_exact_tokens_for_tokens_amount_in_negative() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();

    test.contract.initialize(&test.factory.address);
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.contract.try_swap_exact_tokens_for_tokens(
        &-1,           // amount_in
        &0,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
    );

    assert_eq!(
        result,
        Err(Ok(CombinedRouterError::RouterNegativeNotAllowed))
    );
}

#[test]
fn swap_exact_tokens_for_tokens_amount_out_min_negative() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();

    test.contract.initialize(&test.factory.address);
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.contract.try_swap_exact_tokens_for_tokens(
        &0,            // amount_in
        &-1,           // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
    );

    assert_eq!(
        result,
        Err(Ok(CombinedRouterError::RouterNegativeNotAllowed))
    );
}

#[test]
fn swap_exact_tokens_for_tokens_expired() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.contract.try_swap_exact_tokens_for_tokens(
        &0,            // amount_in
        &0,            // amount_out_min
        &path,         // path
        &test.user,    // to
        &0,            // deadline
    );

    assert_eq!(
        result,
        Err(Ok(CombinedRouterError::RouterDeadlineExpired))
    );
}


#[test]
fn try_swap_exact_tokens_for_tokens_invalid_path() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    let path: Vec<Address> = vec![&test.env, test.token_0.address.clone()];
    let result = test.contract.try_swap_exact_tokens_for_tokens(
        &0,        // amount_in
        &0,        // amount_out_min
        &path,     // path
        &test.user, // to
        &deadline, // deadline
    );
    assert_eq!(result, Err(Ok(CombinedRouterError::LibraryInvalidPath)));
}



#[test]
// Panics because LP does not exist; here panics with a Error(Storage, MissingValue)
// We should implement a pair_address.exist() without needing to call the Factory
#[should_panic]
fn swap_exact_tokens_for_tokens_pair_does_not_exist() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    test.contract.swap_exact_tokens_for_tokens(
        &0, //amount_in
        &0,  // amount_out_min
        &path, // path
        &test.user, // to
        &deadline); // deadline
}

#[test]
fn try_swap_exact_tokens_for_tokens_insufficient_input_amount() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);

    test.env.budget().reset_unlimited();
    let result = test.contract.try_swap_exact_tokens_for_tokens(
        &0,        // amount_in
        &0,        // amount_out_min
        &path,     // path
        &test.user, // to
        &deadline, // deadline
    );
    assert_eq!(result, Err(Ok(CombinedRouterError::LibraryInsufficientInputAmount)));
}



#[test]
fn swap_exact_tokens_for_tokens_insufficient_output_amount() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);

    let amount_in = 1_000_000;

    //(1000000×997×4000000000000000000)÷(1000000000000000000×1000+997×1000000) = 3987999,9

    let expected_amount_out = 3987999;

    test.env.budget().reset_unlimited();
    let result = test.contract.try_swap_exact_tokens_for_tokens(
        &amount_in,       // amount_in
        &(expected_amount_out + 1),  // amount_out_min
        &path,            // path
        &test.user,       // to
        &deadline,        // deadline
    );

    assert_eq!(
        result,
        Err(Ok(CombinedRouterError::RouterInsufficientOutputAmount))
    );
}



#[test]
fn swap_exact_tokens_for_tokens_enough_output_amount() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);

    let amount_in = 1_000_000;

    //(1000000×997×4000000000000000000)÷(1000000000000000000×1000+997×1000000) = 3987999,9

    let expected_amount_out = 3987999;

    test.env.budget().reset_unlimited();
    let executed_amounts = test.contract.swap_exact_tokens_for_tokens(
        &amount_in, //amount_in
        &(expected_amount_out),  // amount_out_min
        &path, // path
        &test.user, // to
        &deadline); // deadline

    assert_eq!(executed_amounts.get(0).unwrap(), amount_in);
    assert_eq!(executed_amounts.get(1).unwrap(), expected_amount_out);
    
}




#[test]
fn swap_exact_tokens_for_tokens_2_hops() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;  
    let initial_user_balance = 10_000_000_000_000_000_000;

    let token_2 = create_token_contract(&test.env, &test.admin);
    token_2.mint(&test.user, &initial_user_balance);
    let amount_0: i128 = 1_000_000_000;
    let amount_1: i128 = 4_000_000_000;

    test.contract.add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &amount_0, //     amount_a_desired: i128,
        &amount_1, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &deadline//     deadline: u64,
    );

    let amount_2: i128 = 8_000_000_000;

    test.contract.add_liquidity(
        &test.token_1.address, //     token_a: Address,
        &token_2.address, //     token_b: Address,
        &amount_1, //     amount_a_desired: i128,
        &amount_2, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &deadline//     deadline: u64,
    );
    
    
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    path.push_back(token_2.address.clone());


    let amount_in = 123_456_789;
    // First out = (123456789*997*4000000000)/(1000000000*1000 + 997*123456789) = 438386277,6
    let first_out = 438386277;
    // Second out = (438386277*997*8000000000)/(4000000000*1000 + 997*438386277) = 788035362,1
    let expected_amount_out = 788035362;

    let executed_amounts = test.contract.swap_exact_tokens_for_tokens(
        &amount_in, //amount_in
        &0,  // amount_out_min
        &path, // path
        &test.user, // to
        &deadline); // deadline

    assert_eq!(executed_amounts.get(0).unwrap(), amount_in);
    assert_eq!(executed_amounts.get(1).unwrap(), first_out);
    assert_eq!(executed_amounts.get(2).unwrap(), expected_amount_out);
    
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance - amount_0 - amount_in);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance - amount_1*2);
    assert_eq!(token_2.balance(&test.user), initial_user_balance -amount_2 + expected_amount_out);
}

#[test]
fn swap_exact_tokens_for_tokens_single_hop() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    let initial_user_balance: i128 = 10_000_000_000_000_000_000;

    let amount_0: i128 = 1_000_000_000;
    let amount_1: i128 = 4_000_000_000;

    // Add liquidity to the token_0 and token_1 pool
    test.contract.add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &amount_0, //     amount_a_desired: i128,
        &amount_1, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &deadline//     deadline: u64,
    );
    
    // Define the swap path (single hop from token_0 to token_1)
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());


    let amount_in = 123_456_789;
    let expected_amount_out = 438386277;

    // Here i should set an allowance for the router to spend the user's tokens
    test.token_0.approve(&test.user, &test.contract.address, &amount_in, &1000);

    // Performing a swap
    let executed_amounts = test.contract.swap_exact_tokens_for_tokens(
        &amount_in, //amount_in
        &0,  // amount_out_min
        &path, // path
        &test.user, // to
        &deadline); // deadline

    // Check the allowance
    assert_eq!(test.token_0.allowance(&test.user, &test.contract.address), amount_in);

    // Check the executed swap amounts
    assert_eq!(executed_amounts.get(0).unwrap(), amount_in);
    assert_eq!(executed_amounts.get(1).unwrap(), expected_amount_out);

    // Verify the final token balances (simplified check)
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance - amount_0 - amount_in);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance - amount_1 + expected_amount_out);
}
