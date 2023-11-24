use crate::test::deposit::add_liquidity;
use crate::test::{SoroswapPairTest};
use soroban_sdk::{testutils::{Ledger}};

#[test]
fn swap() {
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

    //    fn swap(e: Env, amount_0_out: i128, amount_1_out: i128, to: Address) {
    test.contract.swap(&0, &expected_output_amount_1, &test.user);
    
    assert_eq!(test.contract.get_reserves(),
        (amount_0.checked_add(swap_amount_0).unwrap(),
        amount_1.checked_sub(expected_output_amount_1).unwrap(),init_time));

    assert_eq!(test.token_0.balance(&test.contract.address), amount_0.checked_add(swap_amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.contract.address), amount_1.checked_sub(expected_output_amount_1).unwrap());

    // New balances:
    assert_eq!(test.token_0.balance(&test.user), original_0.checked_sub(amount_0).unwrap().checked_sub(swap_amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), original_1.checked_sub(amount_1).unwrap().checked_add(expected_output_amount_1).unwrap());

}