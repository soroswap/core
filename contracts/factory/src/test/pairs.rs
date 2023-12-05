use crate::test::{SoroswapFactoryTest, SoroswapPairClient};
use soroban_sdk::{xdr::{ToXdr},
    Bytes,
    
};

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
    let bytesN_32_salt=test.env.crypto().sha256(&salt);
    let deterministic_pair_address = test.env.deployer().with_address(test.contract.address.clone(), bytesN_32_salt.clone()).deployed_address();

    let pair_address = test.contract.get_pair(&test.token_0.address, &test.token_1.address);
    let pair_address_other_way = test.contract.get_pair(&test.token_0.address, &test.token_1.address);
    assert_eq!(pair_address, deterministic_pair_address);
    assert_eq!(pair_address_other_way, deterministic_pair_address);

    let pair_client = SoroswapPairClient::new(&test.env, &pair_address);
    assert_eq!(pair_client.factory(), test.contract.address);

    assert_eq!(&pair_client.token_0(), &test.token_0.address);
    assert_eq!(&pair_client.token_1(), &test.token_1.address);
}



#[test]
#[should_panic(expected = "SoroswapFactory: pair already exist between token_0 and token_1")]
fn double_pair_creation() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    test.contract.create_pair(&test.token_0.address, &test.token_1.address);
    test.contract.create_pair(&test.token_0.address, &test.token_1.address);
}

#[test]
#[should_panic(expected = "SoroswapFactory: pair already exist between token_0 and token_1")]
fn double_pair_creation_other_way() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    test.contract.create_pair(&test.token_0.address, &test.token_1.address);
    test.contract.create_pair(&test.token_1.address, &test.token_0.address);
}

