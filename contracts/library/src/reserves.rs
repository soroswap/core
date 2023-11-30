use soroban_sdk::{Address, Env};
use crate::tokens::{sort_tokens, pair_for};


mod pair {
    soroban_sdk::contractimport!(
        file = "./src/soroswap_pair_contract.wasm"
    );
}
use pair::Client as SoroswapPairClient;



/// fetches and sorts the reserves for a pair
// function getReserves(address factory, address tokenA, address tokenB) internal view returns (uint reserveA, uint reserveB) {
pub fn get_reserves(e: Env,factory: Address, token_a: Address, token_b: Address) -> (i128, i128) {
    //     (address token0,) = sortTokens(tokenA, tokenB);
    let (token_0,token_1) = sort_tokens(token_a.clone(), token_b.clone());

    //     (uint reserve0, uint reserve1,) = IUniswapV2Pair(pairFor(factory, tokenA, tokenB)).getReserves();
    let pair_address = pair_for(e.clone(), factory, token_0.clone(), token_1.clone());
    let pair_client = SoroswapPairClient::new(&e, &pair_address);
    let (reserve_0, reserve_1, _block_timestamp_last) = pair_client.get_reserves();
    

        //   (reserveA, reserveB) = tokenA == token0 ? (reserve0, reserve1) : (reserve1, reserve0);
    let (reserve_a, reseve_b) =
        if token_a == token_0 {
            (reserve_0, reserve_1) 
        } else {
            (reserve_1, reserve_0) };

    (reserve_a, reseve_b)

}