#![cfg(test)]

extern crate std;
use soroban_sdk::{Env, BytesN, Address, testutils::Address as _, vec, Vec};
use crate::{SoroswapLibrary, SoroswapLibraryClient};

mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
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
    //pair: SoroswapPairClient<'a>,
}

impl<'a> SoroswapLibraryTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();
        let contract = create_soroswap_library_contract(&env);

        let admin = Address::random(&env);
        let user = Address::random(&env);

        let mut token_0 = create_token_contract(&env, &admin);
        let mut token_1 = create_token_contract(&env, &admin);
        if &token_1.address.contract_id() < &token_0.address.contract_id() {
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
        
        SoroswapLibraryTest {
            env,
            contract,
            token_0,
            token_1,
            factory,
            //pair,
        }
    }
}


#[test]
fn test_quote() {
    let test = SoroswapLibraryTest::setup();
    // expect(await router.quote(bigNumberify(1), bigNumberify(100), bigNumberify(200))).to.eq(bigNumberify(2))
    assert_eq!(2,test.contract.quote(&1, &100, &200));
    // expect(await router.quote(bigNumberify(2), bigNumberify(200), bigNumberify(100))).to.eq(bigNumberify(1))
    assert_eq!(1,test.contract.quote(&2, &200, &100));
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient amount")]
fn test_quote_insufficient_amount() {
    // await expect(router.quote(bigNumberify(0), bigNumberify(100), bigNumberify(200))).to.be.revertedWith(
    //   'UniswapV2Library: INSUFFICIENT_AMOUNT'
    // )
    let test = SoroswapLibraryTest::setup();
    test.contract.quote(&0, &100, &200);
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient liquidity")]
fn test_quote_insufficient_liquidity_0() {
    //     await expect(router.quote(bigNumberify(1), bigNumberify(0), bigNumberify(200))).to.be.revertedWith(
    //   'UniswapV2Library: INSUFFICIENT_LIQUIDITY'
    // )
    let test = SoroswapLibraryTest::setup();
    test.contract.quote(&1, &0, &200);
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient liquidity")]
fn test_quote_insufficient_liquidity_1() {
    //     await expect(router.quote(bigNumberify(1), bigNumberify(10), bigNumberify(0))).to.be.revertedWith(
    //   'UniswapV2Library: INSUFFICIENT_LIQUIDITY'
    // )
    let test = SoroswapLibraryTest::setup();
    test.contract.quote(&1, &100, &0);
}

#[test]
fn test_get_amount_out() {
    let test = SoroswapLibraryTest::setup();
    // expect(await router.getAmountOut(bigNumberify(2), bigNumberify(100), bigNumberify(100))).to.eq(bigNumberify(1))
    assert_eq!(1,test.contract.get_amount_out(&2, &100, &100));
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient input amount")]
fn test_get_amount_out_insufficient_input_amount() {
    // await expect(router.getAmountOut(bigNumberify(0), bigNumberify(100), bigNumberify(100))).to.be.revertedWith(
    //   'UniswapV2Library: INSUFFICIENT_INPUT_AMOUNT'
    // )
    let test = SoroswapLibraryTest::setup();
    test.contract.get_amount_out(&0, &100, &100);
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient liquidity")]
fn test_get_amount_out_insufficient_liquidity_0() {
    //      await expect(router.getAmountOut(bigNumberify(2), bigNumberify(0), bigNumberify(100))).to.be.revertedWith(
    //   'UniswapV2Library: INSUFFICIENT_LIQUIDITY'
    // )
    let test = SoroswapLibraryTest::setup();
    test.contract.get_amount_out(&2, &0, &100);
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient liquidity")]
fn test_get_amount_out_insufficient_liquidity_1() {
    //      await expect(router.getAmountOut(bigNumberify(2), bigNumberify(100), bigNumberify(0))).to.be.revertedWith(
    //   'UniswapV2Library: INSUFFICIENT_LIQUIDITY'
    // )
    let test = SoroswapLibraryTest::setup();
    test.contract.get_amount_out(&2, &100, &0);
}

    
#[test]
fn test_get_amount_in() {
    let test = SoroswapLibraryTest::setup();
    // expect(await router.getAmountIn(bigNumberify(1), bigNumberify(100), bigNumberify(100))).to.eq(bigNumberify(2))
    assert_eq!(2,test.contract.get_amount_in(&1, &100, &100));
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient output amount")]
fn test_get_amount_in_insufficient_output_amount() {
    // await expect(router.getAmountIn(bigNumberify(0), bigNumberify(100), bigNumberify(100))).to.be.revertedWith(
    //   'UniswapV2Library: INSUFFICIENT_OUTPUT_AMOUNT'
    // )
    let test = SoroswapLibraryTest::setup();
    test.contract.get_amount_in(&0, &100, &100);
}

#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient liquidity")]
fn test_get_amount_in_insufficient_liquidity_0() {
    //     await expect(router.getAmountIn(bigNumberify(1), bigNumberify(0), bigNumberify(100))).to.be.revertedWith(
    //   'UniswapV2Library: INSUFFICIENT_LIQUIDITY'
    // )
    let test = SoroswapLibraryTest::setup();
    test.contract.get_amount_in(&1, &0, &100);
}


#[test]
#[should_panic(expected = "SoroswapLibrary: insufficient liquidity")]
fn test_get_amount_in_insufficient_liquidity_1() {
    //     await expect(router.getAmountIn(bigNumberify(1), bigNumberify(0), bigNumberify(100))).to.be.revertedWith(
    //   'UniswapV2Library: INSUFFICIENT_LIQUIDITY'
    // )
    let test = SoroswapLibraryTest::setup();
    test.contract.get_amount_in(&1, &100, &0);
}



#[test]
fn test_get_amounts_out() {
    let test = SoroswapLibraryTest::setup();
    
    let path: Vec<Address> =  vec![&test.env, test.token_0.address.clone(), test.token_1.address.clone()];
    
    // expect(await router.getAmountsOut(bigNumberify(2), path)).to.deep.eq([bigNumberify(2), bigNumberify(1)])
    let expected_amounts_out = vec![&test.env, 2, 1];
    let amounts_out = test.contract.get_amounts_out(&test.factory.address, &2, &path);
    assert_eq!(expected_amounts_out,amounts_out);
}

#[test]
#[should_panic(expected = "SoroswapLibrary: invalid path")]
fn test_get_amounts_out_invalid_path() {
    let test = SoroswapLibraryTest::setup();
    
    // await expect(router.getAmountsOut(bigNumberify(2), [token0.address])).to.be.revertedWith(
    //           'UniswapV2Library: INVALID_PATH'
    //         )
    let path: Vec<Address> =  vec![&test.env, test.token_0.address.clone()];
    test.contract.get_amounts_out(&test.factory.address, &2, &path);
}


#[test]
fn test_get_amounts_in() {
    let test = SoroswapLibraryTest::setup();
    
    let path: Vec<Address> =  vec![&test.env, test.token_0.address.clone(), test.token_1.address.clone()];
    
    // expect(await router.getAmountsIn(bigNumberify(1), path)).to.deep.eq([bigNumberify(2), bigNumberify(1)])
    let expected_amounts_in = vec![&test.env, 2, 1];
    let amounts_out = test.contract.get_amounts_in(&test.factory.address, &1, &path);
    assert_eq!(expected_amounts_in,amounts_out);
}

#[test]
#[should_panic(expected = "SoroswapLibrary: invalid path")]
fn test_get_amounts_in_invalid_path() {
    let test = SoroswapLibraryTest::setup();
    
    //     await expect(router.getAmountsIn(bigNumberify(1), [token0.address])).to.be.revertedWith(
    //   'UniswapV2Library: INVALID_PATH'
    // )
    let path: Vec<Address> =  vec![&test.env, test.token_0.address.clone()];
    test.contract.get_amounts_in(&test.factory.address, &1, &path);
}