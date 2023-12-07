#![deny(warnings)]
#![no_std]

use soroban_sdk::{contractclient, contractspecfn, Address, Env, BytesN};
pub struct Spec;

mod error;
pub use error::FactoryError;

/// Interface for SoroswapFactory
#[contractspecfn(name = "Spec", export = false)]
#[contractclient(name = "SoroswapFactoryClient")]

/// Trait defining the interface for a Soroswap Factory contract.
pub trait SoroswapFactoryTrait {

    /*  *** Read-only functions: *** */

    /// Returns the recipient of the fee.
    fn fee_to(e: Env) -> Result<Address, FactoryError>;

    /// Returns the address allowed to change `fee_to`.
    fn fee_to_setter(e: Env) -> Result<Address, FactoryError>;

    /// Checks if fees are enabled.
    fn fees_enabled(e: Env) -> Result<bool, FactoryError>;

    /// Returns the total number of pairs created through the factory so far.
    fn all_pairs_length(e: Env) -> Result<u32, FactoryError>;

    /// Returns the address of the pair for `token_a` and `token_b`, if it has been created.
    fn get_pair(e: Env, token_a: Address, token_b: Address) -> Result<Address, FactoryError>;

    /// Returns the address of the nth pair (0-indexed) created through the factory.
    fn all_pairs(e: Env, n: u32) -> Result<Address, FactoryError>;

    /// Returns a boolean indicating if a pair exists for the given `token_a` and `token_b`.
    fn pair_exists(e: Env, token_a: Address, token_b: Address) -> Result<bool, FactoryError>;

    /*  *** State-Changing Functions: *** */

    /// Sets the `fee_to_setter` address and initializes the factory.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `setter` - The address to set as the `fee_to_setter`.
    /// * `pair_wasm_hash` - The Wasm hash of the SoroswapPair contract.
    fn initialize(e: Env, setter: Address, pair_wasm_hash: BytesN<32>) -> Result<(), FactoryError>;

    /// Sets the `fee_to` address.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `to` - The address to set as the `fee_to`.
    fn set_fee_to(e: Env, to: Address)-> Result<(), FactoryError>;

    /// Sets the `fee_to_setter` address.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `new_setter` - The address to set as the new `fee_to_setter`.
    fn set_fee_to_setter(e: Env, new_setter: Address)-> Result<(), FactoryError>;

    /// Sets whether fees are enabled or disabled.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `is_enabled` - A boolean indicating whether fees are enabled or disabled.
    fn set_fees_enabled(e: Env, is_enabled: bool)-> Result<(), FactoryError>;

    /// Creates a pair for `token_a` and `token_b` if one doesn't exist already.
    /// 
    /// # Arguments
    /// 
    /// * `e` - An instance of the `Env` struct.
    /// * `token_a` - The address of the first token in the pair.
    /// * `token_b` - The address of the second token in the pair.
    fn create_pair(e: Env, token_a: Address, token_b: Address) -> Result<Address, FactoryError>;
}
