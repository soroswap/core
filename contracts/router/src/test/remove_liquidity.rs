use crate::test::{SoroswapRouterTest, SoroswapPairClient};

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

fn add_liquidity(test: &SoroswapRouterTest){
    let ledger_timestamp = 100;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;

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
}

#[test]
fn test_remove_liquidity() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    add_liquidity(&test);

    let ledger_timestamp = 200;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();

    let initial_user_balance: i128 = 10_000_000_000_000_000_000;
    let expected_liquidity: i128 = 2_000_000_000_000_000_000;
    static MINIMUM_LIQUIDITY: i128 = 1000;

    test.contract.remove_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &expected_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap(), //     liquidity: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );
   
   // TODO: Test events

   // Check that not, the user does not have any LP anymore
   let pair_address = test.factory.get_pair(&test.token_0.address, &test.token_1.address);
   let pair_client = SoroswapPairClient::new(&test.env, &pair_address);
   assert_eq!(pair_client.balance(&test.user), 0);

   // Check new user token balances, minus the minimum liquiity
   assert_eq!(test.token_0.balance(&test.user), initial_user_balance.checked_sub(500).unwrap());
   assert_eq!(test.token_1.balance(&test.user), initial_user_balance.checked_sub(2000).unwrap());
   assert_eq!(test.token_0.balance(&pair_address), 500);
   assert_eq!(test.token_1.balance(&pair_address), 2000);
    
}