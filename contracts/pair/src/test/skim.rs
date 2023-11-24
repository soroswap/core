use crate::test::deposit::add_liquidity;
use crate::test::{SoroswapPairTest};
#[test]
#[should_panic]
fn skim_nothing() {
    // zero tokens are being sent
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.skim(&test.user);
}

#[test]
fn skim_with_liquidity_nothing_to_skim() {
    // zero tokens are being sent
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);

    let original_0: i128 = test.token_0.balance(&test.user);
    let original_1: i128 = test.token_1.balance(&test.user);
    let amount_0: i128 = 1_000_000;
    let amount_1: i128 = 4_000_000;
    add_liquidity(&test, &amount_0, &amount_1);

    // New balances:
    assert_eq!(test.token_0.balance(&test.user), original_0.checked_sub(amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), original_1.checked_sub(amount_1).unwrap());
    assert_eq!(test.token_0.balance(&test.contract.address), amount_0);
    assert_eq!(test.token_1.balance(&test.contract.address), amount_1);
    assert_eq!(test.contract.get_reserves(), (amount_0, amount_1,0));

    test.contract.skim(&test.user);
    //no tokens where sent to the user, nothing changed
    assert_eq!(test.token_0.balance(&test.user), original_0.checked_sub(amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), original_1.checked_sub(amount_1).unwrap());
    assert_eq!(test.token_0.balance(&test.contract.address), amount_0);
    assert_eq!(test.token_1.balance(&test.contract.address), amount_1);
    assert_eq!(test.contract.get_reserves(), (amount_0, amount_1,0));
}



#[test]
fn skim() {
    // zero tokens are being sent
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);

    let original_0: i128 = test.token_0.balance(&test.user);
    let original_1: i128 = test.token_1.balance(&test.user);
    let amount_0: i128 = 1_000_000;
    let amount_1: i128 = 4_000_000;
    add_liquidity(&test, &amount_0, &amount_1);

    // New balances:
    assert_eq!(test.token_0.balance(&test.user), original_0.checked_sub(amount_0).unwrap());
    assert_eq!(test.token_1.balance(&test.user), original_1.checked_sub(amount_1).unwrap());
    assert_eq!(test.token_0.balance(&test.contract.address), amount_0);
    assert_eq!(test.token_1.balance(&test.contract.address), amount_1);
    assert_eq!(test.contract.get_reserves(), (amount_0, amount_1,0));

    //extra tokens sent to skim:
    let amount_0_extra: i128 = 123_000_000;
    let amount_1_extra: i128 = 4_586_000;
    test.token_0.transfer(&test.user, &test.contract.address, &amount_0_extra);
    test.token_1.transfer(&test.user, &test.contract.address, &amount_1_extra);
    assert_eq!(test.token_0.balance(&test.contract.address), amount_0 + amount_0_extra);
    assert_eq!(test.token_1.balance(&test.contract.address), amount_1 + amount_1_extra);
    assert_eq!(test.contract.get_reserves(), (amount_0, amount_1,0));

    test.contract.skim(&test.admin);
    //no tokens where sent to the user, nothing changed
    assert_eq!(test.token_0.balance(&test.user), original_0 - amount_0 - amount_0_extra);
    assert_eq!(test.token_1.balance(&test.user), original_1 - amount_1 - amount_1_extra);
    assert_eq!(test.token_0.balance(&test.admin), amount_0_extra);
    assert_eq!(test.token_1.balance(&test.admin), amount_1_extra);
    assert_eq!(test.token_0.balance(&test.contract.address), amount_0);
    assert_eq!(test.token_1.balance(&test.contract.address), amount_1);
    assert_eq!(test.contract.get_reserves(), (amount_0, amount_1,0));
}
