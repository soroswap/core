use soroban_sdk::{Address, String};
use crate::test::{SoroswapLibraryTest};
use crate::error::SoroswapLibraryError;



#[test]
fn sort_tokens_same_address() {
    let test = SoroswapLibraryTest::setup();
    let address_0 = Address::from_string(&String::from_str(&test.env, "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"));
    let result = test.contract.try_sort_tokens(&address_0, &address_0);
    assert_eq!(result, Err(Ok(SoroswapLibraryError::SortIdenticalTokens)));
}

#[test]
fn sort_tokens_correct_order() {
    let test = SoroswapLibraryTest::setup();
    let address_0 = Address::from_string(&String::from_str(&test.env, "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"));
    let address_1 = Address::from_string(&String::from_str(&test.env, "CDMLFMKMMD7MWZP3FKUBZPVHTUEDLSX4BYGYKH4GCESXYHS3IHQ4EIG4"));
    assert_eq!((address_0.clone(), address_1.clone()),test.contract.sort_tokens(&address_0, &address_1));
    assert_eq!((address_0.clone(), address_1.clone()),test.contract.sort_tokens(&address_1, &address_0));
}


#[test]
fn pair_for() {
    let test = SoroswapLibraryTest::setup();
    assert_eq!(test.pair.address,test.contract.pair_for(&test.factory.address, &test.token_0.address, &test.token_1.address));
    assert_eq!(test.pair.address,test.contract.pair_for(&test.factory.address, &test.token_1.address, &test.token_0.address));
}