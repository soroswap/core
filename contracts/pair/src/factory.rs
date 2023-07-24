
use soroban_sdk::{contract, contractimpl, Env, Symbol, Address};

   
const DUMMY: Symbol = Symbol::short("DUMMY");


pub trait FactoryTrait {

    fn fee_to(e: Env) -> Address;
    fn fees_enabled(e: Env) -> bool;
}

#[contract]
pub struct Factory;

#[contractimpl]
impl FactoryTrait for Factory {

    fn fee_to(e: Env) -> Address {
        e.storage().get(&DUMMY).unwrap().unwrap()
    }

    fn fees_enabled(_e: Env) -> bool {
        true
    }
}
