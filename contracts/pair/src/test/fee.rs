use crate::test::{SoroswapPairTest};
use crate::test::deposit::add_liquidity;
use crate::soroswap_pair_token::{SoroswapPairTokenClient};


#[test]
fn fee_off() {
    let test = SoroswapPairTest::setup();    
    test.env.budget().reset_unlimited();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    let expected_liquidity: i128 =  70_710_678;
    let minimum_liquidity: i128 = 1_000;

    assert_eq!(test.contract.k_last(), 0);
    add_liquidity(&test, &amount_0, &amount_1);
    assert_eq!(test.contract.k_last(), 0);

    let swap_amount_0 = 10_000_000;
    let expected_output_amount_1 = 16624979;

    test.token_0.transfer(&test.user, &test.contract.address, &swap_amount_0);
    test.contract.swap(&0, &expected_output_amount_1, &test.user);
    assert_eq!(test.contract.k_last(), 0);

    // Now we need to treat the contract as a SoroswapPairTokenClient
    let pair_token_client = SoroswapPairTokenClient::new(&test.env, &test.env.register_contract(&test.contract.address, crate::SoroswapPairToken {}));
    pair_token_client.transfer(&test.user, &test.contract.address, &expected_liquidity.checked_sub(minimum_liquidity).unwrap());

    // And now we need to treat it again as a SoroswapPairClient
    test.env.register_contract(&test.contract.address, crate::SoroswapPair {});
    // Now the env has that address again as a SoroswapPairClient

    test.contract.withdraw(&test.user);
    assert_eq!(test.contract.k_last(), 0);
    assert_eq!(test.contract.my_balance(&test.user), 0);
    assert_eq!(test.contract.total_shares(), minimum_liquidity);
    assert_eq!(test.contract.my_balance(&test.contract.address), minimum_liquidity);
    assert_eq!(test.token_0.balance(&test.contract.address), 849);
    assert_eq!(test.token_1.balance(&test.contract.address), 1180);
}


#[test]
fn fee_on() {
    let test = SoroswapPairTest::setup();    
    test.env.budget().reset_unlimited();
    test.factory.set_fees_enabled(&true);
    assert_eq!(test.factory.fees_enabled(), true);
    assert_eq!(test.factory.fee_to(), test.admin);
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);

    let amount_0: i128 = 50_000_000;
    let amount_1: i128 = 100_000_000;
    let minimum_liquidity: i128 = 1_000;
    let expected_liquidity: i128 =  70_710_678;

    add_liquidity(&test, &amount_0, &amount_1);

    let swap_amount_0 = 10_000_000;
    // Amount does not changes... only the fee is splitted differently
    let expected_output_amount_1 = 16624979;

    test.token_0.transfer(&test.user, &test.contract.address, &swap_amount_0);
    test.contract.swap(&0, &expected_output_amount_1, &test.user);

    // Now we need to treat the contract as a SoroswapPairTokenClient
    let pair_token_client = SoroswapPairTokenClient::new(&test.env, &test.env.register_contract(&test.contract.address, crate::SoroswapPairToken {}));
    pair_token_client.transfer(&test.user, &test.contract.address, &expected_liquidity.checked_sub(minimum_liquidity).unwrap());

    // And now we need to treat it again as a SoroswapPairClient
    test.env.register_contract(&test.contract.address, crate::SoroswapPair {});
    // Now the env has that address again as a SoroswapPairClient

    test.contract.withdraw(&test.user);
    assert_eq!(test.contract.my_balance(&test.user), 0);

    // Fees are payed in share tokens! Hence some share tokens where sent to the fee_to (admin)
    assert_eq!(test.contract.total_shares(), minimum_liquidity);
    // assert_eq!(test.contract.my_balance(&test.contract.address), minimum_liquidity);
    // assert_eq!(test.token_0.balance(&test.contract.address), 849);
    // assert_eq!(test.token_1.balance(&test.contract.address), 1180);
}
