extern crate std;
use crate::event::{DepositEvent, SkimEvent, SwapEvent, SyncEvent, WithdrawEvent};
use crate::soroswap_pair_token::SoroswapPairTokenClient;
use crate::test::deposit::add_liquidity;
use crate::test::SoroswapPairTest;
use soroban_sdk::{
    symbol_short,
    testutils::{Events, Ledger},
    vec, IntoVal,
};

#[test]
fn deposit_event() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    let amount_0: i128 = 1_001; //
    let amount_1: i128 = 1_001; //
    let expected_liquidity: i128 = 1;
    test.contract.initialize(
        &test.factory.address,
        &test.token_0.address,
        &test.token_1.address,
    );
    test.token_0
        .transfer(&test.user, &test.contract.address, &amount_0);
    test.token_1
        .transfer(&test.user, &test.contract.address, &amount_1);
    let executed_liquidity = test.contract.deposit(&test.user);

    assert_eq!(expected_liquidity, executed_liquidity);

    let deposit_event = test.env.events().all().last().unwrap();

    let expected_deposit_event: DepositEvent = DepositEvent {
        to: test.user.clone(),
        amount_0: amount_0.clone(),
        amount_1: amount_1.clone(),
        liquidity: expected_liquidity.clone(),
        new_reserve_0: amount_1.clone(),
        new_reserve_1: amount_1.clone(),
    };

    assert_eq!(
        vec![&test.env, deposit_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapPair", symbol_short!("deposit")).into_val(&test.env),
                (expected_deposit_event).into_val(&test.env)
            ),
        ]
    );

    let false_deposit_event: DepositEvent = DepositEvent {
        to: test.user,
        amount_0: 0,
        amount_1: amount_1,
        liquidity: expected_liquidity,
        new_reserve_0: amount_1,
        new_reserve_1: amount_1,
    };

    assert_ne!(
        vec![&test.env, deposit_event],
        vec![
            &test.env,
            (
                test.contract.address,
                ("SoroswapPair", symbol_short!("deposit")).into_val(&test.env),
                (false_deposit_event).into_val(&test.env)
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
    test.contract.initialize(
        &test.factory.address,
        &test.token_0.address,
        &test.token_1.address,
    );
    add_liquidity(&test, &amount_0, &amount_1);

    let init_time = 12345;
    test.env.ledger().with_mut(|li| {
        li.timestamp = init_time;
    });

    let swap_amount_0: i128 = 10_000_000;
    let expected_output_amount_1: i128 = 16624979;

    // The user sends the token first:
    test.token_0
        .transfer(&test.user, &test.contract.address, &swap_amount_0);
    test.contract
        .swap(&0, &expected_output_amount_1, &test.user);

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

#[test]
fn withdraw_event() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(
        &test.factory.address,
        &test.token_0.address,
        &test.token_1.address,
    );
    let amount_0: i128 = 3_000_000;
    let amount_1: i128 = 3_000_000;
    let expected_liquidity: i128 = 3_000_000;
    let minimum_liquidity: i128 = 1_000;
    let user_liquidity = expected_liquidity.checked_sub(minimum_liquidity).unwrap();
    add_liquidity(&test, &amount_0, &amount_1);

    // Now we need to treat the contract as a SoroswapPairTokenClient
    let pair_token_client = SoroswapPairTokenClient::new(
        &test.env,
        &test
            .env
            .register_contract(&test.contract.address, crate::SoroswapPairToken {}),
    );
    pair_token_client.transfer(&test.user, &test.contract.address, &user_liquidity);

    // And now we need to treat it again as a SoroswapPairClient
    test.env
        .register_contract(&test.contract.address, crate::SoroswapPair {});

    // Now the env has that address again as a SoroswapPairClient

    let (amount_0_out, amount_1_out) = test.contract.withdraw(&test.user);

    let withdraw_event = test.env.events().all().last().unwrap();

    let expected_withdraw_event: WithdrawEvent = WithdrawEvent {
        to: test.user.clone(),
        liquidity: user_liquidity,
        amount_0: amount_0_out,
        amount_1: amount_1_out,
        new_reserve_0: minimum_liquidity,
        new_reserve_1: minimum_liquidity,
    };

    assert_eq!(
        vec![&test.env, withdraw_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapPair", symbol_short!("withdraw")).into_val(&test.env),
                (expected_withdraw_event).into_val(&test.env)
            ),
        ]
    );

    let false_withdraw_event: WithdrawEvent = WithdrawEvent {
        to: test.user.clone(),
        liquidity: user_liquidity,
        amount_0: 0,
        amount_1: amount_1_out,
        new_reserve_0: minimum_liquidity,
        new_reserve_1: minimum_liquidity,
    };

    assert_ne!(
        vec![&test.env, withdraw_event],
        vec![
            &test.env,
            (
                test.contract.address,
                ("SoroswapPair", symbol_short!("withdraw")).into_val(&test.env),
                (false_withdraw_event).into_val(&test.env)
            ),
        ]
    );
}

#[test]
fn sync_event() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(
        &test.factory.address,
        &test.token_0.address,
        &test.token_1.address,
    );

    let original_0: i128 = test.token_0.balance(&test.user);
    let original_1: i128 = test.token_1.balance(&test.user);
    let amount_0: i128 = 1_000_000;
    let amount_1: i128 = 4_000_000;
    add_liquidity(&test, &amount_0, &amount_1);

    // New balances:
    assert_eq!(
        test.token_0.balance(&test.user),
        original_0.checked_sub(amount_0).unwrap()
    );
    assert_eq!(
        test.token_1.balance(&test.user),
        original_1.checked_sub(amount_1).unwrap()
    );
    assert_eq!(test.token_0.balance(&test.contract.address), amount_0);
    assert_eq!(test.token_1.balance(&test.contract.address), amount_1);
    assert_eq!(test.contract.get_reserves(), (amount_0, amount_1));

    //extra tokens sent to skim:
    let amount_0_extra: i128 = 123_000_000;
    let amount_1_extra: i128 = 4_586_000;
    test.token_0
        .transfer(&test.user, &test.contract.address, &amount_0_extra);
    test.token_1
        .transfer(&test.user, &test.contract.address, &amount_1_extra);
    assert_eq!(
        test.token_0.balance(&test.contract.address),
        amount_0 + amount_0_extra
    );
    assert_eq!(
        test.token_1.balance(&test.contract.address),
        amount_1 + amount_1_extra
    );
    assert_eq!(test.contract.get_reserves(), (amount_0, amount_1));

    test.contract.sync();

    let sync_event = test.env.events().all().last().unwrap();

    let expected_sync_event: SyncEvent = SyncEvent {
        new_reserve_0: (amount_0 + amount_0_extra),
        new_reserve_1: (amount_1 + amount_1_extra),
    };

    assert_eq!(
        vec![&test.env, sync_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapPair", symbol_short!("sync")).into_val(&test.env),
                (expected_sync_event).into_val(&test.env)
            ),
        ]
    );

    let false_sync_event: SyncEvent = SyncEvent {
        new_reserve_0: (0),
        new_reserve_1: (amount_1 + amount_1_extra),
    };

    assert_ne!(
        vec![&test.env, sync_event],
        vec![
            &test.env,
            (
                test.contract.address,
                ("SoroswapPair", symbol_short!("sync")).into_val(&test.env),
                (false_sync_event).into_val(&test.env)
            ),
        ]
    );
}

#[test]
fn skim_event() {
    // zero tokens are being sent
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize(
        &test.factory.address,
        &test.token_0.address,
        &test.token_1.address,
    );

    let original_0: i128 = test.token_0.balance(&test.user);
    let original_1: i128 = test.token_1.balance(&test.user);
    let amount_0: i128 = 1_000_000;
    let amount_1: i128 = 4_000_000;
    add_liquidity(&test, &amount_0, &amount_1);

    // New balances:
    assert_eq!(
        test.token_0.balance(&test.user),
        original_0.checked_sub(amount_0).unwrap()
    );
    assert_eq!(
        test.token_1.balance(&test.user),
        original_1.checked_sub(amount_1).unwrap()
    );
    assert_eq!(test.token_0.balance(&test.contract.address), amount_0);
    assert_eq!(test.token_1.balance(&test.contract.address), amount_1);
    assert_eq!(test.contract.get_reserves(), (amount_0, amount_1));

    //extra tokens sent to skim:
    let amount_0_extra: i128 = 123_000_000;
    let amount_1_extra: i128 = 4_586_000;
    test.token_0
        .transfer(&test.user, &test.contract.address, &amount_0_extra);
    test.token_1
        .transfer(&test.user, &test.contract.address, &amount_1_extra);
    assert_eq!(
        test.token_0.balance(&test.contract.address),
        amount_0 + amount_0_extra
    );
    assert_eq!(
        test.token_1.balance(&test.contract.address),
        amount_1 + amount_1_extra
    );
    assert_eq!(test.contract.get_reserves(), (amount_0, amount_1));

    test.contract.skim(&test.admin);

    let skim_event = test.env.events().all().last().unwrap();

    let expected_skim_event: SkimEvent = SkimEvent {
        skimmed_0: amount_0_extra,
        skimmed_1: amount_1_extra,
    };

    assert_eq!(
        vec![&test.env, skim_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapPair", symbol_short!("skim")).into_val(&test.env),
                (expected_skim_event).into_val(&test.env)
            ),
        ]
    );

    let false_skim_event: SkimEvent = SkimEvent {
        skimmed_0: 0,
        skimmed_1: amount_1_extra,
    };

    assert_ne!(
        vec![&test.env, skim_event],
        vec![
            &test.env,
            (
                test.contract.address,
                ("SoroswapPair", symbol_short!("skim")).into_val(&test.env),
                (false_skim_event).into_val(&test.env)
            ),
        ]
    );
}
