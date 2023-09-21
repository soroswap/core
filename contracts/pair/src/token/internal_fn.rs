use crate::token::balance::{spend_balance, receive_balance};
use crate::token::storage_types::{INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use soroban_token_sdk::TokenUtils;


use soroban_sdk::{Address, Env};

fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

/*
    Function created to bypass the admin.require_auth()
    Because this contract is the token admin for itself,
    it cannot make a cross_contract call to itself and hence 
*/
pub fn internal_burn(e: Env, from: Address, amount: i128) {
    check_nonnegative_amount(amount);

    e.storage().instance().bump(INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD);

    spend_balance(&e, from.clone(), amount);
    TokenUtils::new(&e).events().burn(from, amount);

} 

/*
    Function created to bypass the admin.require_auth()
    Because this contract is the token admin for itself,
    it cannot make a cross_contract call to itself and hence 
*/
pub fn internal_mint(e: Env, to: Address, amount: i128) {
    check_nonnegative_amount(amount);

    e.storage().instance().bump(INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD);

    receive_balance(&e, to.clone(), amount);
    TokenUtils::new(&e).events().mint(e.current_contract_address(), to, amount);

}

pub fn internal_transfer(e: Env, from: Address, to: Address, amount: i128) {

    check_nonnegative_amount(amount);

    e.storage().instance().bump(INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD);

    spend_balance(&e, from.clone(), amount);
    receive_balance(&e, to.clone(), amount);
    TokenUtils::new(&e).events().transfer(from, to, amount);

}