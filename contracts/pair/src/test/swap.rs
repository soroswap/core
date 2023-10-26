use crate::test::deposit::deposit;
use crate::test::{SoroswapPairTest};


#[test]
fn swap() {
    let test = SoroswapPairTest::setup();
    let amount_0 = 1_000_000_000_000_000_000;
    let amount_1 = 4_000_000_000_000_000_000;
    deposit(&test, &amount_0, &amount_1);
}