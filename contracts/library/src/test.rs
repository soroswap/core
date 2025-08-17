#![cfg(test)]

extern crate std;
use crate::{SoroswapLibrary, SoroswapLibraryClient};
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

mod token {
    soroban_sdk::contractimport!(
        file = "../token/target/wasm32v1-none/release/soroban_token_contract.wasm"
    );
    pub type TokenClient<'a> = Client<'a>;
}

mod pair {
    soroban_sdk::contractimport!(file = "../pair/target/wasm32v1-none/release/soroswap_pair.wasm");
    pub type SoroswapPairClient<'a> = Client<'a>;
}

fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../pair/target/wasm32v1-none/release/soroswap_pair.optimized.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

mod factory {
    soroban_sdk::contractimport!(
        file = "../factory/target/wasm32v1-none/release/soroswap_factory.optimized.wasm"
    );
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}

use factory::SoroswapFactoryClient;
use pair::SoroswapPairClient;
use token::TokenClient;

// Useful functions to create contracts

fn create_token_contract<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    TokenClient::new(
        e,
        &e.register_stellar_asset_contract_v2(admin.clone())
            .address(),
    )
}

fn create_soroswap_factory<'a>(e: &Env, setter: &Address) -> SoroswapFactoryClient<'a> {
    let pair_hash = pair_contract_wasm(&e);
    let factory_address = &e.register(factory::WASM, ());
    let factory = SoroswapFactoryClient::new(e, factory_address);
    factory.initialize(&setter, &pair_hash);
    factory
}

fn create_soroswap_library_contract<'a>(e: &Env) -> SoroswapLibraryClient<'a> {
    SoroswapLibraryClient::new(e, &e.register(SoroswapLibrary, ()))
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
        token_0.mint(&user, &10000000000);
        token_1.mint(&user, &10000000000);

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
            user,
        }
    }
}

mod get;
mod quote;
mod tokens;
