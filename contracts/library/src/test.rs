#![cfg(test)]

extern crate std;
use soroban_sdk::{Env, BytesN, Address, testutils::Address as _};
use crate::{SoroswapLibrary, SoroswapLibraryClient};

mod token {
    soroban_sdk::contractimport!(file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}

mod pair {
    soroban_sdk::contractimport!(file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair.wasm");
    pub type SoroswapPairClient<'a> = Client<'a>;
}


fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

mod factory {
    soroban_sdk::contractimport!(file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}

use token::TokenClient;
use pair::SoroswapPairClient;
use factory::SoroswapFactoryClient;

// Useful functions to create contracts


fn create_token_contract<'a>(e: &Env, admin: & Address) -> TokenClient<'a> {
    TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
}


fn create_soroswap_factory<'a>(e: & Env, setter: & Address) -> SoroswapFactoryClient<'a> {
    let pair_hash = pair_contract_wasm(&e);  
    let factory_address = &e.register_contract_wasm(None, factory::WASM);
    let factory = SoroswapFactoryClient::new(e, factory_address); 
    factory.initialize(&setter, &pair_hash);
    factory
}

fn create_soroswap_library_contract<'a>(e: &Env) -> SoroswapLibraryClient<'a> {
    SoroswapLibraryClient::new(e, &e.register_contract(None, SoroswapLibrary {}))
}

// Extended test with factory and a pair
struct SoroswapLibraryTest<'a> {
    env: Env,
    contract: SoroswapLibraryClient<'a>,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    factory: SoroswapFactoryClient<'a>,
    pair: SoroswapPairClient<'a>,
    user: Address,
}

impl<'a> SoroswapLibraryTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();
        let contract = create_soroswap_library_contract(&env);

        let admin = Address::generate(&env);
        let user = Address::generate(&env);

        let mut token_0 = create_token_contract(&env, &admin);
        let mut token_1 = create_token_contract(&env, &admin);
        if &token_1.address < &token_0.address {
            std::mem::swap(&mut token_0, &mut token_1);
        }
        token_0.mint(&user, &10000);
        token_1.mint(&user, &10000);

        let factory = create_soroswap_factory(&env, &admin);
        factory.create_pair(&token_0.address, &token_1.address);

        let pair_address = factory.get_pair(&token_0.address, &token_1.address);
        let pair = SoroswapPairClient::new(&env, &pair_address);

        // function addLiquidity(address tokenA, address tokenB, uint amountADesired, uint amountBDesired, uint amountAMin, uint amountBMin, address to,uint deadline)
        //await router.addLiquidity(
        //       token0.address,
        //       token1.address,
        //       bigNumberify(10000),
        //       bigNumberify(10000),
        //       0,
        //       0,
        //       wallet.address,
        //       MaxUint256,
        //       overrides
        //     )

        //pair.deposit(&user, &10000, &0, &10000, &0);
        env.budget().reset_unlimited();
        
        SoroswapLibraryTest {
            env,
            contract,
            token_0,
            token_1,
            factory,
            pair,
            user
        }
    }
}

mod quote;
mod get;
mod tokens;