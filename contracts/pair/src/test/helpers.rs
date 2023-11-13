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

pub enum SoroswapClient<'a, T> {
    TokenClient(&'a Env, T),
    PairClient(&'a Env, T),
    FactoryClient(&'a Env, T),
    None
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
    WrongBindingType(SoroswapClient<'a, T>),
    InvokeUndefined(SoroswapClient<'a, T>),
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
{
    // type ClientType;
    // fn new(env: &Env, address: Address) -> SoroswapClient<T>;
    fn new(env: &'a Env, address: &'a Address) -> SoroswapClient<'a, ClientType>;
    fn copy(&'a self) -> SoroswapClient<ClientType>;
    fn client(&'a mut self) -> &'a mut ClientType;
    fn address(&'a self) -> &Address;
    fn mock_auth_helper(&'a mut self, alice: &'a Address, fn_name: &'a str, args: Vec<Val>) -> &'a Self {
        let address_copy = self.address();
        let args_clone = args.clone();
        let invoke = MockAuthInvoke {
            contract: &address_copy,
            fn_name,
            args: args_clone,
            sub_invokes: &[],
        };
        let sub_invoke: Box<[MockAuthInvoke<'a>; 0]> = Box::<[MockAuthInvoke<'a>; 0]>::new([]); // TODO: implement sub_invoke .
        let mock_auth = MockAuth {
            address: alice,
            invoke: &invoke,
        };
        self
    }
    // fn mock_auth_invoke(&'a self, fn_name: &'a str, args: Vec<Val>) -> MockAuthInvoke
    // {
    //     let address_copy = self.address();
    //     let args_clone = args.clone();
    //     MockAuthInvoke {
    //         contract: &address_copy,
    //         fn_name,
    //         args: args_clone,
    //         sub_invokes: &[],
    //     }
    // }
}

// impl SoroswapClientTrait<T> for SoroswapClient<T> {

// }

impl<'a> SoroswapClientTrait<'a, TokenClient<'a>> for SoroswapClient<'a, TokenClient<'a>> {
    fn new(env: &'a Env, address: &'a Address) -> SoroswapClient<'a, TokenClient<'a>>{
        let client = TokenClient::new(&env, &env.register_stellar_asset_contract(address.clone()));
        Self::TokenClient(env, client)
    }
    fn copy(&'a self) -> SoroswapClient<TokenClient> {
        let SoroswapClient::TokenClient(env, client) = self else { 
            SoroswapClientError::WrongBindingType(self.copy()).dispatch_error()
        };
        Self::new(&env.clone(), &client.address)
    }
    fn address(&'a self) -> &Address {
        let SoroswapClient::TokenClient(_, client) = self else { SoroswapClientError::WrongBindingType(self.copy()).dispatch_error() };
        &client.address
    }
    fn client(&'a mut self) -> &'a mut TokenClient<'a> {    
        match self {
            Self::TokenClient(_, client) => { 
                client
            },
            _ => SoroswapClientError::WrongBindingType(self.copy()).dispatch_error(),
        }
    }
}

impl<'a> SoroswapClientTrait<'a, SoroswapPairClient<'a>> for SoroswapClient<'a, SoroswapPairClient<'a>> {
    // type ClientType = SoroswapPairClient<'a>;
    fn new(env: &'a Env, address: &'a Address) -> SoroswapClient<'a, SoroswapPairClient<'a>> {
        Self::PairClient(env, SoroswapPairClient::new(&env, &address))
    }
    fn copy(&'a self) -> SoroswapClient<SoroswapPairClient> {
        let SoroswapClient::PairClient(env, client) = self else { SoroswapClientError::WrongBindingType(self.copy()).dispatch_error() };
        Self::new(&env.clone(), &client.address)
    }
    fn address(&'a self) -> &Address {
        let SoroswapClient::PairClient(_, client) = self else { SoroswapClientError::WrongBindingType(self.copy()).dispatch_error() };
        &client.address
    }
    fn client(&'a mut self) -> &'a mut SoroswapPairClient {
        match self {
            Self::PairClient(_, client) => { 
                client
             },
            _ => SoroswapClientError::WrongBindingType(self.copy()).dispatch_error(),
        }
    }
}

impl<'a> SoroswapClientTrait<'a, SoroswapFactoryClient<'a>> for SoroswapClient<'a, SoroswapFactoryClient<'a>> {
    // type ClientType = SoroswapFactoryClient<'a>;
    fn new(env: &'a Env, address: &'a Address) -> SoroswapClient<'a, SoroswapFactoryClient<'a>> {
        Self::FactoryClient(&env, SoroswapFactoryClient::new(&env, &env.register_stellar_asset_contract(address.clone())))
    }
    fn copy(&'a self) -> SoroswapClient<SoroswapFactoryClient> {
        let SoroswapClient::FactoryClient(env, client) = self else { SoroswapClientError::WrongBindingType(self.copy()).dispatch_error() };
        Self::new(&env.clone(), &client.address)
    }
    fn address(&'a self) -> &Address {
        let self_copy = self.copy();
        let SoroswapClient::FactoryClient(_, client) = self else { SoroswapClientError::WrongBindingType(self_copy).dispatch_error() };
        &client.address
    }
    fn client(&'a mut self) -> &'a mut SoroswapFactoryClient {
        match self {
            Self::FactoryClient(_, client) => { 
                client
             },
            _ => SoroswapClientError::WrongBindingType(self.copy()).dispatch_error(),
        }
    }
}

pub struct SoroswapTestApi<'a, T, U: SoroswapClientTrait<'a, T>>
{
    env: Env,
    client: PhantomData<&'a T>,
    test_client: U, // SoroswapClient<'a, T>,
    alice: &'a Address,
    
}

impl<'a, T> SoroswapTestApi<'a, T, SoroswapClient<'a, T>>
where SoroswapClient<'a, T>: SoroswapClientTrait<'a, T>
{
    pub fn new(alice: &'a Address, env: &'a Env) -> Self {
        let test_client = SoroswapClient::<'a, T>::new(&env, &alice) else { todo!() };
        Self {
            env: env.clone(),
            client:PhantomData,
            test_client,
            alice: alice,
        }
    }
    fn invoke(&'a mut self, alice: &'a Address, fn_name: &'a str, args: Vec<Val>) {
        self.test_client.mock_auth_helper(alice, fn_name, args);
    }
    fn auth(self) where Self: Sized {
        // let mock_auth = self.client.mock_auth_helper(alice, &self.mock_auth_invoke.as_ref().expect("Wrong Type."));
    }
}

// impl<'a> SoroswapTestApi<'a, TokenClient<'a>> 
// {
//     pub fn new(alice: &'a Address, env: &Env) -> Self {
//         let client = SoroswapClient::<TokenClient<'a>>::new(&env, alice.clone()) else { todo!() };
//         Self {
//             env: env.clone(),
//             client,
//             alice: alice,
//         }
//     }
// }

// impl<'a> SoroswapTestApi<'a, SoroswapFactoryClient<'a>> 
// {
//     pub fn new(alice: &'a Address, env: &Env) -> Self {
//         let client = SoroswapClient::<SoroswapFactoryClient<'a>>::new(&env, alice.clone()) else { todo!() };
//         Self {
//             env: env.clone(),
//             client,
//             alice: alice,
//         }
//     }
// }

#[test]
fn pair_initialization() {
    let env: Env = Default::default();
    let alice: Address = Address::random(&env);
    let token_api_0 = SoroswapTestApi::<TokenClient, SoroswapClient<TokenClient>>::new(&alice, &env);
    let token_api_1 = SoroswapTestApi::<TokenClient, SoroswapClient<TokenClient>>::new(&alice, &env);
    let mut factory_api = SoroswapTestApi::<SoroswapFactoryClient, SoroswapClient<SoroswapFactoryClient>>::new(&alice, &env);
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    factory_api.invoke(&alice, "initialize", (alice.clone(), pair_hash.clone(),).into_val(&env));
    // factory_api.auth();
    // soroswap_factory_client.mock_auth_helper(&alice_clone, &invoke);
    // let client = soroswap_factory_client.client();
    // let SoroswapClient::FactoryClient(factory) = soroswap_factory_client else { todo!() };
    // factory.create_pair(&token_0.address, &token_1.address);
    // let factory_pair_address = factory.get_pair(&token_0.address, &token_1.address);
    // let new = SoroswapPairClient::new(&env, &factory_pair_address);
    // let pair = Pair::new(token_0.address, token_1.address);
    // assert_eq!((pair.0.clone(), pair.1.clone()), (new.token_0(), new.token_1()));
}