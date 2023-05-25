use soroban_sdk::{Env, Symbol, Address};

const PAIR: Symbol = Symbol::short("PAIR");


pub(crate) fn deposit(e: &Env, sender: Address, amount_0: i128, amount_1: i128) {
    let topics = (PAIR, Symbol::new(e, "deposit"), sender);
    e.events().publish(topics, (amount_0, amount_1));
}
 
pub(crate) fn withdraw(e: &Env, sender: Address, amount_0: i128, amount_1: i128, to: Address) {
    let topics = (PAIR, Symbol::new(e, "withdraw"), sender);
    e.events().publish(topics, (amount_0, amount_1, to));
}

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

pub(crate) fn swap(
    e: &Env,
    sender: Address,
    amount_0_in: i128,
    amount_1_in: i128,
    amount_0_out: i128,
    amount_1_out: i128,
    to: Address,
) {
    let topics = (PAIR, Symbol::new(e, "swap"), sender);
    e.events().publish(topics, (amount_0_in, amount_1_in, amount_0_out,amount_1_out,  to));
}



// event Sync(uint112 reserve0, uint112 reserve1);
pub(crate) fn sync(e: &Env, reserve_0: u32, reserve_1: u32) {
    let topics = (PAIR, Symbol::new(e, "sync"));
    e.events().publish(topics, (reserve_0, reserve_1));
}

