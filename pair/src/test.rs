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

use soroban_sdk::{  testutils::{Events, Ledger, LedgerInfo},
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

const PAIR: Symbol = Symbol::short("PAIR");

#[test]
fn test() {
    let e: Env = Default::default();
    e.mock_all_auths();
    
    let user = Address::random(&e);
    let mut admin_0 = Address::random(&e);
    let mut admin1 = Address::random(&e);
    let mut token_0 = create_token_contract(&e, &admin_0);
    let mut token_1 = create_token_contract(&e, &admin1);
    if &token_1.address.contract_id() < &token_0.address.contract_id() {
        std::mem::swap(&mut token_0, &mut token_1);
        std::mem::swap(&mut admin_0.clone(), &mut admin1.clone());
    }

    let pair_token_wasm_binding = pair_token_wasm(&e);  
    let factory = create_factory_contract(&e, &admin_0, &pair_token_wasm_binding);

    // Test factory initial values:
    assert_eq!(factory.fee_to(), admin_0);
    assert_eq!(factory.fee_to_setter(), admin_0);
    assert_eq!(factory.fees_enabled(), false);

    let liqpool = create_pair_contract(
        &e,
        &factory.address,
        &token_0.address,
        &token_1.address,
    );

    // Test liqpool initial values:
    assert_eq!(liqpool.token_0(), token_0.address);
    assert_eq!(liqpool.token_1(), token_1.address);
    assert_eq!(liqpool.factory(), factory.address);
    assert_eq!(liqpool.get_reserves(), (0,0,0));
    assert_eq!(liqpool.k_last(), 0);
    assert_eq!(liqpool.price_0_cumulative_last(), 0);
    assert_eq!(liqpool.price_1_cumulative_last(), 0);

    let factor = 10000000; // we will use 7 decimals

    token_0.mint(&user, &(1000 * factor));
    assert_eq!(token_0.balance(&user), 1000 * factor);
    token_1.mint(&user, &&(1000 * factor));
    assert_eq!(token_1.balance(&user), 1000 * factor);

    // Testing the deposit function:
    let init_time = 12345;
    e.ledger().set(LedgerInfo {
        timestamp: init_time,
        protocol_version: 1,
        sequence_number: 10,
        network_id: Default::default(),
        base_reserve: 10,
    });

    liqpool.deposit(&user, &(100 * factor), &(100 * factor), &(100 * factor), &(100 * factor));

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
            token_0.address.clone(),
            Symbol::short("transfer"),
            (&user, &liqpool.address, 1000000000_i128).into_val(&e)//from, to, amount
        ),
        (
            user.clone(),
            token_1.address.clone(),
            Symbol::short("transfer"),
            (&user, &liqpool.address, 1000000000_i128).into_val(&e)
        )]
    );

    assert_eq!(liqpool.my_balance(&user), 999999000);
    // We lock forever the minimum_liquidity (1000) in the LP contract itself
    assert_eq!(liqpool.my_balance(&liqpool.address), 1000);
    assert_eq!(token_0.balance(&user), 900 * factor);
    assert_eq!(token_0.balance(&liqpool.address), 100 * factor);
    assert_eq!(token_1.balance(&user), 900 * factor);
    assert_eq!(token_1.balance(&liqpool.address), 100 * factor);

    assert_eq!(liqpool.k_last(), 0);
    // Test fee_to has not yet received any fee
    assert_eq!(liqpool.my_balance(&admin_0), 0);
    // The first deposit, we don't have yet any price accumulated, this will be on the second deposit
    assert_eq!(liqpool.price_0_cumulative_last(), 0);
    assert_eq!(liqpool.price_1_cumulative_last(), 0);
    assert_eq!(liqpool.get_reserves(), (100 * factor, 100 * factor,init_time));


    // Now, let's deposit again in order to have Cumulative Prices.
    let passed_time = 54321;
    e.ledger().set(LedgerInfo {
        timestamp: passed_time + init_time,
        protocol_version: 1,
        sequence_number: 10,
        network_id: Default::default(),
        base_reserve: 10,
    });

    liqpool.deposit(&user, &(100 * factor), &(100 * factor), &(100 * factor), &(100 * factor));
    assert_eq!(liqpool.my_balance(&user), 100 * factor + 999999000);
    assert_eq!(liqpool.my_balance(&liqpool.address), 1000);
    assert_eq!(token_0.balance(&user), 800 * factor);
    assert_eq!(token_0.balance(&liqpool.address), 200 * factor);
    assert_eq!(token_1.balance(&user), 800 * factor);
    assert_eq!(token_1.balance(&liqpool.address), 200 * factor);
    assert_eq!(liqpool.get_reserves(), (200 * factor, 200 * factor,passed_time + init_time));
    let uq64x64_price_0_cumulative_last = liqpool.price_0_cumulative_last();
    let uq64x64_price_1_cumulative_last = liqpool.price_1_cumulative_last();
    let decimals_u128: u128 = 10000000;
    let passed_time_u128: u128 = passed_time.into();
    let expected_price_cumulative_last_decoded: u128 = 1*passed_time_u128*decimals_u128;

    assert_eq!(liqpool.decode_uq64x64_with_7_decimals(&uq64x64_price_0_cumulative_last), expected_price_cumulative_last_decoded);
    assert_eq!(liqpool.decode_uq64x64_with_7_decimals(&uq64x64_price_1_cumulative_last), expected_price_cumulative_last_decoded);


    // TODO: Test event::sync, do it with last_n_events function

    // Testing SWAP
    liqpool.swap(&user, &false, &490000000, &(100 * factor));

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
    //         token_0.address.clone(),
    //         Symbol::short("transfer"),
    //         (&user, &liqpool.address, 970000000_i128).into_val(&e)//from, to, amount
    //     )]
    // );

    // assert_eq!(token_0.balance(&user), 8036324660    );
    // assert_eq!(token_0.balance(&liqpool.address), 1963675340);
    // assert_eq!(token_1.balance(&user), 9490000000);
    // assert_eq!(token_1.balance(&liqpool.address), 510000000);


   // assert_eq!(liqpool.my_balance(&liqpool.address), 0);
}
  // // Testing WITHDRAW
    // liqpool.withdraw(&user, &(100 * factor), &1970000000, &510000000);

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

    // assert_eq!(token_0.balance(&user), &(1000 * factor));
    // assert_eq!(token_1.balance(&user), &(1000 * factor));
    // assert_eq!(liqpool.my_balance(&user), 0);
    // assert_eq!(token_0.balance(&liqpool.address), 0);
    // assert_eq!(token_1.balance(&liqpool.address), 0);
   