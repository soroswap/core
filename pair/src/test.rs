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


use crate::uq64x64::{fraction, encode, decode_with_7_decimals as decode_uq64x64_with_7_decimals};

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
    let mut init_time = 12345;
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
    let mut passed_time = 54321;
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
    let mut uq64x64_price_0_cumulative_last = liqpool.price_0_cumulative_last();
    let mut uq64x64_price_1_cumulative_last = liqpool.price_1_cumulative_last();
    let decimals_u128: u128 = 10000000;
    let mut passed_time_u128: u128 = passed_time.into();
    let mut expected_price_cumulative_last_decoded: u128 = 1*passed_time_u128*decimals_u128;

    assert_eq!(decode_uq64x64_with_7_decimals(uq64x64_price_0_cumulative_last), expected_price_cumulative_last_decoded);
    assert_eq!(decode_uq64x64_with_7_decimals(uq64x64_price_1_cumulative_last), expected_price_cumulative_last_decoded);


    // TODO: Test event::sync, do it with last_n_events function

    // Testing SWAP
    init_time = passed_time + init_time;
    passed_time = 9876;

    e.ledger().set(LedgerInfo {
        timestamp: init_time + passed_time,
        protocol_version: 1,
        sequence_number: 10,
        network_id: Default::default(),
        base_reserve: 10,
    });

    liqpool.swap(&user, &false, &(49 * factor), &(66 * factor));
    
    let y_out = 49*factor;
    let x_in =(1000*200*factor*49*factor)/((200-49)*factor*997)+1;
    let new_balance_user = (800*factor)-x_in;

    // Testing the "deposit" event
    let topics = (PAIR, Symbol::new(&e, "swap"), user.clone());
    // data: (amount_0_in, amount_1_in, amount_0_out,amount_1_out,  to)
    let data = (x_in, 0_i128, 0_i128, y_out, user.clone());
    assert_eq!(last_event_vec(&e),
                vec![&e,    (liqpool.address.clone(),
                            topics.into_val(&e),
                            data.into_val(&e))]);

    // Test to.require_auth();
    assert_eq!(
        e.auths(),
        [(
            user.clone(),
            liqpool.address.clone(),
            Symbol::short("swap"),
            (&user, false, 490000000_i128, 660000000_i128).into_val(&e)
        ),
        (
            user.clone(),
            token_0.address.clone(),
            Symbol::short("transfer"),
            (&user, &liqpool.address, x_in).into_val(&e)//from, to, amount
        )]
    );

    // Token that was bought
    assert_eq!(token_1.balance(&user), (800+49)*factor);
    assert_eq!(token_1.balance(&liqpool.address), (200-49)*factor);
    assert_eq!(token_0.balance(&user), new_balance_user);
    assert_eq!(token_0.balance(&liqpool.address), (200*factor)+x_in);
    assert_eq!(liqpool.my_balance(&liqpool.address), 1000);
    assert_eq!(liqpool.k_last(), 0);
    // Test fee_to has not yet received any fee
    assert_eq!(liqpool.my_balance(&admin_0), 0);
    let expected_reserve_0 = 200 * factor + x_in;
    let expected_reserve_1 = 200 * factor - y_out;
    assert_eq!(liqpool.get_reserves(), (expected_reserve_0, expected_reserve_1,passed_time + init_time));

    uq64x64_price_0_cumulative_last = liqpool.price_0_cumulative_last();
    uq64x64_price_1_cumulative_last = liqpool.price_1_cumulative_last();
    let passed_time_u128: u128 = passed_time.into();
    let expected_price_cumulative_last_decoded: u128 = expected_price_cumulative_last_decoded + 1*passed_time_u128*decimals_u128;

    assert_eq!(decode_uq64x64_with_7_decimals(uq64x64_price_0_cumulative_last), expected_price_cumulative_last_decoded);
    assert_eq!(decode_uq64x64_with_7_decimals(uq64x64_price_1_cumulative_last), expected_price_cumulative_last_decoded);


    // TODO: Test cumulative prices
    // // We will swap again to test that the price has changed and it's correctly added in cumulative price
    // init_time = passed_time + init_time;
    // passed_time = 1029;
    // let passed_time_u128: u128 = 1029;

    // e.ledger().set(LedgerInfo {
    //     timestamp: init_time + passed_time,
    //     protocol_version: 1,
    //     sequence_number: 10,
    //     network_id: Default::default(),
    //     base_reserve: 10,
    // });

    // let encoded_price_0_multiplied_by_time = fraction(expected_reserve_0.try_into().unwrap(), expected_reserve_1.try_into().unwrap())*passed_time_u128;
    // let encoded_price_1_multiplied_by_time = fraction(expected_reserve_1.try_into().unwrap(), expected_reserve_0.try_into().unwrap())*passed_time_u128;
    
    // liqpool.swap(&user, &false, &(10 * factor), &(100 * factor));
    
    // let expected_price_0_cumulative_last_encoded: u128 = uq64x64_price_0_cumulative_last + encoded_price_0_multiplied_by_time;
    // let expected_price_1_cumulative_last_encoded: u128 = uq64x64_price_1_cumulative_last+ encoded_price_1_multiplied_by_time;
    
    // let new_uq64x64_price_0_cumulative_last = liqpool.price_0_cumulative_last();
    // let new_uq64x64_price_1_cumulative_last = liqpool.price_1_cumulative_last();
    // assert_eq!(new_uq64x64_price_0_cumulative_last, expected_price_1_cumulative_last_encoded);
    // assert_eq!(new_uq64x64_price_1_cumulative_last, expected_price_1_cumulative_last_encoded);

    // Testing WITHDRAW
    let pair_token_0_balance = token_0.balance(&liqpool.address);
    let pair_token_1_balance = token_1.balance(&liqpool.address);
    let user_token_0_balance = token_0.balance(&user);
    let user_token_1_balance = token_1.balance(&user);
    let MINIMUM_LIQUIDITY = 1000;
    let mut total_shares = liqpool.total_shares();
    let total_user_shares = liqpool.my_balance(&user);
    let expected_user_out_token_0 = (pair_token_0_balance* total_user_shares) / total_shares;
    let expected_user_out_token_1 = (pair_token_1_balance* total_user_shares) / total_shares;
    let expected_locked_token_0 = pair_token_0_balance - expected_user_out_token_0;
    let expected_locked_token_1 = pair_token_1_balance - expected_user_out_token_1;

    assert_eq!(total_user_shares, (100 * factor + 999999000));

    liqpool.withdraw(&user, &total_user_shares, &0, &0);

    // Testing to.require_auth();
    assert_eq!(
        e.auths(),
        [(
            user.clone(),
            liqpool.address.clone(),
            Symbol::short("withdraw"),
            (&user, total_user_shares, 0_i128, 0_i128).into_val(&e)
        )]
    );

    // Testing the "withdraw" event
    // topics: (PAIR, Symbol::new(e, "withdraw"), sender);
    let topics = (PAIR, Symbol::new(&e, "withdraw"), user.clone());
    // data: (shares_burnt, amount_0, amount_1, to)
    let data = (total_user_shares, expected_user_out_token_0, expected_user_out_token_1, user.clone());
    assert_eq!(last_event_vec(&e),
                vec![&e,    (liqpool.address.clone(),
                            topics.into_val(&e),
                            data.into_val(&e))]);

    assert_eq!(token_1.balance(&liqpool.address), expected_locked_token_1);
    assert_eq!(token_0.balance(&liqpool.address), expected_locked_token_0);
    assert_eq!(liqpool.total_shares(), MINIMUM_LIQUIDITY);
    assert_eq!(token_0.balance(&user), user_token_0_balance + expected_user_out_token_0);
    assert_eq!(token_1.balance(&user), user_token_1_balance + expected_user_out_token_1);
    assert_eq!(liqpool.my_balance(&user), 0);

    // Testing the skim function:
    let pair_token_0_balance = token_0.balance(&liqpool.address);
    let pair_token_1_balance = token_1.balance(&liqpool.address);
    let (reserve_0, reserve_1, last_block) = liqpool.get_reserves();
    assert_eq!(pair_token_0_balance, reserve_0);
    assert_eq!(pair_token_1_balance, reserve_1);

    let user_2 = Address::random(&e);
    assert_eq!(token_0.balance(&user_2), 0);
    assert_eq!(token_1.balance(&user_2), 0);
    token_0.mint(&liqpool.address, &(30 * factor));
    token_1.mint(&liqpool.address, &(40 * factor));
    assert_eq!(token_0.balance(&liqpool.address), reserve_0 + (30 * factor));
    assert_eq!(token_1.balance(&liqpool.address), reserve_1 + (40 * factor));

    liqpool.skim(&user_2);
    assert_eq!(token_0.balance(&user_2), (30 * factor));
    assert_eq!(token_1.balance(&user_2), (40 * factor));
    assert_eq!(token_0.balance(&liqpool.address), reserve_0);
    assert_eq!(token_1.balance(&liqpool.address), reserve_1);

    // Testing the sync function
    // force reserves to match balances
    let pair_token_0_balance = token_0.balance(&liqpool.address);
    let pair_token_1_balance = token_1.balance(&liqpool.address);
    let (reserve_0, reserve_1, last_block) = liqpool.get_reserves();
    assert_eq!(pair_token_0_balance, reserve_0);
    assert_eq!(pair_token_1_balance, reserve_1);

    token_0.mint(&liqpool.address, &(30 * factor));
    token_1.mint(&liqpool.address, &(40 * factor));
    assert_eq!(token_0.balance(&liqpool.address), reserve_0 + (30 * factor));
    assert_eq!(token_1.balance(&liqpool.address), reserve_1 + (40 * factor));

    liqpool.sync();
    let (reserve_0, reserve_1, last_block) = liqpool.get_reserves();
    assert_eq!(token_0.balance(&liqpool.address), reserve_0);
    assert_eq!(token_1.balance(&liqpool.address), reserve_1);



    // TODO: Test when fee is on.
    // Test: 





    


}