#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

mod error;
mod math;
mod quotes;
mod reserves;
mod test;
mod tokens;

pub use error::SoroswapLibraryError;
pub use quotes::{get_amount_in, get_amount_out, get_amounts_in, get_amounts_out, quote};
pub use reserves::{get_reserves_with_factory, get_reserves_with_pair};
pub use tokens::{pair_for, sort_tokens};

pub trait SoroswapLibraryTrait {
    /// Sorts two token addresses in a consistent order.
    ///
    /// # Arguments
    ///
    /// * `token_a` - The address of the first token.
    /// * `token_b` - The address of the second token.
    ///
    /// # Returns
    ///
    /// Returns `Result<(Address, Address), SoroswapLibraryError>` where `Ok` contains a tuple with the sorted token addresses, and `Err` indicates an error such as identical tokens.
    fn sort_tokens(
        token_a: Address,
        token_b: Address,
    ) -> Result<(Address, Address), SoroswapLibraryError>;

    /// Calculates the deterministic address for a pair without making any external calls.
    /// check <https://github.com/paltalabs/deterministic-address-soroban>
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `factory` - The factory address.
    /// * `token_a` - The address of the first token.
    /// * `token_b` - The address of the second token.
    ///
    /// # Returns
    ///
    /// Returns `Result<Address, SoroswapLibraryError>` where `Ok` contains the deterministic address for the pair, and `Err` indicates an error such as identical tokens or an issue with sorting.
    fn pair_for(
        e: Env,
        factory: Address,
        token_a: Address,
        token_b: Address,
    ) -> Result<Address, SoroswapLibraryError>;

    /// Fetches and sorts the reserves for a pair of tokens.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `factory` - The factory address.
    /// * `token_a` - The address of the first token.
    /// * `token_b` - The address of the second token.
    ///
    /// # Returns
    ///
    /// Returns `Result<(i128, i128), SoroswapLibraryError>` where `Ok` contains a tuple of sorted reserves, and `Err` indicates an error such as identical tokens or an issue with sorting.
    fn get_reserves_with_factory(
        e: Env,
        factory: Address,
        token_a: Address,
        token_b: Address,
    ) -> Result<(i128, i128), SoroswapLibraryError>;

    /// Fetches and sorts the reserves for a pair of tokens knowing the pair address
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `pair` - The pair address.
    /// * `token_a` - The address of the first token.
    /// * `token_b` - The address of the second token.
    ///
    /// # Returns
    ///
    /// Returns `Result<(i128, i128), SoroswapLibraryError>` where `Ok` contains a tuple of sorted reserves, and `Err` indicates an error such as identical tokens or an issue with sorting.
    fn get_reserves_with_pair(
        e: Env,
        pair: Address,
        token_a: Address,
        token_b: Address,
    ) -> Result<(i128, i128), SoroswapLibraryError>;

    /// Given some amount of an asset and pair reserves, returns an equivalent amount of the other asset.
    ///
    /// # Arguments
    ///
    /// * `amount_a` - The amount of the first asset.
    /// * `reserve_a` - Reserves of the first asset in the pair.
    /// * `reserve_b` - Reserves of the second asset in the pair.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the calculated equivalent amount, and `Err` indicates an error such as insufficient amount or liquidity
    fn quote(
        amount_a: i128,
        reserve_a: i128,
        reserve_b: i128,
    ) -> Result<i128, SoroswapLibraryError>;

    /// Given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset.
    ///
    /// # Arguments
    ///
    /// * `amount_in` - The input amount of the asset.
    /// * `reserve_in` - Reserves of the input asset in the pair.
    /// * `reserve_out` - Reserves of the output asset in the pair.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the calculated maximum output amount, and `Err` indicates an error such as insufficient input amount or liquidity.
    fn get_amount_out(
        amount_in: i128,
        reserve_in: i128,
        reserve_out: i128,
    ) -> Result<i128, SoroswapLibraryError>;

    /// Given an output amount of an asset and pair reserves, returns a required input amount of the other asset.
    ///
    /// # Arguments
    ///
    /// * `amount_out` - The output amount of the asset.
    /// * `reserve_in` - Reserves of the input asset in the pair.
    /// * `reserve_out` - Reserves of the output asset in the pair.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the required input amount, and `Err` indicates an error such as insufficient output amount or liquidity.
    fn get_amount_in(
        amount_out: i128,
        reserve_in: i128,
        reserve_out: i128,
    ) -> Result<i128, SoroswapLibraryError>;

    /// Performs chained get_amount_out calculations on any number of pairs.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `factory` - The factory address.
    /// * `amount_in` - The input amount.
    /// * `path` - Vector of token addresses representing the path.
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<i128>, SoroswapLibraryError>` where `Ok` contains a vector of calculated amounts, and `Err` indicates an error such as an invalid path.
    fn get_amounts_out(
        e: Env,
        factory: Address,
        amount_in: i128,
        path: Vec<Address>,
    ) -> Result<Vec<i128>, SoroswapLibraryError>;

    /// Performs chained get_amount_in calculations on any number of pairs.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `factory` - The factory address.
    /// * `amount_out` - The output amount.
    /// * `path` - Vector of token addresses representing the path.
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<i128>, SoroswapLibraryError>` where `Ok` contains a vector of calculated amounts, and `Err` indicates an error such as an invalid path.
    fn get_amounts_in(
        e: Env,
        factory: Address,
        amount_out: i128,
        path: Vec<Address>,
    ) -> Result<Vec<i128>, SoroswapLibraryError>;
}

#[contract]
pub struct SoroswapLibrary;

#[contractimpl]
impl SoroswapLibraryTrait for SoroswapLibrary {
    /// Sorts two token addresses in a consistent order.
    ///
    /// # Arguments
    ///
    /// * `token_a` - The address of the first token.
    /// * `token_b` - The address of the second token.
    ///
    /// # Returns
    ///
    /// Returns `Result<(Address, Address), SoroswapLibraryError>` where `Ok` contains a tuple with the sorted token addresses, and `Err` indicates an error such as identical tokens.
    fn sort_tokens(
        token_a: Address,
        token_b: Address,
    ) -> Result<(Address, Address), SoroswapLibraryError> {
        sort_tokens(token_a, token_b)
    }

    /// Calculates the deterministic address for a pair without making any external calls.
    /// check <https://github.com/paltalabs/deterministic-address-soroban>
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `factory` - The factory address.
    /// * `token_a` - The address of the first token.
    /// * `token_b` - The address of the second token.
    ///
    /// # Returns
    ///
    /// Returns `Result<Address, SoroswapLibraryError>` where `Ok` contains the deterministic address for the pair, and `Err` indicates an error such as identical tokens or an issue with sorting.
    fn pair_for(
        e: Env,
        factory: Address,
        token_a: Address,
        token_b: Address,
    ) -> Result<Address, SoroswapLibraryError> {
        pair_for(e, factory, token_a, token_b)
    }

    /// Fetches and sorts the reserves for a pair of tokens using the factory address.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `factory` - The factory address.
    /// * `token_a` - The address of the first token.
    /// * `token_b` - The address of the second token.
    ///
    /// # Returns
    ///
    /// Returns `Result<(i128, i128), SoroswapLibraryError>` where `Ok` contains a tuple of sorted reserves, and `Err` indicates an error such as identical tokens or an issue with sorting.
    fn get_reserves_with_factory(
        e: Env,
        factory: Address,
        token_a: Address,
        token_b: Address,
    ) -> Result<(i128, i128), SoroswapLibraryError> {
        get_reserves_with_factory(e, factory, token_a, token_b)
    }

    /// Fetches and sorts the reserves for a pair of tokens using the pair address.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `pair` - The pair address.
    /// * `token_a` - The address of the first token.
    /// * `token_b` - The address of the second token.
    ///
    /// # Returns
    ///
    /// Returns `Result<(i128, i128), SoroswapLibraryError>` where `Ok` contains a tuple of sorted reserves, and `Err` indicates an error such as identical tokens or an issue with sorting.
    fn get_reserves_with_pair(
        e: Env,
        pair: Address,
        token_a: Address,
        token_b: Address,
    ) -> Result<(i128, i128), SoroswapLibraryError> {
        get_reserves_with_pair(e, pair, token_a, token_b)
    }

    /// Given some amount of an asset and pair reserves, returns an equivalent amount of the other asset.
    ///
    /// # Arguments
    ///
    /// * `amount_a` - The amount of the first asset.
    /// * `reserve_a` - Reserves of the first asset in the pair.
    /// * `reserve_b` - Reserves of the second asset in the pair.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the calculated equivalent amount, and `Err` indicates an error such as insufficient amount or liquidity
    fn quote(
        amount_a: i128,
        reserve_a: i128,
        reserve_b: i128,
    ) -> Result<i128, SoroswapLibraryError> {
        quote(amount_a, reserve_a, reserve_b)
    }

    /// Given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset.
    ///
    /// # Arguments
    ///
    /// * `amount_in` - The input amount of the asset.
    /// * `reserve_in` - Reserves of the input asset in the pair.
    /// * `reserve_out` - Reserves of the output asset in the pair.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the calculated maximum output amount, and `Err` indicates an error such as insufficient input amount or liquidity.
    fn get_amount_out(
        amount_in: i128,
        reserve_in: i128,
        reserve_out: i128,
    ) -> Result<i128, SoroswapLibraryError> {
        get_amount_out(amount_in, reserve_in, reserve_out)
    }

    /// Given an output amount of an asset and pair reserves, returns a required input amount of the other asset.
    ///
    /// # Arguments
    ///
    /// * `amount_out` - The output amount of the asset.
    /// * `reserve_in` - Reserves of the input asset in the pair.
    /// * `reserve_out` - Reserves of the output asset in the pair.
    ///
    /// # Returns
    ///
    /// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the required input amount, and `Err` indicates an error such as insufficient output amount or liquidity.
    fn get_amount_in(
        amount_out: i128,
        reserve_in: i128,
        reserve_out: i128,
    ) -> Result<i128, SoroswapLibraryError> {
        get_amount_in(amount_out, reserve_in, reserve_out)
    }

    /// Performs chained get_amount_out calculations on any number of pairs.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `factory` - The factory address.
    /// * `amount_in` - The input amount.
    /// * `path` - Vector of token addresses representing the path.
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<i128>, SoroswapLibraryError>` where `Ok` contains a vector of calculated amounts, and `Err` indicates an error such as an invalid path.
    fn get_amounts_out(
        e: Env,
        factory: Address,
        amount_in: i128,
        path: Vec<Address>,
    ) -> Result<Vec<i128>, SoroswapLibraryError> {
        get_amounts_out(e, factory, amount_in, path)
    }

    /// Performs chained get_amount_in calculations on any number of pairs.
    ///
    /// # Arguments
    ///
    /// * `e` - The environment.
    /// * `factory` - The factory address.
    /// * `amount_out` - The output amount.
    /// * `path` - Vector of token addresses representing the path.
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<i128>, SoroswapLibraryError>` where `Ok` contains a vector of calculated amounts, and `Err` indicates an error such as an invalid path.
    fn get_amounts_in(
        e: Env,
        factory: Address,
        amount_out: i128,
        path: Vec<Address>,
    ) -> Result<Vec<i128>, SoroswapLibraryError> {
        get_amounts_in(e, factory, amount_out, path)
    }
}
