use soroban_sdk::{Address, testutils::{Ledger},vec, Vec};

use crate::test::{SoroswapRouterTest, create_token_contract};
use crate::test::add_liquidity::add_liquidity;
use crate::error::CombinedRouterError;


#[test]
fn swap_tokens_for_exact_tokens_not_initialized() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.contract.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
    );

    assert_eq!(
        result,
        Err(Ok(CombinedRouterError::RouterNotInitialized))
    );
}

#[test]
fn swap_tokens_for_exact_tokens_amount_out_negative() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.contract.try_swap_tokens_for_exact_tokens(
        &-1,       // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
    );

    assert_eq!(
        result,
        Err(Ok(CombinedRouterError::RouterNegativeNotAllowed))
    );
}

#[test]
fn swap_tokens_for_exact_tokens_amount_in_max_negative() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.contract.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &-1,       // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
    );

    assert_eq!(
        result,
        Err(Ok(CombinedRouterError::RouterNegativeNotAllowed))
    );
}

#[test]
fn swap_tokens_for_exact_tokens_expired() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let path: Vec<Address> = Vec::new(&test.env);

    let result = test.contract.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &0,        // deadline
    );

    assert_eq!(result, Err(Ok(CombinedRouterError::RouterDeadlineExpired)));
}


#[test]
fn try_swap_tokens_for_exact_tokens_invalid_path() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;
    let path: Vec<Address> = vec![&test.env, test.token_0.address.clone()];

    let result = test.contract.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
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
fn swap_tokens_for_exact_tokens_pair_does_not_exist() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    test.contract.swap_tokens_for_exact_tokens(
        &0, //amount_out
        &0,  // amount_in_max
        &path, // path
        &test.user, // to
        &deadline); // deadline
}


#[test]
fn try_swap_tokens_for_exact_tokens_insufficient_output_amount() {
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
    let result = test.contract.try_swap_tokens_for_exact_tokens(
        &0,        // amount_out
        &0,        // amount_in_max
        &path,     // path
        &test.user, // to
        &deadline, // deadline
    );
    assert_eq!(result, Err(Ok(CombinedRouterError::LibraryInsufficientOutputAmount)));
}

#[test]
fn swap_tokens_for_exact_tokens_amount_in_max_not_enough() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);

    let expected_amount_out = 5_000_000;

    let result = test.contract.try_swap_tokens_for_exact_tokens(
        &expected_amount_out, // amount_out
        &0,                   // amount_in_max
        &path,                // path
        &test.user,           // to
        &deadline,            // deadline
    );

    assert_eq!(
        result,
        Err(Ok(CombinedRouterError::RouterExcessiveInputAmount))
    );
}

#[test]
fn swap_tokens_for_exact_tokens_amount_in_max_not_enough_amount_in_should_minus_1() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);

    let expected_amount_out = 5_000_000;
    let amount_in_should = test
        .contract
        .router_get_amounts_in(&expected_amount_out, &path)
        .get(0)
        .unwrap();

    let result = test.contract.try_swap_tokens_for_exact_tokens(
        &expected_amount_out, // amount_out
        &(amount_in_should - 1), // amount_in_max
        &path,                // path
        &test.user,           // to
        &deadline,            // deadline
    );

    assert_eq!(
        result,
        Err(Ok(CombinedRouterError::RouterExcessiveInputAmount))
    );
}


#[test]
fn swap_tokens_for_exact_tokens_amount_in_should() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    let deadline: u64 = test.env.ledger().timestamp() + 1000;  

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    let amount_0: i128 = 1_000_000_000;
    let amount_1: i128 = 4_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);

    let expected_amount_out = 5_000_000;
    let amount_in_should = test.contract.router_get_amounts_in(&expected_amount_out, &path).get(0).unwrap();

    let amounts = test.contract.swap_tokens_for_exact_tokens(
        &expected_amount_out, //amount_out
        &(amount_in_should),  // amount_in_max
        &path, // path
        &test.user, // to
        &deadline); // deadline

    assert_eq!(amounts.get(0).unwrap(), amount_in_should);
    assert_eq!(amounts.get(1).unwrap(), expected_amount_out);

    let original_balance: i128 = 10_000_000_000_000_000_000;
    let expected_amount_0_in = 1255331;
    assert_eq!(expected_amount_0_in, amount_in_should);
    assert_eq!(test.token_0.balance(&test.user), original_balance - amount_0 - expected_amount_0_in);
    assert_eq!(test.token_1.balance(&test.user), original_balance - amount_1 + expected_amount_out);

    let pair_address = test.factory.get_pair(&test.token_0.address, &test.token_1.address);
    assert_eq!(test.token_0.balance(&pair_address), amount_0 + expected_amount_0_in);
    assert_eq!(test.token_1.balance(&pair_address), amount_1 - expected_amount_out);

}


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
    // (r_in*amount_out)*1000 / (r_out - amount_out)*997
    // (1000000000000000000*5000000)*1000 / ((4000000000000000000 - 5000000)*997) + 1 = 1253762,2
    // 1253762
    let amount_in_should =1253762;

    let ledger_timestamp = 100;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    let amounts = test.contract.swap_tokens_for_exact_tokens(
        &expected_amount_out, //amount_out
        &(1253761*2),  // amount_in_max
        &path, // path
        &test.user, // to
        &desired_deadline); // deadline


    assert_eq!(amounts.get(0).unwrap(), amount_in_should);
    assert_eq!(amounts.get(1).unwrap(), expected_amount_out);

    let original_balance: i128 = 10_000_000_000_000_000_000;
    assert_eq!(test.token_0.balance(&test.user), original_balance - amount_0 - amount_in_should);
    assert_eq!(test.token_1.balance(&test.user), original_balance - amount_1 + expected_amount_out);
    let pair_address = test.factory.get_pair(&test.token_0.address, &test.token_1.address);
    assert_eq!(test.token_0.balance(&pair_address), amount_0 + amount_in_should);
    assert_eq!(test.token_1.balance(&pair_address), amount_1 - expected_amount_out);

}




#[test]
fn swap_tokens_for_exact_tokens_2_hops() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    let ledger_timestamp = 100;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

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
        &desired_deadline//     deadline: u64,
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
        &desired_deadline//     deadline: u64,
    );

    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());
    path.push_back(token_2.address.clone());

    let expected_amount_out = 123_456_789;
    // pair token_1, token_2
    // token_1 is r_in, token_2 is r_out
    // (r_in*amount_out)*1000 / (r_out - amount_out)*997
    // (4000000000*123456789)*1000 / ((8000000000 - 123456789)*997) + 1 = 62884578,9
    // 31942963
    let middle_amount_in =62884578;

    // pair token_0, token_1
    // token_0 is r_in, token_1 is r_out
    // first amount in = 
    // (1000000000*62884578)*1000 / ((4000000000 - 62884578)*997) + 1 = 16020308,4

    let amount_in_should =16020308;

    let amounts = test.contract.swap_tokens_for_exact_tokens(
        &expected_amount_out, //amount_out
        &amount_in_should,  // amount_in_max
        &path, // path
        &test.user, // to
        &desired_deadline); // deadline


    assert_eq!(amounts.get(0).unwrap(), amount_in_should); 
    assert_eq!(amounts.get(1).unwrap(), middle_amount_in); 
    assert_eq!(amounts.get(2).unwrap(), expected_amount_out);

    let original_balance: i128 = 10_000_000_000_000_000_000;
    assert_eq!(test.token_0.balance(&test.user), original_balance - amount_0 - amount_in_should);
    assert_eq!(test.token_1.balance(&test.user), original_balance - amount_1*2);
    assert_eq!(token_2.balance(&test.user), original_balance - amount_2 + expected_amount_out);

    let pair_address_0_1 = test.factory.get_pair(&test.token_0.address, &test.token_1.address);
    assert_eq!(test.token_0.balance(&pair_address_0_1), amount_0 + amount_in_should);
    assert_eq!(test.token_1.balance(&pair_address_0_1), amount_1 - middle_amount_in);

    let pair_address_1_2 = test.factory.get_pair(&test.token_1.address, &token_2.address);
    assert_eq!(test.token_1.balance(&pair_address_1_2), amount_1 + middle_amount_in);
    assert_eq!(token_2.balance(&pair_address_1_2), amount_2 - expected_amount_out);
}
