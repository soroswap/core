use crate::test::{SoroswapPairTest};
use soroban_sdk::{  symbol_short,
    testutils::{Events, Ledger},
    Vec,
    Val,
    vec,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, 
    BytesN, 
    Env,
    IntoVal, Symbol};

    
// Pub function that will be used in other tests:

pub fn add_liquidity(test: &SoroswapPairTest, amount_0: &i128, amount_1: &i128){
    
    // User needs to send these tokens first to the contract
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0);
    test.token_1.transfer(&test.user, &test.contract.address, &amount_1);
    test.contract.deposit(&test.user);
}
    
    
#[test]
fn deposit_basic() {
    let test = SoroswapPairTest::setup();
    
    let mut init_time = 12345;
    test.env.ledger().with_mut(|li| {
        li.timestamp = init_time;
    });

    let original_0: i128 = test.token_0.balance(&test.user);
    let original_1: i128 = test.token_1.balance(&test.user);
    let amount_0: i128 = 1_000_000;
    let amount_1: i128 = 4_000_000;
    let expected_liquidity: i128 = 2_000_000;
    let minimum_liquidity: i128 = 1_000;


    // User needs to send these tokens first to the contract
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0);
    test.token_1.transfer(&test.user, &test.contract.address, &amount_1);

    assert_eq!(test.token_0.balance(&test.contract.address), amount_0);
    assert_eq!(test.token_1.balance(&test.contract.address), amount_1);

    // User does not hold any LP token first 
    // TODO: Transform my_balance to balance:
    // https://github.com/soroswap/core/issues
    assert_eq!(test.contract.my_balance(&test.user), 0);

    test.contract.deposit(&test.user);

    // New balances:
    assert_eq!(test.token_0.balance(&test.user), original_0.checked_sub(amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), original_1.checked_sub(amount_1).unwrap());

    // New LP balance:
    assert_eq!(test.contract.my_balance(&test.user), expected_liquidity- minimum_liquidity);
    // Reserves
    assert_eq!(test.contract.get_reserves(), (amount_0, amount_1,init_time));
}



#[test]
fn deposit_basic_2() {
    let test = SoroswapPairTest::setup();
    let amount_0 = 1_000_000_000_000_000_000;
    let amount_1 = 4_000_000_000_000_000_000;
    add_liquidity(&test, &amount_0, &amount_1);
}