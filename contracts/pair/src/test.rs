#![cfg(test)]
extern crate std;
use soroban_sdk::{
    // symbol_short,
    // testutils::{Events},
    // Vec,
    // Val,
    // vec,
    testutils::{Address as _},
    Address, 
    BytesN, 
    Env,
    String,
    // Symbol
};
//use crate::{SoroswapPairClient};

// TOKEN CONTRACT
mod token {
    soroban_sdk::contractimport!(file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
use token::TokenClient;
fn create_token_contract<'a>(e: &Env) -> TokenClient<'a> {
    let token_address = &e.register_contract_wasm(None, token::WASM);
    let token = TokenClient::new(e, token_address);
    token
}

// FACTORY CONTRACT
mod factory {
    soroban_sdk::contractimport!(file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}
use factory::SoroswapFactoryClient;

fn create_factory_contract<'a>(e: & Env, setter: & Address,pair_wasm_hash: & BytesN<32>) -> SoroswapFactoryClient<'a> {
    let factory_address = &e.register_contract_wasm(None, factory::WASM);
    let factory = SoroswapFactoryClient::new(e, factory_address);
    factory.initialize(&setter, pair_wasm_hash);
    factory
}

// PAIR CONTRACT
// WASM
fn pair_token_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "./target/wasm32-unknown-unknown/release/soroswap_pair.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

pub mod pair {
    soroban_sdk::contractimport!(file = "./target/wasm32-unknown-unknown/release/soroswap_pair.wasm");
    pub type SoroswapPairClient<'a> = Client<'a>;
}
use pair::SoroswapPairClient;


fn create_pair_contract<'a>(
    e: & Env
) -> SoroswapPairClient<'a> {
    let pair_address = &e.register_contract_wasm(None, pair::WASM);
    let pair_client = SoroswapPairClient::new(e, pair_address);
    pair_client
}

// THE TEST
pub struct SoroswapPairTest<'a> {
    env: Env,
    admin: Address,
    user: Address,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    factory: SoroswapFactoryClient<'a>,
    contract: SoroswapPairClient<'a>,
}

impl<'a> SoroswapPairTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();
        let user = Address::generate(&env);
        let admin = Address::generate(&env);
        let mut token_0 = create_token_contract(&env);
        let mut token_1 = create_token_contract(&env);
        if &token_1.address < &token_0.address {
            std::mem::swap(&mut token_0, &mut token_1);
        }
        
        let name_0 = String::from_str(&env, "Token 0");
        let symbol_0 = String::from_str(&env, "TOK0");
        let name_1 = String::from_str(&env, "Token 1");
        let symbol_1 = String::from_str(&env, "ABCDEFGHIJ");
        let decimals = 7;

        token_0.initialize(&admin, &decimals, &name_0, &symbol_0);
        token_1.initialize(&admin, &decimals, &name_1, &symbol_1);

        token_0.mint(&user, &123_000_000_000_000_000_000);
        token_1.mint(&user, &321_000_000_000_000_000_000);

        let pair_token_wasm_binding = pair_token_wasm(&env);  
        let factory = create_factory_contract(&env, &admin, &pair_token_wasm_binding);

        let contract = create_pair_contract(
            &env,
        );

        // TODO: Get rid of this hack?
        env.budget().reset_unlimited();
    

        SoroswapPairTest {
            env,
            admin,
            user,
            token_0,
            token_1,
            factory,
            contract,
        }
    }
}
           

// Tests written by esteblock
mod initialize;
mod deposit;
mod swap;
mod withdraw;
mod fee;
mod skim;
mod sync;
mod events;
// mod decode; // wont be used for now

// Test forked by stellar/soroban-examples
mod soroswap_pair_token;

// Uncompleted tests written by labormedia
// now in unused_files folder
// mod operations_helpers;
// mod operations;
// mod helpers;

