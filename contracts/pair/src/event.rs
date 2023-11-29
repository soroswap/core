//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env, Symbol, Address};
const PAIR: Symbol = symbol_short!("PAIR");

// DEPOSIT EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositEvent {
    pub to: Address,
    pub amount_0: i128,
    pub amount_1: i128,
    pub liquidity: i128,
    pub new_reserve_0: i128,
    pub new_reserve_1: i128,
}

pub(crate) fn deposit(
    e: &Env, 
    to: Address,
    amount_0: i128,
    amount_1: i128,
    liquidity: i128,
    new_reserve_0: i128,
    new_reserve_1: i128) {
    
    let event: DepositEvent = DepositEvent {
        to: to,
        amount_0: amount_0,
        amount_1: amount_1,
        liquidity: liquidity,
        new_reserve_0: new_reserve_0,
        new_reserve_1: new_reserve_1
    };
    e.events().publish(("SoroswapPair", symbol_short!("deposit")), event);
}


// SWAP EVENT

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapEvent {
    pub to: Address,
    pub amount_0_in: i128,
    pub amount_1_in: i128,
    pub amount_0_out: i128,
    pub amount_1_out: i128,
}

pub(crate) fn swap(
    e: &Env,
    to: Address,
    amount_0_in: i128,
    amount_1_in: i128,
    amount_0_out: i128,
    amount_1_out: i128,
) {
    let event: SwapEvent = SwapEvent {
        to: to,
        amount_0_in: amount_0_in,
        amount_1_in: amount_1_in,
        amount_0_out: amount_0_out,
        amount_1_out: amount_1_out,
    };
    e.events().publish(("SoroswapPair", symbol_short!("swap")), event);
}


 
pub(crate) fn withdraw(
    e: &Env,
    sender: &Address,
    shares_burnt: i128,
    amount_0: i128,
    amount_1: i128,
    to: &Address
) {
    let topics = (PAIR, Symbol::new(e, "withdraw"), sender);
    e.events().publish(topics, (shares_burnt, amount_0, amount_1, to));
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawEvent {
    pub to: Address,
    pub liquidity: i128,
    pub amount_0: i128,
    pub amount_1: i128,
    pub new_reserve_0: i128,
    pub new_reserve_1: i128,
}




// event Sync(uint112 reserve0, uint112 reserve1);
pub(crate) fn sync(e: &Env, reserve_0: u64, reserve_1: u64) {
    let topics = (PAIR, Symbol::new(e, "sync"));
    e.events().publish(topics, (reserve_0, reserve_1));
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyncEvent {
    pub new_reserve_0: i128,
    pub new_reserve_1: i128
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkimEvent {
    pub skimmed_0: i128,
    pub skimmed_1: i128
}