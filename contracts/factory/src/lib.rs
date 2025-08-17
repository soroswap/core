#![no_std]

mod event;
mod pair;
mod storage;
mod test;

use pair::{create_contract, Pair, PairError};
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};
use soroswap_factory_interface::{FactoryError, SoroswapFactoryTrait};
use storage::*;

impl From<PairError> for FactoryError {
    fn from(pair_error: PairError) -> Self {
        match pair_error {
            PairError::CreatePairIdenticalTokens => FactoryError::CreatePairIdenticalTokens,
            // Handle other conversion cases as needed
        }
    }
}

#[contract]
struct SoroswapFactory;

#[contractimpl]
impl SoroswapFactoryTrait for SoroswapFactory {
    /* *** Read-only functions: *** */

    /// Returns the recipient of the fee.
    ///
    /// # Arguments
    ///
    /// * `e` - An instance of the `Env` struct.
    ///
    /// # Errors
    ///
    /// Returns an error if the Factory is not yet initialized.
    fn fee_to(e: Env) -> Result<Address, FactoryError> {
        if !has_total_pairs(&e) {
            return Err(FactoryError::NotInitialized);
        }
        extend_instance_ttl(&e);
        Ok(get_fee_to(&e))
    }

    /// Returns the address allowed to change the fee recipient.
    ///
    /// # Arguments
    ///
    /// * `e` - An instance of the `Env` struct.
    ///
    /// # Errors
    ///
    /// Returns an error if the Factory is not yet initialized.
    fn fee_to_setter(e: Env) -> Result<Address, FactoryError> {
        if !has_total_pairs(&e) {
            return Err(FactoryError::NotInitialized);
        }
        extend_instance_ttl(&e);
        Ok(get_fee_to_setter(&e))
    }

    /// Checks if fees are enabled.
    ///
    /// # Arguments
    ///
    /// * `e` - An instance of the `Env` struct.
    ///
    /// # Errors
    ///
    /// Returns an error if the Factory is not yet initialized.
    fn fees_enabled(e: Env) -> Result<bool, FactoryError> {
        if !has_total_pairs(&e) {
            return Err(FactoryError::NotInitialized);
        }
        extend_instance_ttl(&e);
        Ok(get_fees_enabled(&e))
    }

    /// Returns the total number of pairs created through the factory so far.
    ///
    /// # Arguments
    ///
    /// * `e` - An instance of the `Env` struct.
    ///
    /// # Errors
    ///
    /// Returns an error if the Factory is not yet initialized.
    fn all_pairs_length(e: Env) -> Result<u32, FactoryError> {
        if !has_total_pairs(&e) {
            return Err(FactoryError::NotInitialized);
        }
        extend_instance_ttl(&e);
        Ok(get_total_pairs(&e))
    }

    /// Returns the address of the pair for `token_a` and `token_b`, if it has been created.
    ///
    /// # Arguments
    ///
    /// * `e` - An instance of the `Env` struct.
    /// * `token_a` - The address of the first token in the pair.
    /// * `token_b` - The address of the second token in the pair.
    ///
    /// # Errors
    ///
    /// Returns an error if the Factory is not yet initialized or if the pair does not exist
    fn get_pair(e: Env, token_a: Address, token_b: Address) -> Result<Address, FactoryError> {
        if !has_total_pairs(&e) {
            return Err(FactoryError::NotInitialized);
        }
        extend_instance_ttl(&e);
        let token_pair = Pair::new(token_a, token_b)?;
        get_pair_address_by_token_pair(&e, token_pair)
    }

    /// Returns the address of the nth pair (0-indexed) created through the factory, or address(0) if not enough pairs have been created yet.
    ///
    /// # Arguments
    ///
    /// * `e` - An instance of the `Env` struct.
    /// * `n` - The index of the pair to retrieve.
    ///
    /// # Errors
    ///
    /// Returns an error if the Factory is not yet initialized or if index `n` does not exist.
    fn all_pairs(e: Env, n: u32) -> Result<Address, FactoryError> {
        if !has_total_pairs(&e) {
            return Err(FactoryError::NotInitialized);
        }
        extend_instance_ttl(&e);
        get_all_pairs(e, n)
    }

    /// Checks if a pair exists for the given `token_a` and `token_b`.
    ///
    /// # Arguments
    ///
    /// * `e` - An instance of the `Env` struct.
    /// * `token_a` - The address of the first token in the pair.
    /// * `token_b` - The address of the second token in the pair.
    ///
    /// # Errors
    ///
    /// Returns an error if the Factory is not yet initialized.
    fn pair_exists(e: Env, token_a: Address, token_b: Address) -> Result<bool, FactoryError> {
        if !has_total_pairs(&e) {
            return Err(FactoryError::NotInitialized);
        }
        extend_instance_ttl(&e);

        let token_pair = Pair::new(token_a, token_b)?;

        // Proceed with the existence check
        Ok(get_pair_exists(&e, token_pair))
    }

    /* *** State-Changing Functions: *** */

    /// Sets the `fee_to_setter` address and initializes the factory.
    ///
    /// # Arguments
    ///
    /// * `e` - An instance of the `Env` struct.
    /// * `setter` - The address to set as the current `fee_to_setter`.
    /// * `pair_wasm_hash` - The Wasm hash of the pair.
    ///
    /// # Errors
    ///
    /// Returns an error if the Factory is already initialized.
    fn initialize(e: Env, setter: Address, pair_wasm_hash: BytesN<32>) -> Result<(), FactoryError> {
        if has_total_pairs(&e) {
            return Err(FactoryError::InitializeAlreadyInitialized);
        }
        put_fee_to_setter(&e, &setter);
        put_fee_to(&e, setter.clone());
        put_pair_wasm_hash(&e, pair_wasm_hash);
        put_total_pairs(&e, 0);
        event::initialized(&e, setter);
        extend_instance_ttl(&e);
        Ok(())
    }

    /// Sets the `fee_to` address.
    ///
    /// # Arguments
    ///
    /// * `e` - An instance of the `Env` struct.
    /// * `to` - The address to set as the `fee_to`.
    ///
    /// # Errors
    ///
    /// Returns an error if the Factory is not yet initialized or if the caller is not the current `fee_to_setter`.
    fn set_fee_to(e: Env, to: Address) -> Result<(), FactoryError> {
        if !has_total_pairs(&e) {
            return Err(FactoryError::NotInitialized);
        }

        extend_instance_ttl(&e);
        let setter = get_fee_to_setter(&e);
        setter.require_auth();

        let old = get_fee_to(&e);
        put_fee_to(&e, to.clone());
        event::new_fee_to(&e, setter, old, to);
        Ok(())
    }

    /// Sets the `fee_to_setter` address.
    ///
    /// # Arguments
    ///
    /// * `e` - An instance of the `Env` struct.
    /// * `new_setter` - The address to set as the new `fee_to_setter`.
    ///
    /// # Errors
    ///
    /// Returns an error if the Factory is not yet initialized or if the caller is not the existing `fee_to_setter`.
    fn set_fee_to_setter(e: Env, new_setter: Address) -> Result<(), FactoryError> {
        if !has_total_pairs(&e) {
            return Err(FactoryError::NotInitialized);
        }

        extend_instance_ttl(&e);
        let setter = get_fee_to_setter(&e);
        setter.require_auth();

        put_fee_to_setter(&e, &new_setter);
        event::new_setter(&e, setter, new_setter);
        Ok(())
    }

    /// Sets whether fees are enabled or disabled.
    ///
    /// # Arguments
    ///
    /// * `e` - An instance of the `Env` struct.
    /// * `is_enabled` - A boolean indicating whether fees are enabled or disabled.
    ///
    /// # Errors
    ///
    /// Returns an error if the Factory is not yet initialized or if the caller is not the current `fee_to_setter`.
    fn set_fees_enabled(e: Env, is_enabled: bool) -> Result<(), FactoryError> {
        if !has_total_pairs(&e) {
            return Err(FactoryError::NotInitialized);
        }

        extend_instance_ttl(&e);
        let setter = get_fee_to_setter(&e);
        setter.require_auth();

        put_fees_enabled(&e, &is_enabled);
        event::new_fees_enabled(&e, is_enabled);
        Ok(())
    }

    /// Creates a pair for `token_a` and `token_b` if one doesn't exist already.
    ///
    /// # Arguments
    ///
    /// * `e` - An instance of the `Env` struct.
    /// * `token_a` - The address of the first token in the pair.
    /// * `token_b` - The address of the second token in the pair.
    ///
    /// # Errors
    ///
    /// Returns an error if the pair is not yet initialized, if `token_a` and `token_b` have identical addresses, or if the pair already exists between `token_a` and `token_b`.
    fn create_pair(e: Env, token_a: Address, token_b: Address) -> Result<Address, FactoryError> {
        if !has_total_pairs(&e) {
            return Err(FactoryError::NotInitialized);
        }

        extend_instance_ttl(&e);
        let token_pair = Pair::new(token_a, token_b)?;

        if get_pair_exists(&e, token_pair.clone()) {
            return Err(FactoryError::CreatePairAlreadyExists);
        }

        let pair_wasm_hash = get_pair_wasm_hash(&e)?;
        let pair_address = create_contract(&e, pair_wasm_hash, &token_pair);

        pair::Client::new(&e, &pair_address).initialize(
            &e.current_contract_address(),
            &token_pair.token_0(),
            &token_pair.token_1(),
        );

        put_pair_address_by_token_pair(&e, token_pair.clone(), &pair_address);
        add_pair_to_all_pairs(&e, &pair_address);

        event::new_pair(
            &e,
            token_pair.token_0().clone(),
            token_pair.token_1().clone(),
            pair_address.clone(),
            get_total_pairs(&e),
        );

        Ok(pair_address)
    }
}
