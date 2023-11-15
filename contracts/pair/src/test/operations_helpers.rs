// Operation tests using test helpers traits and Error handling.


use helpers::*;

#[test]
fn pair_initialization() {
    let env: Env = Default::default();
    let alice: Address = Address::random(&env);
    let factory_client = SoroswapClient::<FactoryClient>::from(&env, &alice);
    let contract_address = test_client.address().clone();
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    let invoke = MockAuthInvoke {
        contract: &contract_address,
        fn_name: "initialize",
        args: (alice.clone(), pair_hash.clone(),).into_val(&env),
        sub_invokes: &[],
    };
    let mock_auth = MockAuth {
        address: &alice,
        invoke: &invoke,
    };
    let mock_auths = &[mock_auth];
    let mocked_client = test_client.mock_auth_helper(&env, &alice, mock_auths);
    assert_eq!(test_client.address(), mocked_client.address());
    let factory_api = SoroswapTest::<FactoryClient, SoroswapClient<FactoryClient>>::initialize(&env, &alice, &mocked_client, mock_auths);
    factory_api.test_client.client().initialize(&alice.clone(), &pair_hash.clone());
    let pair_address = factory_api.create_a_pair();

}