use soroban_sdk::{
    Env,
    Address,
    BytesN,
    symbol_short,
    testutils::{
        Address as _, 
        AuthorizedFunction, 
        AuthorizedInvocation,
        MockAuth,
        MockAuthInvoke,
    },
    Vec,
    Val,
    IntoVal,
    Symbol,
    xdr::{
        ToXdr,
        ScAddress,
        ScVal,
        // ScObject,
        PublicKey,
        AccountId,
        Uint256
    },
    Bytes,
    TryFromVal,
    String,
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
    pub type _SoroswapFactoryClient<'a> = Client<'a>; 
}
use pair::SoroswapPairClient;
use token::TokenClient;
// use factory::SoroswapFactoryClient;
use crate::{ SoroswapFactory, SoroswapFactoryClient};

struct SoroswapFactoryTest<'a> {
    env: Env,
    alice: Address,
    bob: Address,
    factory: SoroswapFactoryClient<'a>,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    pair: SoroswapPairClient<'a>
}

impl<'a> SoroswapFactoryTest<'a> {
    fn new() -> Self {
        
        let env: Env = Default::default();
        env.mock_all_auths();
        let alice = Address::random(&env);
        let bob = Address::random(&env);
        let mut token_0: TokenClient<'a> = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
        let mut token_1: TokenClient<'a> = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
        if &token_1.address.contract_id() < &token_0.address.contract_id() {
            mem::swap(&mut token_0, &mut token_1);
        } else 
        if &token_1.address.contract_id() == &token_0.address.contract_id() {
            panic!("token contract ids are equal");
        }
        // The other form for registering the contract with the environment
        // interface is directly calling the WASM code:
        // 
        // let factory_address = &env.register_contract_wasm(None, factory::WASM);
        //
        let factory_address = &env.register_contract(None, SoroswapFactory);
        let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
        let factory = SoroswapFactoryClient::new(&env, &factory_address);
        factory.initialize(&alice, &pair_hash);
        factory.create_pair(&token_0.address, &token_1.address);
        let pair_address = factory.get_pair(&token_0.address, &token_1.address);
        let pair = SoroswapPairClient::new(&env, &pair_address);

        SoroswapFactoryTest {
            env,
            alice,
            bob,
            factory,
            token_0,
            token_1,
            pair
        }
    }
}

#[test]
pub fn create_and_register_factory_contract() {
    let _factory_test = SoroswapFactoryTest::new();
}

#[test]
pub fn token_client_ne() {
    let factory_test = SoroswapFactoryTest::new();
    assert_ne!(factory_test.token_0.address, factory_test.token_1.address);
}

#[test]
pub fn setter_is_alice() {
    let factory_test = SoroswapFactoryTest::new();
    assert_eq!(factory_test.factory.fee_to_setter(), factory_test.alice);
}

#[test]
pub fn setter_is_not_bob() {
    let factory_test = SoroswapFactoryTest::new();
    assert_ne!(factory_test.factory.fee_to_setter(), factory_test.bob);
}

#[test]
pub fn fees_are_not_enabled() {
    let factory_test = SoroswapFactoryTest::new();
    assert_eq!(factory_test.factory.fees_enabled(), false);
}

#[test]
pub fn set_fee_to_setter_bob() {
    let factory_test = SoroswapFactoryTest::new();
    let bob = factory_test.bob;
    factory_test.factory.set_fee_to_setter(&bob);
    let setter = factory_test.factory.fee_to_setter();
    assert_eq!(setter, bob);
}

#[test]
pub fn authorize_bob() {
    let factory_test = SoroswapFactoryTest::new();
    let factory = factory_test.factory;
    let factory_address = factory.address.clone();
    let alice_address = factory_test.alice.clone();
    let bob = factory_test.bob.clone();
    factory.set_fee_to_setter(&bob);
    let auths = [(
        alice_address,
        AuthorizedInvocation {
            function: AuthorizedFunction::Contract((
                factory_address,
                Symbol::new(&factory.env, "set_fee_to_setter"),
                (bob.clone(),).into_val(&factory.env)
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
    let alice = factory_test.alice.clone();
    let token_a = TokenClient::new(&factory.env, &factory.env.register_stellar_asset_contract(alice.clone()));
    let token_b = TokenClient::new(&factory.env, &factory.env.register_stellar_asset_contract(alice.clone()));
    assert_eq!(factory.pair_exists(&token_a.address, &token_b.address), false);
    assert_eq!(factory.pair_exists(&token_b.address, &token_a.address), false);
}

#[test]
pub fn add_pair() {
    let factory_test = SoroswapFactoryTest::new();
    let factory = factory_test.factory;
    let alice = factory_test.alice.clone();
    let token_a = TokenClient::new(&factory.env, &factory.env.register_stellar_asset_contract(alice.clone()));
    let token_b = TokenClient::new(&factory.env, &factory.env.register_stellar_asset_contract(alice.clone()));
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

#[test]
pub fn compare_deterministic_address() {
    let factory_test = SoroswapFactoryTest::new();
    let env = factory_test.env;
    env.mock_all_auths();

    // Calculating pair address:
    let mut salt = Bytes::new(&env);
    // Append the bytes of token_0 and token_1 to the salt
    salt.append(&factory_test.token_0.address.clone().to_xdr(&env)); 
    salt.append(&factory_test.token_1.address.clone().to_xdr(&env));
    // Hash the salt using SHA256 to generate a new BytesN<32> value
    let bytesN_32_salt=env.crypto().sha256(&salt);
    
    let calculated_pair_address = env.deployer().with_address(factory_test.factory.address.clone(), bytesN_32_salt.clone()).deployed_address();
    assert_eq!(&factory_test.pair.address, &calculated_pair_address);
}

#[test]
pub fn compare_deterministic_address_inverted() {
    let factory_test = SoroswapFactoryTest::new();
    let env = factory_test.env;
    env.mock_all_auths();

    // Calculating pair address:
    let mut salt = Bytes::new(&env);
    // Append the bytes of token_0 and token_1 to the salt
    salt.append(&factory_test.token_1.address.clone().to_xdr(&env)); 
    salt.append(&factory_test.token_0.address.clone().to_xdr(&env));
    // Hash the salt using SHA256 to generate a new BytesN<32> value
    let bytes_n_32_salt=env.crypto().sha256(&salt);
    
    let calculated_pair_address = env.deployer().with_address(factory_test.factory.address.clone(), bytes_n_32_salt.clone()).deployed_address();
    assert_ne!(&factory_test.pair.address, &calculated_pair_address);
}

#[test]
fn set_fee_to_address_from_zero_u8() {    
    let factory_test = SoroswapFactoryTest::new();
    let env = factory_test.env;
    let public_key = PublicKey::PublicKeyTypeEd25519(Uint256([0u8; 32]));
    let sc_account_id = AccountId(public_key);
    let sc_account = ScAddress::Account(sc_account_id);
    let zero_sc_addr = ScVal::Address(sc_account);

    let zero_addr: Address = Address::try_from_val(&env, &zero_sc_addr).unwrap();
    factory_test.factory.set_fee_to(&zero_addr);

    assert_eq!(zero_addr, factory_test.factory.fee_to());
}

#[test]
#[should_panic]
pub fn pair_is_unique_and_unequivocal_same_order() {
    let factory_test = SoroswapFactoryTest::new();
    let factory = factory_test.factory;
    let alice = factory_test.alice.clone();
    let token_a = TokenClient::new(&factory.env, &factory.env.register_stellar_asset_contract(alice.clone()));
    let token_b = TokenClient::new(&factory.env, &factory.env.register_stellar_asset_contract(alice.clone()));
    factory.create_pair(&token_a.address, &token_b.address);
    factory.create_pair(&token_a.address, &token_b.address);
}

#[test]
#[should_panic]
pub fn pair_is_unique_and_unequivocal_inverted_order() {
    let factory_test = SoroswapFactoryTest::new();
    let factory = factory_test.factory;
    let alice = factory_test.alice.clone();
    let token_a = TokenClient::new(&factory.env, &factory.env.register_stellar_asset_contract(alice.clone()));
    let token_b = TokenClient::new(&factory.env, &factory.env.register_stellar_asset_contract(alice.clone()));
    factory.create_pair(&token_a.address, &token_b.address);
    factory.create_pair(&token_b.address, &token_a.address);
}

#[test]
pub fn alice_bob_ne() {
    let factory_test = SoroswapFactoryTest::new();
    assert_ne!(factory_test.alice, factory_test.bob);
}

#[test]
pub fn authorized_invocation() {
    let factory_test = SoroswapFactoryTest::new();
    let factory = factory_test.factory;
    let alice = factory_test.alice.clone();
    let bob = factory_test.bob.clone();

    // alice is not equal to bob
    assert_ne!(alice, bob);
    // alice is fee_to_setter
    assert_eq!(alice, factory.fee_to_setter());

    // e.x.
    // let authorization = 
    // AuthorizedInvocation {
    //     function: 
    //     AuthorizedFunction::Contract((
    //         factory.address.clone(),
    //         Symbol::new(&factory.env, "set_fee_to_setter"),
    //         (bob.clone(),).into_val(&factory.env)
    //     )),
    //     sub_invocations:[].into()
    // };

    let r = factory
        .mock_auths(&[MockAuth {
            address: &alice,
            invoke: &MockAuthInvoke {
                contract: &factory.address,
                fn_name: "set_fee_to_setter",
                args: (&bob,).into_val(&factory_test.env),
                sub_invokes: &[],
            },
        }])
        .set_fee_to_setter(&bob);

    // setter is bob
    assert_eq!(bob, factory.fee_to_setter());
}

#[test]
#[should_panic]
pub fn non_authorized_invocation() {
    let factory_test = SoroswapFactoryTest::new();
    let factory = factory_test.factory;
    let alice = factory_test.alice.clone();
    let bob = factory_test.bob.clone();

    // alice is not equal to bob
    assert_ne!(alice, bob);
    // alice is fee_to_setter
    assert_eq!(alice, factory.fee_to_setter());

    // e.x.
    // let authorization = 
    // AuthorizedInvocation {
    //     function: 
    //     AuthorizedFunction::Contract((
    //         factory.address.clone(),
    //         Symbol::new(&factory.env, "set_fee_to_setter"),
    //         (bob.clone(),).into_val(&factory.env)
    //     )),
    //     sub_invocations:[].into()
    // };

    let r = factory
        .mock_auths(&[MockAuth {
            address: &bob,
            invoke: &MockAuthInvoke {
                contract: &factory.address,
                fn_name: "set_fee_to_setter",
                args: (&bob,).into_val(&factory_test.env),
                sub_invokes: &[],
            },
        }])
        .set_fee_to_setter(&bob);
        
    // setter is bob
    assert_eq!(bob, factory.fee_to_setter());
}