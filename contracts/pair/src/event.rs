//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Address, Env};

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
    new_reserve_1: i128,
) {
    let event: DepositEvent = DepositEvent {
        to: to,
        amount_0: amount_0,
        amount_1: amount_1,
        liquidity: liquidity,
        new_reserve_0: new_reserve_0,
        new_reserve_1: new_reserve_1,
    };
    e.events()
        .publish(("SoroswapPair", symbol_short!("deposit")), event);
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
    e.events()
        .publish(("SoroswapPair", symbol_short!("swap")), event);
}

// WITHDRAW EVENT

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

pub(crate) fn withdraw(
    e: &Env,
    to: Address,
    liquidity: i128,
    amount_0: i128,
    amount_1: i128,
    new_reserve_0: i128,
    new_reserve_1: i128,
) {
    let event: WithdrawEvent = WithdrawEvent {
        to: to,
        liquidity: liquidity,
        amount_0: amount_0,
        amount_1: amount_1,
        new_reserve_0: new_reserve_0,
        new_reserve_1: new_reserve_1,
    };
    e.events()
        .publish(("SoroswapPair", symbol_short!("withdraw")), event);
}

// SYNC EVENT

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyncEvent {
    pub new_reserve_0: i128,
    pub new_reserve_1: i128,
}

pub(crate) fn sync(e: &Env, new_reserve_0: i128, new_reserve_1: i128) {
    let event: SyncEvent = SyncEvent {
        new_reserve_0: new_reserve_0,
        new_reserve_1: new_reserve_1,
    };
    e.events()
        .publish(("SoroswapPair", symbol_short!("sync")), event);
}

// SKIM EVENT

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkimEvent {
    pub skimmed_0: i128,
    pub skimmed_1: i128,
}

pub(crate) fn skim(e: &Env, skimmed_0: i128, skimmed_1: i128) {
    let event: SkimEvent = SkimEvent {
        skimmed_0: skimmed_0,
        skimmed_1: skimmed_1,
    };
    e.events()
        .publish(("SoroswapPair", symbol_short!("skim")), event);
}
