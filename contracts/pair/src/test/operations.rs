use soroban_sdk::{contracttype, xdr::ToXdr, Address, Bytes, BytesN, Env};

mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
mod pair {
    soroban_sdk::contractimport!(file = "./target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm");
}

use soroban_sdk::testutils::Address as _;
use crate::{
    SoroswapPair, 
    SoroswapPairClient,
};
use token::TokenClient;


#[contracttype]
#[derive(Clone)]
pub struct Pair(Address, Address);
impl Pair {
    pub fn new(a: Address, b: Address) -> Self {
        if a < b {
            Pair(a, b)
        } else {
            Pair(b, a)
        }
    }

    pub fn salt(&self, e: &Env) -> BytesN<32> {
        let mut salt = Bytes::new(e);

        // Append the bytes of token_a and token_b to the salt
        salt.append(&self.0.clone().to_xdr(e)); // can be simplified to salt.append(&self.clone().to_xdr(e)); but changes the hash
        salt.append(&self.1.clone().to_xdr(e));

        // Hash the salt using SHA256 to generate a new BytesN<32> value
        e.crypto().sha256(&salt)
    }

    pub fn token_a(&self) -> &Address {
        &self.0
    }

    pub fn token_b(&self) -> &Address {
        &self.1
    }

    // Define a function to create a new contract instance
    pub fn create_contract(
        /*
            Overall, this function is designed to create a new contract
            instance on the blockchain with the given pair_wasm_hash
            value and a unique salt value generated from the token_a and
            token_b values. The salt value is used to ensure that each
            contract instance is unique and can be identified by its hash value.

            The deployer() method of the Env instance is used to actually
            create and deploy the new contract instance. The function returns
            the hash value of the newly created contract instance as a
            BytesN<32> value.
        */
        &self,
        e: &Env,                     // Pass in the current environment as an argument
        pair_wasm_hash: BytesN<32>, // Pass in the hash of the token contract's WASM file
        // token_pair: &Pair,
    ) -> Address {
        // Return the hash of the newly created contract as a Address value

        // Use the deployer() method of the current environment to create a new contract instance
        let pair_hash = e.deployer().upload_contract_wasm(pair_wasm_hash);
        e.deployer()
            .with_current_contract(self.salt(&e)) // Use the salt as a unique identifier for the new contract instance
            .deploy(pair_hash) // Deploy the new contract instance using the given pair_wasm_hash value
    }
}

#[test]
fn pair_initialization() {
    let env: Env = Default::default();
    env.mock_all_auths();
    let alice = Address::random(&env);
    let mut token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let mut token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let pair_hash = env.deployer().upload_contract_wasm(pair::WASM);
    // let contract = Pair::new(token_0.address, token_1.address).create_contract(&env, pair_hash);
    // let new = SoroswapPairClient::new(&env, &contract);
    // let new = Pair::create_contract(env, pair:WASM);
}

// #[test]
fn double_pair_initialization() {
    let env: Env = Default::default();
    let alice = Address::random(&env);
    let mut token_0 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let mut token_1 = TokenClient::new(&env, &env.register_stellar_asset_contract(alice.clone()));
    let new = Pair::new(token_0.address.clone(), token_1.address.clone());
    let new_inverse = Pair::new(token_1.address, token_0.address);
    assert_eq!((new.token_a(),new.token_b()), (new_inverse.token_a(),new_inverse.token_b()));
}