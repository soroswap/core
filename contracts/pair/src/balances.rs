use crate::storage::*;
use crate::{any_token, soroswap_pair_token::SoroswapPairToken};
use soroban_sdk::token::Interface;
use soroban_sdk::{Address, Env};

pub fn get_balance(e: &Env, contract_id: Address) -> i128 {
    // How many "contract_id" tokens does this contract holds?
    // We need to implement the token client
    any_token::TokenClient::new(e, &contract_id).balance(&e.current_contract_address())
}

pub fn get_balance_0(e: &Env) -> i128 {
    // How many "A TOKENS" does the Liquidity Pool holds?
    // How many "A TOKENS" does this contract holds?
    get_balance(e, get_token_0(e))
}

pub fn get_balance_1(e: &Env) -> i128 {
    get_balance(e, get_token_1(e))
}

pub fn get_balance_shares(e: &Env) -> i128 {
    // How many "SHARE" tokens does the Liquidity pool holds?
    // This shares should have been sent by the user when burning their LP positions (withdraw)
    SoroswapPairToken::balance(e.clone(), e.current_contract_address())
}
