use soroban_sdk::{
    Env,
    Address,
    BytesN,
    Bytes,
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Vec,
    Val,
    IntoVal,
};
use core::mem;

mod pair {
    soroban_sdk::contractimport!(file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm");
    pub type SoroswapPairClient<'a> = Client<'a>;
}
mod token {
    soroban_sdk::contractimport!(file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
mod factory {
    soroban_sdk::contractimport!(file = "./target/wasm32-unknown-unknown/release/soroswap_factory_contract.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>; 
}
use pair::SoroswapPairClient;
use token::TokenClient;
use factory::SoroswapFactoryClient;

struct SoroswapFactoryTest<'a> {
    env: Env,
    factory: SoroswapFactoryClient<'a>,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    pair: SoroswapPairClient<'a>
}

impl<'a> SoroswapFactoryTest<'a> {
    fn new() -> Self {
        let env: Env = Default::default();
        env.mock_all_auths();
        let admin = Address::random(&env);
        let user = Address::random(&env);
        let mut token_0: TokenClient<'a> = TokenClient::new(&env, &env.register_stellar_asset_contract(admin.clone()));
        let mut token_1: TokenClient<'a> = TokenClient::new(&env, &env.register_stellar_asset_contract(admin.clone()));
        if &token_1.address.contract_id() < &token_0.address.contract_id() {
            mem::swap(&mut token_0, &mut token_1);
        } else 
        if &token_1.address.contract_id() == &token_0.address.contract_id() {
            panic!("token contract ids are equal");
        }
        token_0.mint(&user, &10000);
        // token_1.mint(&user, &10000);
        let factory_address = &env.register_contract_wasm(None, factory::WASM);
        let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);

        // let contract: SoroswapFactoryClient<'a> = SoroswapFactoryClient::new(&env, factory_address);
        let factory = SoroswapFactoryClient::new(&env, &factory_address);
        factory.initialize(&admin, &pair_hash);
        factory.create_pair(&token_0.address, &token_1.address);
        let pair_address = factory.get_pair(&token_0.address, &token_1.address);
        let pair = SoroswapPairClient::new(&env, &pair_address);

        SoroswapFactoryTest {
            env,
            factory,
            token_0,
            token_1,
            pair
        }
    }
}

#[test]
pub fn create_and_register_factory_contract() {
    let factory_test = SoroswapFactoryTest::new();
}

#[test]
pub fn token_client_ne() {
    let factory_test = SoroswapFactoryTest::new();
    assert_ne!(factory_test.token_0.address, factory_test.token_1.address);
}

#[test]
pub fn compare_address() {
    use crate::{ SoroswapFactory, SoroswapFactoryClient};
    let env: Env = Default::default();
    env.mock_all_auths();
    let factory_test = SoroswapFactoryTest::new();
    let salt = BytesN::from_array(&env, &[0; 32]);
    // let deployed_address = env.deployer().with_address(factory_test.factory.address.clone(), salt.clone()).deployed_address();
    let wasm_hash = env.deployer().upload_contract_wasm(factory::WASM);


    // let factory_client = &env.register_contract(None, SoroswapFactory);
    let factory_client = SoroswapFactoryClient::new(&env, &env.register_contract(None, SoroswapFactory));
    let factory_address = env
        .deployer()
        .with_address(factory_client.address, salt)
        .deploy(wasm_hash);
    assert!(false, "should fail.")
    // assert_eq!(&factory_address, &deployed_address);
}