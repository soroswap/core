use soroban_sdk::{Address, Vec, vec, testutils::Address as _};

use crate::error::CombinedAggregatorError;
use crate::test::{SoroswapAggregatorTest, create_protocols_addresses, create_only_soroswap_protocol_address, create_only_phoenix_protocol_address};
use crate::models::{DexDistribution};
use crate::{dex_constants};

// Helpers functions to create a simple distribution vector for testing
fn create_soroswap_distribution(test: &SoroswapAggregatorTest) -> Vec<DexDistribution> {
    vec![&test.env,
        DexDistribution {
            index: dex_constants::SOROSWAP,
            path: vec![&test.env, test.token_0.address.clone(), test.token_1.address.clone(), test.token_2.address.clone()],
            parts: 3,
        },
        DexDistribution {
            index: dex_constants::SOROSWAP,
            path: vec![&test.env, test.token_0.address.clone(), test.token_2.address.clone()],
            parts: 2,
        },
    ]
}

fn create_phoenix_distribution(test: &SoroswapAggregatorTest) -> Vec<DexDistribution> {
    vec![&test.env,
        DexDistribution {
            index: dex_constants::PHOENIX,
            path: vec![&test.env, test.token_0.address.clone(), test.token_1.address.clone()],
            parts: 3,
        },
    ]
}

#[test]
fn test_soroswap_swap() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_only_soroswap_protocol_address(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let from_token = &test.token_0.address;
    let dest_token = &test.token_1.address;
    let amount = 500_000_000_000_000_000;
    let amount_out_min = 90i128;
    let distribution = create_soroswap_distribution(&test);
    let to = &test.user;
    let deadline = test.env.ledger().timestamp() + 100; // Deadline in the future

    let result = test.aggregator_contract.swap(&from_token, &dest_token, &amount, &amount_out_min, &distribution, &to, &deadline);

    assert_eq!(test.token_0.balance(&test.user), 7_500_000_000_000_000_000);
    assert_eq!(test.token_1.balance(&test.user), 2_000_000_000_000_000_000);
    assert_eq!(test.token_2.balance(&test.user), 5_851_690_580_469_525_867);
}

#[test]
fn test_soroswap_swap_no_protocol_address() {
    let test = SoroswapAggregatorTest::setup();

    //Initialize aggregator
    let initialize_aggregator_addresses = create_only_phoenix_protocol_address(&test);
    test.aggregator_contract.initialize(&test.admin, &initialize_aggregator_addresses);

    let from_token = &test.token_0.address;
    let dest_token = &test.token_1.address;
    let amount = 500_000_000_000_000_000;
    let amount_out_min = 90i128;
    let distribution = create_soroswap_distribution(&test);
    let to = &test.user;
    let deadline = test.env.ledger().timestamp() + 100; // Deadline in the future

    let result = test.aggregator_contract.try_swap(&from_token, &dest_token, &amount, &amount_out_min, &distribution, &to, &deadline);

    assert_eq!(
        result,
        (Err(Ok(CombinedAggregatorError::AggregatorProtocolAddressNotFound)))
    );
}