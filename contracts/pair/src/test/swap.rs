use crate::test::deposit::add_liquidity;
use crate::test::{SoroswapPairTest};


#[test]
fn swap() {
    let test = SoroswapPairTest::setup();
    let amount_0: i128 = 5_000_000_000_000_000_000;
    let amount_1: i128 = 10_000_000_000_000_000_000;
    add_liquidity(&test, &amount_0, &amount_1);

    // let amount_swap_0 = 5_000_000_000_000_000_000;
    // let amount_swap_1 = 10_000_000_000_000_000_000;

}