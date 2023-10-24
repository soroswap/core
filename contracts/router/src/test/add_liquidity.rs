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

#[test]
#[should_panic(expected = "SoroswapRouter: expired")]
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

#[test]
fn test_add_liquidity_create_pair_get_amounts_out() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    
    let ledger_timestamp = 100;
    let desired_deadline = 1000;

    assert!(desired_deadline > ledger_timestamp);

    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    let reserve_0 = 10000;
    let reserve_1 = 10000;

    assert_eq!(test.factory.pair_exists(&test.token_0.address, &test.token_1.address), false);
    test.contract.add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &reserve_0, //     amount_a_desired: i128,
        &reserve_1, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );

    // We test that the pair now exist
    assert_eq!(test.factory.pair_exists(&test.token_0.address, &test.token_1.address), true);

    // We test that the pair was created succesfully
    let pair_address_one_way = test.factory.get_pair(&test.token_0.address, &test.token_1.address);
    let pair_address_other_way = test.factory.get_pair(&test.token_1.address, &test.token_0.address);
    assert_eq!(pair_address_one_way, pair_address_other_way);
    
    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();
    assert_eq!(test.factory.all_pairs(&0), pair_address_one_way); 
    assert_eq!(test.factory.all_pairs_length(), 1);
    
    let pair_client = SoroswapPairClient::new(&test.env, &pair_address_one_way);
    assert_eq!(pair_client.factory(), test.factory.address);
    assert_eq!(pair_client.token_0(), test.token_0.address);
    assert_eq!(pair_client.token_1(), test.token_1.address);
    
    // Correct initial reserves
    assert_eq!(pair_client.get_reserves(), (reserve_0, reserve_1,ledger_timestamp));
    
    // Correct router.getAmountsOut after adding liquidity
    let path = vec![&test.env, test.token_0.address, test.token_1.address];
    assert_eq!(test.contract.router_get_amounts_out(&2, &path), vec![&test.env,2, 1]);



}