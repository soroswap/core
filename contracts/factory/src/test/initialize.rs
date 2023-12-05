extern crate std;
use crate::test::{SoroswapFactoryTest};
use soroban_sdk::{
    IntoVal,
    testutils::{
        AuthorizedInvocation,
        AuthorizedFunction
    },
    Vec,
    Symbol
};

#[test]
#[should_panic(expected = "SoroswapFactory: not yet initialized")]
fn not_yet_initialized_fee_to() {
    let test = SoroswapFactoryTest::setup();
    test.contract.fee_to();
}

#[test]
#[should_panic(expected = "SoroswapFactory: not yet initialized")]
fn not_yet_initialized_fee_to_setter() {
    let test = SoroswapFactoryTest::setup();
    test.contract.fee_to_setter();
}


#[test]
#[should_panic(expected = "SoroswapFactory: not yet initialized")]
fn not_yet_initialized_fee_enabled() {
    let test = SoroswapFactoryTest::setup();
    test.contract.fees_enabled();
}


#[test]
#[should_panic(expected = "SoroswapFactory: not yet initialized")]
fn not_yet_initialized_all_pairs_length() {
    let test = SoroswapFactoryTest::setup();
    test.contract.all_pairs_length();
}


#[test]
#[should_panic(expected = "SoroswapFactory: not yet initialized")]
fn not_yet_initialized_get_pair() {
    let test = SoroswapFactoryTest::setup();
    test.contract.get_pair(&test.token_0.address, &test.token_1.address);
}


#[test]
#[should_panic(expected = "SoroswapFactory: not yet initialized")]
fn not_yet_initialized_all_pairs() {
    let test = SoroswapFactoryTest::setup();
    test.contract.all_pairs(&0);
}


#[test]
#[should_panic(expected = "SoroswapFactory: not yet initialized")]
fn not_yet_initialized_set_fee_to() {
    let test = SoroswapFactoryTest::setup();
    test.contract.set_fee_to(&test.admin);
}


#[test]
#[should_panic(expected = "SoroswapFactory: not yet initialized")]
fn not_yet_initialized_set_fee_to_setter() {
    let test = SoroswapFactoryTest::setup();
    test.contract.set_fee_to_setter(&test.admin);
}


#[test]
#[should_panic(expected = "SoroswapFactory: not yet initialized")]
fn not_yet_initialized_set_fees_enabled() {
    let test = SoroswapFactoryTest::setup();
    test.contract.set_fees_enabled(&true);
}

#[test]
#[should_panic(expected = "SoroswapFactory: not yet initialized")]
fn not_yet_initialized_create_pair() {
    let test = SoroswapFactoryTest::setup();
    test.contract.create_pair(&test.token_0.address, &test.token_1.address);
}



#[test]
#[should_panic(expected = "SoroswapFactory: already initialized")]
fn double_initialize_factory() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);
    test.contract.initialize(&test.admin, &test.pair_wasm);
}


#[test]
fn initialize_basic_info() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    // Let's check basic info
    assert_eq!(test.contract.fee_to_setter(), test.admin);
    assert_ne!(test.contract.fee_to_setter(), test.user);

    assert_eq!(test.contract.fee_to(), test.admin);
    assert_ne!(test.contract.fee_to(), test.user);

    assert_eq!(test.contract.all_pairs_length(), 0);
    assert_eq!(test.contract.fees_enabled(), false);

    test.contract.set_fee_to_setter(&test.user);

    assert_eq!(
         test.env.auths(),
         std::vec![(
             test.admin.clone(),
             AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    Symbol::new(&test.env, "set_fee_to_setter"),
                    (test.user.clone(),).into_val(&test.env)
                )),
                sub_invocations: std::vec![]
            }
         )]
    );

    assert_eq!(test.contract.fee_to_setter(), test.user);
    assert_ne!(test.contract.fee_to_setter(), test.admin);


    test.contract.set_fee_to(&test.user);

    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.user.clone(),
            AuthorizedInvocation {
               function: AuthorizedFunction::Contract((
                   test.contract.address.clone(),
                   Symbol::new(&test.env, "set_fee_to"),
                   (test.user.clone(),).into_val(&test.env)
               )),
               sub_invocations: std::vec![]
           }
        )]
   );

    assert_eq!(test.contract.fee_to(), test.user);
    assert_ne!(test.contract.fee_to(), test.admin);

    test.contract.set_fees_enabled(&true);

    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.user.clone(),
            AuthorizedInvocation {
               function: AuthorizedFunction::Contract((
                   test.contract.address.clone(),
                   Symbol::new(&test.env, "set_fees_enabled"),
                   (true,).into_val(&test.env)
               )),
               sub_invocations: std::vec![]
           }
        )]
   );


    assert_eq!(test.contract.fees_enabled(), true);

    test.contract.set_fees_enabled(&false);

    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.user.clone(),
            AuthorizedInvocation {
               function: AuthorizedFunction::Contract((
                   test.contract.address.clone(),
                   Symbol::new(&test.env, "set_fees_enabled"),
                   (false,).into_val(&test.env)
               )),
               sub_invocations: std::vec![]
           }
        )]
   );

    assert_eq!(test.contract.fees_enabled(), false);
}

