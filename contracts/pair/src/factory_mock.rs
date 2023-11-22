
use soroban_sdk::{symbol_short, contract, contractimpl, Env, Symbol, Address};

   
const DUMMY: Symbol = symbol_short!("DUMMY");


pub trait FactoryMockTrait {

    fn fee_to(e: Env) -> Address;
    fn fees_enabled(e: Env) -> bool;
}

#[contract]
pub struct FactoryMock;

#[contractimpl]
impl FactoryMockTrait for FactoryMock {

    fn fee_to(e: Env) -> Address {
        let address: Address = e.storage().instance().get(&DUMMY).unwrap();
        address
    }

    fn fees_enabled(_e: Env) -> bool {
        true
    }
}
