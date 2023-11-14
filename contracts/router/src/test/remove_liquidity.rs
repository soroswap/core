use crate::test::{SoroswapRouterTest, SoroswapPairClient};
use crate::test::add_liquidity::add_liquidity;

use num_integer::Roots; 
use soroban_sdk::{
    Address,
    testutils::{
        Address as _, 
        MockAuth,
        MockAuthInvoke,
        Ledger},
    vec,
    IntoVal};



#[test]
#[should_panic(expected = "SoroswapRouter: not yet initialized")]
fn test_remove_liquidity_not_yet_initialized() {
    let test = SoroswapRouterTest::setup();
    test.contract.remove_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &0, //     liquidity: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &0//     deadline: u64,
    );
}



#[test]
#[should_panic(expected = "Unauthorized function call for address")] 
fn test_remove_liquidity_not_authorized() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let alice = Address::random(&test.env);
    let bob = Address::random(&test.env);
    // alice is not equal to bob
    assert_ne!(alice, bob);

    /*
        Here we test the remove_liquidity function "to.require_auth();" requirement
        So if alice calls the function but sets "bob" in the "to" argument, this should fail
    */
    test.contract
        .mock_auths(&[MockAuth {
            address: &alice,
            invoke: &MockAuthInvoke {
                contract: &test.contract.address,
                fn_name: "remove_liquidity",
                args: vec![&
                    &test.env,
                    test.token_0.address.into_val(&test.env), //     token_a: Address,
                    test.token_1.address.into_val(&test.env), //     token_b: Address,
                    0.into_val(&test.env), //     liquidity: i128,
                    0.into_val(&test.env), //     amount_a_min: i128,
                    0.into_val(&test.env) , //     amount_b_min: i128,
                    (&bob,).into_val(&test.env), //     to: Address,
                    0.into_val(&test.env)//     deadline: u64,
                    ],
                sub_invokes: &[],
            },
        }])
        .remove_liquidity(
            &test.token_0.address, //     token_a: Address,
            &test.token_1.address, //     token_b: Address,
            &0, //     liquidity: i128,
            &0, //     amount_a_min: i128,
            &0 , //     amount_b_min: i128,
            &bob, //     to: Address,
            &0//     deadline: u64,
        );

}



#[test]
#[should_panic(expected = "SoroswapRouter: expired")]
fn test_remove_liquidity_deadline_expired() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);

    let ledger_timestamp = 100;
    let desired_deadline = 90;
    assert!(desired_deadline < ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    // Here we test the case when deadline has passed
    test.contract.remove_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &0, //     liquidity: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );
}



#[test]
#[should_panic(expected = "SoroswapRouter: pair does not exist")]
fn test_remove_liquidity_pair_does_not_exist() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);

    // We don't create any LP for token0 & token 1
    let ledger_timestamp = 100;
    let desired_deadline = 900;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });


    test.contract.remove_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &0, //     liquidity: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );
}

#[test]
fn test_remove_liquidity_equal_amount_0_minimum_out() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    
    let initial_user_balance = 10000000000000000000;
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance);

    let amount_0: i128 = 10_000_000_000;
    let amount_1: i128 = 10_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);
    let pair_address = test.factory.get_pair(&test.token_0.address, &test.token_1.address);

    let     pair_client = SoroswapPairClient::new(&test.env, &pair_address);
    // Check new balances:
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance.checked_sub(amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance.checked_sub(amount_1).unwrap());
    assert_eq!(test.token_0.balance(&pair_address), amount_0);
    assert_eq!(test.token_1.balance(&pair_address), amount_1);
    // Check LP token balance
    let expected_total_liquidity: i128 = 10_000_000_000; // sqrt(amount_0, amount_1);
    static MINIMUM_LIQUIDITY: i128 = 1000;
    let expected_liquidity: i128 = expected_total_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap();

    assert_eq!(pair_client.balance(&test.user), expected_liquidity);
    assert_eq!(pair_client.total_shares(), expected_total_liquidity);


    let ledger_timestamp = 200;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();

    let (removed_0,removed_1) = test.contract.remove_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &expected_liquidity, //     liquidity: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );
   
    assert_eq!(removed_0, (amount_0*expected_liquidity) / expected_total_liquidity);
    assert_eq!(removed_1, (amount_1*expected_liquidity) / expected_total_liquidity);
    let locked_0 = amount_0- removed_0;
    let locked_1 = amount_1- removed_1;
    
   // TODO: Test events

   // Check that not, the user does not have any LP anymore
   assert_eq!(pair_client.balance(&test.user), 0);

   // Check new user token balances, minus the minimum liquiity
   assert_eq!(test.token_0.balance(&test.user), initial_user_balance.checked_sub(locked_0).unwrap());
   assert_eq!(test.token_1.balance(&test.user), initial_user_balance.checked_sub(locked_1).unwrap());
   assert_eq!(test.token_0.balance(&pair_address), locked_0);
   assert_eq!(test.token_1.balance(&pair_address), locked_1);
    
}


#[test]
fn test_remove_liquidity_equal_amount_exact_minimum_out() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    
    let initial_user_balance = 10000000000000000000;
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance);

    let amount_0: i128 = 10_000_000_000;
    let amount_1: i128 = 10_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);
    let pair_address = test.factory.get_pair(&test.token_0.address, &test.token_1.address);

    let     pair_client = SoroswapPairClient::new(&test.env, &pair_address);
    // Check new balances:
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance.checked_sub(amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance.checked_sub(amount_1).unwrap());
    assert_eq!(test.token_0.balance(&pair_address), amount_0);
    assert_eq!(test.token_1.balance(&pair_address), amount_1);
    // Check LP token balance
    let expected_total_liquidity: i128 = (amount_0 * amount_1).sqrt(); // sqrt(amount_0, amount_1);
    static MINIMUM_LIQUIDITY: i128 = 1000;
    let expected_liquidity: i128 = expected_total_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap();

    assert_eq!(pair_client.balance(&test.user), expected_liquidity);
    assert_eq!(pair_client.total_shares(), expected_total_liquidity);


    let ledger_timestamp = 200;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();
    let expected_to_remove_0 = (amount_0*expected_liquidity) / expected_total_liquidity;
    let expected_to_remove_1 = (amount_1*expected_liquidity) / expected_total_liquidity;

    let (removed_0,removed_1) = test.contract.remove_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &expected_liquidity, //     liquidity: i128,
        &expected_to_remove_0, //     amount_a_min: i128,
        &expected_to_remove_1 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );
   
    assert_eq!(removed_0, expected_to_remove_0);
    assert_eq!(removed_1, expected_to_remove_1);
    let locked_0 = amount_0- removed_0;
    let locked_1 = amount_1- removed_1;
    
   // TODO: Test events

   // Check that not, the user does not have any LP anymore
   assert_eq!(pair_client.balance(&test.user), 0);

   // Check new user token balances, minus the minimum liquiity
   assert_eq!(test.token_0.balance(&test.user), initial_user_balance.checked_sub(locked_0).unwrap());
   assert_eq!(test.token_1.balance(&test.user), initial_user_balance.checked_sub(locked_1).unwrap());
   assert_eq!(test.token_0.balance(&pair_address), locked_0);
   assert_eq!(test.token_1.balance(&pair_address), locked_1);
    
}


#[test]
fn test_remove_liquidity_inequal_amount_0_minimum_out() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    
    let initial_user_balance = 10000000000000000000;
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance);

    let amount_0: i128 = 4_000_000_000_000_000;
    let amount_1: i128 = 999_000_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);
    let pair_address = test.factory.get_pair(&test.token_0.address, &test.token_1.address);

    let     pair_client = SoroswapPairClient::new(&test.env, &pair_address);
    // Check new balances:
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance.checked_sub(amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance.checked_sub(amount_1).unwrap());
    assert_eq!(test.token_0.balance(&pair_address), amount_0);
    assert_eq!(test.token_1.balance(&pair_address), amount_1);
    // Check LP token balance
    let expected_total_liquidity: i128 = (amount_0 * amount_1).sqrt(); // sqrt(amount_0, amount_1);
    static MINIMUM_LIQUIDITY: i128 = 1000;
    let expected_liquidity: i128 = expected_total_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap();

    assert_eq!(pair_client.balance(&test.user), expected_liquidity);
    assert_eq!(pair_client.total_shares(), expected_total_liquidity);


    let ledger_timestamp = 200;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();

    let (removed_0,removed_1) = test.contract.remove_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &expected_liquidity, //     liquidity: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );
   
    assert_eq!(removed_0, (amount_0*expected_liquidity) / expected_total_liquidity);
    assert_eq!(removed_1, (amount_1*expected_liquidity) / expected_total_liquidity);
    let locked_0 = amount_0- removed_0;
    let locked_1 = amount_1- removed_1;
    
   // TODO: Test events

   // Check that not, the user does not have any LP anymore
   assert_eq!(pair_client.balance(&test.user), 0);

   // Check new user token balances, minus the minimum liquiity
   assert_eq!(test.token_0.balance(&test.user), initial_user_balance.checked_sub(locked_0).unwrap());
   assert_eq!(test.token_1.balance(&test.user), initial_user_balance.checked_sub(locked_1).unwrap());
   assert_eq!(test.token_0.balance(&pair_address), locked_0);
   assert_eq!(test.token_1.balance(&pair_address), locked_1);
    
}


#[test]
fn test_remove_liquidity_inequal_amount_exact_minimum_out() {
    let test = SoroswapRouterTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    
    let initial_user_balance = 10000000000000000000;
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance);

    let amount_0: i128 = 4_000_000_000_000_000;
    let amount_1: i128 = 999_000_000_000_000;

    add_liquidity(&test, &amount_0, &amount_1);
    let pair_address = test.factory.get_pair(&test.token_0.address, &test.token_1.address);

    let     pair_client = SoroswapPairClient::new(&test.env, &pair_address);
    // Check new balances:
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance.checked_sub(amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance.checked_sub(amount_1).unwrap());
    assert_eq!(test.token_0.balance(&pair_address), amount_0);
    assert_eq!(test.token_1.balance(&pair_address), amount_1);
    // Check LP token balance
    let expected_total_liquidity: i128 = (amount_0 * amount_1).sqrt(); // sqrt(amount_0, amount_1);
    static MINIMUM_LIQUIDITY: i128 = 1000;
    let expected_liquidity: i128 = expected_total_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap();

    assert_eq!(pair_client.balance(&test.user), expected_liquidity);
    assert_eq!(pair_client.total_shares(), expected_total_liquidity);


    let ledger_timestamp = 200;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();
    let expected_to_remove_0 = (amount_0*expected_liquidity) / expected_total_liquidity;
    let expected_to_remove_1 = (amount_1*expected_liquidity) / expected_total_liquidity;

    let (removed_0,removed_1) = test.contract.remove_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &expected_liquidity, //     liquidity: i128,
        &expected_to_remove_0, //     amount_a_min: i128,
        &expected_to_remove_1 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );
   
    assert_eq!(removed_0, expected_to_remove_0);
    assert_eq!(removed_1, expected_to_remove_1);
    let locked_0 = amount_0- removed_0;
    let locked_1 = amount_1- removed_1;
    
   // TODO: Test events

   // Check that not, the user does not have any LP anymore
   assert_eq!(pair_client.balance(&test.user), 0);

   // Check new user token balances, minus the minimum liquiity
   assert_eq!(test.token_0.balance(&test.user), initial_user_balance.checked_sub(locked_0).unwrap());
   assert_eq!(test.token_1.balance(&test.user), initial_user_balance.checked_sub(locked_1).unwrap());
   assert_eq!(test.token_0.balance(&pair_address), locked_0);
   assert_eq!(test.token_1.balance(&pair_address), locked_1);
    
}
