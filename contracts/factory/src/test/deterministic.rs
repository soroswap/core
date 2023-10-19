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
    Symbol
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
    admin: Address,
    user: Address,
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
            admin,
            user,
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
pub fn setter_is_admin() {
    let factory_test = SoroswapFactoryTest::new();
    assert_eq!(factory_test.factory.fee_to_setter(), factory_test.admin);
}

#[test]
pub fn setter_is_not_user() {
    let factory_test = SoroswapFactoryTest::new();
    assert_ne!(factory_test.factory.fee_to_setter(), factory_test.user);
}

#[test]
pub fn fees_are_not_enabled() {
    let factory_test = SoroswapFactoryTest::new();
    assert_eq!(factory_test.factory.fees_enabled(), false);
}

#[test]
pub fn set_fee_to_setter_user() {
    let factory_test = SoroswapFactoryTest::new();
    let env = factory_test.env;
    let user = factory_test.user;
    factory_test.factory.set_fee_to_setter(&user);
    let setter = factory_test.factory.fee_to_setter();
    assert_eq!(setter, user);
}

#[test]
pub fn authorize_user() {
    let factory_test = SoroswapFactoryTest::new();
    let factory = factory_test.factory;
    let factory_address = factory.address.clone();
    let admin_address = factory_test.admin.clone();
    let user = factory_test.user.clone();
    factory.set_fee_to_setter(&user);
    let auths = [(
        admin_address,
        AuthorizedInvocation {
            function: AuthorizedFunction::Contract((
                factory_address,
                Symbol::new(&factory.env, "set_fee_to_setter"),
                (user.clone(),).into_val(&factory.env)
            )),
            sub_invocations:[].into()
        }
    )];
    assert_eq!(factory.env.auths(), auths);
}

#[test]
pub fn set_fees_enabled() {
    let factory_test = SoroswapFactoryTest::new();
    let factory = factory_test.factory;
    factory.set_fees_enabled(&true);
    assert_eq!(factory.fees_enabled(), true);
}

#[test]
pub fn set_fee_to_factory_address() {
    let factory_test = SoroswapFactoryTest::new();
    let factory = factory_test.factory;
    factory.set_fees_enabled(&true);
    factory.set_fee_to(&factory.address);
    assert_eq!(factory.fee_to(), factory.address);
}

#[test]
pub fn pair_exists_both_directions() {
    let factory_test = SoroswapFactoryTest::new();
    let factory = factory_test.factory;
    let token_0 = factory_test.token_0;
    let token_1 = factory_test.token_1;
    assert_eq!(factory.pair_exists(&token_0.address, &token_1.address), true);
    assert_eq!(factory.pair_exists(&token_1.address, &token_0.address), true);
}

#[test]
pub fn pair_does_not_exists_both_directions() {
    let factory_test = SoroswapFactoryTest::new();
    let factory = factory_test.factory;
    let admin = factory_test.admin.clone();
    let token_a = TokenClient::new(&factory.env, &factory.env.register_stellar_asset_contract(admin.clone()));
    let token_b = TokenClient::new(&factory.env, &factory.env.register_stellar_asset_contract(admin.clone()));
    assert_eq!(factory.pair_exists(&token_a.address, &token_b.address), false);
    assert_eq!(factory.pair_exists(&token_b.address, &token_a.address), false);
}

#[test]
pub fn add_pair() {
    let factory_test = SoroswapFactoryTest::new();
    let factory = factory_test.factory;
    let admin = factory_test.admin.clone();
    let token_a = TokenClient::new(&factory.env, &factory.env.register_stellar_asset_contract(admin.clone()));
    let token_b = TokenClient::new(&factory.env, &factory.env.register_stellar_asset_contract(admin.clone()));
    factory.create_pair(&token_a.address, &token_b.address);
    assert_eq!(factory.pair_exists(&token_a.address, &token_b.address), true);
    assert_eq!(factory.pair_exists(&token_b.address, &token_a.address), true);
}

#[test]
pub fn all_pairs_length_is_one() {
    let factory_test = SoroswapFactoryTest::new();
    assert_eq!(factory_test.factory.all_pairs_length(), 1);
}

#[test]
pub fn pair_address_eq_both_directions() {
    let factory_test = SoroswapFactoryTest::new();
    let token_0_address = factory_test.token_0.address;
    let token_1_address = factory_test.token_1.address;
    let a = factory_test.factory.get_pair(&token_0_address, &token_1_address);
    let b = factory_test.factory.get_pair(&token_1_address, &token_0_address);
    assert_eq!(a, b)
}

#[test]
pub fn compare_pair_address() {
    let factory_test = SoroswapFactoryTest::new();
    let token_0_address = factory_test.token_0.address;
    let token_1_address = factory_test.token_1.address;
    let pair_address = factory_test.factory.get_pair(&token_0_address, &token_1_address);
    assert_eq!(pair_address, factory_test.pair.address);
}

// #[test]
pub fn compare_factory_address() {
    use crate::{ SoroswapFactory, SoroswapFactoryClient};
    let factory_test = SoroswapFactoryTest::new();
    let env = factory_test.env;
    env.mock_all_auths();
    let salt = BytesN::from_array(&env, &[0; 32]);
    let deployed_address = env.deployer().with_address(factory_test.factory.address.clone(), salt.clone()).deployed_address();
    let wasm_hash = env.deployer().upload_contract_wasm(factory::WASM);

    let factory_client = SoroswapFactoryClient::new(&env, &env.register_contract(None, SoroswapFactory));
    let factory_address = env
        .deployer()
        .with_address(factory_client.address, salt)
        .deploy(wasm_hash);
    let _init_fn = symbol_short!("init");
    let _init_fn_args: Vec<Val> = (5u32,).into_val(&env);
    // let res: Val = env.invoke_contract(&factory_address, &init_fn, init_fn_args);
    // assert!(false, "should fail.");
    assert_eq!(&factory_address, &deployed_address);
    // assert!(false, "todo");
}