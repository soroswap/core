//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Address, Env};

// INITIALIZED
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {
    pub setter: Address,
}

pub(crate) fn initialized(e: &Env, setter: Address) {
    let event: InitializedEvent = InitializedEvent { setter: setter };
    e.events()
        .publish(("SoroswapFactory", symbol_short!("init")), event);
}

// NEW PAIR CREATED EVENT: new_pair
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewPairEvent {
    pub token_0: Address,
    pub token_1: Address,
    pub pair: Address,
    pub new_pairs_length: u32,
}

pub(crate) fn new_pair(
    e: &Env,
    token_0: Address,
    token_1: Address,
    pair: Address,
    new_pairs_length: u32,
) {
    let event: NewPairEvent = NewPairEvent {
        token_0: token_0,
        token_1: token_1,
        pair: pair,
        new_pairs_length: new_pairs_length,
    };
    e.events()
        .publish(("SoroswapFactory", symbol_short!("new_pair")), event);
}

// NEW "FEE TO" SETTED: new_fee_to // Event is "fee_to"
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeeToSettedEvent {
    pub setter: Address,
    pub old: Address,
    pub new: Address,
}

pub(crate) fn new_fee_to(e: &Env, setter: Address, old: Address, new: Address) {
    let event: FeeToSettedEvent = FeeToSettedEvent {
        setter: setter,
        old: old,
        new: new,
    };
    e.events()
        .publish(("SoroswapFactory", symbol_short!("fee_to")), event);
}

// NEW "SETTER"
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewSetterEvent {
    pub old: Address,
    pub new: Address,
}

pub(crate) fn new_setter(e: &Env, old: Address, new: Address) {
    let event: NewSetterEvent = NewSetterEvent { old: old, new: new };
    e.events()
        .publish(("SoroswapFactory", symbol_short!("setter")), event);
}

// NEW "FEES ENABLED" BOOL
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewFeesEnabledEvent {
    pub fees_enabled: bool,
}

pub(crate) fn new_fees_enabled(e: &Env, fees_enabled: bool) {
    let event: NewFeesEnabledEvent = NewFeesEnabledEvent {
        fees_enabled: fees_enabled,
    };
    e.events()
        .publish(("SoroswapFactory", symbol_short!("fees")), event);
}
