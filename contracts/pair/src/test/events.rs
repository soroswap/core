use crate::test::{SoroswapPairTest};
use crate::event::{DepositEvent, SwapEvent};
use crate::test::deposit::add_liquidity;
use soroban_sdk::{testutils::{Ledger, Events}, vec, IntoVal, symbol_short};

#[test]
fn deposit_event() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    let amount_0: i128 = 1_001; //
    let amount_1: i128 = 1_001; //
    let expected_liquidity: i128 = 1;
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0);
    test.token_1.transfer(&test.user, &test.contract.address, &amount_1);
    let executed_liquidity = test.contract.deposit(&test.user);

    assert_eq!(expected_liquidity, executed_liquidity);

    let deposit_event = test.env.events().all().last().unwrap();

    let expected_deposit_event: DepositEvent = DepositEvent {
        to: test.user,
        amount_0: amount_0,
        amount_1: amount_1,
        liquidity: expected_liquidity,
        new_reserve_0: amount_1,
        new_reserve_1: amount_1
    };

    assert_eq!(
        vec![&test.env, deposit_event],
        vec![
            &test.env,
            (
                test.contract.address,
                ("SoroswapPair", symbol_short!("deposit")).into_val(&test.env),
                (expected_deposit_event).into_val(&test.env)
            ),
        ]
    );
}


#[test]
fn swap_event() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();

    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    add_liquidity(&test, &amount_0, &amount_1);

    let init_time = 12345;
    test.env.ledger().with_mut(|li| {
        li.timestamp = init_time;
    });

    let swap_amount_0: i128 = 10_000_000;
    let expected_output_amount_1: i128 = 16624979;

    // The user sends the token first:
    test.token_0.transfer(&test.user, &test.contract.address, &swap_amount_0);
    test.contract.swap(&0, &expected_output_amount_1, &test.user);
    
    let swap_event = test.env.events().all().last().unwrap();

    let expected_swap_event: SwapEvent = SwapEvent {
        to: test.user.clone(),
        amount_0_in: swap_amount_0.clone(),
        amount_1_in: 0,
        amount_0_out: 0,
        amount_1_out: expected_output_amount_1.clone(),
    };

    assert_eq!(
        vec![&test.env, swap_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapPair", symbol_short!("swap")).into_val(&test.env),
                (expected_swap_event).into_val(&test.env)
            ),
        ]
    );

    let false_swap_event: SwapEvent = SwapEvent {
        to: test.user,
        amount_0_in: swap_amount_0,
        amount_1_in: 1,
        amount_0_out: 0,
        amount_1_out: expected_output_amount_1,
    };

    assert_ne!(
        vec![&test.env, swap_event],
        vec![
            &test.env,
            (
                test.contract.address,
                ("SoroswapPair", symbol_short!("swap")).into_val(&test.env),
                (false_swap_event).into_val(&test.env)
            ),
        ]
    );
}