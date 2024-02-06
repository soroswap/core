use soroban_sdk::{Address, vec, Vec};
use crate::test::{SoroswapLibraryTest};
use crate::error::SoroswapLibraryError;


#[test]
fn get_amount_out() {
    let test = SoroswapLibraryTest::setup();
    
    
    // 100/101 = 0
    assert_eq!(0,test.contract.get_amount_out(&2, &100, &100));
    // 200/102 = 1.9 = 1
    assert_eq!(1,test.contract.get_amount_out(&3, &100, &100));
    //300/103 = 2.9 = 2
    assert_eq!(2,test.contract.get_amount_out(&4, &100, &100));
    //1001/1002 = 0
    assert_eq!(0,test.contract.get_amount_out(&2, &1001, &1001));
    //2002/1003 = 1.9 = 1
    assert_eq!(1,test.contract.get_amount_out(&3, &1001, &1001));

    // Make real pair quotes:
    let amount_0: i128 = 123456789;
    let amount_1: i128 = 987654321;
    
    //  Add Liquidity:
    test.token_0.transfer(&test.user, &test.pair.address, &amount_0);
    test.token_1.transfer(&test.user, &test.pair.address, &amount_1);
    test.pair.deposit(&test.user);
    assert_eq!(test.pair.get_reserves(), (amount_0, amount_1));

    let initial_0: i128 = test.token_0.balance(&test.user);
    let initial_1: i128 = test.token_1.balance(&test.user);

    //Deposit to do the swap
    let swap_amount_0: i128 = 584244;
    let expected_output_amount_1 = test.contract.get_amount_out(&swap_amount_0, &amount_0, &amount_1);
    test.token_0.transfer(&test.user, &test.pair.address, &swap_amount_0);
    test.pair.swap(&0, &expected_output_amount_1, &test.user);

    // Check new balances:
    let real_out_1 = test.token_1.balance(&test.user) - initial_1;
    assert_eq!(real_out_1,expected_output_amount_1);
    let real_in_0 = initial_0 - test.token_0.balance(&test.user);
    assert_eq!(real_in_0,swap_amount_0);
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
    assert_eq!(3,test.contract.get_amount_in(&1, &100, &100));
    assert_eq!(4,test.contract.get_amount_in(&2, &100, &100));
    assert_eq!(3,test.contract.get_amount_in(&1, &1001, &1001));

    // Make real pair quotes:
    let amount_0: i128 = 123456789;
    let amount_1: i128 = 987654321;
    
    //  Add Liquidity:
    test.token_0.transfer(&test.user, &test.pair.address, &amount_0);
    test.token_1.transfer(&test.user, &test.pair.address, &amount_1);
    test.pair.deposit(&test.user);
    assert_eq!(test.pair.get_reserves(), (amount_0, amount_1));

    let initial_0: i128 = test.token_0.balance(&test.user);
    let initial_1: i128 = test.token_1.balance(&test.user);

    //Deposit to do the swap
    let swap_expected_output_amount_1 = 76543;
    let swap_required_amount_0: i128 = test.contract.get_amount_in(&swap_expected_output_amount_1, &amount_0, &amount_1);
    test.token_0.transfer(&test.user, &test.pair.address, &swap_required_amount_0);
    test.pair.swap(&0, &swap_expected_output_amount_1, &test.user);

    // Check new balances:
    let real_out_1 = test.token_1.balance(&test.user) - initial_1;
    let real_in_0 = initial_0 - test.token_0.balance(&test.user);
    assert_eq!(real_out_1,swap_expected_output_amount_1);
    assert_eq!(real_in_0,swap_required_amount_0);
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

    let expected_amounts_out = vec![&test.env, 3, 1];
    let amounts_out = test.contract.get_amounts_out(&test.factory.address, &3, &path);
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
    
    let expected_amounts_in = vec![&test.env, 3, 1];
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