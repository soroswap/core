use soroban_sdk::{testutils::{Events}, vec, IntoVal, symbol_short};
use soroban_sdk::{xdr::{ToXdr}, Bytes}; // For determinisitic address
use crate::test::{SoroswapFactoryTest};
use crate::event::{
    InitializedEvent,
    NewPairEvent,
    FeeToSettedEvent,
    NewSetterEvent,
    NewFeesEnabledEvent};


#[test]
fn initialized_event() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    let initialized_event = test.env.events().all().last().unwrap();

    let expected_initialized_event: InitializedEvent = InitializedEvent {
        setter: test.admin.clone()
    };

    assert_eq!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("init")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

    let false_initialized_event: InitializedEvent = InitializedEvent {
        setter: test.user,
    };

    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("init")).into_val(&test.env),
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
                ("SoroswapFactory", symbol_short!("iniit")).into_val(&test.env),
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
                ("SoroswapFactoryy", symbol_short!("init")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

}


#[test]
fn new_pair_event() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);
    test.contract.create_pair(&test.token_0.address, &test.token_1.address);

    // Calculating pair address:
    let mut salt = Bytes::new(&test.env);
    salt.append(&test.token_0.address.clone().to_xdr(&test.env)); 
    salt.append(&test.token_1.address.clone().to_xdr(&test.env));
    let bytesN_32_salt=test.env.crypto().sha256(&salt);
    let deterministic_pair_address = test.env.deployer().with_address(test.contract.address.clone(), bytesN_32_salt.clone()).deployed_address();

    let new_pair_event = test.env.events().all().last().unwrap();

    let expected_new_pair_event: NewPairEvent = NewPairEvent {
        token_0: test.token_0.address.clone(),
        token_1: test.token_1.address.clone(),
        pair: deterministic_pair_address.clone(),
        new_pairs_length: 1,
    };

    assert_eq!(
        vec![&test.env, new_pair_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("new_pair")).into_val(&test.env),
                (expected_new_pair_event).into_val(&test.env)
            ),
        ]
    );

    let false_new_pair_event: NewPairEvent = NewPairEvent {
        token_0: test.token_1.address,
        token_1: test.token_0.address,
        pair: deterministic_pair_address,
        new_pairs_length: 1,
    };

    assert_ne!(
        vec![&test.env, new_pair_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("new_pair")).into_val(&test.env),
                (false_new_pair_event).into_val(&test.env)
            ),
        ]
    );


    // Wront symbol_short
    assert_ne!(
        vec![&test.env, new_pair_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("new_pairr")).into_val(&test.env),
                (expected_new_pair_event).into_val(&test.env)
            ),
        ]
    );

    // Wront string
    assert_ne!(
        vec![&test.env, new_pair_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactoryy", symbol_short!("new_pair")).into_val(&test.env),
                (expected_new_pair_event).into_val(&test.env)
            ),
        ]
    );

    // new pair
    test.contract.create_pair(&test.token_2.address, &test.token_3.address);
    // Calculating pair address:
    let mut new_salt = Bytes::new(&test.env);
    new_salt.append(&test.token_2.address.clone().to_xdr(&test.env)); 
    new_salt.append(&test.token_3.address.clone().to_xdr(&test.env));
    let new_bytesN_32_salt=test.env.crypto().sha256(&new_salt);
    let new_deterministic_pair_address = test.env.deployer().with_address(test.contract.address.clone(), new_bytesN_32_salt.clone()).deployed_address();


    let new_expected_new_pair_event: NewPairEvent = NewPairEvent {
        token_0: test.token_2.address.clone(),
        token_1: test.token_3.address.clone(),
        pair: new_deterministic_pair_address.clone(),
        new_pairs_length: 2,
    };
    let new_new_pair_event = test.env.events().all().last().unwrap();

    assert_eq!(
        vec![&test.env, new_new_pair_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("new_pair")).into_val(&test.env),
                (new_expected_new_pair_event).into_val(&test.env)
            ),
        ]
    );
}


#[test]
fn fee_to_event() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);
    test.contract.set_fee_to(&test.user);

    let fee_to_event = test.env.events().all().last().unwrap();

    let expected_fee_to_event: FeeToSettedEvent = FeeToSettedEvent {
        setter: test.admin.clone(),
        old: test.admin.clone(),
        new: test.user.clone(),
    };

    assert_eq!(
        vec![&test.env, fee_to_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("fee_to")).into_val(&test.env),
                (expected_fee_to_event).into_val(&test.env)
            ),
        ]
    );

    let false_fee_to_event: FeeToSettedEvent = FeeToSettedEvent {
        setter: test.admin.clone(),
        old: test.user.clone(),
        new: test.user.clone(),
    };

    assert_ne!(
        vec![&test.env, fee_to_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("fee_to")).into_val(&test.env),
                (false_fee_to_event).into_val(&test.env)
            ),
        ]
    );


    // Wrong symbol_short
    assert_ne!(
        vec![&test.env, fee_to_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("fee_too")).into_val(&test.env),
                (expected_fee_to_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong string
    assert_ne!(
        vec![&test.env, fee_to_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address,
                ("SoroswapFactoryy", symbol_short!("fee_to")).into_val(&test.env),
                (expected_fee_to_event).into_val(&test.env)
            ),
        ]
    );

}


#[test]
fn setter_event() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);
    test.contract.set_fee_to_setter(&test.user);

    let new_setter_event = test.env.events().all().last().unwrap();

    let expected_new_setter_event: NewSetterEvent = NewSetterEvent {
        old: test.admin.clone(),
        new: test.user.clone(),
    };

    assert_eq!(
        vec![&test.env, new_setter_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("setter")).into_val(&test.env),
                (expected_new_setter_event).into_val(&test.env)
            ),
        ]
    );

    let false_new_setter: NewSetterEvent = NewSetterEvent {
        old: test.user.clone(),
        new: test.user.clone(),
    };

    assert_ne!(
        vec![&test.env, new_setter_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("setter")).into_val(&test.env),
                (false_new_setter).into_val(&test.env)
            ),
        ]
    );


    // Wrong symbol_short
    assert_ne!(
        vec![&test.env, new_setter_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("settero")).into_val(&test.env),
                (expected_new_setter_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong string
    assert_ne!(
        vec![&test.env, new_setter_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address,
                ("SoroswapFactoryy", symbol_short!("setter")).into_val(&test.env),
                (expected_new_setter_event).into_val(&test.env)
            ),
        ]
    );

}


#[test]
fn fees_enabled_event() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);
    test.contract.set_fees_enabled(&true);

    let fees_enabled_event = test.env.events().all().last().unwrap();

    let expected_fees_enabled_event: NewFeesEnabledEvent = NewFeesEnabledEvent {
        fees_enabled: true,
    };

    assert_eq!(
        vec![&test.env, fees_enabled_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("fees")).into_val(&test.env),
                (expected_fees_enabled_event).into_val(&test.env)
            ),
        ]
    );

    let false_fees_enabled_event: NewFeesEnabledEvent = NewFeesEnabledEvent {
        fees_enabled: false,
    };

    assert_ne!(
        vec![&test.env, fees_enabled_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("fees")).into_val(&test.env),
                (false_fees_enabled_event).into_val(&test.env)
            ),
        ]
    );


    // Wrong symbol_short
    assert_ne!(
        vec![&test.env, fees_enabled_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address.clone(),
                ("SoroswapFactory", symbol_short!("feess")).into_val(&test.env),
                (expected_fees_enabled_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong string
    assert_ne!(
        vec![&test.env, fees_enabled_event.clone()],
        vec![
            &test.env,
            (
                test.contract.address,
                ("SoroswapFactoryy", symbol_short!("fees")).into_val(&test.env),
                (expected_fees_enabled_event).into_val(&test.env)
            ),
        ]
    );

}


// // NEW "FEES ENABLED" BOOL
// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct NewFeesEnabledEvent {
//     pub fees_enabled: bool
// }

// pub(crate) fn new_fees_enabled(
//     e: &Env,
//     fees_enabled: bool) {
    
//     let event: NewFeesEnabledEvent = NewFeesEnabledEvent {
//         fees_enabled: fees_enabled,
//     };
//     e.events().publish(("SoroswapFactory", symbol_short!("fees")), event);
// }
