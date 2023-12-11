use soroban_sdk::{Address, vec, Vec};
use crate::test::{SoroswapLibraryTest};
use crate::error::SoroswapLibraryError;


#[test]
fn get_amount_out() {
    let test = SoroswapLibraryTest::setup();
    assert_eq!(1,test.contract.get_amount_out(&2, &100, &100));
}

#[test]
fn get_amount_out_insufficient_input_amount() {
    let test = SoroswapLibraryTest::setup();
    let result = test.contract.try_get_amount_out(&0, &100, &100);
    assert_eq!(result, Err(Ok(SoroswapLibraryError::InsufficientInputAmount)));
}

#[test]
fn get_amount_out_insufficient_liquidity_0() {
    let test = SoroswapLibraryTest::setup();
    let result = test.contract.try_get_amount_out(&2, &0, &100);
    assert_eq!(result, Err(Ok(SoroswapLibraryError::InsufficientLiquidity)));
}

#[test]
fn get_amount_out_insufficient_liquidity_1() {
    let test = SoroswapLibraryTest::setup();
    let result = test.contract.try_get_amount_out(&2, &100, &0);
    assert_eq!(result, Err(Ok(SoroswapLibraryError::InsufficientLiquidity)));
}

    
#[test]
fn get_amount_in() {
    let test = SoroswapLibraryTest::setup();
    assert_eq!(2,test.contract.get_amount_in(&1, &100, &100));
}
#[test]
fn get_amount_in_insufficient_output_amount() {
    let test = SoroswapLibraryTest::setup();
    let result = test.contract.try_get_amount_in(&0, &100, &100);
    assert_eq!(result, Err(Ok(SoroswapLibraryError::InsufficientOutputAmount)));
}

#[test]
fn get_amount_in_insufficient_liquidity_0() {
    let test = SoroswapLibraryTest::setup();
    let result = test.contract.try_get_amount_in(&1, &0, &100);
    assert_eq!(result, Err(Ok(SoroswapLibraryError::InsufficientLiquidity)));
}

#[test]
fn get_amount_in_insufficient_liquidity_1() {
    let test = SoroswapLibraryTest::setup();
    let result = test.contract.try_get_amount_in(&1, &100, &0);
    assert_eq!(result, Err(Ok(SoroswapLibraryError::InsufficientLiquidity)));
}


#[test]
fn get_amounts_out() {
    let test = SoroswapLibraryTest::setup();
    
    let path: Vec<Address> =  vec![&test.env, test.token_0.address.clone(), test.token_1.address.clone()];

    // User needs to send these tokens first to the contract
    test.token_0.transfer(&test.user, &test.pair.address, &10000);
    test.token_1.transfer(&test.user, &test.pair.address, &10000);
    test.pair.deposit(&test.user);

    let expected_amounts_out = vec![&test.env, 2, 1];
    let amounts_out = test.contract.get_amounts_out(&test.factory.address, &2, &path);
    assert_eq!(expected_amounts_out,amounts_out);
}
#[test]
fn get_amounts_out_invalid_path() {
    let test = SoroswapLibraryTest::setup();
    let path: Vec<Address> = vec![&test.env, test.token_0.address.clone()];
    let result = test.contract.try_get_amounts_out(&test.factory.address, &2, &path);
    assert_eq!(result, Err(Ok(SoroswapLibraryError::InvalidPath)));
}

#[test]
fn get_amounts_in() {
    let test = SoroswapLibraryTest::setup();

    let path: Vec<Address> =  vec![&test.env, test.token_0.address.clone(), test.token_1.address.clone()];

    // User needs to send these tokens first to the contract
    test.token_0.transfer(&test.user, &test.pair.address, &10000);
    test.token_1.transfer(&test.user, &test.pair.address, &10000);
    test.pair.deposit(&test.user);
    
    let expected_amounts_in = vec![&test.env, 2, 1];
    let amounts_out = test.contract.get_amounts_in(&test.factory.address, &1, &path);
    assert_eq!(expected_amounts_in,amounts_out);
}
#[test]
fn get_amounts_in_invalid_path() {
    let test = SoroswapLibraryTest::setup();

    let path: Vec<Address> = vec![&test.env, test.token_0.address.clone()];
    let result = test.contract.try_get_amounts_in(&test.factory.address, &1, &path);
    assert_eq!(result, Err(Ok(SoroswapLibraryError::InvalidPath)));
}