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
            token::{
                TokenClient,
            },
            pair::{
                WASM as PAIR_WASM,
            },
            factory::{
                FactoryClient,
                WASM as FACTORY_WASM,
            },
        },
        SoroswapPair, 
        SoroswapPairClient as PairClient,
    };

    #[test]
    fn initialization() {
        let env: Env = Default::default();
        let alice: Address = Address::generate(&env);
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
        let alice: Address = Address::generate(&env);
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
        let pair_client = PairClient::new(&env, &pair_address);
        assert_ne!(pair_client.token_0(), pair_client.token_1());
    }

    #[test]
    fn create_2_pair() {
        let env: Env = Default::default();
        let alice: Address = Address::generate(&env);
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
        // let pair_client = PairClient::new(&env, &pair_address);
        // assert_ne!(pair_client.token_0(), pair_client.token_1());
        let pair_list: alloc::vec::Vec<Address> = alloc::vec::Vec::new();
        for _ in 0..=1 {
            let pair_address = factory_api.create_a_pair();
        }
    }

    #[test]
    fn create_2_pair_two_times() {
        let env: Env = Default::default();
        let alice: Address = Address::generate(&env);
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
        // let pair_client = PairClient::new(&env, &pair_address);
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
        // The variables in this scope outlives any consecutive invocation or contract state trasnformation
        // for this specific test unless explicitly dropped. Variables referenced in MockAuths needs to outlive their call,
        // which is why they are created in the begginings of the isolated test.
        let alice: Address = Address::generate(&env);
        let factory_client = SoroswapClient::<FactoryClient>::from(&env);
        let pair_hash = env.deployer().upload_contract_wasm(PAIR_WASM);
        let fn_name = "initialize";
        let sub_invokes = &[];
        let args = (alice.clone(), pair_hash.clone(),).into_val(&env);
        let invoke = SoroswapClient::<FactoryClient>::generate_mock_auth_invoke(
            &alice,
            factory_client.address(),
            fn_name,
            args,
            sub_invokes,
        );
        let mock_auth = MockAuth {
            address: &alice,
            invoke: &invoke,
        };
        let mock_auths = &[mock_auth];
        let mut mocked_client = factory_client.mock_auth_helper(&env, &alice, mock_auths);
        let factory_api = SoroswapTest::<FactoryClient, SoroswapClient<FactoryClient>>::initialize(&env, &alice, &mut mocked_client, mock_auths);
        let mut pair_list: alloc::vec::Vec<Address> = alloc::vec::Vec::new();
        for n in 0..=2 {
            pair_list.push(factory_api.create_a_pair());
        }
    }

    // #[test]
    fn create_1_000_001_pairs() {
        let env: Env = Default::default();
        let alice: Address = Address::generate(&env);
        let factory_client = SoroswapClient::<FactoryClient>::from(&env);
        let contract_address = factory_client.address().clone();
        let pair_hash = env.deployer().upload_contract_wasm(PAIR_WASM);
        let fn_name = "initialize";
        let sub_invokes = &[];
        let args = (alice.clone(), pair_hash.clone(),).into_val(&env);
        let invoke = SoroswapClient::<FactoryClient>::generate_mock_auth_invoke(
            &alice,
            factory_client.address(),
            fn_name,
            args,
            sub_invokes,
        );
        let mock_auth = MockAuth {
            address: &alice,
            invoke: &invoke,
        };
        let mock_auths = &[mock_auth];
        let mut mocked_client = factory_client.mock_auth_helper(&env, &alice, mock_auths);
        let factory_api = SoroswapTest::<FactoryClient, SoroswapClient<FactoryClient>>::initialize(&env, &alice, &mut mocked_client, mock_auths);
        // let pair_address = factory_api.create_a_pair();
        // let pair_client = PairClient::new(&env, &pair_address);
        // assert_ne!(pair_client.token_0(), pair_client.token_1());
        let mut pair_list: alloc::vec::Vec<Address> = alloc::vec::Vec::new();
        for n in 0..=1_000_000 {
            pair_list.push(factory_api.create_a_pair());
        }
    }

    #[test]
    fn mocked_mint() {
        let env: Env = Default::default();
        let alice: Address = Address::generate(&env);
        let factory_client = SoroswapClient::<FactoryClient>::from(&env);
        let pair_hash = env.deployer().upload_contract_wasm(PAIR_WASM);
        let fn_name = "initialize";
        let sub_invokes = &[];
        let args = (alice.clone(), pair_hash.clone(),).into_val(&env);
        let invoke = SoroswapClient::<FactoryClient>::generate_mock_auth_invoke(
            factory_client.address(),
            &alice,
            fn_name,
            args,
            sub_invokes,
        );
        let mock_auth = MockAuth {
            address: &alice,
            invoke: &invoke,
        };
        let mock_auths = &[mock_auth];
        let mut mocked_client = factory_client.mock_auth_helper(&env, &alice, mock_auths);
        let factory_api = SoroswapTest::<FactoryClient, SoroswapClient<FactoryClient>>::initialize(&env, &alice, &mut mocked_client, mock_auths);
        let pair_address = factory_api.create_a_pair();
        // let mut pairs_list: alloc::vec::Vec<Address> = alloc::vec::Vec::new();
        // for _ in 0..1 { // : pair UZI
        //     pairs_list.push(factory_api.create_a_pair());
        // }
        let pair_client = SoroswapClient::<PairClient>::from(&env, &pair_address);
        let token_0 = pair_client.token_0();
        let token_1 = pair_client.token_1();
        let token_0_client = SoroswapClient::<TokenClient>::from_token_address(&env, &token_0);
        let token_1_client = SoroswapClient::<TokenClient>::from_token_address(&env, &token_1);

        let bob = Address::generate(&env);
        // invoke declaration for token_0
        let amount: i128 = 10_000_000_000;
        let token_0_sub_invokes = &[];
        let token_0_invoke = SoroswapClient::<FactoryClient>::generate_mock_auth_invoke(
            &token_0,
            &alice,
            "mint",
            (alice.clone(), amount,).into_val(&env),
            token_0_sub_invokes,
        );
        let mock_auth_token_0 = MockAuth {
            address: &alice,
            invoke: &token_0_invoke,
        };
        let mock_auths_token_0 = &[mock_auth_token_0];
        // mint for token_0 of the pair
        let mocked_client_0 = token_0_client.mock_auth_helper(&env, &alice, mock_auths_token_0);
        mocked_client_0.mint(&alice, &alice, &amount, Some(mock_auths_token_0));
        let balance_0 = mocked_client_0.balance(&alice);
        assert_eq!(balance_0, amount);

        // invoke declaration for token_1
        let amount_1: i128 = 10_000_000_000;
        let token_1_sub_invokes = &[];
        let token_1_invoke = SoroswapClient::<FactoryClient>::generate_mock_auth_invoke(
            &token_1,
            &alice,
            "mint",
            (alice.clone(), amount_1,).into_val(&env),
            token_1_sub_invokes,
        );
        let mock_auth_token_1 = MockAuth {
            address: &alice,
            invoke: &token_1_invoke,
        };
        let mock_auths_token_1 = &[mock_auth_token_1];
        // mint for token_1 of the pair
        let mocked_client_1 = token_1_client.mock_auth_helper(&env, &alice, mock_auths_token_1);
        mocked_client_1.mint(&alice, &alice, &amount_1, Some(mock_auths_token_1));
        let balance_1 = mocked_client_1.balance(&alice);
        assert_eq!(balance_1, amount_1);

        // factory_api.create_a_pair();
        // let mut pair_list: alloc::vec::Vec<Address> = alloc::vec::Vec::new();
        // for _ in 0..=1 {
            // pair_list.push(factory_api.create_a_pair());
        // }
    }
}