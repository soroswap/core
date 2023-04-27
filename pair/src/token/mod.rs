#![allow(unused)]
use soroban_sdk::{Bytes, BytesN, Env};

soroban_sdk::contractimport!(
    file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
);