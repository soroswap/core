#![no_std]
use soroban_sdk::{contract, contractimpl, contractmeta, Address, Env, IntoVal}; 
use soroban_sdk::token::Interface;
use num_integer::Roots; 
use soroswap_factory_interface::SoroswapFactoryClient;
use event::{
    WithdrawEvent,
    SwapEvent,
    SyncEvent,
    SkimEvent};

mod test;
mod soroswap_pair_token;
mod event;
mod uq64x64;
mod storage;
mod balances;

// ANY TOKEN CONTRACT
// TODO: Simplify this and use a any_token_interface
pub mod any_token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}

use storage::*;
use balances::*;
use soroswap_pair_token::{SoroswapPairToken, internal_mint, internal_burn};
use uq64x64::fraction;


static MINIMUM_LIQUIDITY: i128 = 1000;

// Metadata that is added on to the WASM custom section
contractmeta!(
    key = "Description",
    val = "Constant product AMM with a .3% swap fee"
);

pub trait SoroswapPairTrait{
    // Sets the token contract addresses for this pool
    fn initialize_pair(e: Env, factory: Address, token_0: Address, token_1: Address);

    fn deposit(e:Env, to: Address)  -> i128;

    // Swaps. This function should be called from another contract that has already sent tokens to the pair contract
    fn swap(e: Env, amount_0_out: i128, amount_1_out: i128, to: Address);

    fn withdraw(e: Env, to: Address) -> (i128, i128);

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

    fn price_0_cumulative_last(e: Env) -> u128;
    fn price_1_cumulative_last(e: Env) -> u128;

    fn get_reserves(e: Env) -> (i128, i128, u64);

    // TODO: Just use the token "balance" function
    fn my_balance(e: Env, id: Address) -> i128;
    // TODO: Analize using "total_supply"
    fn total_shares(e: Env) -> i128;
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
    fn initialize_pair(e: Env, factory: Address, token_0: Address, token_1: Address) {
        assert!(!has_token_0(&e), "SoroswapPair: already initialized");

        if token_0 >= token_1 {
            panic!("SoroswapPair: token_0 must be less than token_1");
        }

        put_factory(&e, factory);

        SoroswapPairToken::initialize(
            e.clone(),
            e.current_contract_address(),
            7,
            "Soroswap LP Token".into_val(&e),
            "SOROSWAP-LP".into_val(&e),
        );

        put_token_0(&e, token_0);
        put_token_1(&e, token_1);
        put_total_shares(&e, 0);
        put_reserve_0(&e, 0);
        put_reserve_1(&e, 0);
    }

    /// Returns the address of the first token in the Soroswap pair.
    fn token_0(e: Env) -> Address {
        get_token_0(&e)
    }

    /// Returns the address of the second token in the Soroswap pair.
    fn token_1(e: Env) -> Address {
        get_token_1(&e)
    }

    /// Returns the address of the Soroswap factory contract.
    fn factory(e: Env) -> Address {
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
    fn deposit(e: Env, to: Address) -> i128 {
        assert!(has_token_0(&e), "SoroswapPair: not yet initialized");

        let (mut reserve_0, mut reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));
        let amount_0 = balance_0.checked_sub(reserve_0).unwrap();
        let amount_1 = balance_1.checked_sub(reserve_1).unwrap();
        if amount_0 <= 0 {
            panic!("SoroswapPair: insufficient amount of token 0 sent")
        }
        if amount_1 <= 0 {
            panic!("SoroswapPair: insufficient amount of token 1 sent")
        }

        let fee_on: bool = mint_fee(&e, reserve_0, reserve_1);
        let total_shares = get_total_shares(&e);

        let liquidity = if total_shares == 0 {
            // When the liquidity pool is being initialized, we block the minimum liquidity forever in this contract
            mint_shares(&e, &e.current_contract_address(), MINIMUM_LIQUIDITY);
            let previous_liquidity = (amount_0.checked_mul(amount_1).unwrap()).sqrt();
            if previous_liquidity <= MINIMUM_LIQUIDITY {
                panic!("SoroswapPair: insufficient first liquidity minted")
            }
            (previous_liquidity).checked_sub(MINIMUM_LIQUIDITY).unwrap()
        } else {
            let shares_0 = (amount_0.checked_mul(total_shares).unwrap()).checked_div(reserve_0).unwrap();
            let shares_1 = (amount_1.checked_mul(total_shares).unwrap()).checked_div(reserve_1).unwrap();
            shares_0.min(shares_1)
        };

        if liquidity <= 0 {
            panic!("SoroswapPair: insufficient liquidity minted")
        }

        mint_shares(&e, &to, liquidity.clone());
        update(&e, balance_0, balance_1, reserve_0.try_into().unwrap(), reserve_1.try_into().unwrap());

        (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        if fee_on {
            put_klast(&e, reserve_0.checked_mul(reserve_1).unwrap());
        }

        event::deposit(&e, to, amount_0, amount_1, liquidity, reserve_0, reserve_1);
        liquidity 
    }

    /// Executes a token swap within the Soroswap pair.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    /// * `amount_0_out` - The desired amount of the first token to receive.
    /// * `amount_1_out` - The desired amount of the second token to receive.
    /// * `to` - The address where the swapped tokens will be sent.
    fn swap(e: Env, amount_0_out: i128, amount_1_out: i128, to: Address) {
        assert!(has_token_0(&e), "SoroswapPair: not yet initialized");

        let (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));

        if amount_0_out == 0 && amount_1_out == 0 {
            panic!("SoroswapPair: insufficient output amount")
        }
        if amount_0_out < 0 || amount_1_out < 0 {
            panic!("SoroswapPair: negatives dont supported")
        }
        if amount_0_out >= reserve_0 || amount_1_out >= reserve_1 {
            panic!("SoroswapPair: insufficient liquidity")
        }
        if to == get_token_0(&e) || to == get_token_1(&e) {
            panic!("SoroswapPair: invalid to")
        }

        if amount_0_out > 0 {
            transfer_token_0_from_pair(&e, &to, amount_0_out);
        }
        if amount_1_out > 0 {
            transfer_token_1_from_pair(&e, &to, amount_1_out);
        }

        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));

        let amount_0_in = if balance_0 > reserve_0.checked_sub(amount_0_out).unwrap() {
            balance_0.checked_sub(reserve_0.checked_sub(amount_0_out).unwrap()).unwrap()
        } else {
            0
        };
        let amount_1_in = if balance_1 > reserve_1.checked_sub(amount_1_out).unwrap() {
            balance_1.checked_sub(reserve_1.checked_sub(amount_1_out).unwrap()).unwrap()
        } else {
            0
        };

        if amount_0_in == 0 && amount_1_in == 0 {
            panic!("SoroswapPair: insufficient input amount")
        }
        if amount_0_in < 0 || amount_1_in < 0 {
            panic!("SoroswapPair: negatives dont supported")
        }

        let fee_0 = (amount_0_in.checked_mul(3).unwrap()).checked_div(1000).unwrap();
        let fee_1 = (amount_1_in.checked_mul(3).unwrap()).checked_div(1000).unwrap();

        let balance_0_minus_fee = balance_0.checked_sub(fee_0).unwrap();
        let balance_1_minus_fee = balance_1.checked_sub(fee_1).unwrap();

        if balance_0_minus_fee.checked_mul(balance_1_minus_fee).unwrap() <
            reserve_0.checked_mul(reserve_1).unwrap() {
            panic!("SoroswapPair: K constant is not met")
        }

        update(&e, balance_0, balance_1, reserve_0.try_into().unwrap(), reserve_1.try_into().unwrap());
        event::swap(&e, &to, amount_0_in, amount_1_in, amount_0_out, amount_1_out, &to);
    }


    /// Withdraws liquidity from the Soroswap pair, burning LP tokens and returning the corresponding tokens to the user.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    /// * `to` - The address where the withdrawn tokens will be sent.
    ///
    /// # Returns
    /// A tuple containing the amounts of token 0 and token 1 withdrawn from the pair.
    fn withdraw(e: Env, to: Address) -> (i128, i128) {
        assert!(has_token_0(&e), "SoroswapPair: not yet initialized");

        let balance_shares = get_balance_shares(&e);
        if balance_shares == 0 {
            panic!("SoroswapPair: liquidity was not initialized yet");
        }

        let (mut reserve_0, mut reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        let (mut balance_0, mut balance_1) = (get_balance_0(&e), get_balance_1(&e));
        let user_sent_shares = balance_shares.checked_sub(MINIMUM_LIQUIDITY).unwrap();

        if user_sent_shares <= 0 {
            panic!("SoroswapPair: insufficient sent shares");
        }

        let fee_on: bool = mint_fee(&e, reserve_0, reserve_1);
        let total_shares = get_total_shares(&e);

        let amount_0 = (balance_0.checked_mul(user_sent_shares).unwrap()).checked_div(total_shares).unwrap();
        let amount_1 = (balance_1.checked_mul(user_sent_shares).unwrap()).checked_div(total_shares).unwrap();

        if amount_0 <= 0 || amount_1 <= 0 {
            panic!("SoroswapPair: insufficient liquidity burned");
        }

        burn_shares(&e, user_sent_shares);

        transfer_token_0_from_pair(&e, &to, amount_0);
        transfer_token_1_from_pair(&e, &to, amount_1);

        (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));

        update(&e, balance_0, balance_1, reserve_0.try_into().unwrap(), reserve_1.try_into().unwrap());

        (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        if fee_on {
            put_klast(&e, reserve_0.checked_mul(reserve_1).unwrap());
        }

        event::withdraw(&e, &to, user_sent_shares, amount_0, amount_1, &to);
        (amount_0, amount_1)
    }

    /// Skims excess tokens from reserves and sends them to the specified address.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    /// * `to` - The address where the excess tokens will be sent.
    fn skim(e: Env, to: Address) {
        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));
        let (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        transfer_token_0_from_pair(&e, &to, balance_0.checked_sub(reserve_0).unwrap());
        transfer_token_1_from_pair(&e, &to, balance_1.checked_sub(reserve_1).unwrap());
    }

    /// Forces reserves to match current balances.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    fn sync(e: Env) {
        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));
        let (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        update(&e, balance_0, balance_1, reserve_0.try_into().unwrap(), reserve_1.try_into().unwrap());
    }

    /// Returns the current reserves and the last block timestamp.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    ///
    /// # Returns
    /// A tuple containing the reserves of token 0 and token 1, along with the last block timestamp.
    fn get_reserves(e: Env) -> (i128, i128, u64) {
        (get_reserve_0(&e), get_reserve_1(&e), get_block_timestamp_last(&e))
    }

    /// Returns the total number of LP shares in circulation.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    ///
    /// # Returns
    /// The total number of LP shares.
    fn total_shares(e: Env) -> i128 {
        get_total_shares(&e)
    }

    /// Returns the balance of LP shares for a specific address.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    /// * `id` - The address for which the LP share balance is queried.
    ///
    /// # Returns
    /// The balance of LP shares for the specified address.
    fn my_balance(e: Env, id: Address) -> i128 {
        SoroswapPairToken::balance(e.clone(), id)
    }

    /// Returns the value of the last product of reserves (`K`) stored in the contract.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    ///
    /// # Returns
    /// The value of the last product of reserves (`K`).
    fn k_last(e: Env) -> i128 {
        get_klast(&e)
    }

    /// Returns the cumulative price of the first token since the last liquidity event.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    ///
    /// # Returns
    /// The cumulative price of the first token since the last liquidity event.
    fn price_0_cumulative_last(e: Env) -> u128 {
        get_price_0_cumulative_last(&e)
    }

    /// Returns the cumulative price of the second token since the last liquidity event.
    ///
    /// # Arguments
    /// * `e` - The runtime environment.
    ///
    /// # Returns
    /// The cumulative price of the second token since the last liquidity event.
    fn price_1_cumulative_last(e: Env) -> u128 {
        get_price_1_cumulative_last(&e)
    }
    
}




fn burn_shares(e: &Env, amount: i128) {
    let total = get_total_shares(e);
    internal_burn(e.clone(), e.current_contract_address(), amount);
    put_total_shares(&e, total.checked_sub(amount).unwrap());
}

fn mint_shares(e: &Env, to: &Address, amount: i128) {
    let total = get_total_shares(e);
    internal_mint(e.clone(), to.clone(), amount);
    //put_total_shares(e, total + amount);
    put_total_shares(&e, total.checked_add(amount).unwrap());
}


fn transfer(e: &Env, contract_id: Address, to: &Address, amount: i128) {
    any_token::TokenClient::new(e, &contract_id).transfer(&e.current_contract_address(), &to, &amount);
}

fn transfer_token_0_from_pair(e: &Env, to: &Address, amount: i128) {
    // Execute the transfer function in TOKEN_A to send "amount" of tokens from this Pair contract to "to"
    transfer(e, get_token_0(e), &to, amount);
}

fn transfer_token_1_from_pair(e: &Env, to: &Address, amount: i128) {
    transfer(e, get_token_1(e), &to, amount);
}

fn mint_fee(e: &Env, reserve_0: i128, reserve_1: i128) -> bool{

    /*
            accumulated fees are collected only when liquidity is deposited
            or withdrawn. The contract computes the accumulated fees, and mints new liquidity tokens
            to the fee beneficiary, immediately before any tokens are minted or burned 
    */

    let factory = get_factory(&e);
    let factory_client = SoroswapFactoryClient::new(&e, &factory);
    let fee_on = factory_client.fees_enabled();
    let klast = get_klast(&e);
     
    if fee_on{
        let fee_to: Address = factory_client.fee_to();

        if klast != 0 {
            let root_k = (reserve_0.checked_mul(reserve_1).unwrap()).sqrt();
            let root_klast = (klast).sqrt();
            if root_k > root_klast{
                let total_shares = get_total_shares(&e);
                let numerator = total_shares.checked_mul(root_k.checked_sub(root_klast).unwrap()).unwrap();
                let denominator = root_k.checked_mul(5_i128).unwrap().checked_add(root_klast).unwrap();
                let liquidity_pool_shares_fees = numerator.checked_div(denominator).unwrap();

                if liquidity_pool_shares_fees > 0 {
                    mint_shares(&e, &fee_to, liquidity_pool_shares_fees);
                }
            }
        }
    } else if klast != 0{
        put_klast(&e, 0);
    }

    fee_on
}

//function _update(uint balance0, uint balance1, uint112 _reserve0, uint112 _reserve1) private {
fn update(e: &Env, balance_0: i128, balance_1: i128, reserve_0: u64, reserve_1: u64) {
    // require(balance0 <= uint112(-1) && balance1 <= uint112(-1), 'UniswapV2: OVERFLOW');
    
    // Here we accept balances as i128, but we don't want them to be greater than the u64 MAX
    // This is becase u64 will be used to calculate the price as a UQ64x64
    let u_64_max: u64 = u64::MAX;
    let u_64_max_into_i128: i128 = u_64_max.into();

    if balance_0 > u_64_max_into_i128 {
        panic!("SoroswapPair: OVERFLOW")
    }
    if balance_1 > u_64_max_into_i128 {
        panic!("SoroswapPair: OVERFLOW")
    }

    // uint32 blockTimestamp = uint32(block.timestamp % 2**32);
    // In Uniswap this is done for gas usage optimization in Solidity. This will overflow in the year 2106. 
    // For Soroswap we can use u64, and will overflow in the year 2554,

    let block_timestamp: u64 = e.ledger().timestamp();
    let block_timestamp_last: u64 = get_block_timestamp_last(&e);

    // uint32 timeElapsed = blockTimestamp - blockTimestampLast; // overflow is desired
    let time_elapsed: u64 = block_timestamp - block_timestamp_last;

    // if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) {
    if time_elapsed > 0 && reserve_0 != 0 && reserve_1 != 0 {
        //     // * never overflows, and + overflow is desired
        //     price0CumulativeLast += uint(UQ112x112.encode(_reserve1).uqdiv(_reserve0)) * timeElapsed;
        //     price1CumulativeLast += uint(UQ112x112.encode(_reserve0).uqdiv(_reserve1)) * timeElapsed; 
        
        let price_0_cumulative_last: u128 = get_price_0_cumulative_last(&e);
        let price_1_cumulative_last: u128 = get_price_1_cumulative_last(&e);
        // TODO: Check in detail if this can or not overflow. We don't want functions to panic because of this
        put_price_0_cumulative_last(&e, price_0_cumulative_last + fraction(reserve_1, reserve_0).checked_mul(time_elapsed.into()).unwrap());
        put_price_1_cumulative_last(&e, price_1_cumulative_last + fraction(reserve_0, reserve_1).checked_mul(time_elapsed.into()).unwrap());
    }
    // reserve0 = uint112(balance0);
    // reserve1 = uint112(balance1);
    put_reserve_0(&e, balance_0);
    put_reserve_1(&e, balance_1);

    // blockTimestampLast = blockTimestamp;
    put_block_timestamp_last(&e, block_timestamp);

    // emit Sync(reserve0, reserve1);
    event::sync(&e, reserve_0, reserve_1);
}
