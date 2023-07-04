use soroban_sdk::{Env, Symbol, Address};

// PairCreated(address indexed token0, address indexed token1, address pair, uint);
pub(crate) fn pair_created(e: &Env, token_pair: (Address, Address), pair: &Address, all_pairs_length: u32) {
    let topics = (Symbol::new(e, "increase_allowance"), token_pair.0, token_pair.1, pair);
    e.events().publish(topics, all_pairs_length);
}
