use soroban_sdk::{testutils::{Events}, vec, IntoVal, symbol_short};
use crate::test::{SoroswapRouterTest};
use crate::event::{
    InitializedEvent,
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
