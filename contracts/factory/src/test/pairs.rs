use crate::test::{SoroswapFactoryTest, SoroswapPairClient};
use soroban_sdk::{xdr::{ToXdr},
    Bytes,
};
use soroswap_factory_interface::{FactoryError};


#[test]
fn create_pair_one_way() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    assert_eq!(test.contract.all_pairs_length(), 0);
    assert_eq!(test.contract.pair_exists(&test.token_0.address, &test.token_1.address), false);
    assert_eq!(test.contract.pair_exists(&test.token_1.address, &test.token_0.address), false);

    test.contract.create_pair(&test.token_0.address, &test.token_1.address);

    assert_eq!(test.contract.all_pairs_length(), 1);
    assert_eq!(test.contract.pair_exists(&test.token_0.address, &test.token_1.address), true);
    assert_eq!(test.contract.pair_exists(&test.token_1.address, &test.token_0.address), true);

    // Calculating pair address:
    let mut salt = Bytes::new(&test.env);
    salt.append(&test.token_0.address.clone().to_xdr(&test.env)); 
    salt.append(&test.token_1.address.clone().to_xdr(&test.env));
    let bytes_n_32_salt=test.env.crypto().sha256(&salt);
    let deterministic_pair_address = test.env.deployer().with_address(test.contract.address.clone(), bytes_n_32_salt.clone()).deployed_address();

    let pair_address = test.contract.get_pair(&test.token_0.address, &test.token_1.address);
    let pair_address_other_way = test.contract.get_pair(&test.token_0.address, &test.token_1.address);
    assert_eq!(pair_address, deterministic_pair_address);
    assert_eq!(pair_address_other_way, deterministic_pair_address);

    let get_pair_0 = test.contract.all_pairs(&0);
    assert_eq!(pair_address, get_pair_0);

    let pair_client = SoroswapPairClient::new(&test.env, &pair_address);
    assert_eq!(pair_client.factory(), test.contract.address);

    assert_eq!(&pair_client.token_0(), &test.token_0.address);
    assert_eq!(&pair_client.token_1(), &test.token_1.address);
}


#[test]
fn double_pair_creation() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    test.contract.create_pair(&test.token_0.address, &test.token_1.address);
    let res = test.contract.try_create_pair(&test.token_0.address, &test.token_1.address);

    assert_eq!(res, Err(Ok(FactoryError::CreatePairAlreadyExists)));
}

#[test]
fn double_pair_creation_other_way() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    test.contract.create_pair(&test.token_0.address, &test.token_1.address);
    let res = test.contract.try_create_pair(&test.token_1.address, &test.token_0.address);

    assert_eq!(res, Err(Ok(FactoryError::CreatePairAlreadyExists)));
}

#[test]
fn get_pair_does_not_exist() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    let res = test.contract.try_get_pair(&test.token_0.address, &test.token_1.address);
    assert_eq!(res, Err(Ok(FactoryError::PairDoesNotExist)));
}


#[test]
fn create_identical_tokens() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    let res = test.contract.try_create_pair(&test.token_0.address, &test.token_0.address);

    assert_eq!(res, Err(Ok(FactoryError::CreatePairIdenticalTokens)));
}


#[test]
fn create_pair_index_does_not_exist() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    test.contract.create_pair(&test.token_0.address, &test.token_1.address);

    let res = test.contract.try_all_pairs(&1);
    assert_eq!(res, Err(Ok(FactoryError::IndexDoesNotExist)));

}

#[test]
fn no_pair_index_does_not_exist() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);
    let res = test.contract.try_all_pairs(&0);
    assert_eq!(res, Err(Ok(FactoryError::IndexDoesNotExist)));

}
