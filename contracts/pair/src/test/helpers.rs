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
    Val,
    Vec,
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

pub enum Clients<'a> {
    TokenClient(&'a TokenClient<'a>),
    PairClient(&'a SoroswapPairClient<'a>),
    FactoryClient(&'a SoroswapFactoryClient<'a>)
}

impl<'a> Clients<'a> {
    pub fn mock_auth_helper(&'a self, alice: &'a Address, contract: &'a Address, fn_name: &'a str, args: Vec<Val>) -> &Self {

        match self {
            Clients::TokenClient(token_client) => {
                Clients::TokenClient(
                    &token_client
                    .mock_auths(&[
                        MockAuth {
                            address: &alice.clone(),
                            invoke: 
                                &MockAuthInvoke {
                                    contract,
                                    fn_name,
                                    args,
                                    sub_invokes: &[],
                                },
                        }
                    ]))
            },
            Clients::PairClient(pair_client) => {
                Clients::PairClient(
                    &pair_client
                    .mock_auths(&[
                        MockAuth {
                            address: &alice.clone(),
                            invoke: 
                                &MockAuthInvoke {
                                    contract,
                                    fn_name,
                                    args,
                                    sub_invokes: &[],
                                },
                        }
                    ]))
            },
            Clients::FactoryClient(factory_client) => {
                Clients::FactoryClient(
                    &factory_client
                    .mock_auths(&[
                        MockAuth {
                            address: &alice.clone(),
                            invoke: 
                                &MockAuthInvoke {
                                    contract,
                                    fn_name,
                                    args,
                                    sub_invokes: &[],
                                },
                        }
                    ]))
            },
        };

        self
    }

}