#![no_std] 
use soroban_sdk::{
    contract, contractimpl,
    Address, Env, Vec, 
};

mod test;
mod tokens;
mod reserves;
mod quotes;

pub use tokens::{
    sort_tokens,
    pair_for
};
pub use reserves::{
    get_reserves
};
pub use quotes::{
    quote, 
    get_amount_out, 
    get_amount_in, 
    get_amounts_out, 
    get_amounts_in
};



pub trait SoroswapLibraryTrait {
    
    /// returns sorted token addresses, used to handle return values from pairs sorted in this order
    fn sort_tokens(token_a: Address, token_b: Address) -> (Address, Address);

    /// calculates the deterministic address for a pair without making any external calls
    /// check <https://github.com/paltalabs/deterministic-address-soroban>
    fn pair_for(e: Env, factory: Address, token_a: Address, token_b: Address) -> Address;

    /// fetches and sorts the reserves for a pair
    fn get_reserves(e: Env,factory: Address, token_a: Address, token_b: Address) -> (i128, i128);

    /// given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    fn quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> i128;

    /// given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
    fn get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128;

    /// given an output amount of an asset and pair reserves, returns a required input amount of the other asset
    fn get_amount_in(amount_out: i128, reserve_in: i128, reserve_out: i128) -> i128;

    /// performs chained getAmountOut calculations on any number of pairs 
    fn get_amounts_out(e: Env, factory: Address, amount_in: i128, path: Vec<Address>) -> Vec<i128>;
    
    /// performs chained getAmountIn calculations on any number of pairs
    fn get_amounts_in(e:Env, factory: Address, amount_out: i128, path: Vec<Address>) -> Vec<i128>;
    


   
}

#[contract]
pub struct SoroswapLibrary;

#[contractimpl]
impl SoroswapLibraryTrait for SoroswapLibrary {

    /// returns sorted token addresses, used to handle return values from pairs sorted in this order
    // function sortTokens(address tokenA, address tokenB) internal pure returns (address token0, address token1) {
    fn sort_tokens(token_a: Address, token_b: Address) -> (Address, Address) {
        sort_tokens(token_a, token_b)
    }


    /// calculates the CREATE2 address for a pair without making any external calls
    // function pairFor(address factory, address tokenA, address tokenB) internal pure returns (address pair) {
    fn pair_for(e: Env, factory: Address, token_a: Address, token_b: Address) -> Address {
        pair_for(e, factory, token_a, token_b)
    }


    /// fetches and sorts the reserves for a pair
    // function getReserves(address factory, address tokenA, address tokenB) internal view returns (uint reserveA, uint reserveB) {
    fn get_reserves(e: Env,factory: Address, token_a: Address, token_b: Address) -> (i128, i128) {
        get_reserves(e, factory, token_a, token_b)

    }

    /// given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    // function quote(uint amountA, uint reserveA, uint reserveB) internal pure returns (uint amountB) {
    fn quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> i128 {
        quote(amount_a, reserve_a, reserve_b)
    }
    

    /// given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
    // function getAmountOut(uint amountIn, uint reserveIn, uint reserveOut) internal pure returns (uint amountOut) {
    fn get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128 {
        get_amount_out(amount_in, reserve_in, reserve_out)
    }

    /// given an output amount of an asset and pair reserves, returns a required input amount of the other asset
    // function getAmountIn(uint amountOut, uint reserveIn, uint reserveOut) internal pure returns (uint amountIn) {
    fn get_amount_in(amount_out: i128, reserve_in: i128, reserve_out: i128) -> i128 {
        get_amount_in(amount_out, reserve_in, reserve_out)
    }

    /// performs chained getAmountOut calculations on any number of pairs 
    // function getAmountsOut(address factory, uint amountIn, address[] memory path) internal view returns (uint[] memory amounts) {
    fn get_amounts_out(e: Env, factory: Address, amount_in: i128, path: Vec<Address>) -> Vec<i128> {
        get_amounts_out(e, factory, amount_in, path)
    }

    /// performs chained getAmountIn calculations on any number of pairs
    // function getAmountsIn(address factory, uint amountOut, address[] memory path) internal view returns (uint[] memory amounts) {
    fn get_amounts_in(e:Env, factory: Address, amount_out: i128, path: Vec<Address>) -> Vec<i128> {
        get_amounts_in(e, factory, amount_out, path)
    }



}
