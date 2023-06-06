
use soroban_sdk::{contractimpl, Env, Symbol, Address};

   
const DUMMY: Symbol = Symbol::short("DUMMY");


pub trait FactoryTrait {

    fn fee_to(e: Env) -> Address;
    fn fee_on(e: Env) -> bool;
}

pub struct Factory;

#[contractimpl]
impl FactoryTrait for Factory {

    fn fee_to(e: Env) -> Address {
        e.storage().get(&DUMMY).unwrap().unwrap()
    }

    fn fee_on(e: Env) -> bool {
        true
    }
}
