//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env, Address, Vec};

// INITIALIZED
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {
    pub state: bool
}

pub(crate) fn initialized(e: &Env, state: bool) {
    
    let event: InitializedEvent = InitializedEvent {
        state: state
    };
    e.events().publish(("SoroswapAggregator", symbol_short!("init")), event);
}

// SWAP EVENT
#[contracttype] 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapEvent {
    pub path: Vec<Address>,
    pub amounts: Vec<i128>,
    pub to: Address
}

/// Publishes an `SwapEvent` to the event stream.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// * `path` - A vector representing the trading route, where the first element is the input token 
///            and the last is the output token. Intermediate elements represent pairs to trade through.
/// * `amounts` - A vector containing the amounts of tokens traded at each step of the trading route.
/// * `to` - The address where the output tokens will be sent to.
pub(crate) fn swap(
    e: &Env,
    path: Vec<Address>,
    amounts: Vec<i128>,
    to: Address
) {
    let event = SwapEvent {
        path,
        amounts,
        to,
    };

    e.events().publish(("SoroswapAggregator", symbol_short!("swap")), event);
}