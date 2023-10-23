use crate::test::SoroswapRouterTest;

use soroban_sdk::{Address, testutils::Address as _};



#[test]
fn test_initialize_and_get_factory() {
    let test = SoroswapRouterTest::setup();
    let factory = Address::random(&test.env);
    test.contract.initialize(&factory);
    assert_eq!(factory, test.contract.get_factory());
}

#[test]
#[should_panic(expected = "SoroswapRouter: not yet initialized")]
fn test_get_factory_not_yet_initialized() {
    let test = SoroswapRouterTest::setup();
    let factory = Address::random(&test.env);
    assert_eq!(factory, test.contract.get_factory());
}

#[test]
#[should_panic(expected = "SoroswapRouter: already initialized")]
fn test_initialize_twice() {
    let test = SoroswapRouterTest::setup();
    let factory = Address::random(&test.env);
    test.contract.initialize(&factory);
    let factory_another = Address::random(&test.env);
    test.contract.initialize(&factory_another);
}
