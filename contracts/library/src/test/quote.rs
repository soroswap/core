use crate::test::{SoroswapLibraryTest};
use crate::error::SoroswapLibraryError;

#[test]
fn quote_insufficient_amount() {
    let test = SoroswapLibraryTest::setup();
    let result = test.contract.try_quote(&0, &100, &200);
    assert_eq!(result, Err(Ok(SoroswapLibraryError::InsufficientAmount)));
}

#[test]
fn quote_insufficient_liquidity_0() {
    let test = SoroswapLibraryTest::setup();
    let result = test.contract.try_quote(&1, &0, &200);
    assert_eq!(result, Err(Ok(SoroswapLibraryError::InsufficientLiquidity)));
}

#[test]
fn quote_insufficient_liquidity_1() {
    let test = SoroswapLibraryTest::setup();
    let result = test.contract.try_quote(&1, &100, &0);
    assert_eq!(result, Err(Ok(SoroswapLibraryError::InsufficientLiquidity)));
}

#[test]
fn quote() {
    let test = SoroswapLibraryTest::setup();
    assert_eq!(2,test.contract.quote(&1, &100, &200));
    assert_eq!(1,test.contract.quote(&2, &200, &100));
}
