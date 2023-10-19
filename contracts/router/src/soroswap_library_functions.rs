use soroswap_library;
use soroban_sdk::{Address, Env, Vec};

/// returns sorted token addresses, used to handle return values from pairs sorted in this order
// function sortTokens(address tokenA, address tokenB) internal pure returns (address token0, address token1) {
pub fn soroswap_library_sort_tokens(token_a: Address, token_b: Address) -> (Address, Address) {
    soroswap_library::sort_tokens(token_a, token_b)
}


/// calculates the CREATE2 address for a pair without making any external calls
// function pairFor(address factory, address tokenA, address tokenB) internal pure returns (address pair) {
pub fn soroswap_library_pair_for(e: Env, factory: Address, token_a: Address, token_b: Address) -> Address {
    soroswap_library::pair_for(e, factory, token_a, token_b)
}


/// fetches and sorts the reserves for a pair
// function getReserves(address factory, address tokenA, address tokenB) internal view returns (uint reserveA, uint reserveB) {
pub fn soroswap_library_get_reserves(e: Env,factory: Address, token_a: Address, token_b: Address) -> (i128, i128) {
    soroswap_library::get_reserves(e, factory, token_a, token_b)

}

/// given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
// function quote(uint amountA, uint reserveA, uint reserveB) internal pure returns (uint amountB) {
pub fn soroswap_library_quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> i128 {
    soroswap_library::quote(amount_a, reserve_a, reserve_b)
}


/// given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
// function getAmountOut(uint amountIn, uint reserveIn, uint reserveOut) internal pure returns (uint amountOut) {
pub fn soroswap_library_get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128 {
    soroswap_library::get_amount_out(amount_in, reserve_in, reserve_out)
}

/// given an output amount of an asset and pair reserves, returns a required input amount of the other asset
// function getAmountIn(uint amountOut, uint reserveIn, uint reserveOut) internal pure returns (uint amountIn) {
pub fn soroswap_library_get_amount_in(amount_out: i128, reserve_in: i128, reserve_out: i128) -> i128 {
    soroswap_library::get_amount_in(amount_out, reserve_in, reserve_out)
}

/// performs chained getAmountOut calculations on any number of pairs 
// function getAmountsOut(address factory, uint amountIn, address[] memory path) internal view returns (uint[] memory amounts) {
pub fn soroswap_library_get_amounts_out(e: Env, factory: Address, amount_in: i128, path: Vec<Address>) -> Vec<i128> {
    soroswap_library::get_amounts_out(e, factory, amount_in, path)
}

/// performs chained getAmountIn calculations on any number of pairs
// function getAmountsIn(address factory, uint amountOut, address[] memory path) internal view returns (uint[] memory amounts) {
pub fn soroswap_library_get_amounts_in(e:Env, factory: Address, amount_out: i128, path: Vec<Address>) -> Vec<i128> {
    soroswap_library::get_amounts_in(e, factory, amount_out, path)
}