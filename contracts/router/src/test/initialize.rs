use soroban_sdk::{testutils::Address as _, Address};

use crate::error::CombinedRouterError;
use crate::test::SoroswapRouterTest;

#[test]
fn test_initialize_and_get_factory() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    assert_eq!(test.factory.address, test.contract.get_factory());
}

#[test]
fn test_get_factory_not_yet_initialized() {
    let test = SoroswapRouterTest::setup();
    let result = test.contract.try_get_factory();

    assert_eq!(result, Err(Ok(CombinedRouterError::RouterNotInitialized)));
}

#[test]
fn test_initialize_twice() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);

    let factory_another = Address::generate(&test.env);
    let result_second_init = test.contract.try_initialize(&factory_another);
    assert_eq!(
        result_second_init,
        Err(Ok(CombinedRouterError::RouterInitializeAlreadyInitialized))
    );
}
