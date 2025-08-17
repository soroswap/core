#![no_std]
use num_integer::Roots;
use soroban_sdk::{contract, contractimpl, contractmeta, Address, Env, String};
use soroban_token_sdk::metadata::TokenMetadata;
use soroswap_factory_interface::SoroswapFactoryClient;

mod balances;
mod error;
mod event;
mod math;
mod soroswap_pair_token;
mod storage;
mod strings;
mod test;

// ANY TOKEN CONTRACT
// TODO: Simplify this and use a any_token_interface
pub mod any_token {
    soroban_sdk::contractimport!(
        file = "../token/target/wasm32v1-none/release/soroban_token_contract.wasm"
    );
    pub type TokenClient<'a> = Client<'a>;
}

use balances::*;
use error::SoroswapPairError;
use math::CheckedCeilingDiv;
use soroswap_pair_token::{internal_burn, internal_mint, write_metadata, SoroswapPairToken};
use storage::*;
use strings::TakeFirstNCharsAndConcat;

static MINIMUM_LIQUIDITY: i128 = 1000;

fn create_symbol(e: &Env, symbol_0: &String, symbol_1: &String) -> String {
    let symbol_0_short = symbol_0.take_first_n_chars(&e, 6);
    let symbol_1_short = symbol_1.take_first_n_chars(&e, 6);
    let hyphen = String::from_str(&e, "-");
    let end = String::from_str(&e, "-SOROSWAP-LP");
    symbol_0_short
        .concat(&e, hyphen)
        .concat(&e, symbol_1_short)
        .concat(&e, end)
}

fn create_name(e: &Env, symbol_0: &String, symbol_1: &String) -> String {
    let symbol_0_short = symbol_0.take_first_n_chars(&e, 6);
    let symbol_1_short = symbol_1.take_first_n_chars(&e, 6);
    let hyphen = String::from_str(&e, "-");
    let end = String::from_str(&e, " Soroswap LP Token");
    symbol_0_short
        .concat(&e, hyphen)
        .concat(&e, symbol_1_short)
        .concat(&e, end)
}

// Metadata that is added on to the WASM custom section
contractmeta!(
    key = "Description",
    val = "Soroswap.Finance Protocol - Constant product AMM with a .3% swap fee"
);

pub trait SoroswapPairTrait {
    // Sets the token contract addresses for this pool
    fn initialize(
        e: Env,
        factory: Address,
        token_0: Address,
        token_1: Address,
    ) -> Result<(), SoroswapPairError>;

    fn deposit(e: Env, to: Address) -> Result<i128, SoroswapPairError>;

    // Swaps. This function should be called from another contract that has already sent tokens to the pair contract
    fn swap(
        e: Env,
        amount_0_out: i128,
        amount_1_out: i128,
        to: Address,
    ) -> Result<(), SoroswapPairError>;

    fn withdraw(e: Env, to: Address) -> Result<(i128, i128), SoroswapPairError>;

    // transfers the excess token balances from the pair to the specified to address,
    // ensuring that the balances match the reserves by subtracting the reserve amounts
    // from the current balances.
    fn skim(e: Env, to: Address);

    // updates the reserves of the pair to match the current token balances.
    // It retrieves the balances and reserves from the environment, then calls the update
    // function to synchronize the reserves with the balances.
    fn sync(e: Env);

    fn token_0(e: Env) -> Address;
    fn token_1(e: Env) -> Address;
    fn factory(e: Env) -> Address;

    fn k_last(e: Env) -> i128;

    fn get_reserves(e: Env) -> (i128, i128);
}

#[contract]
struct SoroswapPair;

#[contractimpl]
impl SoroswapPairTrait for SoroswapPair {
    /// Initializes a new Soroswap pair by setting token addresses, factory, and initial reserves.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    /// * `factory` - The address of the Soroswap factory contract.
    /// * `token_0` - The address of the first token in the pair.
    /// * `token_1` - The address of the second token in the pair.
    fn initialize(
        e: Env,
        factory: Address,
        token_0: Address,
        token_1: Address,
    ) -> Result<(), SoroswapPairError> {
        if has_token_0(&e) {
            return Err(SoroswapPairError::InitializeAlreadyInitialized);
        }

        if token_0 >= token_1 {
            return Err(SoroswapPairError::InitializeTokenOrderInvalid);
        }

        put_factory(&e, factory);

        let symbol_0: String = any_token::TokenClient::new(&e, &token_0).symbol();
        let symbol_1: String = any_token::TokenClient::new(&e, &token_1).symbol();

        let decimal: u32 = 7;
        let name: String = create_name(&e, &symbol_0, &symbol_1);
        let symbol: String = create_symbol(&e, &symbol_0, &symbol_1);

        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        );

        put_token_0(&e, token_0);
        put_token_1(&e, token_1);
        put_reserve_0(&e, 0);
        put_reserve_1(&e, 0);
        extend_instance_ttl(&e);

        Ok(())
    }

    /// Returns the address of the first token in the Soroswap pair.
    fn token_0(e: Env) -> Address {
        extend_instance_ttl(&e);
        get_token_0(&e)
    }

    /// Returns the address of the second token in the Soroswap pair.
    fn token_1(e: Env) -> Address {
        extend_instance_ttl(&e);
        get_token_1(&e)
    }

    /// Returns the address of the Soroswap factory contract.
    fn factory(e: Env) -> Address {
        extend_instance_ttl(&e);
        get_factory(&e)
    }

    /// Deposits tokens into the Soroswap pair and mints LP tokens in return.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    /// * `to` - The address where the minted LP tokens will be sent.
    ///
    /// # Returns
    /// The amount of minted LP tokens.
    /// Possible errors:
    /// - `SoroswapPairError::NotInitialized`: The Soroswap pair has not been initialized.
    /// - `SoroswapPairError::DepositInsufficientAmountToken0`: Insufficient amount of token 0 sent.
    /// - `SoroswapPairError::DepositInsufficientAmountToken1`: Insufficient amount of token 1 sent.
    /// - `SoroswapPairError::DepositInsufficientFirstLiquidity`: Insufficient first liquidity minted.
    /// - `SoroswapPairError::DepositInsufficientLiquidityMinted`: Insufficient liquidity minted.
    /// - `SoroswapPairError::UpdateOverflow`: Overflow occurred during update.
    fn deposit(e: Env, to: Address) -> Result<i128, SoroswapPairError> {
        extend_instance_ttl(&e);

        if !has_token_0(&e) {
            return Err(SoroswapPairError::NotInitialized);
        }

        let (mut reserve_0, mut reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));
        let amount_0 = balance_0
            .checked_sub(reserve_0)
            .ok_or(SoroswapPairError::DepositInsufficientAmountToken0)?;
        let amount_1 = balance_1
            .checked_sub(reserve_1)
            .ok_or(SoroswapPairError::DepositInsufficientAmountToken1)?;

        if amount_0 <= 0 {
            return Err(SoroswapPairError::DepositInsufficientAmountToken0);
        }

        if amount_1 <= 0 {
            return Err(SoroswapPairError::DepositInsufficientAmountToken1);
        }

        let fee_on: bool = mint_fee(&e, reserve_0, reserve_1);
        let total_supply = SoroswapPairToken::total_supply(e.clone());

        let liquidity = if total_supply == 0 {
            // When the liquidity pool is being initialized, we block the minimum liquidity forever in this contract
            internal_mint(e.clone(), e.current_contract_address(), MINIMUM_LIQUIDITY);
            let previous_liquidity = (amount_0.checked_mul(amount_1).unwrap()).sqrt();
            if previous_liquidity <= MINIMUM_LIQUIDITY {
                return Err(SoroswapPairError::DepositInsufficientFirstLiquidity);
            }
            (previous_liquidity).checked_sub(MINIMUM_LIQUIDITY).unwrap()
        } else {
            let shares_0 = (amount_0.checked_mul(total_supply).unwrap())
                .checked_div(reserve_0)
                .unwrap();
            let shares_1 = (amount_1.checked_mul(total_supply).unwrap())
                .checked_div(reserve_1)
                .unwrap();
            shares_0.min(shares_1)
        };

        if liquidity <= 0 {
            return Err(SoroswapPairError::DepositInsufficientLiquidityMinted);
        }

        internal_mint(e.clone(), to.clone(), liquidity.clone());
        update(&e, balance_0, balance_1);

        (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        if fee_on {
            put_klast(&e, reserve_0.checked_mul(reserve_1).unwrap());
        }

        event::deposit(&e, to, amount_0, amount_1, liquidity, reserve_0, reserve_1);

        Ok(liquidity)
    }

    /// Executes a token swap within the Soroswap pair.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    /// * `amount_0_out` - The desired amount of the first token to receive.
    /// * `amount_1_out` - The desired amount of the second token to receive.
    /// * `to` - The address where the swapped tokens will be sent.
    ////// # Errors
    /// Returns an error if the swap cannot be executed. Possible errors include:
    /// - `SoroswapPairError::NotInitialized`
    /// - `SoroswapPairError::SwapInsufficientOutputAmount`
    /// - `SoroswapPairError::SwapNegativesOutNotSupported`
    /// - `SoroswapPairError::SwapInsufficientLiquidity`
    /// - `SoroswapPairError::SwapInvalidTo`
    /// - `SoroswapPairError::SwapInsufficientInputAmount`
    /// - `SoroswapPairError::SwapNegativesInNotSupported`
    /// - `SoroswapPairError::SwapKConstantNotMet`: If the K constant condition is not met after the swap.
    fn swap(
        e: Env,
        amount_0_out: i128,
        amount_1_out: i128,
        to: Address,
    ) -> Result<(), SoroswapPairError> {
        extend_instance_ttl(&e);

        if !has_token_0(&e) {
            return Err(SoroswapPairError::NotInitialized);
        }

        let (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));

        if amount_0_out == 0 && amount_1_out == 0 {
            return Err(SoroswapPairError::SwapInsufficientOutputAmount);
        }
        if amount_0_out < 0 || amount_1_out < 0 {
            return Err(SoroswapPairError::SwapNegativesOutNotSupported);
        }
        if amount_0_out >= reserve_0 || amount_1_out >= reserve_1 {
            return Err(SoroswapPairError::SwapInsufficientLiquidity);
        }
        if to == get_token_0(&e) || to == get_token_1(&e) {
            return Err(SoroswapPairError::SwapInvalidTo);
        }

        if amount_0_out > 0 {
            transfer_token_0_from_pair(&e, &to, amount_0_out);
        }
        if amount_1_out > 0 {
            transfer_token_1_from_pair(&e, &to, amount_1_out);
        }

        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));

        let amount_0_in = if balance_0 > reserve_0.checked_sub(amount_0_out).unwrap() {
            balance_0
                .checked_sub(reserve_0.checked_sub(amount_0_out).unwrap())
                .unwrap()
        } else {
            0
        };
        let amount_1_in = if balance_1 > reserve_1.checked_sub(amount_1_out).unwrap() {
            balance_1
                .checked_sub(reserve_1.checked_sub(amount_1_out).unwrap())
                .unwrap()
        } else {
            0
        };

        if amount_0_in == 0 && amount_1_in == 0 {
            return Err(SoroswapPairError::SwapInsufficientInputAmount);
        }
        if amount_0_in < 0 || amount_1_in < 0 {
            return Err(SoroswapPairError::SwapNegativesInNotSupported);
        }

        let fee_0 = (amount_0_in.checked_mul(3).unwrap())
            .checked_ceiling_div(1000)
            .unwrap();
        let fee_1 = (amount_1_in.checked_mul(3).unwrap())
            .checked_ceiling_div(1000)
            .unwrap();

        let balance_0_minus_fee = balance_0.checked_sub(fee_0).unwrap();
        let balance_1_minus_fee = balance_1.checked_sub(fee_1).unwrap();

        if balance_0_minus_fee
            .checked_mul(balance_1_minus_fee)
            .unwrap()
            < reserve_0.checked_mul(reserve_1).unwrap()
        {
            return Err(SoroswapPairError::SwapKConstantNotMet);
        }

        update(&e, balance_0, balance_1);

        event::swap(&e, to, amount_0_in, amount_1_in, amount_0_out, amount_1_out);

        Ok(())
    }

    /// Withdraws liquidity from the Soroswap pair, burning LP tokens and returning the corresponding tokens to the user.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    /// * `to` - The address where the withdrawn tokens will be sent.
    ///
    /// # Returns
    /// A tuple containing the amounts of token 0 and token 1 withdrawn from the pair.
    fn withdraw(e: Env, to: Address) -> Result<(i128, i128), SoroswapPairError> {
        extend_instance_ttl(&e);

        if !has_token_0(&e) {
            return Err(SoroswapPairError::NotInitialized);
        }

        let balance_shares = get_balance_shares(&e);
        if balance_shares == 0 {
            return Err(SoroswapPairError::WithdrawLiquidityNotInitialized);
        }

        let (mut reserve_0, mut reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        let (mut balance_0, mut balance_1) = (get_balance_0(&e), get_balance_1(&e));
        let user_sent_shares = balance_shares.checked_sub(MINIMUM_LIQUIDITY).unwrap();

        if user_sent_shares <= 0 {
            return Err(SoroswapPairError::WithdrawInsufficientSentShares);
        }

        let fee_on: bool = mint_fee(&e, reserve_0, reserve_1);
        let total_supply = SoroswapPairToken::total_supply(e.clone());

        let amount_0 = (balance_0.checked_mul(user_sent_shares).unwrap())
            .checked_div(total_supply)
            .unwrap();
        let amount_1 = (balance_1.checked_mul(user_sent_shares).unwrap())
            .checked_div(total_supply)
            .unwrap();

        if amount_0 <= 0 || amount_1 <= 0 {
            return Err(SoroswapPairError::WithdrawInsufficientLiquidityBurned);
        }

        internal_burn(e.clone(), e.current_contract_address(), user_sent_shares);

        transfer_token_0_from_pair(&e, &to, amount_0);
        transfer_token_1_from_pair(&e, &to, amount_1);

        (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));

        update(&e, balance_0, balance_1);

        (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        if fee_on {
            put_klast(&e, reserve_0.checked_mul(reserve_1).unwrap());
        }

        event::withdraw(
            &e,
            to,
            user_sent_shares,
            amount_0,
            amount_1,
            reserve_0,
            reserve_1,
        );
        Ok((amount_0, amount_1))
    }

    /// Skims excess tokens from reserves and sends them to the specified address.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    /// * `to` - The address where the excess tokens will be sent.
    fn skim(e: Env, to: Address) {
        extend_instance_ttl(&e);

        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));
        let (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        let skimmed_0 = balance_0.checked_sub(reserve_0).unwrap();
        let skimmed_1 = balance_1.checked_sub(reserve_1).unwrap();
        transfer_token_0_from_pair(&e, &to, skimmed_0);
        transfer_token_1_from_pair(&e, &to, skimmed_1);
        event::skim(&e, skimmed_0, skimmed_1);
    }

    /// Forces reserves to match current balances.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    fn sync(e: Env) {
        extend_instance_ttl(&e);

        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));
        update(&e, balance_0, balance_1);
    }

    /// Returns the current reserves and the last block timestamp.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    ///
    /// # Returns
    /// A tuple containing the reserves of token 0 and token 1.
    fn get_reserves(e: Env) -> (i128, i128) {
        extend_instance_ttl(&e);

        (get_reserve_0(&e), get_reserve_1(&e))
    }

    /// Returns the value of the last product of reserves (`K`) stored in the contract.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    ///
    /// # Returns
    /// The value of the last product of reserves (`K`).
    fn k_last(e: Env) -> i128 {
        extend_instance_ttl(&e);

        get_klast(&e)
    }
}

fn transfer(e: &Env, contract_id: Address, to: &Address, amount: i128) {
    any_token::TokenClient::new(e, &contract_id).transfer(
        &e.current_contract_address(),
        &to,
        &amount,
    );
}

fn transfer_token_0_from_pair(e: &Env, to: &Address, amount: i128) {
    // Execute the transfer function in TOKEN_A to send "amount" of tokens from this Pair contract to "to"
    transfer(e, get_token_0(e), &to, amount);
}

fn transfer_token_1_from_pair(e: &Env, to: &Address, amount: i128) {
    transfer(e, get_token_1(e), &to, amount);
}

fn mint_fee(e: &Env, reserve_0: i128, reserve_1: i128) -> bool {
    /*
            accumulated fees are collected only when liquidity is deposited
            or withdrawn. The contract computes the accumulated fees, and mints new liquidity tokens
            to the fee beneficiary, immediately before any tokens are minted or burned
    */

    let factory = get_factory(&e);
    let factory_client = SoroswapFactoryClient::new(&e, &factory);
    let fee_on = factory_client.fees_enabled();
    let klast = get_klast(&e);

    if fee_on {
        let fee_to: Address = factory_client.fee_to();

        if klast != 0 {
            let root_k = (reserve_0.checked_mul(reserve_1).unwrap()).sqrt();
            let root_klast = (klast).sqrt();
            if root_k > root_klast {
                let total_supply = SoroswapPairToken::total_supply(e.clone());
                let numerator = total_supply
                    .checked_mul(root_k.checked_sub(root_klast).unwrap())
                    .unwrap();
                let denominator = root_k
                    .checked_mul(5_i128)
                    .unwrap()
                    .checked_add(root_klast)
                    .unwrap();
                let liquidity_pool_shares_fees = numerator.checked_div(denominator).unwrap();

                if liquidity_pool_shares_fees > 0 {
                    internal_mint(e.clone(), fee_to, liquidity_pool_shares_fees);
                }
            }
        }
    } else if klast != 0 {
        put_klast(&e, 0);
    }

    fee_on
}

fn update(e: &Env, balance_0: i128, balance_1: i128) {
    put_reserve_0(&e, balance_0);
    put_reserve_1(&e, balance_1);
    event::sync(&e, balance_0, balance_1);
}
