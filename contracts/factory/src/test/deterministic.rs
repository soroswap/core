mod deterministic {
    use soroban_sdk::{testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    xdr::ToXdr,
    Address, 
    BytesN, 
    Env,
    Bytes,
    IntoVal,
    Symbol};
    use crate::{pair, SoroswapFactoryClient};

    soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;

    #[test]
    pub fn token_client_ne() {
        let e: Env = Default::default();
        e.mock_all_auths();

        let admin = Address::random(&e);
        let mut token_0 = TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()));
        let mut token_1 = TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()));

        assert_ne!(token_0.address, token_1.address);
    }

    #[test]
    pub fn create_factory_contract() {
        soroban_sdk::contractimport!(
            file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
        );
        let e: Env = Default::default();
        let pair_token_wasm_binding = e.deployer().upload_contract_wasm(WASM);
        let admin = Address::random(&e);
        let factory = SoroswapFactoryClient::new(&e, &e.register_contract(None, crate::SoroswapFactory {}));
        
        factory.initialize(&admin, &pair_token_wasm_binding);
    }

    #[test]
    pub fn compare_address() {
        // Create two tokens in order to create a pair using the factory
        // let mut token_0 = create_token_contract(&e, &admin);
        // let mut token_1 = create_token_contract(&e, &admin);
        // let pair_expected_address = guess_contract_address( &e,
        //     &factory.address, 
        //     &token_1.address, 
        //     &token_0.address);
        // let pair_address = factory.get_pair(&token_0.address, &token_1.address);
        // assert_eq!(&pair_expected_address, &pair_address);
    }


    pub fn guess_contract_address(
        e: &Env,
        factory: &Address,
        token_a: &Address,
        token_b: &Address,
    ) -> BytesN<32> {
        let token_0;
        let token_1;
        if token_a < token_b {
            token_0 = token_a;
            token_1 = token_b;
        }
        else {
            token_0 = token_b;
            token_1 = token_a;
        }
        let mut salt = Bytes::new(e);
        salt.append(&factory.to_xdr(e));
        salt.append(&token_0.to_xdr(e));
        salt.append(&token_1.to_xdr(e));
        let salt_hash = e.crypto().sha256(&salt);
        // let contract_address = Address::try_from(&salt_hash.as_ref()[12..]);
        // contract_address.unwrap_or_else(|_| BytesN::zero())
        salt_hash
    }
}