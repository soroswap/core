use crate::test::{SoroswapPairTest};
use crate::token::{SoroswapPairTokenClient, SoroswapPairToken};
use soroban_sdk::{String};

// // TOKEN CONTRACT
// mod token {
//     soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
//     pub type TokenClient<'a> = Client<'a>;
// }
// //use token::TokenClient;



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
    
    assert_eq!(test.token_0.symbol(), String::from_slice(&test.env, "TOKEN0"));
    assert_eq!(test.token_1.symbol(), String::from_slice(&test.env, "TOKEN1"));
    assert_eq!(test.token_0.name(), String::from_slice(&test.env, "Token 0"));
    assert_eq!(test.token_1.name(), String::from_slice(&test.env, "Token 1"));

    // Test liqpool initial values:
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    assert_eq!(test.contract.token_0(), test.token_0.address);
    assert_eq!(test.contract.token_1(), test.token_1.address);
    assert_eq!(test.contract.factory(), test.factory.address);
    assert_eq!(test.contract.get_reserves(), (0,0,0));
    assert_eq!(test.contract.k_last(), 0);
    assert_eq!(test.contract.price_0_cumulative_last(), 0);
    assert_eq!(test.contract.price_1_cumulative_last(), 0);
    
    // Test pair as token
    /*
    For the purpose of testing SoroswapPairToken functions, we would need to "register" the contract
    again into the test env:
    https://docs.rs/soroban-sdk/20.0.0-rc2/soroban_sdk/struct.Env.html#method.register_contract_wasm
    This is because env.register_contract(Client) just takes into account the functions given by that client
    And register_contract_wasm does not knows how to handle the panic errors

    However, here we will use the same address, in order to get the already written info
    */
    
    let pair_token_client = SoroswapPairTokenClient::new(&test.env, &test.env.register_contract(&test.contract.address, crate::SoroswapPairToken {}));
    assert_eq!(pair_token_client.symbol(), String::from_slice(&test.env, "SOROSWAP-LP"));
    assert_eq!(pair_token_client.name(), String::from_slice(&test.env, "Soroswap LP Token"));
    assert_eq!(pair_token_client.decimals(), 7);
}
