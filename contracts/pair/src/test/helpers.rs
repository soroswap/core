extern crate alloc;
use alloc::boxed::Box;
use core::{
    result,
    fmt,
    marker::PhantomData
};
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

mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
mod pair {
    soroban_sdk::contractimport!(file = "./target/wasm32-unknown-unknown/release/soroswap_pair.wasm");
}
mod factory {
    soroban_sdk::contractimport!(file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory.wasm");
    pub type FactoryClient<'a> = Client<'a>; 
}

use soroban_sdk::testutils::Address as _;
use crate::{
    SoroswapPair, 
    SoroswapPairClient,
};
use token::TokenClient;
use factory::{
    FactoryClient,
    WASM as FACTORY_WASM,
};

#[derive(Copy, Clone)]
pub enum SoroswapClient<'a, T> {
    TokenClient(&'a Env, T),
    PairClient(&'a Env, T),
    FactoryClient(&'a Env, T),
    None
}

impl<'a> SoroswapClient<'a, FactoryClient<'a>> {
    // initialize
    fn new(env: &'a Env, address: &Address) -> SoroswapClient<'a, FactoryClient<'a>> {
        SoroswapClient::FactoryClient(env, FactoryClient::new(&env, &env.register_stellar_asset_contract(address.clone())) )
    }
    fn env_client(&'a mut self) -> (&'a Env, &'a mut FactoryClient) {
        match self {
            Self::FactoryClient(env, client) => { 
                (env,client)
             },
            _ => SoroswapClientError::WrongBindingType(self).dispatch_error(),
        }
    }
    fn mock_auth(&'a mut self, alice: &Address, contract_address: &Address, fn_name: &str, args: Vec<Val>) {
        // let args_clone = args.clone();
        let invoke = MockAuthInvoke {
            contract: contract_address,
            fn_name,
            args,
            sub_invokes: &[],
        };
        let sub_invoke: Box<[MockAuthInvoke<'a>; 0]> = Box::<[MockAuthInvoke<'a>; 0]>::new([]); // TODO: implement sub_invoke .
        let mock_auth = MockAuth {
            address: alice,
            invoke: &invoke,
        };
        let (env,client) = self.env_client();
        let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
        client
        .mock_auths(&[mock_auth]);
    }
    fn initialize(&'a mut self) {
        let (env,client) = self.env_client();
    }
}

impl<'a> SoroswapClient<'a, TokenClient<'a>> {
    // initialize
    fn new(env: &'a Env, address: &Address) -> SoroswapClient<'a, TokenClient<'a>> {
        Self::TokenClient(&env, TokenClient::new(&env, &env.register_stellar_asset_contract(address.clone())))
    }
    fn env_client(&'a mut self) -> (&'a Env, &'a mut TokenClient) {
        match self {
            Self::TokenClient(env, client) => { 
                (env,client)
             },
            _ => SoroswapClientError::WrongBindingType(self).dispatch_error(),
        }
    }
}

impl<T> fmt::Display for SoroswapClient<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TokenClient(_, client_type) => "TokenClient",
                Self::PairClient(_, client_type) => "PairClient",
                Self::FactoryClient(_, client_type) => "FactoryClient",
                Self::None => "None"
            }
        )
    }
}

pub enum SoroswapClientError<'a, T> {
    WrongBindingType(&'a SoroswapClient<'a, T>),
    InvokeUndefined(&'a SoroswapClient<'a, T>),
    CodeUnreachable,

}

impl<T> fmt::Display for SoroswapClientError<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::WrongBindingType(client_type) => "Wrong binding for type {client_type}",
                Self::InvokeUndefined(client_type) => "Undefined invoke parameters for type {client_type}",
                Self::CodeUnreachable => "This code is intended to be unreachable."
            }
        )
    }
}

trait SoroswapError {
    fn dispatch_error(self) -> ! ;
}

impl<'a, T> SoroswapError for SoroswapClientError<'a, T>
{
    fn dispatch_error(self) -> ! {
        panic!("{}", self)
    }
}

pub trait SoroswapClientTrait<'a, ClientType>
where Self: Sized
{
    fn new_from(env: &'a Env, address: &Address) -> SoroswapClient<'a, ClientType>;
    fn client(&'a mut self) -> &'a mut ClientType;
    fn address(&self) -> &Address;
    fn mock_auths(&'a mut self, mock_auths: &[MockAuth]);
    fn mock_auth_helper(&'a mut self, alice: &'a Address, contract: &Address, fn_name: &str, args: Vec<Val>) {
        let args_clone = args.clone();
        let invoke = MockAuthInvoke {
            contract: &contract,
            fn_name,
            args: args_clone,
            sub_invokes: &[],
        };
        let sub_invoke: Box<[MockAuthInvoke<'a>; 0]> = Box::<[MockAuthInvoke<'a>; 0]>::new([]); // TODO: implement sub_invoke .
        let mock_auth = MockAuth {
            address: alice,
            invoke: &invoke,
        };
        self.mock_auths(&[mock_auth]);
    }
}

impl<'a> SoroswapClientTrait<'a, TokenClient<'a>> for SoroswapClient<'a, TokenClient<'a>> {
    fn new_from(env: &'a Env, address: &Address) -> SoroswapClient<'a, TokenClient<'a>> {
        let client = TokenClient::new(&env, &env.register_stellar_asset_contract(address.clone()));
        Self::TokenClient(env, client)
    }
    fn address(&self) -> &Address {
        let SoroswapClient::TokenClient(_, client) = self else { SoroswapClientError::WrongBindingType(self).dispatch_error() };
        &client.address
    }
    fn client(&mut self) -> &mut TokenClient<'a> {    
        match self {
            Self::TokenClient(_, client) => { 
                client
            },
            _ => SoroswapClientError::WrongBindingType(self).dispatch_error(),
        }
    }
    fn mock_auths(&'a mut self, mock_auths: &[MockAuth]) {
        self.client().mock_auths(mock_auths);
    }
}

impl<'a> SoroswapClientTrait<'a, SoroswapPairClient<'a>> for SoroswapClient<'a, SoroswapPairClient<'a>> {
    fn new_from(env: &'a Env, address: &Address) -> SoroswapClient<'a, SoroswapPairClient<'a>> {
        Self::PairClient(env, SoroswapPairClient::new(&env, &address))
    }
    fn address(&self) -> &Address {
        let SoroswapClient::PairClient(_, client) = self else { SoroswapClientError::WrongBindingType(self).dispatch_error() };
        &client.address
    }
    fn client(&mut self) -> &mut SoroswapPairClient<'a> {
        match self {
            Self::PairClient(_, client) => { 
                client
             },
            _ => SoroswapClientError::WrongBindingType(self).dispatch_error(),
        }
    }
    fn mock_auths(&'a mut self, mock_auths: &[MockAuth]) {
        self.client().mock_auths(mock_auths);
    }
}

impl<'a> SoroswapClientTrait<'a, FactoryClient<'a>> for SoroswapClient<'a, FactoryClient<'a>> {
    fn new_from(env: &'a Env, contract_address: &Address) -> SoroswapClient<'a, FactoryClient<'a>> {
        Self::FactoryClient(&env, FactoryClient::new(&env, &env.register_stellar_asset_contract(contract_address.clone())));
        SoroswapClient::<FactoryClient<'a>>::new(env, contract_address)
    }
    fn address(&self) -> &Address {
        let SoroswapClient::FactoryClient(_, client) = self else { SoroswapClientError::WrongBindingType(self).dispatch_error() };
        &client.address
    }
    fn client(&'a mut self) -> &'a mut FactoryClient {
        match self {
            Self::FactoryClient(_, client) => { 
                client
             },
            _ => SoroswapClientError::WrongBindingType(self).dispatch_error(),
        }
    }
    fn mock_auths(&'a mut self, mock_auths: &[MockAuth]) {
        self.client().mock_auths(mock_auths);
    }
}

#[derive(Clone)]
pub struct SoroswapTest<'a, T, U: SoroswapClientTrait<'a, T>>
{
    env: Env,
    client: PhantomData<&'a T>,
    test_client: U, // SoroswapClient<'a, T>,
    alice: Address,
}

impl<'a, T> SoroswapTest<'a, T, SoroswapClient<'a, T>>
where SoroswapClient<'a, T>: SoroswapClientTrait<'a, T>
{
    pub fn new(alice: &Address, env: &'a Env) -> Self {
        let test_client = SoroswapClient::<'a, T>::new_from(&env, &alice);
        Self {
            env: env.clone(),
            client:PhantomData,
            test_client,
            alice: alice.clone(),
        }
    }
    fn address(&'a self) -> &'a Address {
        self.test_client.address()
    }
    fn invoke(&'a mut self, alice: &'a Address, contract: &'a Address, fn_name: &str, args: Vec<Val>) {
        self
        .test_client
        .mock_auth_helper(alice, contract, fn_name, args)
        ;
    }
}

impl<'a> SoroswapTest<'a, FactoryClient<'a>, SoroswapClient<'a, FactoryClient<'a>>> {
    fn initialize(&'a mut self, alice: &'a Address, contract: &'a Address, fn_name: &str, args: Vec<Val>) {
        self.test_client.mock_auth_helper(alice, contract, fn_name, args);
    }
}

#[test]git 
fn pair_initialization() {
    let env: Env = Default::default();
    let alice: Address = Address::random(&env);
    let token_api_0 = SoroswapTest::<TokenClient, SoroswapClient<TokenClient>>::new(&alice, &env);
    let token_api_1 = SoroswapTest::<TokenClient, SoroswapClient<TokenClient>>::new(&alice, &env);
    let mut factory_api = SoroswapTest::<FactoryClient, SoroswapClient<FactoryClient>>::new(&alice, &env);
    let factory_address = factory_api.address().clone();
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    factory_api.initialize(&alice, &factory_address, "initialize", (alice.clone(), pair_hash.clone(),).into_val(&env));
}