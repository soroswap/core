extern crate std;
use crate::test::SoroswapFactoryTest;
use soroban_sdk::{
    testutils::{AuthorizedFunction, AuthorizedInvocation},
    IntoVal, Symbol,
};
//use super::*; // Import the necessary modules and types
use soroswap_factory_interface::FactoryError;

#[test]
fn not_yet_initialized_fee_to() {
    let test = SoroswapFactoryTest::setup();
    let res = test.contract.try_fee_to();
    assert_eq!(res, Err(Ok(FactoryError::NotInitialized)));
}

#[test]
fn not_yet_initialized_fee_to_setter() {
    let test = SoroswapFactoryTest::setup();
    let res = test.contract.try_fee_to_setter();
    assert_eq!(res, Err(Ok(FactoryError::NotInitialized)));
}

#[test]
fn not_yet_initialized_fee_enabled() {
    let test = SoroswapFactoryTest::setup();
    let res = test.contract.try_fees_enabled();
    assert_eq!(res, Err(Ok(FactoryError::NotInitialized)));
}

#[test]
fn not_yet_initialized_all_pairs_length() {
    let test = SoroswapFactoryTest::setup();
    let res = test.contract.try_all_pairs_length();
    assert_eq!(res, Err(Ok(FactoryError::NotInitialized)));
}

#[test]
fn not_yet_initialized_get_pair() {
    let test = SoroswapFactoryTest::setup();
    let res = test
        .contract
        .try_get_pair(&test.token_0.address, &test.token_1.address);
    assert_eq!(res, Err(Ok(FactoryError::NotInitialized)));
}

#[test]
fn not_yet_initialized_all_pairs() {
    let test = SoroswapFactoryTest::setup();
    let res = test.contract.try_all_pairs(&0);
    assert_eq!(res, Err(Ok(FactoryError::NotInitialized)));
}

#[test]
fn not_yet_initialized_set_fee_to() {
    let test = SoroswapFactoryTest::setup();
    let res = test.contract.try_set_fee_to(&test.admin);
    assert_eq!(res, Err(Ok(FactoryError::NotInitialized)));
}

#[test]
fn not_yet_initialized_set_fee_to_setter() {
    let test = SoroswapFactoryTest::setup();
    let res = test.contract.try_set_fee_to_setter(&test.admin);
    assert_eq!(res, Err(Ok(FactoryError::NotInitialized)));
}

#[test]
fn not_yet_initialized_set_fees_enabled() {
    let test = SoroswapFactoryTest::setup();
    let res = test.contract.try_set_fees_enabled(&true);
    assert_eq!(res, Err(Ok(FactoryError::NotInitialized)));
}

#[test]
fn not_yet_initialized_create_pair() {
    let test = SoroswapFactoryTest::setup();
    let res = test
        .contract
        .try_create_pair(&test.token_0.address, &test.token_1.address);
    assert_eq!(res, Err(Ok(FactoryError::NotInitialized)));
}

#[test]
fn double_initialize_factory() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);
    let res = test.contract.try_initialize(&test.admin, &test.pair_wasm);
    assert_eq!(res, Err(Ok(FactoryError::InitializeAlreadyInitialized)));
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
