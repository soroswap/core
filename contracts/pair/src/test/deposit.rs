use crate::test::{SoroswapPairTest};
use soroban_sdk::{testutils::{Ledger}};
use crate::test::pair::SoroswapPairError;


    
// Pub function that will be used in other tests:

pub fn add_liquidity(test: &SoroswapPairTest, amount_0: &i128, amount_1: &i128) -> i128 {
    
    // User needs to send these tokens first to the contract
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0);
    test.token_1.transfer(&test.user, &test.contract.address, &amount_1);
    test.contract.deposit(&test.user)
}
    
#[test]
// #[should_panic(expected = "SoroswapPair: not yet initialized")]
fn deposit_not_yet_initialized() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    let res = test.contract.try_deposit(&test.user);
    assert_eq!(res, Err(Ok(SoroswapPairError::NotInitialized)));
}

#[test]
// #[should_panic(expected = "SoroswapPair: insufficient amount of token 0 sent")]
fn deposit_zero_tokens_sent() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let res = test.contract.try_deposit(&test.user);
    assert_eq!(res, Err(Ok(SoroswapPairError::DepositInsufficientAmountToken0)));
}

#[test]
// #[should_panic(expected = "SoroswapPair: insufficient amount of token 1 sent")]
fn deposit_only_token_0_sent() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    let amount_0: i128 = 1_000_000;
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0);
    let res = test.contract.try_deposit(&test.user);
    assert_eq!(res, Err(Ok(SoroswapPairError::DepositInsufficientAmountToken1)));
}

#[test]
// #[should_panic(expected = "SoroswapPair: insufficient first liquidity minted")]
fn deposit_insufficient_first_liquidity() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    // If we just send 1,000 of each, the liq to be minted will be sqrt(1000*1000) - 1000 = 0, not enough
    let amount_0: i128 = 1_000;
    let amount_1: i128 = 1_000;
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0);
    test.token_1.transfer(&test.user, &test.contract.address, &amount_1);
    let res = test.contract.try_deposit(&test.user);
    assert_eq!(res, Err(Ok(SoroswapPairError::DepositInsufficientFirstLiquidity)));
}



#[test]
fn deposit_sufficient_first_liquidity() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    // If we just send 1,000 of each, the liq to be minted will be sqrt(1000*1000) - 1000 = 0, not enough
    let amount_0: i128 = 1_001; //
    let amount_1: i128 = 1_001; //
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0);
    test.token_1.transfer(&test.user, &test.contract.address, &amount_1);
    test.contract.deposit(&test.user);
}

#[test]
fn deposit_basic() {
    let test = SoroswapPairTest::setup();
    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();
    
    let init_time = 12345;
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
    assert_eq!(test.contract.my_balance(&test.user), 0);

    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    test.contract.deposit(&test.user);

    // New balances:
    assert_eq!(test.token_0.balance(&test.user), original_0.checked_sub(amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), original_1.checked_sub(amount_1).unwrap());

    // New LP balance:
    assert_eq!(test.contract.my_balance(&test.user), expected_liquidity- minimum_liquidity);
    // Reserves
    assert_eq!(test.contract.get_reserves(), (amount_0, amount_1));
}



#[test]
fn deposit_basic_2() {
    let test = SoroswapPairTest::setup();
    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0 = 1_000_000_000_000_000_000;
    let amount_1 = 4_000_000_000_000_000_000;
    add_liquidity(&test, &amount_0, &amount_1);
}