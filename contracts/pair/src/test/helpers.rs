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
    fn new(env: &'a Env, alice: &Address) -> SoroswapClient<'a, FactoryClient<'a>> {
        // SoroswapClient::FactoryClient(env, FactoryClient::new(&env, &env.register_stellar_asset_contract(alice.clone())) )
        SoroswapClient::FactoryClient(env, FactoryClient::new(&env, &env.register_contract_wasm(None, FACTORY_WASM)) )
    }
    fn from(env: &'a Env, factory_client: FactoryClient<'a>) -> SoroswapClient<'a, FactoryClient<'a>> {
        Self::FactoryClient(env, factory_client)
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
    fn from(env: &'a Env, address: &Address) -> SoroswapClient<'a, TokenClient<'a>> {
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
    // 
    //  Please note that type MockAuth references to data of unknown size.
    // 
    // fn mock_auth<'a>(alice: &'a Address, contract: &'a Address, fn_name: &'a str, args: Vec<Val>) -> MockAuth<'a> {
    //     let args_clone = args.clone();

    //     let sub_invoke: Box<[MockAuthInvoke<'a>; 0]> = Box::<[MockAuthInvoke<'a>; 0]>::new([]); // TODO: implement sub_invoke .
    //     let mock_auth = MockAuth {
    //         address: alice,
    //         invoke: &invoke,
    //     };
    //     mock_auth
    // }
    //

pub trait SoroswapClientTrait<'a, ClientType: 'a>
where Self: Sized
{
    fn from(env: &'a Env, address: &'a Address, mock_auths: &'a [MockAuth]) -> SoroswapClient<'a, ClientType>;
    fn client(&'a self) -> &'a ClientType;
    fn address(&self) -> &Address;
    fn mock_auth_helper(&'a self, env: &'a Env, alice: &'a Address, mock_auths: &'a [MockAuth<'a>; 1]) -> Self;
}

impl<'a> SoroswapClientTrait<'a, TokenClient<'a>> for SoroswapClient<'a, TokenClient<'a>> {
    fn from(env: &'a Env, address: &Address, mock_auths: &'a [MockAuth]) -> SoroswapClient<'a, TokenClient<'a>> {
        let client = TokenClient::new(&env, &env.register_stellar_asset_contract(address.clone()));
        Self::TokenClient(env, client)
    }
    fn address(&self) -> &Address {
        let SoroswapClient::TokenClient(_, client) = self else { SoroswapClientError::WrongBindingType(self).dispatch_error() };
        &client.address
    }
    fn client(&'a self) -> &'a TokenClient<'a> {    
        match self {
            Self::TokenClient(_, client) => { 
                &client
            },
            _ => SoroswapClientError::WrongBindingType(&self).dispatch_error(),
        }
    }
    fn mock_auth_helper(&'a self, env: &'a Env, alice: &'a Address, mock_auths: &'a [MockAuth<'a>; 1]) -> Self {
        let ref client = self.client();
        Self::TokenClient(env, client.mock_auths(mock_auths))
    }
}

impl<'a> SoroswapClientTrait<'a, SoroswapPairClient<'a>> for SoroswapClient<'a, SoroswapPairClient<'a>> {
    fn from(env: &'a Env, address: &Address, mock_auths: &'a [MockAuth]) -> SoroswapClient<'a, SoroswapPairClient<'a>> {
        Self::PairClient(env, SoroswapPairClient::new(&env, &address))
    }
    fn address(&self) -> &Address {
        let SoroswapClient::PairClient(_, client) = self else { SoroswapClientError::WrongBindingType(self).dispatch_error() };
        &client.address
    }
    fn client(&'a self) -> &'a SoroswapPairClient<'a> {
        match self {
            Self::PairClient(_, client) => { 
                &client
             },
            _ => SoroswapClientError::WrongBindingType(&self).dispatch_error(),
        }
    }
    fn mock_auth_helper(&'a self, env: &'a Env, alice: &'a Address, mock_auths: &'a [MockAuth<'a>; 1]) -> Self {
        let ref client = self.client();
        Self::PairClient(env, client.mock_auths(mock_auths))
    }
}

impl<'a> SoroswapClientTrait<'a, FactoryClient<'a>> for SoroswapClient<'a, FactoryClient<'a>> {
    fn from(env: &'a Env, contract_address: &Address, mock_auths: &'a [MockAuth]) -> SoroswapClient<'a, FactoryClient<'a>> {
        // Self::FactoryClient(&env, FactoryClient::new(&env, &env.register_stellar_asset_contract(contract_address.clone())));
        SoroswapClient::<FactoryClient<'a>>::new(env, contract_address)
    }
    fn address(&self) -> &Address {
        let SoroswapClient::FactoryClient(_, client) = self else { SoroswapClientError::WrongBindingType(self).dispatch_error() };
        &client.address
    }
    fn client(&'a self) -> &'a FactoryClient<'a> {
        match self {
            Self::FactoryClient(_, client) => { 
                &client
             },
            _ => SoroswapClientError::WrongBindingType(&self).dispatch_error(),
        }
    }
    fn mock_auth_helper(&'a self, env: &'a Env, alice: &'a Address, mock_auths: &'a [MockAuth<'a>; 1]) -> Self {
        let ref client = self.client();
        Self::FactoryClient(env, client.mock_auths(mock_auths))
    }
}

#[derive(Clone)]
pub struct SoroswapTest<'a, T, U: SoroswapClientTrait<'a, T>>
{
    env: Env,
    client: PhantomData<&'a T>,
    test_client: &'a U, // SoroswapClient<'a, T>,
    alice: Address,
    mock_auths: &'a [MockAuth<'a>; 1]
}

// impl<'a, T> SoroswapTest<'a, T, SoroswapClient<'a, T>>
//  where SoroswapClient<'a, T>: SoroswapClientTrait<'a, T>
// {
//     fn address(&'a self) -> &'a Address {
//         self.test_client.address()
//     }
// }

impl<'a> SoroswapTest<'a, FactoryClient<'a>, SoroswapClient<'a, FactoryClient<'a>>> {
    fn initialize(env: &'a Env, alice: &'a Address, test_client: &'a SoroswapClient<'a, FactoryClient<'a>>, mock_auths: &'a [MockAuth<'a>; 1]) -> Self {
        let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
        let contract_address = test_client.address();
        Self {
            env: env.clone(),
            client:PhantomData,
            test_client,
            alice: alice.clone(),
            mock_auths
        }
    }
    fn address(&'a self) -> &'a Address {
        self.test_client.address()
    }
}

#[test]
fn pair_initialization() {
    let env: Env = Default::default();
    let alice: Address = Address::random(&env);
    let test_client = SoroswapClient::<FactoryClient>::new(&env, &alice);
    let contract_address = test_client.address().clone();
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let invoke = MockAuthInvoke {
        contract: &contract_address,
        fn_name: "initialize",
        args: (alice.clone(), pair_hash.clone(),).into_val(&env),
        sub_invokes: &[],
    };
    let mock_auth = MockAuth {
        address: &alice,
        invoke: &invoke,
    };
    let mock_auths = &[mock_auth];
    let mocked_client = test_client.mock_auth_helper(&env, &alice, mock_auths);
    assert_eq!(test_client.address(), mocked_client.address());
    let factory_api = SoroswapTest::<FactoryClient, SoroswapClient<FactoryClient>>::initialize(&env, &alice, &mocked_client, mock_auths);
    factory_api.test_client.client().initialize(&alice.clone(), &pair_hash.clone());
    let client = factory_api.test_client.client();

    let token_0 = SoroswapClient::<TokenClient>::from(&env, &alice);
    let token_1 = SoroswapClient::<TokenClient>::from(&env, &alice);

    client.create_pair(&token_0.address(), &token_1.address());
    let factory_address = factory_api.address();
}