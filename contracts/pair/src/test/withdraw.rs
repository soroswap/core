use crate::test::{SoroswapPairTest};
use crate::test::deposit::add_liquidity;
use crate::soroswap_pair_token::{SoroswapPairTokenClient};


    
#[test]
#[should_panic(expected = "SoroswapPair: not yet initialized")]
fn withdraw_not_yet_initialized() {
    let test = SoroswapPairTest::setup();
    test.env.budget().reset_unlimited();
    test.contract.withdraw(&test.user);
}

   
#[test]
#[should_panic(expected = "SoroswapPair: liquidity was not initialized yet")]
fn withdraw_not_yet_deposited() {
    let test = SoroswapPairTest::setup();    
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    test.contract.withdraw(&test.user);
}

   
#[test]
#[should_panic(expected = "SoroswapPair: insufficient sent shares")]
fn withdraw_not_shares_sent() {
    let test = SoroswapPairTest::setup();    
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    add_liquidity(&test, &amount_0, &amount_1);
    test.contract.withdraw(&test.user);
}


#[test]
fn withdraw() {
    let test = SoroswapPairTest::setup();    
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0: i128 = 3_000_000;
    let amount_1: i128 = 3_000_000;
    let expected_liquidity: i128 =  3_000_000;
    let minimum_liquidity: i128 = 1_000;
    add_liquidity(&test, &amount_0, &amount_1);

    // Now we need to treat the contract as a SoroswapPairTokenClient
    let pair_token_client = SoroswapPairTokenClient::new(&test.env, &test.env.register_contract(&test.contract.address, crate::SoroswapPairToken {}));
    pair_token_client.transfer(&test.user, &test.contract.address, &expected_liquidity.checked_sub(minimum_liquidity).unwrap());

    // And now we need to treat it again as a SoroswapPairClient
    test.env.register_contract(&test.contract.address, crate::SoroswapPair {});
    // Now the env has that address again as a SoroswapPairClient

    test.contract.withdraw(&test.user);
    assert_eq!(test.contract.my_balance(&test.user), 0);
    assert_eq!(test.contract.total_shares(), minimum_liquidity);
    assert_eq!(test.token_0.balance(&test.contract.address), 1000);
    assert_eq!(test.token_1.balance(&test.contract.address), 1000);

    let original_total_supply_0: i128 = 123_000_000_000_000_000_000; // from the test file
    let original_total_supply_1: i128 = 321_000_000_000_000_000_000; // from the test file

    assert_eq!(test.token_0.balance(&test.user), original_total_supply_0.checked_sub(1000).unwrap());
    assert_eq!(test.token_1.balance(&test.user), original_total_supply_1.checked_sub(1000).unwrap());

    
}
