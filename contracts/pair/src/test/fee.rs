use crate::test::{SoroswapPairTest};
use crate::test::deposit::add_liquidity;
use crate::soroswap_pair_token::{SoroswapPairTokenClient};
use num_integer::Roots; 



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
    assert_eq!(test.contract.get_reserves(), (amount_0,amount_1,0));
    assert_eq!(test.contract.k_last(), 0);

    let swap_amount_0 = 10_000_000;
    let expected_output_amount_1 = 16624979;

    test.token_0.transfer(&test.user, &test.contract.address, &swap_amount_0);
    test.contract.swap(&0, &expected_output_amount_1, &test.user);
    assert_eq!(test.contract.get_reserves(), (amount_0+swap_amount_0,amount_1-expected_output_amount_1,0));
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
    assert_eq!(test.contract.get_reserves(), (849,1180,0));

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

    assert_eq!(test.contract.k_last(), 0);
    add_liquidity(&test, &amount_0, &amount_1);

    // If we deposit with fee on, we should see a change in the klast paramenter
    //klast should be the new reserves (amount0 and amount1)
    assert_eq!(test.contract.k_last(), amount_0.checked_mul(amount_1).unwrap());
    assert_eq!(test.contract.total_shares(), expected_liquidity);


    let swap_amount_0 = 10_000_000;
    // Amount does not changes... only the fee is splitted differently
    let expected_output_amount_1 = 16624979;

    test.token_0.transfer(&test.user, &test.contract.address, &swap_amount_0);
    test.contract.swap(&0, &expected_output_amount_1, &test.user);
    //klast does not gets updated in swaps
    assert_eq!(test.contract.k_last(), amount_0.checked_mul(amount_1).unwrap());
    let new_expected_reserve_0= amount_0+swap_amount_0; // 60000000
    let new_expected_reserve_1= amount_1-expected_output_amount_1; // 83375021
    assert_eq!(test.contract.get_reserves(), (new_expected_reserve_0,new_expected_reserve_1,0));
    
    let k2_root=70728362; // new_expected_reserve_0.checked_mul(new_expected_reserve_1).unwrap().sqrt();
    assert_eq!(new_expected_reserve_0.checked_mul(new_expected_reserve_1).unwrap().sqrt(), k2_root);

    let k1_root = 70_710_678; // amount_0.checked_mul(amount_1).unwrap().sqrt();
    assert_eq!(amount_0.checked_mul(amount_1).unwrap().sqrt(), k1_root);

    // After the swap, k2 should be greater than k1
    assert_eq!(k2_root > k1_root, true);

    // Now we need to treat the contract as a SoroswapPairTokenClient
    let pair_token_client = SoroswapPairTokenClient::new(&test.env, &test.env.register_contract(&test.contract.address, crate::SoroswapPairToken {}));
    pair_token_client.transfer(&test.user, &test.contract.address, &expected_liquidity.checked_sub(minimum_liquidity).unwrap());

    // And now we need to treat it again as a SoroswapPairClient
    test.env.register_contract(&test.contract.address, crate::SoroswapPair {});
    // Now the env has that address again as a SoroswapPairClient

    assert_eq!(test.contract.total_shares(), expected_liquidity);
    test.contract.withdraw(&test.user);
    // n = expected_liquidity*(k2_root-k1_root)/(5k2_root + k1_root)
    // = 2946,719213655 --> 2946
    let n = 2946;
    let numerator = expected_liquidity.checked_mul(k2_root-k1_root).unwrap(); //1250447629752
    assert_eq!(numerator, 1250447629752);
    let denominator = (5_i128).checked_mul(k2_root).unwrap().checked_add(k1_root).unwrap();
    assert_eq!(denominator, 424352488);
    assert_eq!(n, numerator/denominator);
    assert_eq!(numerator.checked_div(denominator).unwrap(), n);
    
    // whe should have minted n shares to the admin:
    assert_eq!(test.contract.total_shares(), minimum_liquidity.checked_add(n).unwrap());
    assert_eq!(test.contract.my_balance(&test.contract.address), minimum_liquidity);
    assert_eq!(test.contract.my_balance(&test.admin), n);



    // TEST USER TOKEN BALANCES:
    // Because this is done before the withdraw, the user will see a difference in the total amount
    // that received while withdrawing:
    // the user had a liquidity position equal to (expected_liquidity-minimum_liquidity)
    let user_lp = 70709678; // (expected_liquidity-minimum_liquidity)
    assert_eq!(user_lp, (expected_liquidity-minimum_liquidity));

    // and it should have received: new_expected_reserve_0*(user_lp/(expected_liquidity+n))
    // (60000000*70709678)/(70710678+2946) = 59996651
    assert_eq!(new_expected_reserve_0, 60000000);
    let expected_user_out_token_0 = 59996651;
    assert_eq!(expected_user_out_token_0, (new_expected_reserve_0*user_lp)/(expected_liquidity+n));

    // similar for token 1:
    // (83375021*70709678)/(70710678+2946) =83370368
    let expected_user_out_token_1 = 83370368;
    assert_eq!(expected_user_out_token_1, (new_expected_reserve_1*user_lp)/(expected_liquidity+n));

    let original_total_supply_0: i128 = 123_000_000_000_000_000_000; // from the test file
    let original_total_supply_1: i128 = 321_000_000_000_000_000_000; // from the test file
    assert_eq!(test.token_0.balance(&test.user), original_total_supply_0-amount_0-swap_amount_0+expected_user_out_token_0);
    assert_eq!(test.token_1.balance(&test.user), original_total_supply_1-amount_1+expected_output_amount_1+expected_user_out_token_1);


    let after_withdraw_expected_reserve_0= 3349; // amount_0+swap_amount_0-expected_user_out_token_0; // 3349
    let after_withdraw_expected_reserve_1= 4653; //amount_1-expected_output_amount_1-expected_user_out_token_1; // 4653
    assert_eq!(after_withdraw_expected_reserve_0, amount_0+swap_amount_0-expected_user_out_token_0);
    assert_eq!(after_withdraw_expected_reserve_1, amount_1-expected_output_amount_1-expected_user_out_token_1);
    assert_eq!(test.contract.get_reserves(), (after_withdraw_expected_reserve_0,after_withdraw_expected_reserve_1,0));
    assert_eq!(test.contract.k_last(), after_withdraw_expected_reserve_0.checked_mul(after_withdraw_expected_reserve_1).unwrap());
    // assert_eq!(test.contract.my_balance(&test.user), 0);
    


    // // TEST ADMIN TOKEN BALANCES (IN N SHARES)
    
    let expected_admin_out_token_0 = 2500; // (3349*2946)/(1000+2946) = 2500
    let expected_admin_out_token_1 = 3473; // (4653*2946)/(1000+2946) = 3473

    // Now we need to treat the contract as a SoroswapPairTokenClient
    let pair_token_client = SoroswapPairTokenClient::new(&test.env, &test.env.register_contract(&test.contract.address, crate::SoroswapPairToken {}));
    pair_token_client.transfer(&test.admin, &test.contract.address, &n);

    // And now we need to treat it again as a SoroswapPairClient
    test.env.register_contract(&test.contract.address, crate::SoroswapPair {});
    // Now the env has that address again as a SoroswapPairClient

    assert_eq!(test.contract.total_shares(), 1000+2946);
    test.contract.withdraw(&test.admin);
    assert_eq!(test.token_0.balance(&test.admin), expected_admin_out_token_0);
    assert_eq!(test.token_1.balance(&test.admin), expected_admin_out_token_1);
 
    
}
