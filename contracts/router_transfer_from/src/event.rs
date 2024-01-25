//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env, Address, Vec};

// INITIALIZED
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {
    pub factory: Address
}

pub(crate) fn initialized(e: &Env, factory: Address) {
    
    let event: InitializedEvent = InitializedEvent {
        factory: factory
    };
    e.events().publish(("SoroswapRouter", symbol_short!("init")), event);
}

// ADD LIQUIDITY EVENT
#[contracttype] 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddLiquidityEvent {
    pub token_a: Address,
    pub token_b: Address,
    pub pair: Address,
    pub amount_a: i128,
    pub amount_b: i128,
    pub liquidity: i128,
    pub to: Address
}

/// Publishes an `AddLiquidityEvent` to the event stream.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// * `token_a` - The address of the first token in the liquidity pair.
/// * `token_b` - The address of the second token in the liquidity pair.
/// * `pair` - The address of the liquidity pair.
/// * `amount_a` - The amount of `token_a` to add to the liquidity.
/// * `amount_b` - The amount of `token_b` to add to the liquidity.
/// * `liquidity` - The amount of liquidity tokens minted.
/// * `to` - The address to receive the liquidity tokens.
pub(crate) fn add_liquidity(
    e: &Env,
    token_a: Address,
    token_b: Address,
    pair: Address,
    amount_a: i128,
    amount_b: i128,
    liquidity: i128,
    to: Address,
) {
    let event = AddLiquidityEvent {
        token_a,
        token_b,
        pair,
        amount_a,
        amount_b,
        liquidity,
        to,
    };

    e.events().publish(("SoroswapRouter", symbol_short!("add")), event);
}

 

// REMOVE LIQUIDITY EVENT
#[contracttype] 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoveLiquidityEvent {
    pub token_a: Address,
    pub token_b: Address,
    pub pair: Address,
    pub amount_a: i128,
    pub amount_b: i128,
    pub liquidity: i128,
    pub to: Address
}


/// Publishes an `RemoveLiquidityEvent` to the event stream.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// * `token_a` - The address of the first token in the liquidity pair.
/// * `token_b` - The address of the second token in the liquidity pair.
/// * `pair` - The address of the liquidity pair.
/// * `amount_a` - The amount of `token_a` removed from the pool.
/// * `amount_b` - The amount of `token_b` removed from the pool.
/// * `liquidity` - The amount of liquidity tokens burned.
/// * `to` - The address to receive the token_a and token_b.
pub(crate) fn remove_liquidity(
    e: &Env,
    token_a: Address,
    token_b: Address,
    pair: Address,
    amount_a: i128,
    amount_b: i128,
    liquidity: i128,
    to: Address,
) {
    let event = RemoveLiquidityEvent {
        token_a,
        token_b,
        pair,
        amount_a,
        amount_b,
        liquidity,
        to,
    };

    e.events().publish(("SoroswapRouter", symbol_short!("remove")), event);
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

    e.events().publish(("SoroswapRouter", symbol_short!("swap")), event);
}