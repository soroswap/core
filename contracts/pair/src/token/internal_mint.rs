
use crate::token::balance::{receive_balance};
use crate::token::storage_types::INSTANCE_BUMP_AMOUNT;
use crate::token::event;

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
pub fn internal_mint(e: Env, to: Address, amount: i128) {
    check_nonnegative_amount(amount);

    e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

    receive_balance(&e, to.clone(), amount);
    event::mint(&e, e.current_contract_address(), to, amount);
}