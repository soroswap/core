// Operation tests using test helpers traits and Error handling.

#[cfg(test)]
mod tests {
    extern crate alloc;
    use soroban_sdk::{
        Env,
        Address,
        testutils::{
            MockAuth,
            MockAuthInvoke,
            Address as _,
        },
        IntoVal,
    };
    use crate::{
        test::helpers::{
            SoroswapTest,
            SoroswapClient,
            SoroswapClientTrait,
            token,
            pair::{
                WASM as PAIR_WASM,
            },
            factory::{
                FactoryClient,
                WASM as FACTORY_WASM,
            },
        },
        SoroswapPair, 
        SoroswapPairClient,
    };

    #[test]
    fn initialization() {
        let env: Env = Default::default();
        let alice: Address = Address::random(&env);
        let factory_client = SoroswapClient::<FactoryClient>::from(&env);
        let contract_address = factory_client.address().clone();
        let pair_hash = env.deployer().upload_contract_wasm(PAIR_WASM);
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
        let mut mocked_client = factory_client.mock_auth_helper(&env, &alice, mock_auths);
        let mut factory_api = SoroswapTest::<FactoryClient, SoroswapClient<FactoryClient>>::initialize(&env, &alice, &mut mocked_client, mock_auths);
        let pair_address = factory_api.create_a_pair();
    }

    #[test]
    fn pair_token_addresses_ne() {
        let env: Env = Default::default();
        let alice: Address = Address::random(&env);
        let factory_client = SoroswapClient::<FactoryClient>::from(&env);
        let contract_address = factory_client.address().clone();
        let pair_hash = env.deployer().upload_contract_wasm(PAIR_WASM);
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
        let mut mocked_client = factory_client.mock_auth_helper(&env, &alice, mock_auths);
        let mut factory_api = SoroswapTest::<FactoryClient, SoroswapClient<FactoryClient>>::initialize(&env, &alice, &mut mocked_client, mock_auths);
        let pair_address = factory_api.create_a_pair();
        let pair_client = SoroswapPairClient::new(&env, &pair_address);
        assert_ne!(pair_client.token_0(), pair_client.token_1());
    }

    #[test]
    fn create_2_pair() {
        let env: Env = Default::default();
        let alice: Address = Address::random(&env);
        let factory_client = SoroswapClient::<FactoryClient>::from(&env);
        let contract_address = factory_client.address().clone();
        let pair_hash = env.deployer().upload_contract_wasm(PAIR_WASM);
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
        let mut mocked_client = factory_client.mock_auth_helper(&env, &alice, mock_auths);
        let factory_api = SoroswapTest::<FactoryClient, SoroswapClient<FactoryClient>>::initialize(&env, &alice, &mut mocked_client, mock_auths);
        // let pair_address = factory_api.create_a_pair();
        // let pair_client = SoroswapPairClient::new(&env, &pair_address);
        // assert_ne!(pair_client.token_0(), pair_client.token_1());
        let pair_list: alloc::vec::Vec<Address> = alloc::vec::Vec::new();
        for n in 0..=1 {
            let pair_address = factory_api.create_a_pair();
        }
    }

    #[test]
    #[should_panic]
    fn create_3_pair() {
        let env: Env = Default::default();
        let alice: Address = Address::random(&env);
        let factory_client = SoroswapClient::<FactoryClient>::from(&env);
        let contract_address = factory_client.address().clone();
        let pair_hash = env.deployer().upload_contract_wasm(PAIR_WASM);
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
        let mut mocked_client = factory_client.mock_auth_helper(&env, &alice, mock_auths);
        let factory_api = SoroswapTest::<FactoryClient, SoroswapClient<FactoryClient>>::initialize(&env, &alice, &mut mocked_client, mock_auths);
        // let pair_address = factory_api.create_a_pair();
        // let pair_client = SoroswapPairClient::new(&env, &pair_address);
        // assert_ne!(pair_client.token_0(), pair_client.token_1());
        let pair_list: alloc::vec::Vec<Address> = alloc::vec::Vec::new();
        for n in 0..=2 {
            let pair_address = factory_api.create_a_pair();
        }
    }

    // #[test]
    fn create_1_000_001_pairs() {
        let env: Env = Default::default();
        let alice: Address = Address::random(&env);
        let factory_client = SoroswapClient::<FactoryClient>::from(&env);
        let contract_address = factory_client.address().clone();
        let pair_hash = env.deployer().upload_contract_wasm(PAIR_WASM);
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
        let mut mocked_client = factory_client.mock_auth_helper(&env, &alice, mock_auths);
        let factory_api = SoroswapTest::<FactoryClient, SoroswapClient<FactoryClient>>::initialize(&env, &alice, &mut mocked_client, mock_auths);
        // let pair_address = factory_api.create_a_pair();
        // let pair_client = SoroswapPairClient::new(&env, &pair_address);
        // assert_ne!(pair_client.token_0(), pair_client.token_1());
        let pair_list: alloc::vec::Vec<Address> = alloc::vec::Vec::new();
        for n in 0..1_000_000 {
            let pair_address = factory_api.create_a_pair();
        }
    }
}