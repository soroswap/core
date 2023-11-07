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
        Ledger,
    },
};
use num_integer::Roots;

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


// A simple Pair for ordering the token's addresses and biding the salt.
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
}

// As a general rule we will refer to alice as the deployer of contracts, bob and charlie are secondary 
// token identifiers that could or not sign contract deployements, depending on the functionality being 
// tested. The rule is alice is the main identifier token generated from the cryptographic
// methods of the library, in this case using the random generator provided.

#[test]
fn pair_initialization() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let factory_address = &env.register_contract_wasm(None, FACTORY_WASM);
    let factory = SoroswapFactoryClient::new(&env, &factory_address);
    factory
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &factory.address,
                    fn_name: "initialize",
                    args: (alice.clone(), pair_hash.clone(),).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .initialize(&alice.clone(), &pair_hash.clone());
    factory.create_pair(&token_0.address, &token_1.address);
    let factory_pair_address = factory.get_pair(&token_0.address, &token_1.address);
    let new = SoroswapPairClient::new(&env, &factory_pair_address);
    let pair = Pair::new(token_0.address, token_1.address);
    assert_eq!((pair.0.clone(), pair.1.clone()), (new.token_0(), new.token_1()));
}

#[test]
#[should_panic]
fn token_mint_not_auth() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let bob = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let amount: i128 = 10_000;
    token_0
    .mint(&bob.clone(), &amount);
    let bob_balance = token_0.balance(&bob.clone());
    assert_eq!(bob_balance, amount);
}

#[test]
fn token_mint_mock_auths() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let bob = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let amount: i128 = 10_000;
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address,
                    fn_name: "mint",
                    args: (bob.clone(), amount,).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&bob.clone(), &amount);
    
    let bob_balance = token_0.balance(&bob.clone());

    assert_eq!(bob_balance, amount);
}

#[test]
fn pair_init_zero_balance_alice() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let factory_address = &env.register_contract_wasm(None, FACTORY_WASM);
    let factory = SoroswapFactoryClient::new(&env, &factory_address);
    factory
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &factory.address,
                    fn_name: "initialize",
                    args: (alice.clone(), pair_hash.clone(),).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .initialize(&alice.clone(), &pair_hash.clone());
    factory.create_pair(&token_0.address, &token_1.address);
    let factory_pair_address = factory.get_pair(&token_0.address, &token_1.address);
    let new = SoroswapPairClient::new(&env, &factory_pair_address);
    let asserted: (i128, i128) = (token_0.balance(&alice.clone()), token_1.balance(&alice.clone()));
    assert_eq!(asserted, (0,0));
}

#[test]
fn token_init_zero_balance_bob() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let bob = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let asserted: (i128, i128) = (token_0.balance(&bob.clone()), token_1.balance(&bob.clone()));
    assert_eq!(asserted, (0,0));
}

#[test]
fn token_init_some_balance_alice() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &1001);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),1_002_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &1002);
    let asserted: (i128, i128) = (token_0.balance(&alice.clone()), token_1.balance(&alice.clone()));
    assert_eq!(asserted, (1001,1002));
}

#[test]
fn token_init_some_balance_bob() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let bob = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (bob.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&bob, &1001);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (bob.clone(),1_002_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&bob, &1002);
    let asserted: (i128, i128) = (token_0.balance(&bob.clone()), token_1.balance(&bob.clone()));
    assert_eq!(asserted, (1001,1002));
}

#[test]
fn pair_mock_auth_initialization() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let factory_address = &env.register_contract_wasm(None, FACTORY_WASM);
    let factory = SoroswapFactoryClient::new(&env, &factory_address);
    factory
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &factory.address,
                    fn_name: "initialize",
                    args: (alice.clone(), pair_hash.clone(),).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .initialize(&alice.clone(), &pair_hash.clone());
    factory.create_pair(&token_0.address, &token_1.address);
    let factory_pair_address = factory.get_pair(&token_0.address, &token_1.address);
    let pair = SoroswapPairClient::new(&env, &factory_pair_address);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),2_002_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &2002);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(), 1_002_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &1002);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "transfer",
                    args: (alice.clone(),&pair.address.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&alice.clone(), &pair.address.clone(), &1001);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "transfer",
                    args: (alice.clone(),&pair.address.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&alice.clone(), &pair.address.clone(), &1001);

    let x = token_0.balance(&alice.clone());
    assert_eq!(x, 1001);

    let y = token_1.balance(&alice.clone());
    assert_eq!(y, 1);

    let l = pair.deposit(&alice.clone());
    assert_eq!(1001_i128.checked_mul(1001_i128).unwrap().sqrt().checked_sub(1000_i128).unwrap(), l);

    let b = pair.my_balance(&alice.clone());
    assert!(b == 1);

}

#[test]
fn pair_mock_auth_withdraw() {
    let env: Env = Default::default();
    env.budget().reset_unlimited();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let factory_address = &env.register_contract_wasm(None, FACTORY_WASM);
    let factory = SoroswapFactoryClient::new(&env, &factory_address);
    factory
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &factory.address,
                    fn_name: "initialize",
                    args: (alice.clone(), pair_hash.clone(),).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .initialize(&alice.clone(), &pair_hash.clone());
    factory.create_pair(&token_0.address, &token_1.address);
    let factory_pair_address = factory.get_pair(&token_0.address, &token_1.address);
    let pair = SoroswapPairClient::new(&env, &factory_pair_address);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),2_002_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &2002);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(), 1_002_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &1002);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "transfer",
                    args: (alice.clone(),&pair.address.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&alice.clone(), &pair.address.clone(), &1001);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "transfer",
                    args: (alice.clone(),&pair.address.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&alice.clone(), &pair.address.clone(), &1001);

    let x = token_0.balance(&alice.clone());
    assert_eq!(x, 1001);

    let y = token_1.balance(&alice.clone());
    assert_eq!(y, 1);
    // assert_eq!(y, 1002);

    let liquidity = pair.deposit(&alice.clone());
    assert_eq!(liquidity, 1001_i128.checked_mul(1001_i128).unwrap().sqrt().checked_sub(1000).unwrap());

    let b = pair.my_balance(&alice.clone());
    assert!(b == 1);

    let pair_as_token = TokenClient::new(&env, &pair.address);

    pair_as_token
    .mock_auths(&[
            MockAuth {
                address: &alice.clone(),
                invoke: 
                    &MockAuthInvoke {
                        contract: &pair.address.clone(),
                        fn_name: "transfer",
                        args: (alice.clone(), pair.address.clone(),liquidity,).into_val(&env),
                        sub_invokes: &[],
                    },
            }
        ])
    .transfer(&alice.clone(), &pair.address.clone(), &liquidity);

    pair
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &pair.address.clone(),
                    fn_name: "withdraw",
                    args: (alice.clone(),).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .withdraw(&alice.clone());

    assert_eq!(pair.my_balance(&alice), 0);
    assert_eq!(token_0.balance(&alice), 1002);
    assert_eq!(token_1.balance(&alice), 2);
}

#[test]
fn mint_double_factory_initialization() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let factory_address = &env.register_contract_wasm(None, FACTORY_WASM);
    let factory = SoroswapFactoryClient::new(&env, &factory_address);
    factory
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &factory.address,
                    fn_name: "initialize",
                    args: (alice.clone(), pair_hash.clone(),).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .initialize(&alice.clone(), &pair_hash.clone());
    factory.create_pair(&token_0.address, &token_1.address);
    let factory_pair_address = factory.get_pair(&token_0.address, &token_1.address);
    let new = SoroswapPairClient::new(&env, &factory_pair_address);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice.clone(), &1001);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &1001);
    let factory_a = SoroswapFactoryClient::new(&env, &new.factory());
    let factory_b = SoroswapFactoryClient::new(&env, &new.factory());
    assert!(factory_a.pair_exists(&token_0.address.clone(), &token_1.address.clone()));
    assert!(factory_b.pair_exists(&token_0.address.clone(), &token_1.address.clone()));
    assert_eq!(factory_a.address, factory_b.address);
}

#[test]
fn factory_is_unique_and_pair_not_created() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let factory_address = &env.register_contract_wasm(None, FACTORY_WASM);
    let factory = SoroswapFactoryClient::new(&env, &factory_address);
    factory
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &factory.address,
                    fn_name: "initialize",
                    args: (alice.clone(), pair_hash.clone(),).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .initialize(&alice.clone(), &pair_hash.clone());
    factory.create_pair(&token_0.address, &token_1.address);
    let factory_pair_address = factory.get_pair(&token_0.address, &token_1.address);
    let new = SoroswapPairClient::new(&env, &factory_pair_address);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &1001);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &1001);
    let factory_a = SoroswapFactoryClient::new(&env, &new.factory());
    let factory_b = SoroswapFactoryClient::new(&env, &new.factory());
    let token_2 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_3 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    assert!(factory.pair_exists(&token_0.address.clone(), &token_1.address.clone()));
    assert!(factory_a.pair_exists(&token_0.address.clone(), &token_1.address.clone()));
    assert!(factory_b.pair_exists(&token_0.address.clone(), &token_1.address.clone()));
    assert!(!factory_a.pair_exists(&token_2.address.clone(), &token_3.address.clone()));
    assert!(!factory_b.pair_exists(&token_2.address.clone(), &token_3.address.clone()));
    assert_eq!(factory_a.address, factory_b.address);
}

#[test]
fn two_pairs_initialization_alice() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_2 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_3 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    assert_ne!(token_0.address, token_1.address);
    assert_ne!(token_2.address, token_3.address);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000);
    token_2
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_2.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000);
    token_3
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_3.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000);
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let factory_address = &env.register_contract_wasm(None, FACTORY_WASM);
    let factory = SoroswapFactoryClient::new(&env, &factory_address);
    factory
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &factory.address,
                    fn_name: "initialize",
                    args: (alice.clone(), pair_hash.clone(),).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .initialize(&alice.clone(), &pair_hash.clone());
    
    let factory_pair_address_0_1 = factory.create_pair(&token_0.address.clone(), &token_1.address.clone());
    let factory_pair_address_2_3 = factory.create_pair(&token_2.address, &token_3.address);

    assert!(factory.pair_exists(&token_0.address.clone(), &token_1.address.clone()));
    assert!(factory.pair_exists(&token_2.address.clone(), &token_3.address.clone()));

    assert_ne!(factory_pair_address_0_1, factory_pair_address_2_3);

}

#[test]
fn two_pairs_alice_bob_deposit() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let bob = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_2 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_3 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    assert_ne!(token_0.address, token_1.address);
    assert_ne!(token_2.address, token_3.address);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (bob.clone(),10_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&bob, &10_000);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (bob.clone(),10_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&bob, &10_000);
    token_2
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_2.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000);
    token_3
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_3.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000);
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let factory_address = &env.register_contract_wasm(None, FACTORY_WASM);
    let factory = SoroswapFactoryClient::new(&env, &factory_address);
    factory
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &factory.address,
                    fn_name: "initialize",
                    args: (alice.clone(), pair_hash.clone(),).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .initialize(&alice.clone(), &pair_hash.clone());
    
    let factory_pair_address_0_1 = factory.create_pair(&token_0.address.clone(), &token_1.address.clone());
    let factory_pair_address_2_3 = factory.create_pair(&token_2.address, &token_3.address);

    assert!(factory.pair_exists(&token_0.address.clone(), &token_1.address.clone()));
    assert!(factory.pair_exists(&token_2.address.clone(), &token_3.address.clone()));

    assert_ne!(factory_pair_address_0_1, factory_pair_address_2_3);

    let pair_0_1 = SoroswapPairClient::new(&env, &factory_pair_address_0_1);
    let pair_2_3 = SoroswapPairClient::new(&env, &factory_pair_address_2_3);

    assert_ne!(pair_0_1.address, pair_2_3.address);

    token_0
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1.address.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1.address.clone(), &1001);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1.address.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1.address.clone(), &1001);

    let l = pair_0_1.deposit(&bob.clone());

    assert_eq!(l, 1001_i128.checked_mul(1001_i128).unwrap().sqrt() - 1000_i128);
    
}

#[test]
fn two_pairs_swap_bob_mock_all() {
    let env: Env = Default::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| {
        li.timestamp = 100;
    });
    env.budget().reset_unlimited();
    let alice = Address::random(&env);
    let bob = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_2 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_3 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    assert_ne!(token_0.address, token_1.address);
    assert_ne!(token_2.address, token_3.address);
    token_0.mint(&alice, &50_000_000);
    token_1.mint(&alice, &50_000_000);
    token_0.mint(&bob, &50_000_000);
    token_1.mint(&bob, &50_000_000);
    token_2.mint(&alice, &50_000_000);
    token_3.mint(&alice, &50_000_000);
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let factory_address = &env.register_contract_wasm(None, FACTORY_WASM);
    let factory = SoroswapFactoryClient::new(&env, &factory_address);
    factory.initialize(&alice.clone(), &pair_hash.clone());
    
    let factory_pair_address_0_1 = factory.create_pair(&token_0.address.clone(), &token_1.address.clone());
    let factory_pair_address_2_3 = factory.create_pair(&token_2.address, &token_3.address);

    assert!(factory.pair_exists(&token_0.address.clone(), &token_1.address.clone()));
    assert!(factory.pair_exists(&token_2.address.clone(), &token_3.address.clone()));

    assert_ne!(factory_pair_address_0_1, factory_pair_address_2_3);

    // There is another form for initializing pair given the factory:
    // ```
    // let pair_0_1 = SoroswapPairClient::new(&env, &env.register_contract(None, crate::SoroswapPair {}));
    // pair_0_1.initialize_pair(&factory.address.clone(), &token_0.address.clone(), &token_1.address.clone());
    // ```
    let pair_0_1 = SoroswapPairClient::new(&env, &factory_pair_address_0_1);

    let pair_2_3 = SoroswapPairClient::new(&env, &factory_pair_address_2_3);


    assert_ne!(pair_0_1.address, pair_2_3.address);

    let pair = Pair::new(token_0.address.clone(), token_1.address.clone());

    token_0.transfer(&bob.clone(), &pair_0_1.address.clone(), &10_000_000);
    token_1.transfer(&bob.clone(), &pair_0_1.address.clone(), &10_000_000);

    let l = pair_0_1.deposit(&bob.clone());

    assert_eq!(l, 10000000_i128.checked_mul(10000000_i128).unwrap().sqrt() - 1000_i128);

    token_0.transfer(&bob.clone(), &pair_0_1.address.clone(), &10_000_000);

    pair_0_1.swap(&0, &1_000_000, &bob.clone());

}

#[test]
fn bigger_quantity() {
    let env: Env = Default::default();
    env.budget().reset_unlimited();
    env.ledger().with_mut(|li| {
        li.timestamp = 100;
    });
    let alice = Address::random(&env);
    let bob = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    assert_ne!(token_0.address, token_1.address);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000_000);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000_000);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (bob.clone(),10_000_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&bob, &10_000_000);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (bob.clone(),10_000_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&bob, &10_000_000);
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let factory_address = &env.register_contract_wasm(None, FACTORY_WASM);
    let factory = SoroswapFactoryClient::new(&env, &factory_address);
    factory
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &factory.address,
                    fn_name: "initialize",
                    args: (alice.clone(), pair_hash.clone(),).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .initialize(&alice.clone(), &pair_hash.clone());
    
    let factory_pair_address_0_1 = factory.create_pair(&token_0.address.clone(), &token_1.address.clone());

    assert!(factory.pair_exists(&token_0.address.clone(), &token_1.address.clone()));

    let pair_0_1 = SoroswapPairClient::new(&env, &factory_pair_address_0_1);

    token_0
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1.address.clone(),1_001_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1.address.clone(), &1001000);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1.address.clone(),1_001_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1.address.clone(), &1001000);

    let l = pair_0_1.deposit(&bob.clone());

    assert_eq!(l, 1001000_i128.checked_mul(1001000_i128).unwrap().sqrt() - 1000_i128);

    token_0
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1.address.clone(),1_001_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1.address.clone(), &1001000);

    pair_0_1.swap(&0, &10_000, &bob.clone());

}

#[test]
fn bigger_pair_quantity_bob() {
    let env: Env = Default::default();
    env.budget().reset_unlimited();
    env.ledger().with_mut(|li| {
        li.timestamp = 100;
    });
    let alice = Address::random(&env);
    let bob = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    assert_ne!(token_0.address, token_1.address);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000_000);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000_000);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (bob.clone(),10_001_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&bob, &10_001_000);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (bob.clone(),10_001_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&bob, &10_001_000);

    assert_eq!(token_0.balance(&bob.clone()), 10_001_000);
    assert_eq!(token_1.balance(&bob.clone()), 10_001_000);

    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let factory_address = &env.register_contract_wasm(None, FACTORY_WASM);
    let factory = SoroswapFactoryClient::new(&env, &factory_address);
    factory
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &factory.address,
                    fn_name: "initialize",
                    args: (alice.clone(), pair_hash.clone(),).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .initialize(&alice.clone(), &pair_hash.clone());
    
    let factory_pair_address_0_1 = factory.create_pair(&token_0.address.clone(), &token_1.address.clone());

    assert!(factory.pair_exists(&token_0.address.clone(), &token_1.address.clone()));

    let pair_0_1_as_token = TokenClient::new(&env, &factory_pair_address_0_1);

    assert_eq!(token_0.balance(&bob.clone()), 10_001_000);
    assert_eq!(token_1.balance(&bob.clone()), 10_001_000);

    let pair_0_1_as_pair = SoroswapPairClient::new(&env, &factory_pair_address_0_1);

    assert_eq!(pair_0_1_as_pair.address, pair_0_1_as_token.address);

    assert_eq!(token_0.balance(&bob), 10001000);
    assert_eq!(token_1.balance(&bob), 10001000);

    token_0
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1_as_token.address.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1_as_token.address.clone(), &1001);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1_as_token.address.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1_as_token.address.clone(), &1001);

    let bob_liquidity = pair_0_1_as_pair.deposit(&bob.clone());

    assert_eq!(bob_liquidity, 1001_i128.checked_mul(1001_i128).unwrap().sqrt() - 1000_i128);

    assert_eq!(token_0.balance(&bob), 10001000 - 1001);
    assert_eq!(token_1.balance(&bob), 10001000 - 1001);

    assert_eq!(bob_liquidity, 1);

    pair_0_1_as_token
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &pair_0_1_as_token.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1_as_pair.address.clone(),bob_liquidity).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1_as_pair.address.clone(), &bob_liquidity);

    let bob_balance = pair_0_1_as_pair.my_balance(&bob.clone());

    assert_eq!(bob_balance, 0);

    assert_eq!(token_0.balance(&bob), 10001000 - 1001);
    assert_eq!(token_1.balance(&bob), 10001000 - 1001);

    token_0
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1_as_token.address.clone(),10_000_i128/*_000*/).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1_as_token.address.clone(), &10000/*000*/); // Use all balance
    token_1
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1_as_token.address.clone(),10_000_i128/*_000*/).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1_as_token.address.clone(), &10000/*000*/); // Use all balance

    let bob_liquidity_0_1 = pair_0_1_as_pair.deposit(&bob);
    let bob_balance_0_1 = pair_0_1_as_pair.my_balance(&bob.clone());

    // At the second deposit, the liquidty formula is different.
    assert_eq!(bob_balance_0_1, 10_000);
    assert_eq!(bob_liquidity_0_1, 10_000_i128.checked_mul(10_000_i128).expect("Mul").sqrt()/*.checked_sub(1000_i128)*/);
    assert_eq!(token_0.balance(&bob), 10_001_000 - 10_000 - 1001);
    assert_eq!(token_1.balance(&bob), 10_001_000 - 10_000 - 1001);

    pair_0_1_as_token
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &pair_0_1_as_token.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1_as_pair.address.clone(),bob_liquidity_0_1).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1_as_pair.address.clone(), &bob_liquidity_0_1);

    // Bob's pair balance is 0 again.
    assert_eq!(pair_0_1_as_pair.my_balance(&bob.clone()), 0);

    pair_0_1_as_pair.withdraw(&bob);

    let bob_balance = (token_0.balance(&bob), token_1.balance(&bob));
    assert_eq!(bob_balance.0, 10_001_000 - 1000);
    assert_eq!(bob_balance.1, 10_001_000 - 1000);

    let first_token_first_pair = TokenClient::new(&env, &pair_0_1_as_pair.token_0());
    let second_token_first_pair = TokenClient::new(&env, &pair_0_1_as_pair.token_1());

    assert!(first_token_first_pair.address == token_0.address || first_token_first_pair.address == token_1.address);

    first_token_first_pair
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &first_token_first_pair.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1_as_token.address.clone(),(10_001_000 - 1000) as i128/*_000*/).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1_as_token.address.clone(), &(10_001_000 - 1000)/*000*/); // Use all balance
    // token_1
    // .mock_auths(&[
    //     MockAuth {
    //         address: &bob.clone(),
    //         invoke: 
    //             &MockAuthInvoke {
    //                 contract: &token_1.address.clone(),
    //                 fn_name: "transfer",
    //                 args: (bob.clone(),&pair_0_1_as_token.address.clone(),(10_001_000 - 1000) as i128/*_000*/).into_val(&env),
    //                 sub_invokes: &[],
    //             },
    //     }
    // ])
    // .transfer(&bob.clone(), &pair_0_1_as_token.address.clone(), &(10_001_000 - 1000)/*000*/);

    // let bob_liquidity = pair_0_1_as_pair.deposit(&bob);
    // assert_eq!(bob_liquidity, 10_001_000_i128.checked_sub(1000).expect("Sub"));

    assert_eq!(pair_0_1_as_token.balance(&bob), 0);
    assert_eq!(first_token_first_pair.balance(&bob), 0);
    assert_eq!(second_token_first_pair.balance(&bob), 10_000_000);
    assert_eq!(pair_0_1_as_pair.get_reserves(), (1_000,1_000,100));

    // assert_eq!((10_001_000_i128 - 1000).checked_mul(997).expect("Mul").checked_div(1000).expect("Div"), 0);
    // pair_0_1_as_pair.swap(&0, &(10_001_000_i128 - 1000).checked_mul(997).expect("Mul").checked_div(1000).expect("Div"), &bob.clone());

    pair_0_1_as_pair.swap(&999, &0, &bob.clone());
    assert_eq!(pair_0_1_as_token.balance(&bob), 0);
    assert_eq!(first_token_first_pair.balance(&bob), 999);
    assert_eq!(second_token_first_pair.balance(&bob), 10_000_000);
    assert_eq!(pair_0_1_as_pair.get_reserves(), (10_000_001,1_000,100));
}

#[test]
fn two_pairs_swap_bob() {
    let env: Env = Default::default();
    env.budget().reset_unlimited();
    env.ledger().with_mut(|li| {
        li.timestamp = 100;
    });
    let alice = Address::random(&env);
    let bob = Address::random(&env);
    let token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_2 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let token_3 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    assert_ne!(token_0.address, token_1.address);
    assert_ne!(token_2.address, token_3.address);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000_000);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (alice.clone(),10_000_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&alice, &10_000_000);
    token_0
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "mint",
                    args: (bob.clone(),10_000_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&bob, &10_000_000);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "mint",
                    args: (bob.clone(),10_000_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&bob, &10_000_000);
    token_2
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_2.address.clone(),
                    fn_name: "mint",
                    args: (bob.clone(),10_000_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&bob, &10_000_000);
    token_3
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_3.address.clone(),
                    fn_name: "mint",
                    args: (bob.clone(),10_000_000_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .mint(&bob, &10_000_000);

    assert_eq!(token_0.balance(&bob.clone()), 10_000_000);
    assert_eq!(token_1.balance(&bob.clone()), 10_000_000);

    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let factory_address = &env.register_contract_wasm(None, FACTORY_WASM);
    let factory = SoroswapFactoryClient::new(&env, &factory_address);
    factory
    .mock_auths(&[
        MockAuth {
            address: &alice.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &factory.address,
                    fn_name: "initialize",
                    args: (alice.clone(), pair_hash.clone(),).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .initialize(&alice.clone(), &pair_hash.clone());
    
    let factory_pair_address_0_1 = factory.create_pair(&token_0.address.clone(), &token_1.address.clone());
    let factory_pair_address_2_3 = factory.create_pair(&token_2.address, &token_3.address);

    assert!(factory.pair_exists(&token_0.address.clone(), &token_1.address.clone()));
    assert!(factory.pair_exists(&token_2.address.clone(), &token_3.address.clone()));

    assert_ne!(factory_pair_address_0_1, factory_pair_address_2_3);

    let pair_0_1_as_token = TokenClient::new(&env, &factory_pair_address_0_1);
    let pair_2_3_as_token = TokenClient::new(&env, &factory_pair_address_2_3);

    assert_ne!(pair_0_1_as_token.address, pair_2_3_as_token.address);

 

    assert_eq!(token_0.balance(&bob.clone()), 10_000_000);
    assert_eq!(token_1.balance(&bob.clone()), 10_000_000);

    let pair_0_1_as_pair = SoroswapPairClient::new(&env, &factory_pair_address_0_1);
    let pair_2_3_as_pair = SoroswapPairClient::new(&env, &factory_pair_address_2_3);

    assert_eq!(pair_0_1_as_pair.address, pair_0_1_as_token.address);

    assert_eq!(token_0.balance(&bob), 10000000);
    assert_eq!(token_1.balance(&bob), 10000000);

    token_0
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_0.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1_as_token.address.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1_as_token.address.clone(), &1001);
    token_1
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_1.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1_as_token.address.clone(),1_001_i128).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1_as_token.address.clone(), &1001);

    let bob_liquidity = pair_0_1_as_pair.deposit(&bob.clone());

    assert_eq!(token_0.balance(&bob), 10000000 - 1001);
    assert_eq!(token_1.balance(&bob), 10000000 - 1001);

    assert_eq!(bob_liquidity, 1);

    pair_0_1_as_token
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &pair_0_1_as_token.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_0_1_as_pair.address.clone(),bob_liquidity).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_0_1_as_pair.address.clone(), &bob_liquidity);

    let bob_balance = pair_0_1_as_pair.my_balance(&bob.clone());

    assert_eq!(bob_balance, 0);

    assert_eq!(token_0.balance(&bob), 10000000 - 1001);
    assert_eq!(token_1.balance(&bob), 10000000 - 1001);


    // Second token pair contracts.


    assert_eq!(token_2.balance(&bob), 10000000);
    assert_eq!(token_3.balance(&bob), 10000000);

    token_2
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_2.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_2_3_as_token.address.clone(),10_000_i128/*_000*/).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_2_3_as_token.address.clone(), &10000/*000*/); // Use all balance
    token_3
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &token_3.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_2_3_as_token.address.clone(),10_000_i128/*_000*/).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_2_3_as_token.address.clone(), &10000/*000*/); // Use all balance

    assert_eq!(bob_liquidity, 1001_i128.checked_mul(1001_i128).unwrap().sqrt() - 1000_i128);

    let bob_liquidity_2_3 = pair_2_3_as_pair.deposit(&bob);
    let bob_balance_2_3 = pair_2_3_as_pair.my_balance(&bob.clone());
    assert_eq!(bob_liquidity_2_3, 10_000_i128.checked_mul(10_000_i128).unwrap().sqrt().checked_sub(1000_i128).unwrap());

    assert_eq!(pair_2_3_as_pair.get_reserves(), (10_000, 10_000, 100));

    let second_token_second_pair = TokenClient::new(&env, &pair_2_3_as_pair.token_1());

    second_token_second_pair
    .mock_auths(&[
        MockAuth {
            address: &bob.clone(),
            invoke: 
                &MockAuthInvoke {
                    contract: &second_token_second_pair.address.clone(),
                    fn_name: "transfer",
                    args: (bob.clone(),&pair_2_3_as_token.address.clone(),10_000_i128/*_000*/).into_val(&env),
                    sub_invokes: &[],
                },
        }
    ])
    .transfer(&bob.clone(), &pair_2_3_as_token.address.clone(), &10000/*000*/); // Use all balance

    pair_2_3_as_pair.swap(&0, &9970, &bob.clone());

    let (reserve_2_3_token_0, reserve_2_3_token_1, timestamp) = pair_2_3_as_pair.get_reserves();

    assert_eq!(reserve_2_3_token_0, 10_000);
    // Reserve token_1 example:
    // total_swap_amount = received + fee = 10_000
    // received = total_swap_amount * 997 / 1000 = 9970
    // => fee = total_swap_amount - total_swap_amount * 997 / 1000 = 30
    // 
    // reserves = initial_reserves + new_deposits - new_withdraws + new_fees
    // :. reserves = 10_000 + 0 - 0 + (total_swap_amount - total_swap_amount * 997 / 1000)
    assert_eq!(reserve_2_3_token_1, 10_000_i128.checked_add(10_000_i128.checked_sub(10_000_i128.checked_mul(997).expect("Mul").checked_div(1000).expect("Div")).expect("Sub")).expect("Add"));
    assert_eq!(timestamp, 100);

    
}

