use crate::test::{SoroswapPairTest};
use crate::event::{DepositEvent};
use soroban_sdk::{testutils::{Ledger, Events}, vec, IntoVal, symbol_short, Address, Vec, Val, Env};

fn last_event_vec(e: &Env) -> Vec<(Address, Vec<Val>, Val)>{
    vec![&e, e.events().all().last().unwrap()]
}


#[test]
fn deposit() {
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