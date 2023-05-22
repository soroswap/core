use soroban_sdk::{Env, Symbol, Address};

//  event Mint(address indexed sender, uint amount0, uint amount1);
pub(crate) fn deposit(e: &Env, sender: Address, amount_0: i128, amount_1: i128) {
    let topics = (Symbol::new(e, "deposit"), sender, amount_0);
    e.events().publish(topics, amount_1);
}
 
// // event Burn(address indexed sender, uint amount0, uint amount1, address indexed to);
// pub(crate) fn burn(e: &Env, sender: Address, amount_0: u32, amount_1: u32, to: Address) {
//     let topics = (Symbol::new(e, "burn"), sender, amount_0, amount_1);
//     e.events().publish(topics, to);
// }

/*
event Swap(
     address indexed sender,
     uint amount0In,
     uint amount1In,
     uint amount0Out,
     uint amount1Out,
     address indexed to
 );
*/

// pub(crate) fn swap(
//     e: &Env,
//     sender: Address,
//     amount_0_in: u32,
//     amount_1_in: u32,
//     amount_0_out: u32,
//     amount_1_out: u32,
//     to: Address,
// ) {
//     let topics = (
//         Symbol::new(e, "swap"),
//         sender,
//         amount_0_in,
//         amount_1_in,
//         amount_0_out,
//         amount_1_out
//     );
//     e.events().publish(topics, to);
// }



// event Sync(uint112 reserve0, uint112 reserve1);

// TODO: Implement when implementing sync function
// pub(crate) fn sync(e: &Env, reserve_0: u32, reserve_1: u32) {
//     let topics = (Symbol::new(e, "sync"), reserve_0);
//     e.events().publish(topics, reserve_1);

