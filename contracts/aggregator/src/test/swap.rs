use soroban_sdk::{Address, testutils::Address as _};

use crate::error::CombinedAggregatorError;
use crate::test::{SoroswapAggregatorTest, create_protocols_addresses, create_test_distribution};


#[test]
fn test_swap() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let from_token = &test.token_0.address;
    let dest_token = &test.token_1.address;
    let amount = 500_000_000_000_000_000;
    let amount_out_min = 90i128;
    let distribution = create_test_distribution(&test);
    let to = &test.user;
    let deadline = test.env.ledger().timestamp() + 100; // Deadline in the future

    let result = test.aggregator_contract.swap(&from_token, &dest_token, &amount, &amount_out_min, &distribution, &to, &deadline);

    assert_eq!(test.token_0.balance(&test.user), 7_500_000_000_000_000_000);
    assert_eq!(test.token_1.balance(&test.user), 2_000_000_000_000_000_000);
    assert_eq!(test.token_2.balance(&test.user), 5_851_690_580_469_525_867);
}