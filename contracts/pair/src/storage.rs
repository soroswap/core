use soroban_sdk::{ Env, Address, ConversionError,
    TryFromVal, Val}; 

#[derive(Clone, Copy)] 
#[repr(u32)]

pub enum DataKey {
    Token0 = 0, // token0, instance type of data;
    Token1 = 1, // token1, instance type of data;
    Reserve0 = 2, // reserve0, instance type of data;
    Reserve1 = 3, // reserve1, instance type of data;
    Factory = 4, // factory, instance type of data;
    KLast = 5 // last k, instance type of data;

}

// We will follow the token standar for instance bumping

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

impl TryFromVal<Env, DataKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn get_factory(e: &Env) -> Address {
    e.storage().instance().
get(&DataKey::Factory).unwrap()
}

// Helper function in order to know if the contract has been initialized or not
pub fn has_token_0(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Token0)
}

pub fn get_token_0(e: &Env) -> Address {
    e.storage().instance().
get(&DataKey::Token0).unwrap()
}

pub fn get_token_1(e: &Env) -> Address {
    e.storage().instance().
get(&DataKey::Token1).unwrap()
}

pub fn get_reserve_0(e: &Env) -> i128 {
    e.storage().instance().
get(&DataKey::Reserve0).unwrap()
}

pub fn get_reserve_1(e: &Env) -> i128 {
    e.storage().instance().
get(&DataKey::Reserve1).unwrap()
}



pub fn get_klast(e: &Env) -> i128 {
    if let Some(klast) = e.storage().instance().
get(&DataKey::KLast) {
        klast
    } else {
        0
    }
}

pub fn put_factory(e: &Env, factory: Address) {
    e.storage().instance().
set(&DataKey::Factory, &factory);
}

pub fn put_token_0(e: &Env, contract_id: Address) {
    e.storage().instance().
set(&DataKey::Token0, &contract_id);
}

pub fn put_token_1(e: &Env, contract_id: Address) {
    e.storage().instance().
set(&DataKey::Token1, &contract_id);
}

pub fn put_reserve_0(e: &Env, amount: i128) {
    if amount < 0 {
        panic!("put_reserve_0: amount cannot be negative")
    }
    e.storage().instance().
set(&DataKey::Reserve0, &amount)
}

pub fn put_reserve_1(e: &Env, amount: i128) {
    if amount < 0 {
        panic!("put_reserve_1: amount cannot be negative")
    }
    e.storage().instance().
set(&DataKey::Reserve1, &amount)
}


pub fn put_klast(e: &Env, klast: i128) {
    e.storage().instance().
set(&DataKey::KLast, &klast);
}