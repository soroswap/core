use soroban_sdk::{Address, Vec, vec, testutils::Address as _};

use crate::error::CombinedAggregatorError;
use crate::test::{SoroswapAggregatorTest, create_protocols_addresses};
use crate::models::{ProtocolAddressPair};

#[test]
fn test_get_protocols() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_protocols_addresses(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let result = test.aggregator_contract.get_protocols();

    assert_eq!(result, initialize_aggregator_addresses);
}

#[test]
fn test_get_protocols_not_yet_initialized() {
    let test = SoroswapAggregatorTest::setup();

    //Update aggregator
    let update_aggregator_addresses = create_protocols_addresses(&test);
    let result = test.aggregator_contract.try_update_protocols(&update_aggregator_addresses);

    assert_eq!(result, Err(Ok(CombinedAggregatorError::AggregatorNotInitialized)));
}