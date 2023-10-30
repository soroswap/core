use soroban_sdk::{
    contracttype, 
    xdr::ToXdr, 
    Address, 
    Bytes, 
    BytesN, 
    Env, 
    IntoVal,
    testutils::{
        MockAuth,
        MockAuthInvoke,
    },
};

mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
mod pair {
    soroban_sdk::contractimport!(file = "./target/wasm32-unknown-unknown/release/soroswap_pair.wasm");
}
mod factory {
    soroban_sdk::contractimport!(file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>; 
}

use soroban_sdk::testutils::Address as _;
use crate::{
    SoroswapPair, 
    SoroswapPairClient,
};
use token::TokenClient;
use factory::{
    SoroswapFactoryClient,
    WASM as FACTORY_WASM,
};


#[contracttype]
#[derive(Clone)]
pub struct Pair(Address, Address);
impl Pair {
    pub fn new(a: Address, b: Address) -> Self {
        if a < b {
            Pair(a, b)
        } else {
            Pair(b, a)
        }
    }

    pub fn client(&self, env: &Env, pair_hash: BytesN<32>,client_address: Address) -> SoroswapPairClient {
        let pair = Pair::new(self.0.clone(), self.1.clone());
        let factory_address = &env.register_contract_wasm(None, FACTORY_WASM);
        let factory = SoroswapFactoryClient::new(&env, &factory_address);
        factory.initialize(&client_address, &pair_hash.clone());
        factory.create_pair(&self.0, &self.1);
        let factory_pair_address = factory.get_pair(&self.0, &self.1);
        SoroswapPairClient::new(&env, &factory_pair_address)
    }

    pub fn salt(&self, e: &Env) -> BytesN<32> {
        let mut salt = Bytes::new(e);

        // Append the bytes of token_a and token_b to the salt
        salt.append(&self.0.clone().to_xdr(e)); // can be simplified to salt.append(&self.clone().to_xdr(e)); but changes the hash
        salt.append(&self.1.clone().to_xdr(e));

        // Hash the salt using SHA256 to generate a new BytesN<32> value
        e.crypto().sha256(&salt)
    }

    pub fn token_a(&self) -> &Address {
        &self.0
    }

    pub fn token_b(&self) -> &Address {
        &self.1
    }

    // Define a function to create a new contract instance
    pub fn create_contract(
        &self,
        env: &Env,                     // Pass in the current environment as an argument
        pair_wasm_hash: BytesN<32>, // Pass in the hash of the token contract's WASM file
    ) -> Address {
        // Return the hash of the newly created contract as a Address value
        let pair_client = SoroswapPairClient::new(env, &env.register_contract(None, SoroswapPair {}));
        env.deployer().with_address(pair_client.address.clone(), self.salt(&env).clone()).deployed_address()
    }
}

// As a general rule we will refer to alice as the deployer of contracts, bob and charlie are secondary 
// token identifiers that could or not sign contract deployements, depending on the functionality being 
// tested. The rule is alice is the main identifier token generated from the cryptographic
// methods of the library, in this case using the random generator provided.

#[test]
fn pair_initialization() {
    let env: Env = Default::default();
    env.mock_all_auths();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let pair = Pair::new(token_0.address, token_1.address);
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let new = pair.client(&env, pair_hash, alice);
    assert_eq!((pair.0.clone(), pair.1.clone()), (new.token_0(), new.token_1()));
}

#[test]
fn pair_init_zero_balance_alice() {
    let env: Env = Default::default();
    env.mock_all_auths();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let pair = Pair::new(token_0.address.clone(), token_1.address.clone());
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let new = pair.client(&env, pair_hash, alice.clone());
    let asserted: (i128, i128) = (token_0.balance(&alice.clone()), token_1.balance(&alice.clone()));
    assert_eq!(asserted, (0,0));
}

#[test]
fn pair_init_zero_balance_bob() {
    let env: Env = Default::default();
    env.mock_all_auths();
    let alice = Address::random(&env);
    let bob = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let pair = Pair::new(token_0.address.clone(), token_1.address.clone());
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let new = pair.client(&env, pair_hash, alice.clone());
    let asserted: (i128, i128) = (token_0.balance(&bob.clone()), token_1.balance(&bob.clone()));
    assert_eq!(asserted, (0,0));
}

#[test]
fn pair_init_some_balance() {
    let env: Env = Default::default();
    env.mock_all_auths();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    token_0.mint(&alice, &1001);
    token_1.mint(&alice, &1002);
    let pair = Pair::new(token_0.address.clone(), token_1.address.clone());
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let new = pair.client(&env, pair_hash, alice.clone());
    let asserted: (i128, i128) = (token_0.balance(&alice.clone()), token_1.balance(&alice.clone()));
    assert_eq!(asserted, (1001,1002));
}

#[test]
fn pair_mock_auth_initialization() {
    let env: Env = Default::default();
    env.mock_all_auths();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let pair = Pair::new(token_0.address.clone(), token_1.address.clone());
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let new = pair.client(&env, pair_hash, alice.clone());
    assert_eq!((pair.0.clone(), pair.1.clone()), (new.token_0(), new.token_1()));
    token_0.mint(&alice, &1001);
    token_1.mint(&alice, &1001);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &new.address,
                    fn_name: "transfer",
                    args: (alice.clone(),&new.address,1_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    // .mock_auths(&[ 
    //     MockAuth {
    //         address: &alice.clone(),
    //         invoke: 
    //             &MockAuthInvoke {
    //                 contract: &new.address,
    //                 fn_name: "deposit",
    //                 args: (alice.clone(),10_000_i128).into_val(&env),
    //                 sub_invokes: &[],
    //             },
    //     }
    // ])
    // .deposit(&alice.clone())
    ;
}

#[test]
fn mint_double_factory_initialization() {
    let env: Env = Default::default();
    env.mock_all_auths();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    token_0.mint(&alice, &1001);
    token_1.mint(&alice, &1001);
    let pair = Pair::new(token_0.address.clone(), token_1.address.clone());
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let new = pair.client(&env, pair_hash, alice);
    let factory_a = SoroswapFactoryClient::new(&env, &new.factory());
    let factory_b = SoroswapFactoryClient::new(&env, &new.factory());
    assert!(factory_a.pair_exists(&token_0.address.clone(), &token_1.address.clone()));
    assert!(factory_b.pair_exists(&token_0.address.clone(), &token_1.address.clone()));
    assert_eq!(factory_a.address, factory_b.address);
}

#[test]
fn pair_not_created() {
    let env: Env = Default::default();
    env.mock_all_auths();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    token_0.mint(&alice, &1001);
    token_1.mint(&alice, &1001);
    let pair = Pair::new(token_0.address.clone(), token_1.address.clone());
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let new = pair.client(&env, pair_hash, alice.clone());
    let factory_a = SoroswapFactoryClient::new(&env, &new.factory());
    let factory_b = SoroswapFactoryClient::new(&env, &new.factory());
    let token_2 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_3 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    assert!(!factory_a.pair_exists(&token_2.address.clone(), &token_3.address.clone()));
    assert!(!factory_b.pair_exists(&token_2.address.clone(), &token_3.address.clone()));
    assert_eq!(factory_a.address, factory_b.address);
}

#[test]
fn two_pairs_initialization_alice() {
    let env: Env = Default::default();
    env.mock_all_auths();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_2 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_3 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    assert_ne!(token_0.address, token_1.address);
    assert_ne!(token_2.address, token_3.address);
    token_0.mint(&alice, &10_000);
    token_1.mint(&alice, &10_000);
    token_2.mint(&alice, &10_000);
    token_3.mint(&alice, &10_000);
    let pair_0_1 = Pair::new(token_0.address.clone(), token_1.address.clone());
    let pair_2_3 = Pair::new(token_2.address.clone(), token_3.address.clone());
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let new_0_1 = pair_0_1.client(&env, pair_hash.clone(), alice.clone());
    let factory_a = SoroswapFactoryClient::new(&env, &new_0_1.factory());
    let new_2_3 = pair_2_3.client(&env, pair_hash.clone(), alice.clone());
    assert_ne!(new_0_1.address, new_2_3.address);
    // let factory_b = SoroswapFactoryClient::new(&env, &new_2_3.factory());
    // assert!(factory_a.pair_exists(&token_2.address.clone(), &token_3.address.clone()));
    // assert!(factory_b.pair_exists(&token_2.address.clone(), &token_3.address.clone()));
    assert!(factory_a.pair_exists(&token_0.address.clone(), &token_1.address.clone()));
    // assert!(!factory_a.pair_exists(&token_2.address.clone(), &token_3.address.clone()));
    // assert_eq!(factory_a.address, factory_b.address);
}