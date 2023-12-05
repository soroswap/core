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



// // NEW PAIR CREATED EVENT: new_pair
// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct NewPair {
//     pub token_0: Address,
//     pub token_1: Address,
//     pub pair: Address,
//     pub new_pairs_length: u32
// }

// pub(crate) fn new_pair(
//     e: &Env, 
//     token_0: Address,
//     token_1: Address,
//     pair: Address,
//     new_pairs_length: u32) {
    
//     let event: NewPair = NewPair {
//         token_0: token_0,
//         token_1: token_1,
//         pair: pair,
//         new_pairs_length: new_pairs_length,
//     };
//     e.events().publish(("SoroswapFactory", symbol_short!("new_pair")), event);
// }



// // NEW "FEE TO" SETTED: new_fee_to // Event is "fee_to"
// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct FeeToSetted {
//     pub setter: Address,
//     pub old: Address,
//     pub new: Address
// }

// pub(crate) fn new_fee_to(
//     e: &Env,
//     setter: Address, 
//     old: Address,
//     new: Address) {
    
//     let event: FeeToSetted = FeeToSetted {
//         setter: setter,
//         old: old,
//         new: new
//     };
//     e.events().publish(("SoroswapFactory", symbol_short!("fee_to")), event);
// }


// // NEW "SETTER"
// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct NewSetterEvent {
//     pub old: Address,
//     pub new: Address
// }

// pub(crate) fn new_setter(
//     e: &Env,
//     old: Address,
//     new: Address) {
    
//     let event: NewSetterEvent = NewSetterEvent {
//         old: old,
//         new: new
//     };
//     e.events().publish(("SoroswapFactory", symbol_short!("setter")), event);
// }



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
