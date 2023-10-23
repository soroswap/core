use crate::test::SoroswapRouterTest;

use soroban_sdk::{Address, testutils::Address as _};



#[test]
fn test_initialize_and_get_factory() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    assert_eq!(test.factory.address, test.contract.get_factory());
}

#[test]
#[should_panic(expected = "SoroswapRouter: not yet initialized")]
fn test_get_factory_not_yet_initialized() {
    let test = SoroswapRouterTest::setup();
    test.contract.get_factory();
}

#[test]
#[should_panic(expected = "SoroswapRouter: already initialized")]
fn test_initialize_twice() {
    let test = SoroswapRouterTest::setup();
    test.contract.initialize(&test.factory.address);
    let factory_another = Address::random(&test.env);
    test.contract.initialize(&factory_another);
}
