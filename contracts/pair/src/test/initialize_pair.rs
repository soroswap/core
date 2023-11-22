use crate::test::{SoroswapPairTest};
use soroban_sdk::{testutils::{Ledger}};


#[test]
#[should_panic(expected = "SoroswapPair: token_0 must be less than token_1")]
fn initialize_pair_token_1_less_than_token_0() {
    let test = SoroswapPairTest::setup();
    test.contract.initialize_pair(&test.factory.address, &test.token_1.address, &test.token_0.address);    
}


#[test]
#[should_panic(expected = "SoroswapPair: already initialized")]
fn double_initialize_pair() {
    let test = SoroswapPairTest::setup();
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
}


#[test]
fn initialize_pair_initial_values() {
    let test = SoroswapPairTest::setup();
    assert_eq!(test.factory.fee_to(), test.admin);
    assert_eq!(test.factory.fee_to_setter(), test.admin);
    assert_eq!(test.factory.fees_enabled(), false);

     // Test liqpool initial values:
     test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
     assert_eq!(test.contract.token_0(), test.token_0.address);
     assert_eq!(test.contract.token_1(), test.token_1.address);
     assert_eq!(test.contract.factory(), test.factory.address);
     assert_eq!(test.contract.get_reserves(), (0,0,0));
     assert_eq!(test.contract.k_last(), 0);
     assert_eq!(test.contract.price_0_cumulative_last(), 0);
     assert_eq!(test.contract.price_1_cumulative_last(), 0);

}
