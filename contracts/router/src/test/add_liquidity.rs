use crate::test::{SoroswapRouterTest, SoroswapPairClient};
extern crate std;
use crate::error::{CombinedRouterError};


use soroban_sdk::{
    Address,
    testutils::{
        Address as _, 
        MockAuth,
        MockAuthInvoke,
        Ledger},
    vec,
    IntoVal};


// Pub function that will be used in other tests:


pub fn add_liquidity(
    test: &SoroswapRouterTest, 
    amount_0: &i128,
    amount_1: &i128) -> (i128, i128, i128){
    let ledger_timestamp = 100;
    let desired_deadline = 1000;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });


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
        )

}

#[test]
fn test_add_liquidity_not_yet_initialized() {
    let test = SoroswapRouterTest::setup();
    let result = test.contract.try_add_liquidity(
        &test.token_0.address,  // token_a: Address,
        &test.token_1.address,  // token_b: Address,
        &10000,                 // amount_a_desired: i128,
        &10000,                 // amount_b_desired: i128,
        &0,                     // amount_a_min: i128,
        &0,                     // amount_b_min: i128,
        &test.user,             // to: Address,
        &0,                     // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedRouterError::RouterNotInitialized)));
}


#[test]
fn test_add_liquidity_amount_a_desired_negative() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let result = test.contract.try_add_liquidity(
        &test.token_0.address,  // token_a: Address,
        &test.token_1.address,  // token_b: Address,
        &-1,                    // amount_a_desired: i128,
        &10000,                 // amount_b_desired: i128,
        &0,                     // amount_a_min: i128,
        &0,                     // amount_b_min: i128,
        &test.user,             // to: Address,
        &0,                     // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedRouterError::RouterNegativeNotAllowed)));
}

#[test]
fn test_add_liquidity_amount_b_desired_negative() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let result = test.contract.try_add_liquidity(
        &test.token_0.address,  // token_a: Address,
        &test.token_1.address,  // token_b: Address,
        &10000,                 // amount_a_desired: i128,
        &-1,                    // amount_b_desired: i128,
        &0,                     // amount_a_min: i128,
        &0,                     // amount_b_min: i128,
        &test.user,             // to: Address,
        &0,                     // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedRouterError::RouterNegativeNotAllowed)));
}

#[test]
fn test_add_liquidity_amount_a_min_negative() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let result = test.contract.try_add_liquidity(
        &test.token_0.address,  // token_a: Address,
        &test.token_1.address,  // token_b: Address,
        &10000,                 // amount_a_desired: i128,
        &10000,                 // amount_b_desired: i128,
        &-1,                    // amount_a_min: i128,
        &0,                     // amount_b_min: i128,
        &test.user,             // to: Address,
        &0,                     // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedRouterError::RouterNegativeNotAllowed)));
}

#[test]
fn test_add_liquidity_amount_b_min_negative() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let result = test.contract.try_add_liquidity(
        &test.token_0.address,  // token_a: Address,
        &test.token_1.address,  // token_b: Address,
        &10000,                 // amount_a_desired: i128,
        &10000,                 // amount_b_desired: i128,
        &0,                     // amount_a_min: i128,
        &-1,                    // amount_b_min: i128,
        &test.user,             // to: Address,
        &0,                     // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedRouterError::RouterNegativeNotAllowed)));
}


#[test]
#[should_panic(expected = "Unauthorized function call for address")]
fn test_add_liquidity_not_authorized() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let alice = Address::random(&test.env);
    let bob = Address::random(&test.env);
    // alice is not equal to bob
    assert_ne!(alice, bob);

    /*
        Here we test the add_liquidity function "to.require_auth();" requirement
        So if alice calls the function but sets "bob" in the "to" argument, this should fail
    */
    test.contract
        .mock_auths(&[MockAuth {
            address: &alice,
            invoke: &MockAuthInvoke {
                contract: &test.contract.address,
                fn_name: "add_liquidity",
                args: vec![&
                    &test.env,
                    test.token_0.address.into_val(&test.env), //     token_a: Address,
                    test.token_1.address.into_val(&test.env), //     token_b: Address,
                    0.into_val(&test.env), //     amount_a_desired: i128,
                    0.into_val(&test.env), //     amount_b_desired: i128,
                    0.into_val(&test.env), //     amount_a_min: i128,
                    0.into_val(&test.env) , //     amount_b_min: i128,
                    (&bob,).into_val(&test.env), //     to: Address,
                    0.into_val(&test.env)//     deadline: u64,
                    ],
                sub_invokes: &[],
            },
        }])
        .add_liquidity(
            &test.token_0.address, //     token_a: Address,
            &test.token_1.address, //     token_b: Address,
            &0, //     amount_a_desired: i128,
            &0, //     amount_b_desired: i128,
            &0, //     amount_a_min: i128,
            &0 , //     amount_b_min: i128,
            &bob, //     to: Address,
            &0//     deadline: u64,
        );

}


// #[test]
// fn test_add_liquidity_authorized() {
//     let test = SoroswapRouterTest::setup();
//     test.contract.initialize(&test.factory.address);
//     let ledger_timestamp = 100;
//     let desired_deadline = 1000;

//     assert!(desired_deadline > ledger_timestamp);

//     test.env.ledger().with_mut(|li| {
//         li.timestamp = ledger_timestamp;
//     });

//     let initial_user_balance = 10_000_000_000_000_000_000;
//     let amount_0: i128 = 1_000_000_000_000_000_000;
//     let amount_1: i128 = 4_000_000_000_000_000_000;
//     let expected_liquidity: i128 = 2_000_000_000_000_000_000;

//     // Check initial user value of every token:
//     assert_eq!(test.token_0.balance(&test.user), initial_user_balance);
//     assert_eq!(test.token_1.balance(&test.user), initial_user_balance);

//     assert_eq!(test.factory.pair_exists(&test.token_0.address, &test.token_1.address), false);
//     let deterministic_pair_address = test.contract.router_pair_for(&test.token_0.address, &test.token_1.address);

//     /*
//         Here we test the add_liquidity function "to.require_auth();" requirement
//         So if alice calls the function but sets "bob" in the "to" argument, this should fail

//         The Router Contract does 3 cross-contract calls that requires auth
//         TokenClient::new(&e, &token_a).transfer(&to, &pair, &amount_a); THIS ONE REQUIRES AUTH
//         TokenClient::new(&e, &token_b).transfer(&to, &pair, &amount_b); THIS ONE REQUIRES AUTH
//         let liquidity = SoroswapPairClient::new(&e, &pair).deposit(&to); THIS ONE DOES NOT REQUIRES AUTH
//     */
//     test.contract
//         .mock_auths(&[MockAuth {
//             address: &test.user,
//             invoke: &MockAuthInvoke {
//                 contract: &test.contract.address,
//                 fn_name: "add_liquidity",
//                 args: vec![&
//                     &test.env,
//                     test.token_0.address.into_val(&test.env), //     token_a: Address,
//                     test.token_1.address.into_val(&test.env), //     token_b: Address,
//                     amount_0.into_val(&test.env), //     amount_a_desired: i128,
//                     amount_1.into_val(&test.env), //     amount_b_desired: i128,
//                     0.into_val(&test.env), //     amount_a_min: i128,
//                     0.into_val(&test.env) , //     amount_b_min: i128,
//                     (&test.user,).into_val(&test.env), //     to: Address,
//                     desired_deadline.into_val(&test.env)//     deadline: u64,
//                     ],
//                 sub_invokes: &[MockAuthInvoke {
//                     contract: &test.token_0.address,
//                     fn_name: "transfer",
//                     args: vec![&
//                         &test.env,
//                         test.user.into_val(&test.env), //     to: Address, (from)
//                         deterministic_pair_address.into_val(&test.env), //     pair: Address, (to)
//                         amount_0.into_val(&test.env), //     amount_a: i128, (amount)
//                     ],
//                     sub_invokes: &[],
//                 },
//                 MockAuthInvoke {
//                     contract: &test.token_1.address,
//                     fn_name: "transfer",
//                     args: vec![&
//                         &test.env,
//                         test.user.into_val(&test.env), //     to: Address, (from)
//                         deterministic_pair_address.into_val(&test.env), //     pair: Address, (to)
//                         amount_1.into_val(&test.env), //     amount_a: i128, (amount)
//                     ],
//                     sub_invokes: &[],
//                 }
//                 ],
//             },
//         }])
//         .add_liquidity(
//             &test.token_0.address, //     token_a: Address,
//             &test.token_1.address, //     token_b: Address,
//             &amount_0, //     amount_a_desired: i128,
//             &amount_1, //     amount_b_desired: i128,
//             &0, //     amount_a_min: i128,
//             &0 , //     amount_b_min: i128,
//             &test.user, //     to: Address,
//             &desired_deadline//     deadline: u64,
//         );

// }

#[test]
fn test_add_liquidity_deadline_expired() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);

    let alice = Address::random(&test.env);
    let bob = Address::random(&test.env);
    // alice is not equal to bob
    assert_ne!(alice, bob);

    let ledger_timestamp = 100;
    let desired_deadline = 90;

    assert!(desired_deadline < ledger_timestamp);

    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    // /*
    //     Here we test the case when the deadline has passed
    //  */
    let result = test.contract.try_add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &0, //     amount_a_desired: i128,
        &0, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &bob, //     to: Address,
        &desired_deadline, //     deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedRouterError::RouterDeadlineExpired)));
}

// test pair exist, pair does not exist


#[test]
fn test_add_liquidity() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    
    let ledger_timestamp = 100;
    let desired_deadline = 1000;

    assert!(desired_deadline > ledger_timestamp);

    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    let initial_user_balance = 10_000_000_000_000_000_000;
    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;
    let expected_liquidity: i128 = 2_000_000_000_000_000_000;

    // Check initial user value of every token:
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance);

    assert_eq!(test.factory.pair_exists(&test.token_0.address, &test.token_1.address), false);
    let deterministic_pair_address = test.contract.router_pair_for(&test.token_0.address, &test.token_1.address);
    let (added_token_0, added_token_1, added_liquidity) = test.contract.add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &amount_0, //     amount_a_desired: i128,
        &amount_1, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );

    assert_eq!(added_token_0, amount_0);
    assert_eq!(added_token_1, amount_1);
    assert_eq!(added_liquidity, expected_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap());

    // TODO: Test events:

    // We test that the pair now exist
    assert_eq!(test.factory.pair_exists(&test.token_0.address, &test.token_1.address), true);

    // We test that the pair was created succesfully
    let pair_address = test.factory.get_pair(&test.token_0.address, &test.token_1.address);
    let pair_address_other_way = test.factory.get_pair(&test.token_1.address, &test.token_0.address);

    // We test that the addresses where correctly generated
    assert_eq!(pair_address, pair_address_other_way);
    assert_eq!(pair_address, deterministic_pair_address);
    
    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();
    // We test that factory has only 1 pair
    assert_eq!(test.factory.all_pairs(&0), pair_address); 
    assert_eq!(test.factory.all_pairs_length(), 1);
    
    let pair_client = SoroswapPairClient::new(&test.env, &pair_address);
    // We test that pair contract has assigned correctly the factory and tokens
    assert_eq!(pair_client.factory(), test.factory.address);
    assert_eq!(pair_client.token_0(), test.token_0.address);
    assert_eq!(pair_client.token_1(), test.token_1.address);

    // Check new balances:
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance.checked_sub(amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance.checked_sub(amount_1).unwrap());
    assert_eq!(test.token_0.balance(&pair_address), amount_0);
    assert_eq!(test.token_1.balance(&pair_address), amount_1);
   
    // Check initial reserves
    assert_eq!(pair_client.get_reserves(), (amount_0, amount_1));

    // Check initial total_shares
    assert_eq!(pair_client.total_shares(), expected_liquidity);

    // Check user LP balance
    static MINIMUM_LIQUIDITY: i128 = 1000;
    assert_eq!(pair_client.balance(&test.user), expected_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap());

    // We can provide liquidity again and should not panic
    let (new_added_token_0, new_added_token_1, new_added_liquidity) = test.contract.add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &amount_0, //     amount_a_desired: i128,
        &amount_1, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );

    assert_eq!(new_added_token_0, amount_0);
    assert_eq!(new_added_token_1, amount_1);
    assert_eq!(new_added_liquidity, expected_liquidity);
}

// We test that amount deducted from user is the same showed on reserve.
#[test]
fn test_add_liquidity_deducted_amount_reserve() {
    let test = SoroswapRouterTest::setup_deducted_reserve();
    test.env.budget().reset_unlimited();
    test.contract.initialize(&test.factory.address);
    
    let ledger_timestamp = 100;
    let desired_deadline = 1000;

    assert!(desired_deadline > ledger_timestamp);

    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    let initial_user_balance = 24_995_705_032_704;
    let amount_0: i128 = 1_000_000_000_000;
    let amount_1: i128 = 10_000_000_000_000;

    test.env.budget().reset_unlimited();
    // Check initial user value of every token:
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance);
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance);

    assert_eq!(test.factory.pair_exists(&test.token_0.address, &test.token_1.address), false);
    
    // Parameters are set as sent on frontend
    test.contract.add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &amount_0, //     amount_a_desired: i128,
        &amount_1, //     amount_b_desired: i128,
        &amount_0, //     amount_a_min: i128,
        &amount_1 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );

    // We test that the pair now exist
    assert_eq!(test.factory.pair_exists(&test.token_0.address, &test.token_1.address), true);
    test.env.budget().reset_unlimited();

    // We test that the pair was created succesfully
    let pair_address = test.factory.get_pair(&test.token_0.address, &test.token_1.address);
    let pair_address_other_way = test.factory.get_pair(&test.token_1.address, &test.token_0.address);
    assert_eq!(pair_address, pair_address_other_way);
    
    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();
    // We test that factory has only 1 pair
    assert_eq!(test.factory.all_pairs(&0), pair_address); 
    assert_eq!(test.factory.all_pairs_length(), 1);
    
    let pair_client = SoroswapPairClient::new(&test.env, &pair_address);
    // We test that pair contract has assigned correctly the factory and tokens
    assert_eq!(pair_client.factory(), test.factory.address);
    assert_eq!(pair_client.token_0(), test.token_0.address);
    assert_eq!(pair_client.token_1(), test.token_1.address);
    test.env.budget().reset_unlimited();

    // Check new balances:
    assert_eq!(test.token_0.balance(&test.user), initial_user_balance.checked_sub(amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), initial_user_balance.checked_sub(amount_1).unwrap());
    assert_eq!(test.token_0.balance(&pair_address), amount_0);
    assert_eq!(test.token_1.balance(&pair_address), amount_1);
   
    // Check initial reserves
    assert_eq!(pair_client.get_reserves(), (amount_0, amount_1));

}

// insufficient ammount (a and b)

#[test]
fn insufficient_b_amount() {
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
    
    add_liquidity(&test, &amount_0, &amount_1);
    
    // We can provide liquidity again and should not panic
    let result = test.contract.try_add_liquidity(
        &test.token_0.address,    // token_a: Address,
        &test.token_1.address,    // token_b: Address,
        &amount_0,                // amount_a_desired: i128,
        &amount_1,                // amount_b_desired: i128,
        &(amount_0),              // amount_a_min: i128,
        &(amount_1 + 1),          // amount_b_min: i128,
        &test.user,               // to: Address,
        &desired_deadline,        // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedRouterError::RouterInsufficientBAmount)));
}

#[test]
fn insufficient_a_amount() {
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
    
    add_liquidity(&test, &amount_0, &amount_1);
    
    // We can provide liquidity again and should not panic
    let result = test.contract.try_add_liquidity(
        &test.token_0.address,    // token_a: Address,
        &test.token_1.address,    // token_b: Address,
        &(amount_0 + 1),          // amount_a_desired: i128,
        &amount_1,                // amount_b_desired: i128,
        &(amount_0 + 1),          // amount_a_min: i128,
        &amount_1,                // amount_b_min: i128,
        &test.user,               // to: Address,
        &desired_deadline,        // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedRouterError::RouterInsufficientAAmount)));
}


#[test]
fn amount_a_desired_higher() {
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
    
    add_liquidity(&test, &amount_0, &amount_1);
    
    // We can provide liquidity again and should not panic
    let (new_added_token_0, new_added_token_1, _new_added_liquidity) = test.contract.add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &(amount_0+1), //     amount_a_desired: i128,
        &amount_1, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );

    assert_eq!(new_added_token_0, amount_0);
    assert_eq!(new_added_token_1, amount_1);
}


#[test]
fn amount_b_desired_higher() {
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
    
    add_liquidity(&test, &amount_0, &amount_1);
    
    // We can provide liquidity again and should not panic
    let (new_added_token_0, new_added_token_1, _new_added_liquidity) = test.contract.add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &(amount_0), //     amount_a_desired: i128,
        &(amount_1+1), //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );

    assert_eq!(new_added_token_0, amount_0);
    assert_eq!(new_added_token_1, amount_1);
}