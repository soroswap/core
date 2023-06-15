#![cfg(test)]
extern crate std;

use crate::{SoroswapPairClient};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}

mod factory {
    soroban_sdk::contractimport!(file = "../factory/target/wasm32-unknown-unknown/release/soroswap_factory_contract.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}


use token::TokenClient;
use factory::SoroswapFactoryClient;
//use factory::SoroswapFactory;

use crate::test::factory::WASM;

use soroban_sdk::{  testutils::Events,
                    Vec,
                    RawVal,
                    vec,
                    testutils::Address as _,
                    Address, 
                    BytesN, 
                    Env,
                    IntoVal, Symbol};

fn create_token_contract<'a>(e: &'a Env, admin: &'a Address) -> TokenClient<'a> {
    TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
}

fn create_factory_contract<'a>(
    e: &'a Env,
    setter: &'a Address,
    pair_wasm_hash: &'a BytesN<32>
) -> SoroswapFactoryClient<'a> {
    let factory_address = &e.register_contract_wasm(None, factory::WASM);
    let factory = SoroswapFactoryClient::new(e, factory_address);
    factory.initialize(&setter, pair_wasm_hash);
    factory.set_fee_to(&setter);
    factory
}

fn pair_token_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "./target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
    );
    e.install_contract_wasm(WASM)
}


fn create_pair_contract<'a>(
    e: &'a Env,
    factory: &'a Address,
    token_a: &'a Address,
    token_b: &'a Address,
) -> SoroswapPairClient<'a> {
    let liqpool = SoroswapPairClient::new(e, &e.register_contract(None, crate::SoroswapPair {}));
    liqpool.initialize_pair(factory, token_a, token_b);
    liqpool
}

fn last_event_vec(e: &Env) -> Vec<(Address, Vec<RawVal>, RawVal)>{
    vec![&e, e.events().all().last().unwrap().unwrap()]
}

#[test]
fn test() {
    const PAIR: Symbol = Symbol::short("PAIR");

    let e: Env = Default::default();

    e.mock_all_auths();

    let mut admin0 = Address::random(&e);
    let mut admin1 = Address::random(&e);

    let mut token0 = create_token_contract(&e, &admin0);
    let mut token1 = create_token_contract(&e, &admin1);
    if &token1.address.contract_id() < &token0.address.contract_id() {
        std::mem::swap(&mut token0, &mut token1);
        std::mem::swap(&mut admin0.clone(), &mut admin1.clone());
    }

    let pair_token_wasm_binding = pair_token_wasm(&e);  
    let factory = create_factory_contract(&e, &admin0, &pair_token_wasm_binding);


    let user = Address::random(&e);
    let liqpool = create_pair_contract(
        &e,
        &factory.address,
        &token0.address,
        &token1.address,
    );


    token0.mint(&user, &10000000000);
    assert_eq!(token0.balance(&user), 10000000000);

    token1.mint(&user, &10000000000);
    assert_eq!(token1.balance(&user), 10000000000);


    liqpool.deposit(&user, &1000000000, &1000000000, &1000000000, &1000000000);

    // Testing the "deposit" event
    // topics = (PAIR, Symbol::new(e, "deposit"), sender);
    let topics = (PAIR, Symbol::new(&e, "deposit"), user.clone());
    let data = (1000000000_i128, 1000000000_i128);
    assert_eq!(last_event_vec(&e),
                vec![&e,    (liqpool.address.clone(),
                            topics.into_val(&e),
                            data.into_val(&e))]);
   
    assert_eq!(
        e.auths(),
        [(
            user.clone(),
            liqpool.address.clone(),
            Symbol::short("deposit"),
            (&user, 1000000000_i128, 1000000000_i128, 1000000000_i128, 1000000000_i128).into_val(&e)
        ),
        (
            user.clone(),
            token0.address.clone(),
            Symbol::short("transfer"),
            (&user, &liqpool.address, 1000000000_i128).into_val(&e)//from, to, amount
        ),
        (
            user.clone(),
            token1.address.clone(),
            Symbol::short("transfer"),
            (&user, &liqpool.address, 1000000000_i128).into_val(&e)
        )]
    );

    assert_eq!(liqpool.my_balance(&user), 999999000);
    // We lock forever the minimum_liquidity (1000) in the LP contract itself
    assert_eq!(liqpool.my_balance(&liqpool.address), 1000);
    assert_eq!(token0.balance(&user), 9000000000);
    assert_eq!(token0.balance(&liqpool.address), 1000000000);
    assert_eq!(token1.balance(&user), 9000000000);
    assert_eq!(token1.balance(&liqpool.address), 1000000000);

    // Testing SWAP
    liqpool.swap(&user, &false, &490000000, &1000000000);

    // // Testing the "deposit" event
    // // topics: (PAIR, Symbol::new(e, "swap"), sender);
    // let topics = (PAIR, Symbol::new(&e, "swap"), user.clone());
    // // data: (amount_0_in, amount_1_in, amount_0_out,amount_1_out,  to)
    // let data = (970000000_i128, 0_i128, 0_i128, 490000000_i128, user.clone());
    // assert_eq!(last_event_vec(&e),
    //             vec![&e,    (liqpool.address.clone(),
    //                         topics.into_val(&e),
    //                         data.into_val(&e))]);

    // // Test to.require_auth();
    // assert_eq!(
    //     e.auths(),
    //     [(
    //         user.clone(),
    //         liqpool.address.clone(),
    //         Symbol::short("swap"),
    //         (&user, false, 490000000_i128, 1000000000_i128).into_val(&e)
    //     ),
    //     (
    //         user.clone(),
    //         token0.address.clone(),
    //         Symbol::short("transfer"),
    //         (&user, &liqpool.address, 970000000_i128).into_val(&e)//from, to, amount
    //     )]
    // );

    assert_eq!(token0.balance(&user), 8036324660    );
    assert_eq!(token0.balance(&liqpool.address), 1963675340);
    assert_eq!(token1.balance(&user), 9490000000);
    assert_eq!(token1.balance(&liqpool.address), 510000000);


   // assert_eq!(liqpool.my_balance(&liqpool.address), 0);
}
  // // Testing WITHDRAW
    // liqpool.withdraw(&user, &1000000000, &1970000000, &510000000);

    // // Testing the "withdraw" event
    // // topics: (PAIR, Symbol::new(e, "withdraw"), sender);
    // let topics = (PAIR, Symbol::new(&e, "withdraw"), user.clone());
    // // data: (amount_0, amount_1, to)
    // let data = (1970000000_i128, 510000000_i128, user.clone());
    // assert_eq!(last_event_vec(&e),
    //             vec![&e,    (liqpool.address.clone(),
    //                         topics.into_val(&e),
    //                         data.into_val(&e))]);

    // // // Testing the "withdraw" event
    // // let topics = (Symbol::new(&e, "withdraw"), user.clone(), 1970000000_i128, 510000000_i128);
    // // assert_eq!(
    // //     last_event_vec(&e),
    // //     vec![&e, (  liqpool.address.clone(),
    // //                 topics.into_val(&e),
    // //                 user.clone().into_val(&e)),
    // //         ]
    // // );

    // // Testing to.require_auth();
    // assert_eq!(
    //     e.auths(),
    //     [(
    //         user.clone(),
    //         liqpool.address.clone(),
    //         Symbol::short("withdraw"),
    //         (&user, 1000000000_i128, 1970000000_i128, 510000000_i128).into_val(&e)
    //     )]
    // );

    // assert_eq!(token0.balance(&user), 10000000000);
    // assert_eq!(token1.balance(&user), 10000000000);
    // assert_eq!(liqpool.my_balance(&user), 0);
    // assert_eq!(token0.balance(&liqpool.address), 0);
    // assert_eq!(token1.balance(&liqpool.address), 0);
   