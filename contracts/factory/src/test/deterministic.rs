use soroban_sdk::{
    Env,
    Address,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation}
};

mod pair {
    soroban_sdk::contractimport!(file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm");
    pub type SoroswapPairClient<'a> = Client<'a>;
}
mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
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
    contract: SoroswapFactoryClient<'a>,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    pair: SoroswapPairClient<'a>
}

impl<'a> SoroswapFactoryTest<'a> {
    fn new() -> Self {

        let env: Env = Default::default();
        let admin = Address::random(&env);
        let user = Address::random(&env);
        let factory_address = &env.register_contract_wasm(None, token::WASM);
        let contract: SoroswapFactoryClient<'a> = SoroswapFactoryClient::new(&env, factory_address);
        let token_0: TokenClient<'a> = TokenClient::new(&env, &env.register_stellar_asset_contract(admin.clone()));
        let token_1: TokenClient<'a> = TokenClient::new(&env, &env.register_stellar_asset_contract(admin.clone()));

        token_0.mint(&user, &10000);
        token_1.mint(&user, &10000);
        let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
        let factory = SoroswapFactoryClient::new(&env, factory_address);
        let pair_address = factory.get_pair(&token_0.address, &token_1.address);
        let pair = SoroswapPairClient::new(&env, &pair_address);
        factory.initialize(&admin, &pair_hash);

        SoroswapFactoryTest {
            env,
            contract,
            token_0,
            token_1,
            pair
        }
    }
}

mod deterministic {
    use soroban_sdk::{
        Env,
        Address,
        BytesN,
        Bytes,
        xdr::ToXdr,
        testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation}
    };
    use crate::test::deterministic::SoroswapFactoryTest;

    #[test]
    pub fn token_client_ne() {
        let factory_test = SoroswapFactoryTest::new();
        assert_eq!(factory_test.token_0.address, factory_test.token_1.address);
    }

    // #[test]
    pub fn create_factory_contract() {
        let factory_test = SoroswapFactoryTest::new();
    }

    #[test]
    pub fn compare_address() {
        // Create two tokens in order to create a pair using the factory
        // let mut token_0 = create_token_contract(&e, &admin);
        // let mut token_1 = create_token_contract(&e, &admin);
        // let pair_expected_address = guess_contract_address( &e,
        //     &factory.address, 
        //     &token_1.address, 
        //     &token_0.address);
        // let pair_address = factory.get_pair(&token_0.address, &token_1.address);
        // assert_eq!(&pair_expected_address, &pair_address);
    }


    pub fn guess_contract_address(
        e: &Env,
        factory: &Address,
        token_a: &Address,
        token_b: &Address,
    ) -> BytesN<32> {
        let token_0;
        let token_1;
        if token_a < token_b {
            token_0 = token_a;
            token_1 = token_b;
        }
        else {
            token_0 = token_b;
            token_1 = token_a;
        }
        let mut salt = Bytes::new(e);
        salt.append(&factory.to_xdr(e));
        salt.append(&token_0.to_xdr(e));
        salt.append(&token_1.to_xdr(e));
        let salt_hash = e.crypto().sha256(&salt);
        // let contract_address = Address::try_from(&salt_hash.as_ref()[12..]);
        // contract_address.unwrap_or_else(|_| BytesN::zero())
        salt_hash
    }
}