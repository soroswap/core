use soroban_sdk::{ Env, Address, ConversionError,
    TryFromVal, Val}; 

#[derive(Clone, Copy)] 
#[repr(u32)]

pub enum DataKey {
    Token0 = 0, // address public token0;
    Token1 = 1, // address public token1;
    Reserve0 = 2, //uint112 private reserve0;
    Reserve1 = 3, // uint112 private reserve1;
    Factory = 4, 
    TotalShares = 5, // TODO: Delete when implementing the token interface,
    BlockTimestampLast = 6, // accessible via getReserves,
    Price0CumulativeLast = 7, // uint public price0CumulativeLast;
    Price1CumulativeLast = 8, // uint public price1CumulativeLast;
    KLast = 9

}


impl TryFromVal<Env, DataKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
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

pub fn get_total_shares(e: &Env) -> i128 {
    e.storage().instance().
get(&DataKey::TotalShares).unwrap()
}

pub fn get_reserve_0(e: &Env) -> i128 {
    e.storage().instance().
get(&DataKey::Reserve0).unwrap()
}

pub fn get_reserve_1(e: &Env) -> i128 {
    e.storage().instance().
get(&DataKey::Reserve1).unwrap()
}

pub fn get_block_timestamp_last(e: &Env) -> u64 {
    if let Some(block_timestamp_last) = e.storage().instance().
get(&DataKey::BlockTimestampLast) {
        block_timestamp_last
    } else {
        0
    }
}

pub fn get_price_0_cumulative_last(e: &Env) -> u128 {
    if let Some(price) = e.storage().instance().
get(&DataKey::Price0CumulativeLast) {
        price
    } else {
        0
    }
}

pub fn get_price_1_cumulative_last(e: &Env) -> u128 {
    if let Some(price) = e.storage().instance().
get(&DataKey::Price1CumulativeLast) {
        price
    } else {
        0
    }
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

pub fn put_total_shares(e: &Env, amount: i128) {
    e.storage().instance().
set(&DataKey::TotalShares, &amount)
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

pub fn put_block_timestamp_last(e: &Env, block_timestamp_last: u64) {
    e.storage().instance().
set(&DataKey::BlockTimestampLast, &block_timestamp_last);
}

pub fn put_price_0_cumulative_last(e: &Env, price_0_cumulative_last: u128) {
    e.storage().instance().
set(&DataKey::Price0CumulativeLast, &price_0_cumulative_last);
}

pub fn put_price_1_cumulative_last(e: &Env, price_1_cumulative_last: u128) {
    e.storage().instance().
set(&DataKey::Price1CumulativeLast, &price_1_cumulative_last);
}

pub fn put_klast(e: &Env, klast: i128) {
    e.storage().instance().
set(&DataKey::KLast, &klast);
}