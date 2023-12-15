use crate::test::deposit::add_liquidity;
use crate::test::{SoroswapPairTest};
use soroban_sdk::{testutils::{Ledger}};
use crate::error::SoroswapPairError;

    
#[test]
// #[should_panic(expected = "SoroswapPair: not yet initialized")]
fn try_swap_not_yet_initialized() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    let result = test.contract.try_swap(&0, &0, &test.user);
    assert_eq!(result, Err(Ok(SoroswapPairError::NotInitialized)));
}

#[test]
// #[should_panic(expected = "SoroswapPair: insufficient output amount")]
fn try_swap_amounts_zero() {
    let test = SoroswapPairTest::setup();    
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let result = test.contract.try_swap(&0, &0, &test.user);
    assert_eq!(result, Err(Ok(SoroswapPairError::SwapInsufficientOutputAmount)));
}

#[test]
// #[should_panic(expected = "SoroswapPair: negatives dont supported")]
fn try_swap_amount_0_negative() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let result = test.contract.try_swap(&-1, &1, &test.user);
    assert_eq!(result, Err(Ok(SoroswapPairError::SwapNegativesOutNotSupported)));
}

#[test]
// #[should_panic(expected = "SoroswapPair: negatives dont supported")]
fn try_swap_amount_1_negative() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let result = test.contract.try_swap(&1, &-1, &test.user);
    assert_eq!(result, Err(Ok(SoroswapPairError::SwapNegativesOutNotSupported)));
}

#[test]
// #[should_panic(expected = "SoroswapPair: insufficient liquidity")]
fn try_swap_no_liquidity() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let result = test.contract.try_swap(&1, &1, &test.user);
    assert_eq!(result, Err(Ok(SoroswapPairError::SwapInsufficientLiquidity)));
}

#[test]
// #[should_panic(expected = "SoroswapPair: invalid to")]
fn try_swap_to_token_0() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    add_liquidity(&test, &amount_0, &amount_1);
    let result = test.contract.try_swap(&1000, &0, &test.token_0.address);
    assert_eq!(result, Err(Ok(SoroswapPairError::SwapInvalidTo)));
}

#[test]
// #[should_panic(expected = "SoroswapPair: invalid to")]
fn try_swap_to_token_1() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    add_liquidity(&test, &amount_0, &amount_1);
    let result = test.contract.try_swap(&1000, &0, &test.token_1.address);
    assert_eq!(result, Err(Ok(SoroswapPairError::SwapInvalidTo)));
}


#[test]
// #[should_panic(expected = "SoroswapPair: insufficient input amount")]
fn try_swap_token_0_insufficient_input() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    add_liquidity(&test, &amount_0, &amount_1);
    let result = test.contract.try_swap(&1000, &0, &test.user);
    assert_eq!(result, Err(Ok(SoroswapPairError::SwapInsufficientInputAmount)));
}

#[test]
// #[should_panic(expected = "SoroswapPair: insufficient input amount")]
fn try_swap_token_1_insufficient_input() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    add_liquidity(&test, &amount_0, &amount_1);
    let result = test.contract.try_swap(&0, &1000, &test.user);
    assert_eq!(result, Err(Ok(SoroswapPairError::SwapInsufficientInputAmount)));
}

#[test]
// #[should_panic(expected = "SoroswapPair: K constant is not met")]
fn try_swap_token_0_low_sent() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    add_liquidity(&test, &amount_0, &amount_1);
    test.token_0.transfer(&test.user, &test.contract.address, &1);
    let result = test.contract.try_swap(&0, &1000, &test.user);
    assert_eq!(result, Err(Ok(SoroswapPairError::SwapKConstantNotMet)));
}

#[test]
// #[should_panic(expected = "SoroswapPair: K constant is not met")]
fn try_swap_token_1_low_sent() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    add_liquidity(&test, &amount_0, &amount_1);
    test.token_1.transfer(&test.user, &test.contract.address, &1);
    let result = test.contract.try_swap(&1000, &0, &test.user);
    assert_eq!(result, Err(Ok(SoroswapPairError::SwapKConstantNotMet)));
}



#[test]
fn swap_token_0() {
    let test = SoroswapPairTest::setup();
    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();
    
    let original_0: i128 = test.token_0.balance(&test.user);
    let original_1: i128 = test.token_1.balance(&test.user);

    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    add_liquidity(&test, &amount_0, &amount_1);

    let init_time = 12345;
    test.env.ledger().with_mut(|li| {
        li.timestamp = init_time;
    });

    let swap_amount_0: i128 = 10_000_000;
    let expected_output_amount_1: i128 = 16624979;

    // The user sends the token first:
    test.token_0.transfer(&test.user, &test.contract.address, &swap_amount_0);

    test.contract.swap(&0, &expected_output_amount_1, &test.user);
    
    assert_eq!(test.contract.get_reserves(),
        (amount_0.checked_add(swap_amount_0).unwrap(),
        amount_1.checked_sub(expected_output_amount_1).unwrap()));

    assert_eq!(test.token_0.balance(&test.contract.address), amount_0.checked_add(swap_amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.contract.address), amount_1.checked_sub(expected_output_amount_1).unwrap());
    assert_eq!(test.token_0.balance(&test.user), original_0.checked_sub(amount_0).unwrap().checked_sub(swap_amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), original_1.checked_sub(amount_1).unwrap().checked_add(expected_output_amount_1).unwrap());

}


#[test]
fn swap_token_1() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    
    let original_0: i128 = test.token_0.balance(&test.user);
    let original_1: i128 = test.token_1.balance(&test.user);

    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    add_liquidity(&test, &amount_0, &amount_1);
    let init_time = 12345;
    test.env.ledger().with_mut(|li| {
        li.timestamp = init_time;
    });

    let swap_amount_1: i128 = 10_000_000;
    let expected_output_amount_0: i128 = 4533054;

    // The user sends the token first:
    test.token_1.transfer(&test.user, &test.contract.address, &swap_amount_1);

    test.contract.swap(&expected_output_amount_0, &0, &test.user);
    
    assert_eq!(test.contract.get_reserves(),
        (amount_0.checked_sub(expected_output_amount_0).unwrap(),
        amount_1.checked_add(swap_amount_1).unwrap()));

    assert_eq!(test.token_0.balance(&test.contract.address), amount_0.checked_sub(expected_output_amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.contract.address), amount_1.checked_add(swap_amount_1).unwrap());
    assert_eq!(test.token_0.balance(&test.user), original_0.checked_sub(amount_0).unwrap().checked_add(expected_output_amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), original_1.checked_sub(amount_1).unwrap().checked_sub(swap_amount_1).unwrap());

}

#[test]
// #[should_panic(expected = "SoroswapPair: K constant is not met")]
fn try_swap_token_1_optimal_plus_1() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    
    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    add_liquidity(&test, &amount_0, &amount_1);
    let init_time = 12345;
    test.env.ledger().with_mut(|li| {
        li.timestamp = init_time;
    });

    let swap_amount_1: i128 = 10_000_000;
    let expected_output_amount_0: i128 = 4533054 + 1;

    // The user sends the token first:
    test.token_1.transfer(&test.user, &test.contract.address, &swap_amount_1);

    let result = test.contract.try_swap(&expected_output_amount_0, &0, &test.user);
    assert_eq!(result, Err(Ok(SoroswapPairError::SwapKConstantNotMet)));
}
