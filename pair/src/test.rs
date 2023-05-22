#![cfg(test)]
extern crate std;

use crate::{SoroswapPairClient};

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_contract.wasm");
    pub type TokenClient = Client;
}

use token::TokenClient;


use soroban_sdk::{  testutils::Events,
                    vec,
                    testutils::Address as _,
                    Address, 
                    BytesN, 
                    Env,
                    IntoVal, Symbol};

fn create_token_contract(e: &Env, admin: &Address) -> TokenClient {
    TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
}

fn create_pair_contract(
    e: &Env,
    factory: &Address,
    token_a: &BytesN<32>,
    token_b: &BytesN<32>,
) -> SoroswapPairClient {
    let liqpool = SoroswapPairClient::new(e, &e.register_contract(None, crate::SoroswapPair {}));
    liqpool.initialize_pair(factory, token_a, token_b);
    liqpool
}

// fn last_event_vec(e: &Env){

// }

 
#[test]
fn test() {
    let e: Env = Default::default();

    let mut admin0 = Address::random(&e);
    let mut admin1 = Address::random(&e);

    let mut token0 = create_token_contract(&e, &admin0);
    let mut token1 = create_token_contract(&e, &admin1);
    if &token1.contract_id < &token0.contract_id {
        std::mem::swap(&mut token0, &mut token1);
        std::mem::swap(&mut admin0, &mut admin1);
    }
    let user = Address::random(&e);
    let liqpool = create_pair_contract(
        &e,
        &admin0,
        &token0.contract_id,
        &token1.contract_id,
    );


    token0.mint(&admin0, &user, &1000);
    assert_eq!(token0.balance(&user), 1000);

    token1.mint(&admin1, &user, &1000);
    assert_eq!(token1.balance(&user), 1000);


    liqpool.deposit(&user, &100, &100, &100, &100);

    // TODO: Test Events when we can take the last event
    let topics = (Symbol::new(&e, "deposit"), user.clone(), 100_i128);
    let events_vec = e.events().all();
    let last_event = events_vec.last().unwrap().unwrap();

    assert_eq!(
        vec![&e, last_event],
        vec![
            &e,
            (   liqpool.contract_id.clone(),
                topics.into_val(&e),
                100_i128.into_val(&e)),
        ]
    );
   
    assert_eq!(
        e.recorded_top_authorizations(),
        std::vec![(
            user.clone(),
            liqpool.contract_id.clone(),
            Symbol::short("deposit"),
            (&user, 100_i128, 100_i128, 100_i128, 100_i128).into_val(&e)
        )]
    );

    // // Test the event:

    // assert_eq!(
    //     e.events().all(),
    //     vec![
    //         &e,
    //         (
    //             liqpool.contract_id.clone(),
    //             (Symbol::short("mint"), &user, 49_i128.into_val(&e)).into_val(&e),
    //             49_i128.into_val(&e)
    //         )
    //     ]
    // );


    // let topics = (Symbol::new(e, "mint"), sender, amount_0);
    // e.events().publish(topics, amount_1);

    // (
    //     contract_id.clone(),
    //     (Symbol::short("COUNTER"), Symbol::short("increment")).into_val(&env),
    //     1u32.into_val(&env)
    // )
    // let topics = (Symbol::new(&e, "deposit"), user.clone(), 49_i128);
    // assert_eq!(
    //     e.events().all(),
    //     vec![
    //         &e,
    //         (   liqpool.contract_id.clone(),
    //             topics.into_val(&e),
    //             49_i128.into_val(&e)),
    //     ]
    // );

    assert_eq!(liqpool.my_balance(&user), 100);
    assert_eq!(liqpool.my_balance(&liqpool.address()), 0);
    assert_eq!(token0.balance(&user), 900);
    assert_eq!(token0.balance(&liqpool.address()), 100);
    assert_eq!(token1.balance(&user), 900);
    assert_eq!(token1.balance(&liqpool.address()), 100);

    liqpool.swap(&user, &false, &49, &100);

    // Test to.require_auth();
    assert_eq!(
        e.recorded_top_authorizations(),
        std::vec![(
            user.clone(),
            liqpool.contract_id.clone(),
            Symbol::short("swap"),
            (&user, false, 49_i128, 100_i128).into_val(&e)
        )]
    );

    assert_eq!(token0.balance(&user), 803);
    assert_eq!(token0.balance(&liqpool.address()), 197);
    assert_eq!(token1.balance(&user), 949);
    assert_eq!(token1.balance(&liqpool.address()), 51);

    liqpool.withdraw(&user, &100, &197, &51);

    assert_eq!(
        e.recorded_top_authorizations(),
        std::vec![(
            user.clone(),
            liqpool.contract_id.clone(),
            Symbol::short("withdraw"),
            (&user, 100_i128, 197_i128, 51_i128).into_val(&e)
        )]
    );

    assert_eq!(token0.balance(&user), 1000);
    assert_eq!(token1.balance(&user), 1000);
    assert_eq!(liqpool.my_balance(&user), 0);
    assert_eq!(token0.balance(&liqpool.address()), 0);
    assert_eq!(token1.balance(&liqpool.address()), 0);
    assert_eq!(liqpool.my_balance(&liqpool.address()), 0);
}
