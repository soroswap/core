#![cfg(test)]
//extern crate std;

use soroban_sdk::{Env};


use crate::{SoroswapLibrary, SoroswapLibraryClient};
// const provider = new MockProvider({
//     hardfork: 'istanbul',
//     mnemonic: 'horn horn horn horn horn horn horn horn horn horn horn horn',
//     gasLimit: 9999999
//   })
//   const [wallet] = provider.getWallets()
//   const loadFixture = createFixtureLoader(provider, [wallet])

//   let token0: Contract
//   let token1: Contract
//   let router: Contract
//   beforeEach(async function() {
//     const fixture = await loadFixture(v2Fixture)
//     token0 = fixture.token0
//     token1 = fixture.token1
//     router = fixture.router02
//   })


// Token contract
mod token_contract {
    soroban_sdk::contractimport!(
        file = "../token/soroban_token_contract.wasm"
    );
}


fn create_soroswap_library_contract<'a>(e: &Env) -> SoroswapLibraryClient<'a> {
    SoroswapLibraryClient::new(e, &e.register_contract(None, SoroswapLibrary {}))
}

struct SoroswapLibraryTest<'a> {
    contract: SoroswapLibraryClient<'a>,
}

impl<'a> SoroswapLibraryTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        let contract = create_soroswap_library_contract(&env);
        SoroswapLibraryTest {
            contract,
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
// it('getAmountIn', async () => {
//     expect(await router.getAmountIn(bigNumberify(1), bigNumberify(100), bigNumberify(100))).to.eq(bigNumberify(2))
//     await expect(router.getAmountIn(bigNumberify(0), bigNumberify(100), bigNumberify(100))).to.be.revertedWith(
//       'UniswapV2Library: INSUFFICIENT_OUTPUT_AMOUNT'
//     )
//     await expect(router.getAmountIn(bigNumberify(1), bigNumberify(0), bigNumberify(100))).to.be.revertedWith(
//       'UniswapV2Library: INSUFFICIENT_LIQUIDITY'
//     )
//     await expect(router.getAmountIn(bigNumberify(1), bigNumberify(100), bigNumberify(0))).to.be.revertedWith(
//       'UniswapV2Library: INSUFFICIENT_LIQUIDITY'
//     )
//   })

//   it('getAmountsOut', async () => {
//     await token0.approve(router.address, MaxUint256)
//     await token1.approve(router.address, MaxUint256)
//     await router.addLiquidity(
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

//     await expect(router.getAmountsOut(bigNumberify(2), [token0.address])).to.be.revertedWith(
//       'UniswapV2Library: INVALID_PATH'
//     )
//     const path = [token0.address, token1.address]
//     expect(await router.getAmountsOut(bigNumberify(2), path)).to.deep.eq([bigNumberify(2), bigNumberify(1)])
//   })

//   it('getAmountsIn', async () => {
//     await token0.approve(router.address, MaxUint256)
//     await token1.approve(router.address, MaxUint256)
//     await router.addLiquidity(
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

//     await expect(router.getAmountsIn(bigNumberify(1), [token0.address])).to.be.revertedWith(
//       'UniswapV2Library: INVALID_PATH'
//     )
//     const path = [token0.address, token1.address]
//     expect(await router.getAmountsIn(bigNumberify(1), path)).to.deep.eq([bigNumberify(2), bigNumberify(1)])
//   })
// })

