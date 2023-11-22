#![cfg(test)]
extern crate std;
use soroban_sdk::{  symbol_short,
    testutils::{Events, Ledger},
    Vec,
    Val,
    vec,
    testutils::{Address as _},
    Address, 
    BytesN, 
    Env,
    IntoVal, Symbol};
use crate::{SoroswapPairClient};

// TOKEN CONTRACT
mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
use token::TokenClient;
fn create_token_contract<'a>(e: &Env, admin: & Address) -> TokenClient<'a> {
    TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
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

fn pair_token_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "./target/wasm32-unknown-unknown/release/soroswap_pair.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

fn create_pair_contract<'a>(
    e: & Env,
    factory: & Address,
    token_a: & Address,
    token_b: & Address,
) -> SoroswapPairClient<'a> {
    let liqpool = SoroswapPairClient::new(e, &e.register_contract(None, crate::SoroswapPair {}));
    liqpool
}

// HELPERS
fn last_event_vec(e: &Env) -> Vec<(Address, Vec<Val>, Val)>{
    vec![&e, e.events().all().last().unwrap()]
}
const PAIR: Symbol = symbol_short!("PAIR");


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
        let user = Address::random(&env);
        let admin = Address::random(&env);
        let mut token_0 = create_token_contract(&env, &admin);
        let mut token_1 = create_token_contract(&env, &admin);
        if &token_1.address.contract_id() < &token_0.address.contract_id() {
            std::mem::swap(&mut token_0, &mut token_1);
        }
        
        token_0.mint(&user, &123_000_000_000_000_000_000);
        token_1.mint(&user, &321_000_000_000_000_000_000);

        let pair_token_wasm_binding = pair_token_wasm(&env);  
        let factory = create_factory_contract(&env, &admin, &pair_token_wasm_binding);

        let contract = create_pair_contract(
            &env,
            &factory.address,
            &token_0.address,
            &token_1.address,
        );

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
           


// TESTS MODS (in ./test/ folder)
mod initialize_pair;
mod operations;
mod decode;
mod deposit;
mod swap;
mod helpers;
mod operations_helpers;


// #[test]
// fn test() {
//     let e: Env = Default::default();
//     e.mock_all_auths();
    
//     let user = Address::random(&e);
//     let admin = Address::random(&e);
//     let mut token_0 = create_token_contract(&e, &admin);
//     let mut token_1 = create_token_contract(&e, &admin);
//     if &token_1.address.contract_id() < &token_0.address.contract_id() {
//         std::mem::swap(&mut token_0, &mut token_1);
//     }
  
//     let factory = create_factory_contract(&e, &admin_0, &pair_token_wasm(&e));

//     // Test factory initial values:
//     assert_eq!(factory.fee_to(), admin_0);
//     assert_eq!(factory.fee_to_setter(), admin_0);
//     assert_eq!(factory.fees_enabled(), false);

//     let liqpool = create_pair_contract(
//         &e,
//         &factory.address,
//         &token_0.address,
//         &token_1.address,
//     );

//     // Test liqpool initial values:
//     assert_eq!(liqpool.token_0(), token_0.address);
//     assert_eq!(liqpool.token_1(), token_1.address);
//     assert_eq!(liqpool.factory(), factory.address);
//     assert_eq!(liqpool.get_reserves(), (0,0,0));
//     assert_eq!(liqpool.k_last(), 0);
//     assert_eq!(liqpool.price_0_cumulative_last(), 0);
//     assert_eq!(liqpool.price_1_cumulative_last(), 0);

//     let factor = 10000000; // we will use 7 decimals

//     token_0.mint(&user, &(1000 * factor));
//     assert_eq!(token_0.balance(&user), 1000 * factor);
//     token_1.mint(&user, &&(1000 * factor));
//     assert_eq!(token_1.balance(&user), 1000 * factor);

//     // Testing the deposit function:
//     let mut init_time = 12345;

//     e.ledger().with_mut(|li| {
//         li.timestamp = init_time;
//     });

//     // liqpool.deposit(&user, &(100 * factor), &(100 * factor), &(100 * factor), &(100 * factor));

//     // let topics = (PAIR, Symbol::new(&e, "deposit"), user.clone());
//     // let data = (1000000000_i128, 1000000000_i128);
//     // assert_eq!(last_event_vec(&e),
//     //             vec![&e,    (liqpool.address.clone(),
//     //                         topics.into_val(&e),
//     //                         data.into_val(&e))]);
   
//     // // TODO: Test with sub_invotations!
//     // // assert_eq!(
//     // //     e.auths(),
//     // //     std::vec![(
//     // //         user.clone(),
//     // //         AuthorizedInvocation {
//     // //             function: AuthorizedFunction::Contract((
//     // //                 liqpool.address.clone(),
//     // //                 symbol_short!("deposit"),
//     // //                 (&user, 1000000000_i128, 1000000000_i128, 1000000000_i128, 1000000000_i128).into_val(&e)
//     // //             )),
//     // //             sub_invocations: std::vec![]
//     // //         }
//     // //     ),
//     // //     (
//     // //         user.clone(),
//     // //         AuthorizedInvocation {
//     // //             function: AuthorizedFunction::Contract((
//     // //                 token_0.address.clone(),
//     // //                 symbol_short!("transfer"),
//     // //                 (&user, &liqpool.address, 1000000000_i128).into_val(&e)//from, to, amount
//     // //             )),
//     // //             sub_invocations: std::vec![]
//     // //         }
//     // //     ),
//     // //     (
//     // //         user.clone(),
//     // //         AuthorizedInvocation {
//     // //             function: AuthorizedFunction::Contract((
//     // //                 token_1.address.clone(),
//     // //                 symbol_short!("transfer"),
//     // //                 (&user, &liqpool.address, 1000000000_i128).into_val(&e)
//     // //             )),
//     // //             sub_invocations: std::vec![]
//     // //         }
//     // //     )]
//     // // );


//     // assert_eq!(liqpool.my_balance(&user), 999999000);
//     // // We lock forever the minimum_liquidity (1000) in the LP contract itself
//     // assert_eq!(liqpool.my_balance(&liqpool.address), 1000);
//     // assert_eq!(token_0.balance(&user), 900 * factor);
//     // assert_eq!(token_0.balance(&liqpool.address), 100 * factor);
//     // assert_eq!(token_1.balance(&user), 900 * factor);
//     // assert_eq!(token_1.balance(&liqpool.address), 100 * factor);

//     // assert_eq!(liqpool.k_last(), 0);
//     // // Test fee_to has not yet received any fee
//     // assert_eq!(liqpool.my_balance(&admin_0), 0);
//     // // The first deposit, we don't have yet any price accumulated, this will be on the second deposit
//     // assert_eq!(liqpool.price_0_cumulative_last(), 0);
//     // assert_eq!(liqpool.price_1_cumulative_last(), 0);
//     // assert_eq!(liqpool.get_reserves(), (100 * factor, 100 * factor,init_time));


//     // // Now, let's deposit again in order to have Cumulative Prices.
//     // let mut passed_time = 54321;

//     // e.ledger().with_mut(|li| {
//     //     li.timestamp = passed_time + init_time;
//     // });

//     // liqpool.deposit(&user, &(100 * factor), &(100 * factor), &(100 * factor), &(100 * factor));
//     // assert_eq!(liqpool.my_balance(&user), 100 * factor + 999999000);
//     // assert_eq!(liqpool.my_balance(&liqpool.address), 1000);
//     // assert_eq!(token_0.balance(&user), 800 * factor);
//     // assert_eq!(token_0.balance(&liqpool.address), 200 * factor);
//     // assert_eq!(token_1.balance(&user), 800 * factor);
//     // assert_eq!(token_1.balance(&liqpool.address), 200 * factor);
//     // assert_eq!(liqpool.get_reserves(), (200 * factor, 200 * factor,passed_time + init_time));
//     // let mut uq64x64_price_0_cumulative_last = liqpool.price_0_cumulative_last();
//     // let mut uq64x64_price_1_cumulative_last = liqpool.price_1_cumulative_last();
//     // let decimals_u128: u128 = 10000000;
//     // let passed_time_u128: u128 = passed_time.into();
//     // let expected_price_cumulative_last_decoded: u128 = 1*passed_time_u128*decimals_u128;

//     // assert_eq!(decode_uq64x64_with_7_decimals(uq64x64_price_0_cumulative_last), expected_price_cumulative_last_decoded);
//     // assert_eq!(decode_uq64x64_with_7_decimals(uq64x64_price_1_cumulative_last), expected_price_cumulative_last_decoded);


//     // // TODO: Test event::sync, do it with last_n_events function

//     // // Testing SWAP
//     // init_time = passed_time + init_time;
//     // passed_time = 9876;

//     // e.ledger().with_mut(|li| {
//     //     li.timestamp = init_time + passed_time;
//     // });

//     // liqpool.swap(&user, &false, &(49 * factor), &(66 * factor));
    
//     // let y_out = 49*factor;
//     // let x_in =(1000*200*factor*49*factor)/((200-49)*factor*997)+1;
//     // let new_balance_user = (800*factor)-x_in;

//     // // Testing the "deposit" event
//     // let topics = (PAIR, Symbol::new(&e, "swap"), user.clone());
//     // // data: (amount_0_in, amount_1_in, amount_0_out,amount_1_out,  to)
//     // let data = (x_in, 0_i128, 0_i128, y_out, user.clone());
//     // assert_eq!(last_event_vec(&e),
//     //             vec![&e,    (liqpool.address.clone(),
//     //                         topics.into_val(&e),
//     //                         data.into_val(&e))]);

//     // // TODO: Test with sub_invocations!
//     // // // Test to.require_auth();
//     // // assert_eq!(
//     // //     e.auths(),
//     // //     std::vec![(
//     // //         user.clone(),
//     // //         AuthorizedInvocation {
//     // //             function: AuthorizedFunction::Contract((
//     // //                 liqpool.address.clone(),
//     // //                 symbol_short!("swap"),
//     // //                 (&user, false, 490000000_i128, 660000000_i128).into_val(&e)
//     // //             )),
//     // //             sub_invocations: std::vec![]
//     // //         }
//     // //     ),
//     // //     (
//     // //         user.clone(),
//     // //         AuthorizedInvocation {
//     // //             function: AuthorizedFunction::Contract((
//     // //                 token_0.address.clone(),
//     // //                 symbol_short!("transfer"),
//     // //                 (&user, &liqpool.address, x_in).into_val(&e)//from, to, amount
//     // //             )),
//     // //             sub_invocations: std::vec![]
//     // //         }
//     // //     )]
//     // // );

//     // // Token that was bought
//     // assert_eq!(token_1.balance(&user), (800+49)*factor);
//     // assert_eq!(token_1.balance(&liqpool.address), (200-49)*factor);
//     // assert_eq!(token_0.balance(&user), new_balance_user);
//     // assert_eq!(token_0.balance(&liqpool.address), (200*factor)+x_in);
//     // assert_eq!(liqpool.my_balance(&liqpool.address), 1000);
//     // assert_eq!(liqpool.k_last(), 0);
//     // // Test fee_to has not yet received any fee
//     // assert_eq!(liqpool.my_balance(&admin_0), 0);
//     // let expected_reserve_0 = 200 * factor + x_in;
//     // let expected_reserve_1 = 200 * factor - y_out;
//     // assert_eq!(liqpool.get_reserves(), (expected_reserve_0, expected_reserve_1,passed_time + init_time));

//     // uq64x64_price_0_cumulative_last = liqpool.price_0_cumulative_last();
//     // uq64x64_price_1_cumulative_last = liqpool.price_1_cumulative_last();
//     // let passed_time_u128: u128 = passed_time.into();
//     // let expected_price_cumulative_last_decoded: u128 = expected_price_cumulative_last_decoded + 1*passed_time_u128*decimals_u128;

//     // assert_eq!(decode_uq64x64_with_7_decimals(uq64x64_price_0_cumulative_last), expected_price_cumulative_last_decoded);
//     // assert_eq!(decode_uq64x64_with_7_decimals(uq64x64_price_1_cumulative_last), expected_price_cumulative_last_decoded);


//     // // TODO: Test cumulative prices
//     // // // We will swap again to test that the price has changed and it's correctly added in cumulative price
//     // // init_time = passed_time + init_time;
//     // // passed_time = 1029;
//     // // let passed_time_u128: u128 = 1029;

//     // // e.ledger().with_mut(|li| {
//     // //     li.timestamp = init_time + passed_time;
//     // // });

//     // // let encoded_price_0_multiplied_by_time = fraction(expected_reserve_0.try_into().unwrap(), expected_reserve_1.try_into().unwrap())*passed_time_u128;
//     // // let encoded_price_1_multiplied_by_time = fraction(expected_reserve_1.try_into().unwrap(), expected_reserve_0.try_into().unwrap())*passed_time_u128;
    
//     // // liqpool.swap(&user, &false, &(10 * factor), &(100 * factor));
    
//     // // let expected_price_0_cumulative_last_encoded: u128 = uq64x64_price_0_cumulative_last + encoded_price_0_multiplied_by_time;
//     // // let expected_price_1_cumulative_last_encoded: u128 = uq64x64_price_1_cumulative_last+ encoded_price_1_multiplied_by_time;
    
//     // // let new_uq64x64_price_0_cumulative_last = liqpool.price_0_cumulative_last();
//     // // let new_uq64x64_price_1_cumulative_last = liqpool.price_1_cumulative_last();
//     // // assert_eq!(new_uq64x64_price_0_cumulative_last, expected_price_1_cumulative_last_encoded);
//     // // assert_eq!(new_uq64x64_price_1_cumulative_last, expected_price_1_cumulative_last_encoded);

//     // // Testing WITHDRAW
//     // let pair_token_0_balance = token_0.balance(&liqpool.address);
//     // let pair_token_1_balance = token_1.balance(&liqpool.address);
//     // let user_token_0_balance = token_0.balance(&user);
//     // let user_token_1_balance = token_1.balance(&user);
//     // let minimum_liquidity = 1000;
//     // let total_shares = liqpool.total_shares();
//     // let total_user_shares = liqpool.my_balance(&user);
//     // let expected_user_out_token_0 = (pair_token_0_balance* total_user_shares) / total_shares;
//     // let expected_user_out_token_1 = (pair_token_1_balance* total_user_shares) / total_shares;
//     // let expected_locked_token_0 = pair_token_0_balance - expected_user_out_token_0;
//     // let expected_locked_token_1 = pair_token_1_balance - expected_user_out_token_1;

//     // assert_eq!(total_user_shares, (100 * factor + 999999000));

//     // liqpool.withdraw(&user, &total_user_shares, &0, &0);

//     // // Testing to.require_auth();
//     // assert_eq!(
//     //     e.auths(),
//     //     std::vec![(
//     //         user.clone(),
//     //         AuthorizedInvocation {
//     //             function: AuthorizedFunction::Contract((
//     //                 liqpool.address.clone(),
//     //                 symbol_short!("withdraw"),
//     //                 (&user, total_user_shares, 0_i128, 0_i128).into_val(&e) 
//     //             )),
//     //             sub_invocations: std::vec![]
//     //         }
//     //     )]
//     // );

//     // // Testing the "withdraw" event
//     // // topics: (PAIR, Symbol::new(e, "withdraw"), sender);
//     // let topics = (PAIR, Symbol::new(&e, "withdraw"), user.clone());
//     // // data: (shares_burnt, amount_0, amount_1, to)
//     // let data = (total_user_shares, expected_user_out_token_0, expected_user_out_token_1, user.clone());
//     // assert_eq!(last_event_vec(&e),
//     //             vec![&e,    (liqpool.address.clone(),
//     //                         topics.into_val(&e),
//     //                         data.into_val(&e))]);

//     // assert_eq!(token_1.balance(&liqpool.address), expected_locked_token_1);
//     // assert_eq!(token_0.balance(&liqpool.address), expected_locked_token_0);
//     // assert_eq!(liqpool.total_shares(), minimum_liquidity);
//     // assert_eq!(token_0.balance(&user), user_token_0_balance + expected_user_out_token_0);
//     // assert_eq!(token_1.balance(&user), user_token_1_balance + expected_user_out_token_1);
//     // assert_eq!(liqpool.my_balance(&user), 0);

//     // // Testing the skim function:
//     // let pair_token_0_balance = token_0.balance(&liqpool.address);
//     // let pair_token_1_balance = token_1.balance(&liqpool.address);
//     // let (reserve_0, reserve_1, _last_block) = liqpool.get_reserves();
//     // assert_eq!(pair_token_0_balance, reserve_0);
//     // assert_eq!(pair_token_1_balance, reserve_1);

//     // let user_2 = Address::random(&e);
//     // assert_eq!(token_0.balance(&user_2), 0);
//     // assert_eq!(token_1.balance(&user_2), 0);
//     // token_0.mint(&liqpool.address, &(30 * factor));
//     // token_1.mint(&liqpool.address, &(40 * factor));
//     // assert_eq!(token_0.balance(&liqpool.address), reserve_0 + (30 * factor));
//     // assert_eq!(token_1.balance(&liqpool.address), reserve_1 + (40 * factor));

//     // liqpool.skim(&user_2);
//     // assert_eq!(token_0.balance(&user_2), (30 * factor));
//     // assert_eq!(token_1.balance(&user_2), (40 * factor));
//     // assert_eq!(token_0.balance(&liqpool.address), reserve_0);
//     // assert_eq!(token_1.balance(&liqpool.address), reserve_1);

//     // // Testing the sync function
//     // // force reserves to match balances
//     // let pair_token_0_balance = token_0.balance(&liqpool.address);
//     // let pair_token_1_balance = token_1.balance(&liqpool.address);
//     // let (reserve_0, reserve_1, _last_block) = liqpool.get_reserves();
//     // assert_eq!(pair_token_0_balance, reserve_0);
//     // assert_eq!(pair_token_1_balance, reserve_1);

//     // token_0.mint(&liqpool.address, &(30 * factor));
//     // e.budget().reset_unlimited(); 
//     // token_1.mint(&liqpool.address, &(40 * factor));
//     // assert_eq!(token_0.balance(&liqpool.address), reserve_0 + (30 * factor));
//     // assert_eq!(token_1.balance(&liqpool.address), reserve_1 + (40 * factor));

//     // liqpool.sync();
//     // let (reserve_0, reserve_1, _last_block) = liqpool.get_reserves();
//     // assert_eq!(token_0.balance(&liqpool.address), reserve_0);
//     // assert_eq!(token_1.balance(&liqpool.address), reserve_1);



//     // // // TODO: Test when fee is on.
//     // // // Test: 




// }