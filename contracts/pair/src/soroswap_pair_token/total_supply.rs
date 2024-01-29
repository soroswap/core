use soroban_sdk::{Env};

use crate::soroswap_pair_token::storage_types::DataKey;
use crate::error::SoroswapPairError;

pub fn read_total_supply(e: &Env) -> i128 {
    let key = DataKey::TotalSupply;
    e.storage().instance().get(&key).unwrap_or(0)
}

pub fn write_total_supply(e: &Env, id: &i128) {
    let key = DataKey::TotalSupply;
    e.storage().instance().set(&key, id);
}

pub fn increase_total_supply(e: &Env, amount: i128) -> Result<(), SoroswapPairError> {
    let total_supply = read_total_supply(&e);
    let new_total_supply = total_supply.checked_add(amount)
    //  .expect("Integer overflow occurred while increasing total supply.");
    //  TokenTotalSupplyIncreaseOverflow = 126,
        .ok_or(SoroswapPairError::TokenTotalSupplyIncreaseOverflow)?;

    write_total_supply(&e, &new_total_supply);
    Ok(())
}

pub fn decrease_total_supply(e: &Env, amount: i128) -> Result<(), SoroswapPairError> {
    let total_supply = read_total_supply(&e);
    if total_supply < amount {
        //  panic!("insufficient total supply");
        //  TokenTotalSupplyInsufficient = 127,
        return Err(SoroswapPairError::TokenTotalSupplyInsufficient);
    }
    let new_total_supply = total_supply.checked_sub(amount)
        //  .expect("Integer underflow occurred while decreasing total supply.");
        //  TokenTotalSupplyDecreaseUnderflow = 128,
        .ok_or(SoroswapPairError::TokenTotalSupplyDecreaseUnderflow)?;

    write_total_supply(&e, &new_total_supply);
    Ok(())
}