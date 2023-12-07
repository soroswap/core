use soroban_sdk::{
    testutils::{Events, Ledger},
    vec, 
    IntoVal, 
    symbol_short,
    Vec,
    Address};
use crate::test::{SoroswapRouterTest};
use crate::test::add_liquidity::add_liquidity;
use crate::event::{
    InitializedEvent,
    AddLiquidityEvent,
    RemoveLiquidityEvent,
    SwapEvent
};


#[test]
fn initialized_event() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);

    let initialized_event = test.env.events().all().last().unwrap();

    let expected_initialized_event: InitializedEvent = InitializedEvent {
        factory: test.factory.address.clone()
    };

    assert_eq!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapRouter", symbol_short!("init")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

    let false_initialized_event: InitializedEvent = InitializedEvent {
        factory: test.user,
    };

    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapRouter", symbol_short!("init")).into_val(&test.env),
                (false_initialized_event).into_val(&test.env)
            ),
        ]
    );


    // Wront symbol_short
    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapRouter", symbol_short!("iniit")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

    // Wront string
    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address,
                ("SoroswapRouterr", symbol_short!("init")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

}



#[test]
fn add_liquidity_event() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);

    let amount_0: i128 = 1_000_000_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000_000_000;

    let (deposited_amount_0, 
        deposited_amount_1, 
        received_liquidity) =add_liquidity(&test, &amount_0, &amount_1);
    let deterministic_pair_address = test.contract.router_pair_for(&test.token_0.address, &test.token_1.address);


    let add_liquidity_event = test.env.events().all().last().unwrap();

    let expected_add_liquidity_event: AddLiquidityEvent = AddLiquidityEvent {
        token_a: test.token_0.address.clone(),
        token_b: test.token_1.address.clone(),
        pair: deterministic_pair_address.clone(),
        amount_a: deposited_amount_0.clone(),
        amount_b: deposited_amount_1.clone(),
        liquidity: received_liquidity,
        to: test.user.clone(),
    };

    assert_eq!(
        vec![&test.env, add_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapRouter", symbol_short!("add")).into_val(&test.env),
                (expected_add_liquidity_event).into_val(&test.env)
            ),
        ]
    );

    let false_add_liquidity_event: AddLiquidityEvent = AddLiquidityEvent {
        token_a: test.token_0.address.clone(),
        token_b: test.token_1.address.clone(),
        pair: deterministic_pair_address,
        amount_a: deposited_amount_0.clone(),
        amount_b: deposited_amount_1.clone(),
        liquidity: 0, // False value
        to: test.user.clone(),
    };

    assert_ne!(
        vec![&test.env, add_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapRouter", symbol_short!("add")).into_val(&test.env),
                (false_add_liquidity_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong symbol_short
    assert_ne!(
        vec![&test.env, add_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapRouter", symbol_short!("addd")).into_val(&test.env),
                (expected_add_liquidity_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong string
    assert_ne!(
        vec![&test.env, add_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address,
                ("SoroswapRouterr", symbol_short!("add")).into_val(&test.env),
                (expected_add_liquidity_event).into_val(&test.env)
            ),
        ]
    );
}




#[test]
fn remove_liquidity_event() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);

    let ledger_timestamp = 100;
    let desired_deadline = 900;
    assert!(desired_deadline > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    let amount_0: i128 = 10_000_000_000;
    let amount_1: i128 = 20_000_000_000;
    // let expected_liquidity: i128 = 14142134623;//14142135623,7 - 1000;

    let (_deposited_amount_0, 
        _deposited_amount_1, 
        received_liquidity) = add_liquidity(&test, &amount_0, &amount_1);


    // let expected_to_remove_0 = (amount_0*expected_liquidity) / expected_total_liquidity;
    // let expected_to_remove_1 = (amount_1*expected_liquidity) / expected_total_liquidity;

    // (10000000000 * 14142134623) / 14142135623 = 9999999292
    let expected_to_remove_0 = 9999999292;

    // (20000000000 * 14142134623) / 14142135623 = 19999998585;
    let expected_to_remove_1 = 19999998585;

    test.contract.remove_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &received_liquidity, //     liquidity: i128,
        &expected_to_remove_0, //     amount_a_min: i128,
        &expected_to_remove_1 , //     amount_b_min: i128,
        &test.user, //     to: Address,
        &desired_deadline//     deadline: u64,
    );
    let deterministic_pair_address = test.contract.router_pair_for(&test.token_0.address, &test.token_1.address);


    let remove_liquidity_event = test.env.events().all().last().unwrap();

    let expected_remove_liquidity_event: RemoveLiquidityEvent = RemoveLiquidityEvent {
        token_a: test.token_0.address.clone(),
        token_b: test.token_1.address.clone(),
        pair: deterministic_pair_address.clone(),
        amount_a: expected_to_remove_0.clone(),
        amount_b: expected_to_remove_1.clone(),
        liquidity: received_liquidity,
        to: test.user.clone(),
    };

    assert_eq!(
        vec![&test.env, remove_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapRouter", symbol_short!("remove")).into_val(&test.env),
                (expected_remove_liquidity_event).into_val(&test.env)
            ),
        ]
    );

    let false_remove_liquidity_event: RemoveLiquidityEvent = RemoveLiquidityEvent {
        token_a: test.token_0.address.clone(),
        token_b: test.token_1.address.clone(),
        pair: deterministic_pair_address.clone(),
        amount_a: (expected_to_remove_0.clone()+1),
        amount_b: expected_to_remove_1.clone(),
        liquidity: received_liquidity,
        to: test.user.clone(),
    };

    assert_ne!(
        vec![&test.env, remove_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapRouter", symbol_short!("remove")).into_val(&test.env),
                (false_remove_liquidity_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong symbol_short
    assert_ne!(
        vec![&test.env, remove_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapRouter", symbol_short!("removee")).into_val(&test.env),
                (expected_remove_liquidity_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong string
    assert_ne!(
        vec![&test.env, remove_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address,
                ("SoroswapRouterr", symbol_short!("remove")).into_val(&test.env),
                (expected_remove_liquidity_event).into_val(&test.env)
            ),
        ]
    );
}



#[test]
fn swap_exact_tokens_for_tokens_event() {
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

    let swap_event = test.env.events().all().last().unwrap();

    let expected_swap_event: SwapEvent = SwapEvent {
        path: path.clone(),
        amounts: executed_amounts.clone(),
        to: test.user.clone(),
    };

    assert_eq!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapRouter", symbol_short!("swap")).into_val(&test.env),
                (expected_swap_event).into_val(&test.env)
            ),
        ]
    );


    let mut false_path: Vec<Address> = Vec::new(&test.env);
    false_path.push_back(test.token_1.address.clone());
    false_path.push_back(test.token_0.address.clone());


    let false_swap_event: SwapEvent = SwapEvent {
        path: false_path.clone(),
        amounts: executed_amounts.clone(),
        to: test.user.clone(),
    };

    assert_ne!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapRouter", symbol_short!("swap")).into_val(&test.env),
                (false_swap_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong symbol_short
    assert_ne!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapRouter", symbol_short!("swape")).into_val(&test.env),
                (expected_swap_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong string
    assert_ne!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address,
                ("SoroswapRouterr", symbol_short!("swap")).into_val(&test.env),
                (expected_swap_event).into_val(&test.env)
            ),
        ]
    );
    
}