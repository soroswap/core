/*
    TESTING ROUTER LIBRARY FUNCTIONS
    
    fn router_quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> i128;
    fn router_get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128;
    fn router_get_amount_in(amount_out: i128, reserve_in: i128, reserve_out: i128) -> i128;
    fn router_get_amounts_out(
        e: Env,
        amount_in: i128,
        path: Vec<Address>,
    ) -> Vec<i128>;
    fn router_get_amounts_in(
        e: Env,
        amount_out: i128,
        path: Vec<Address>,
    ) -> Vec<i128>;
*/
use crate::test::{SoroswapRouterTest, SoroswapPairClient};
use crate::test::add_liquidity::add_liquidity;


use soroban_sdk::{
    Address,
    testutils::{
        Address as _, 
        MockAuth,
        MockAuthInvoke,
        Ledger},
    vec,
    IntoVal};


// router_quote

#[test]
fn test_quote() {
    let test = SoroswapRouterTest::setup();
    assert_eq!(2,test.contract.router_quote(&1, &100, &200));
    assert_eq!(1,test.contract.router_quote(&2, &200, &100));
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient amount")]
fn test_quote_insufficient_amount() {
    // await expect(router.quote(bigNumberify(0), bigNumberify(100), bigNumberify(200))).to.be.revertedWith(
    //   'UniswapV2Library: INSUFFICIENT_AMOUNT'
    // )
    let test = SoroswapRouterTest::setup();
    test.contract.router_quote(&0, &100, &200);
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient liquidity")]
fn test_quote_insufficient_liquidity_0() {
    let test = SoroswapRouterTest::setup();
    test.contract.router_quote(&1, &0, &200);
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient liquidity")]
fn test_quote_insufficient_liquidity_1() {
    let test = SoroswapRouterTest::setup();
    test.contract.router_quote(&1, &100, &0);
}

// router_get_amount_out


#[test]
fn test_get_amount_out() {
    let test = SoroswapRouterTest::setup();
    assert_eq!(1,test.contract.router_get_amount_out(&2, &100, &100));
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient input amount")]
fn test_get_amount_out_insufficient_input_amount() {
    let test = SoroswapRouterTest::setup();
    test.contract.router_get_amount_out(&0, &100, &100);
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient liquidity")]
fn test_get_amount_out_insufficient_liquidity_0() {
    let test = SoroswapRouterTest::setup();
    test.contract.router_get_amount_out(&2, &0, &100);
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient liquidity")]
fn test_get_amount_out_insufficient_liquidity_1() {
    let test = SoroswapRouterTest::setup();
    test.contract.router_get_amount_out(&2, &100, &0);
}

// router_get_amount_in

    
#[test]
fn test_get_amount_in() {
    let test = SoroswapRouterTest::setup();
    assert_eq!(2,test.contract.router_get_amount_in(&1, &100, &100));
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient output amount")]
fn test_get_amount_in_insufficient_output_amount() {
    let test = SoroswapRouterTest::setup();
    test.contract.router_get_amount_in(&0, &100, &100);
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient liquidity")]
fn test_get_amount_in_insufficient_liquidity_0() {
    let test = SoroswapRouterTest::setup();
    test.contract.router_get_amount_in(&1, &0, &100);
}


#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient liquidity")]
fn test_get_amount_in_insufficient_liquidity_1() {
    let test = SoroswapRouterTest::setup();
    test.contract.router_get_amount_in(&1, &100, &0);
}

// router_get_amounts_out


#[test]
#[should_panic(expected = "SoroswapLibrary: invalid path")]
fn test_get_amounts_out_invalid_path() {
    let test = SoroswapRouterTest::setup();   
    test.contract.initialize(&test.factory.address); 
    let path =  vec![&test.env, test.token_0.address];
    test.contract.router_get_amounts_out(&2, &path);
}

// #[test]
// #[should_panic(expected = "SoroswapRouter: not yet initialized")]
// fn test_get_amounts_out_not_yet_initialized() {
//     let test = SoroswapRouterTest::setup();   
//     let path = vec![&test.env, test.token_0.address, test.token_1.address];
//     test.contract.router_get_amounts_out(&2, &path);
// }

#[test]
fn test_get_amounts_out() {
    let test = SoroswapRouterTest::setup();
    
    // TODO: Get rid of this hack?
    test.env.budget().reset_unlimited();

    test.contract.initialize(&test.factory.address);

    let amount_0: i128 = 10_000;
    let amount_1: i128 = 10_000;

    add_liquidity(&test, &amount_0, &amount_1);

    let path = vec![&test.env, test.token_0.address, test.token_1.address];
    assert_eq!(test.contract.router_get_amounts_out(&2, &path), vec![&test.env,2, 1]);
}







    
