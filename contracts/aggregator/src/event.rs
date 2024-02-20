//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env, Address, Vec};
use crate::models::{ProtocolAddressPair, DexDistribution};

// INITIALIZED
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {
    pub state: bool,
    pub protocol_addresses: Vec<ProtocolAddressPair>
}

pub(crate) fn initialized(e: &Env, state: bool, protocol_addresses: Vec<ProtocolAddressPair>) {
    
    let event: InitializedEvent = InitializedEvent {
        state: state,
        protocol_addresses,
    };
    e.events().publish(("SoroswapAggregator", symbol_short!("init")), event);
}

// SWAP EVENT
#[contracttype] 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapEvent {
    pub amount_in: i128,
    pub distribution: Vec<DexDistribution>,
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
    amount_in: i128,
    distribution: Vec<DexDistribution>,
    to: Address
) {
    let event = SwapEvent {
        amount_in,
        distribution,
        to,
    };

    e.events().publish(("SoroswapAggregator", symbol_short!("swap")), event);
}
// UPDATE PROTOCOL EVENT
#[contracttype] 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateProtocolsEvent {
    pub protocol_addresses: Vec<ProtocolAddressPair>
}

/// Publishes an `UpdateProtocolsEvent` to the event stream.
pub(crate) fn protocols_updated(
    e: &Env,
    protocol_addresses: Vec<ProtocolAddressPair>
) {
    let event = UpdateProtocolsEvent {
        protocol_addresses,
    };

    e.events().publish(("SoroswapAggregator", symbol_short!("update")), event);
}