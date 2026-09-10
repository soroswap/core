use crate::test::pair::SoroswapPairError;
use crate::test::SoroswapPairTest;
use soroban_sdk::String;

#[test]
// #[should_panic(expected = "SoroswapPair: token_0 must be less than token_1")]
fn initialize_token_1_less_than_token_0() {
    let test = SoroswapPairTest::setup();
    let res = test.contract.try_initialize(
        &test.factory.address,
        &test.token_1.address,
        &test.token_0.address,
    );
    assert_eq!(res, Err(Ok(SoroswapPairError::InitializeTokenOrderInvalid)));
}

#[test]
// #[should_panic(expected = "SoroswapPair: already initialized")]
fn double_initialize() {
    let test = SoroswapPairTest::setup();
    test.contract.initialize(
        &test.factory.address,
        &test.token_0.address,
        &test.token_1.address,
    );
    let res = test.contract.try_initialize(
        &test.factory.address,
        &test.token_0.address,
        &test.token_1.address,
    );
    assert_eq!(
        res,
        Err(Ok(SoroswapPairError::InitializeAlreadyInitialized))
    );
}

#[test]
fn initialize_initial_values_0() {
    let test = SoroswapPairTest::setup();
    assert_eq!(test.factory.fee_to(), test.admin);
    assert_eq!(test.factory.fee_to_setter(), test.admin);
    assert_eq!(test.factory.fees_enabled(), false);

    assert_eq!(test.token_0.symbol(), String::from_str(&test.env, "TOK0"));
    assert_eq!(
        test.token_1.symbol(),
        String::from_str(&test.env, "ABCDEFGHIJ")
    );
    assert_eq!(test.token_0.name(), String::from_str(&test.env, "Token 0"));
    assert_eq!(test.token_1.name(), String::from_str(&test.env, "Token 1"));

    // Test liqpool initial values:
    test.contract.initialize(
        &test.factory.address,
        &test.token_0.address,
        &test.token_1.address,
    );
    assert_eq!(test.contract.token_0(), test.token_0.address);
    assert_eq!(test.contract.token_1(), test.token_1.address);
    assert_eq!(test.contract.factory(), test.factory.address);
    assert_eq!(test.contract.get_reserves(), (0, 0));
    assert_eq!(test.contract.k_last(), 0);
    assert_eq!(test.contract.total_supply(), 0);
    assert_eq!(test.contract.k_last(), 0);

    assert_eq!(
        test.contract.symbol(),
        String::from_str(&test.env, "TOK0-ABCDEF-SOROSWAP-LP")
    );
    assert_eq!(
        test.contract.name(),
        String::from_str(&test.env, "TOK0-ABCDEF Soroswap LP Token")
    );
    assert_eq!(test.contract.decimals(), 7);
}
