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
    vec,
};
extern crate alloc;
use alloc::boxed::Box;

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

pub enum SoroswapClient<T> {
    TokenClient(T),
    PairClient(T),
    FactoryClient(T)
}

impl<'a> SoroswapClient<TokenClient<'a>> {
    fn new(env: &Env, address: Address) -> SoroswapClient<TokenClient<'a>> {
        let client = TokenClient::new(&env, &env.register_stellar_asset_contract(address));
        Self::TokenClient(client)
    }
}

impl<'a> SoroswapClient<SoroswapPairClient<'a>> {
    fn new(env: &Env, address: Address) -> SoroswapClient<SoroswapPairClient<'a>> {
        Self::PairClient(SoroswapPairClient::new(&env, &address))
    }
}

impl<'a> SoroswapClient<SoroswapFactoryClient<'a>> {
    fn new(env: &Env, address: Address) -> SoroswapClient<SoroswapFactoryClient<'a>> {
        Self::FactoryClient(SoroswapFactoryClient::new(&env, &env.register_stellar_asset_contract(address)))
    }
}

pub trait ClientHelpers<'a> {
    type ClientType;
    // fn new(env: &Env, address: &Address) -> SoroswapClient<'a, Self::ClientType>;
    fn address(&self) -> Address;
    fn mock_auth(self) -> Self::ClientType;
    fn mock_auth_helper(&'a mut self, alice: Address, fn_name: &'a str, args: Vec<Val>);
}

impl<'a> ClientHelpers<'a> for SoroswapClient<TokenClient<'a>> {
    type ClientType = TokenClient<'a>;
    fn address(&self) -> Address {
        let SoroswapClient::TokenClient(client) = self else { panic!("Wrong generic type.") };
        client.address.clone()
    }
    fn mock_auth(self) -> Self::ClientType {
        let SoroswapClient::TokenClient(client) = self else { panic!("Wrong generic type.") };
        client
    }
    fn mock_auth_helper(&'a mut self, alice: Address, fn_name: &'a str, args: Vec<Val>) {
        let sub_invoke: Box<[MockAuthInvoke<'a>; 0]> = Box::<[MockAuthInvoke<'a>; 0]>::new([]);
        let mock_auth_invoke = MockAuthInvoke {
            contract: &self.address(),
            fn_name,
            args: args.clone(),
            sub_invokes: &[],
        };
        let mock_auth = TestAuth::Mock(MockAuth {
            address: &alice,
            invoke: &mock_auth_invoke,
        });
        let TestAuth::Mock(mock_auth) = mock_auth.clone();
        let auth = [mock_auth,];
        let SoroswapClient::TokenClient(client) = self else { panic!("Wrong generic type.") };
        client.mock_auths(&auth);
    }
}

impl<'a> ClientHelpers<'a> for SoroswapClient<SoroswapFactoryClient<'a>> {
    type ClientType = SoroswapFactoryClient<'a>;
    fn address(&self) -> Address {
        let SoroswapClient::FactoryClient(client) = self else { panic!("Wrong generic type.") };
        client.address.clone()
    }
    fn mock_auth(self) -> Self::ClientType {
        let SoroswapClient::FactoryClient(client) = self else { panic!("Wrong generic type.") };
        client
    }
    fn mock_auth_helper(&'a mut self, alice: Address, fn_name: &'a str, args: Vec<Val>) {
        let sub_invoke: Box<[MockAuthInvoke<'a>; 0]> = Box::<[MockAuthInvoke<'a>; 0]>::new([]);
        let mock_auth_invoke = MockAuthInvoke {
            contract: &self.address(),
            fn_name,
            args: args.clone(),
            sub_invokes: &[],
        };
        let mock_auth = TestAuth::Mock(MockAuth {
            address: &alice,
            invoke: &mock_auth_invoke,
        });
        let TestAuth::Mock(mock_auth) = mock_auth.clone();
        let auth = [mock_auth,];
        let SoroswapClient::FactoryClient(client) = self else { panic!("Wrong generic type.") };
        client.mock_auths(&auth);
    }
}

impl<'a> ClientHelpers<'a> for SoroswapClient<SoroswapPairClient<'a>> {
    type ClientType = SoroswapPairClient<'a>;
    fn address(&self) -> Address {
        let SoroswapClient::PairClient(client) = self else { panic!("Wrong generic type.") };
        client.address.clone()
    }
    fn mock_auth(self) -> Self::ClientType {
        let SoroswapClient::PairClient(client) = self else { panic!("Wrong generic type.") };
        client
    }
    fn mock_auth_helper(&'a mut self, alice: Address, fn_name: &'a str, args: Vec<Val>) {
        let sub_invoke: Box<[MockAuthInvoke<'a>; 0]> = Box::<[MockAuthInvoke<'a>; 0]>::new([]);
        let mock_auth_invoke = MockAuthInvoke {
            contract: &self.address(),
            fn_name,
            args: args.clone(),
            sub_invokes: &[],
        };
        let mock_auth = TestAuth::Mock(MockAuth {
            address: &alice,
            invoke: &mock_auth_invoke,
        });
        let TestAuth::Mock(mock_auth) = mock_auth.clone();
        let auth = [mock_auth,];
        let SoroswapClient::PairClient(client) = self else { panic!("Wrong generic type.") };
        client.mock_auths(&auth);
    }
}

enum TestAuth<'a> {
    Mock(MockAuth<'a>)
}

impl<'a> Clone for TestAuth<'a> {
    fn clone(&self) -> TestAuth<'a> {
        let TestAuth::Mock(mock_auth) = self;
        TestAuth::Mock(
            MockAuth {
                address: &mock_auth.address,
                invoke: &mock_auth.invoke,
            }
        )
    }
}

pub struct SoroswapTestApi<'a, T: ClientHelpers<'a>> {
    client: &'a mut T,
    alice: Address,
    mock_auth_invoke: MockAuthInvoke<'a>,
    sub_invoke: Box<[MockAuthInvoke<'a>]>,
    mock_auth: TestAuth<'a>,
    auth_vec: Box<&'a [MockAuth<'a>]>,
}

impl<'a, T: ClientHelpers<'a>> SoroswapTestApi<'a, T> {
    pub fn auth(&'a mut self, alice: Address, contract: &'a Address, fn_name: &'a str, args: Vec<Val>) {
        self.client.mock_auth_helper(alice, fn_name, args);
    }
}

use crate::test::operations::Pair;
#[test]
fn pair_initialization<'a>() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let SoroswapClient::TokenClient(token_0) = SoroswapClient::<TokenClient<'a>>::new(&env, alice.clone()) else { todo!() };
    let SoroswapClient::TokenClient(token_1) = SoroswapClient::<TokenClient<'a>>::new(&env, alice.clone()) else { todo!() };
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let mut soroswap_factory_client = SoroswapClient::<SoroswapFactoryClient<'a>>::new(&env, alice.clone()) else { todo!() };
    soroswap_factory_client.mock_auth_helper(alice.clone(), "initialize", (alice.clone(), pair_hash,).into_val(&env));
    // let SoroswapClient::FactoryClient(factory) = soroswap_factory_client else { todo!() };
    // factory.create_pair(&token_0.address, &token_1.address);
    // let factory_pair_address = factory.get_pair(&token_0.address, &token_1.address);
    // let new = SoroswapPairClient::new(&env, &factory_pair_address);
    // let pair = Pair::new(token_0.address, token_1.address);
    // assert_eq!((pair.0.clone(), pair.1.clone()), (new.token_0(), new.token_1()));
}