//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env, Address};

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

    e.events().publish(("SoroswapRouter", symbol_short!("add_liq")), event);
}


// // SWAP EVENT

// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct SwapEvent {
//     pub to: Address,
//     pub amount_0_in: i128,
//     pub amount_1_in: i128,
//     pub amount_0_out: i128,
//     pub amount_1_out: i128,
// }

// pub(crate) fn swap(
//     e: &Env,
//     to: Address,
//     amount_0_in: i128,
//     amount_1_in: i128,
//     amount_0_out: i128,
//     amount_1_out: i128,
// ) {
//     let event: SwapEvent = SwapEvent {
//         to: to,
//         amount_0_in: amount_0_in,
//         amount_1_in: amount_1_in,
//         amount_0_out: amount_0_out,
//         amount_1_out: amount_1_out,
//     };
//     e.events().publish(("SoroswapPair", symbol_short!("swap")), event);
// }

// // WITHDRAW EVENT


// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct WithdrawEvent {
//     pub to: Address,
//     pub liquidity: i128,
//     pub amount_0: i128,
//     pub amount_1: i128,
//     pub new_reserve_0: i128,
//     pub new_reserve_1: i128,
// }

// pub(crate) fn withdraw(
//     e: &Env,
//     to: Address,
//     liquidity: i128,
//     amount_0: i128,
//     amount_1: i128,
//     new_reserve_0: i128,
//     new_reserve_1: i128,
// ) {
//     let event: WithdrawEvent = WithdrawEvent {
//         to: to,
//         liquidity: liquidity,
//         amount_0: amount_0,
//         amount_1: amount_1,
//         new_reserve_0: new_reserve_0,
//         new_reserve_1: new_reserve_1,
//     };
//     e.events().publish(("SoroswapPair", symbol_short!("withdraw")), event);
// }

// // SYNC EVENT

// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct SyncEvent {
//     pub new_reserve_0: i128,
//     pub new_reserve_1: i128,
// }

// pub(crate) fn sync(e: &Env, new_reserve_0: i128, new_reserve_1: i128) {
//     let event: SyncEvent = SyncEvent {
//         new_reserve_0: new_reserve_0,
//         new_reserve_1: new_reserve_1,
//     };
//     e.events().publish(("SoroswapPair", symbol_short!("sync")), event);
// }


// // SKIM EVENT

// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct SkimEvent {
//     pub skimmed_0: i128,
//     pub skimmed_1: i128,
// }

// pub(crate) fn skim(e: &Env, skimmed_0: i128, skimmed_1: i128) {
//     let event: SkimEvent = SkimEvent {
//         skimmed_0: skimmed_0,
//         skimmed_1: skimmed_1,
//     };
//     e.events().publish(("SoroswapPair", symbol_short!("skim")), event);
// }