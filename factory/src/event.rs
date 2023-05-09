use soroban_sdk::{Address, Env, Symbol, BytesN};

// PairCreated(address indexed token0, address indexed token1, address pair, uint);
pub(crate) fn pair_created(e: &Env, token_0: BytesN<32>, token_1: BytesN<32>, pair: BytesN<32>, all_pairs_length: u32) {
    let topics = (Symbol::new(e, "increase_allowance"), token_0, token_1, pair);
    e.events().publish(topics, all_pairs_length);
}
