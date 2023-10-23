use crate::test::SoroswapRouterTest;

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
fn test_add_liquidity_not_yet_initialized() {
    let test = SoroswapRouterTest::setup();
    test.contract.add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &10000, //     amount_a_desired: i128,
        &10000, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &0//     deadline: u64,
    );
}
    

#[test]
#[should_panic(expected = "Unauthorized function call for address")]
fn test_add_liquidity_not_authorized() {
    let test = SoroswapRouterTest::setup();
    let factory = Address::random(&test.env);
    test.contract.initialize(&factory);
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
                args: vec![
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

#[test]
#[should_panic(expected = "SoroswapRouter: expired")]
fn test_add_liquidity_deadline_expired() {
    let test = SoroswapRouterTest::setup();
    let factory = Address::random(&test.env);
    test.contract.initialize(&factory);

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
    //     Here we test the case when deadline has passed
    //  */
    test.contract.add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &0, //     amount_a_desired: i128,
        &0, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &bob, //     to: Address,
        &desired_deadline//     deadline: u64,
    );
}

// #[test]
// #[should_panic(expected = "SoroswapRouter: not yet initialized")]
// fn test_add_liquidity_not_yet_initialized() {
//     let test = SoroswapRouterTest::setup();
    
//     let ledger_timestamp = 100;
//     let desired_deadline = 1000;

//     assert!(desired_deadline > ledger_timestamp);

//     test.env.ledger().with_mut(|li| {
//         li.timestamp = ledger_timestamp;
//     });

//     test.contract.add_liquidity(
//         &test.token_0.address, //     token_a: Address,
//         &test.token_1.address, //     token_b: Address,
//         &10000, //     amount_a_desired: i128,
//         &10000, //     amount_b_desired: i128,
//         &0, //     amount_a_min: i128,
//         &0 , //     amount_b_min: i128,
//         &test.user, //     to: Address,
//         &desired_deadline//     deadline: u64,
//     );
// }