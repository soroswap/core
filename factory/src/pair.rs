
// Import necessary types from the Soroban SDK
#![allow(unused)]
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env};

// Import the Soroban Token contract from its WASM file
soroban_sdk::contractimport!(
    file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
);

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

    e: &Env, // Pass in the current environment as an argument
    pair_wasm_hash: &BytesN<32>, // Pass in the hash of the token contract's WASM file
    token_a: &Address, // Pass in the hash of the first token
    token_b: &Address, // Pass in the hash of the second token
) -> Address { // Return the hash of the newly created contract as a Address value

    // Create a new Bytes instance using the current environment
    let mut salt = Bytes::new(e);
    
    // Append the bytes of token_a and token_b to the salt
    salt.append(&token_a.to_xdr(e));
    salt.append(&token_b.to_xdr(e));

    // Hash the salt using SHA256 to generate a new BytesN<32> value
    let salt = e.crypto().sha256(&salt);

    // Use the deployer() method of the current environment to create a new contract instance
    e.deployer()
        .with_current_contract(&salt) // Use the salt as a unique identifier for the new contract instance
        .deploy(pair_wasm_hash) // Deploy the new contract instance using the given pair_wasm_hash value
}