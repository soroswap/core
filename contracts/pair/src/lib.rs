#![no_std]
use soroban_sdk::{contract, contractimpl, contractmeta, Address, Env, IntoVal}; 
use soroban_sdk::token::Interface;
use num_integer::Roots; 
use soroswap_factory_interface::SoroswapFactoryClient;

mod test;
mod token;
mod event;
mod uq64x64;
mod storage;

// ANY TOKEN CONTRACT
mod any_token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}

use storage::*;
use token::{SoroswapPairToken, internal_mint, internal_burn};
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
    fn swap (e: Env, amount_0_out: i128, amount_1_out: i128, to: Address);

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
                "SOROSWAP-LP".into_val(&e)
            );

        put_token_0(&e, token_0);
        put_token_1(&e, token_1);
        put_total_shares(&e, 0);
        put_reserve_0(&e, 0);
        put_reserve_1(&e, 0);
    }

    fn token_0(e: Env) -> Address {
        get_token_0(&e)
    }

    fn token_1(e: Env) -> Address {
        get_token_1(&e)
    }

    fn factory(e: Env) -> Address {
        get_factory(&e)
    }

    fn deposit(e: Env, to: Address) -> i128 {
        //     (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings
        let (mut reserve_0, mut reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));

        //     uint balance0 = IERC20(token0).balanceOf(address(this));
        //     uint balance1 = IERC20(token1).balanceOf(address(this));
        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));

        //     uint amount0 = balance0.sub(_reserve0);
        let amount_0 = balance_0.checked_sub(reserve_0).unwrap();

        //     uint amount1 = balance1.sub(_reserve1);
        let amount_1 = balance_1.checked_sub(reserve_1).unwrap();

        //     bool feeOn = _mintFee(_reserve0, _reserve1);
        let fee_on: bool = mint_fee(&e, reserve_0, reserve_1);

        //  uint _totalSupply = totalSupply; // gas savings, must be defined here since totalSupply can update in _mintFee
        let total_shares = get_total_shares(&e);

        // if (_totalSupply == 0) {
        let liquidity = if total_shares == 0 {
            // _mint(address(0), MINIMUM_LIQUIDITY); // permanently lock the first MINIMUM_LIQUIDITY tokens
            // When the liquidity pool is being initialized, we block the minimum liquidity forever in this contract
            mint_shares(&e, &e.current_contract_address(), MINIMUM_LIQUIDITY); 
            // and liquidity get's this value:
            // liquidity = Math.sqrt(amount0.mul(amount1)).sub(MINIMUM_LIQUIDITY);
            ((amount_0.checked_mul(amount_1).unwrap()).sqrt()).checked_sub(MINIMUM_LIQUIDITY).unwrap()
        }
        else{
                // liquidity = Math.min(amount0.mul(_totalSupply) / _reserve0, amount1.mul(_totalSupply) / _reserve1);
                let shares_a = (amount_0.checked_mul(total_shares).unwrap()).checked_div(reserve_0).unwrap();
                let shares_b = (amount_1.checked_mul(total_shares).unwrap()).checked_div(reserve_1).unwrap();
                shares_a.min(shares_b)
        };
        

        // require(liquidity > 0, 'UniswapV2: INSUFFICIENT_LIQUIDITY_MINTED');
        if liquidity <= 0 { panic!("SoroswapPair: insufficient liquidity minted") }

        // _mint(to, liquidity);
        mint_shares(&e, &to, liquidity.clone());

        // _update(balance0, balance1, _reserve0, _reserve1);
        update(&e, balance_0, balance_1, reserve_0.try_into().unwrap(), reserve_1.try_into().unwrap());
       
        // Reserves where updated
        (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        // if (feeOn) kLast = uint(reserve0).mul(reserve1); // reserve0 and reserve1 are up-to-date
        if fee_on {
            put_klast(&e, reserve_0.checked_mul(reserve_1).unwrap());
        }

        //emit Mint(msg.sender, amount0, amount1);
        event::deposit(&e, &to, amount_0, amount_1);

        // returns (uint liquidity)
        liquidity
    }

    /// this low-level function should be called from a contract which performs important safety checks
    fn swap(e: Env, amount_0_out: i128, amount_1_out: i128, to: Address) {
        
        // require(amount0Out > 0 || amount1Out > 0, 'UniswapV2: INSUFFICIENT_OUTPUT_AMOUNT');
        if amount_0_out == 0 && amount_1_out == 0 { panic!("SoroswapPair: insufficient output amount") }
        if amount_0_out < 0 || amount_1_out < 0 { panic!("SoroswapPair: negatives dont supported") }

        // (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings
        let (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        
        // require(amount0Out < _reserve0 && amount1Out < _reserve1, 'UniswapV2: INSUFFICIENT_LIQUIDITY');
        if amount_0_out >= reserve_0|| amount_1_out >= reserve_1 { panic!("SoroswapPair: insufficient liquidity") }

        //     uint balance0;
        //     uint balance1;
        //     { // scope for _token{0,1}, avoids stack too deep errors
        //     address _token0 = token0;
        //     address _token1 = token1;

        //     require(to != _token0 && to != _token1, 'UniswapV2: INVALID_TO');
        if to == get_token_0(&e) || to == get_token_1(&e) {panic!("SoroswapPair: invalid to")}
        
        // if (amount0Out > 0) _safeTransfer(_token0, to, amount0Out); // optimistically transfer tokens
        // if (amount1Out > 0) _safeTransfer(_token1, to, amount1Out); // optimistically transfer tokens
        if amount_0_out > 0 {transfer_token_0_from_pair(&e, &to, amount_0_out);}
        if amount_1_out > 0 {transfer_token_1_from_pair(&e, &to, amount_1_out);}
            
        /*
            In Uniswap, Flashloans are allowed. In Soroban this is not possible. Here we don't need this line:
            // if (data.length > 0) IUniswapV2Callee(to).uniswapV2Call(msg.sender, amount0Out, amount1Out, data);
         */

        //     balance0 = IERC20(_token0).balanceOf(address(this));
        //     balance1 = IERC20(_token1).balanceOf(address(this));
        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));

        // uint amount0In = balance0 > _reserve0 - amount0Out ? balance0 - (_reserve0 - amount0Out) : 0;
        // uint amount1In = balance1 > _reserve1 - amount1Out ? balance1 - (_reserve1 - amount1Out) : 0;
        let amount_0_in = if balance_0 > reserve_0.checked_sub(amount_0_out).unwrap() {
            balance_0.checked_sub(reserve_0.checked_sub(amount_0_out).unwrap()).unwrap()
        } else{
            0
        };

        let amount_1_in = if balance_1 > reserve_1.checked_sub(amount_1_out).unwrap() {
            balance_1.checked_sub(reserve_1.checked_sub(amount_1_out).unwrap()).unwrap()
        } else{
            0
        };

        //     require(amount0In > 0 || amount1In > 0, 'UniswapV2: INSUFFICIENT_INPUT_AMOUNT');
        if amount_0_in == 0 && amount_1_in == 0 {panic!("SoroswapPair: insufficient input amount")}
        if amount_0_in < 0 || amount_1_in < 0 { panic!("SoroswapPair: negatives dont supported") }

        // uint balance0Adjusted = balance0.mul(1000).sub(amount0In.mul(3));
        // uint balance1Adjusted = balance1.mul(1000).sub(amount1In.mul(3));
        let balance_0_adjusted = balance_0.checked_mul(1000).unwrap().checked_sub(amount_0_in.checked_mul(3).unwrap()).unwrap();
        let balance_1_adjusted = balance_1.checked_mul(1000).unwrap().checked_sub(amount_1_in.checked_mul(3).unwrap()).unwrap();

        // require(balance0Adjusted.mul(balance1Adjusted) >= uint(_reserve0).mul(_reserve1).mul(1000**2), 'UniswapV2: K');
        if balance_0_adjusted.checked_mul(balance_1_adjusted).unwrap() <
            reserve_0.checked_mul(reserve_1).unwrap().checked_mul(1000_i128.pow(2)).unwrap() {
                panic!("SoroswapPair: K constant is not met")
            }

        // _update(balance0, balance1, _reserve0, _reserve1);
        update(&e, balance_0, balance_1, reserve_0.try_into().unwrap(), reserve_1.try_into().unwrap());

        // emit Swap(msg.sender, amount0In, amount1In, amount0Out, amount1Out, to);
        event::swap(&e, &to, amount_0_in, amount_1_in, amount_0_out, amount_1_out, &to);


    }

    /// this low-level function should be called from a contract which performs important safety checks
    fn withdraw(e: Env, to: Address) -> (i128, i128) { // returns (uint amount0, uint amount1)
        // (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings
        let (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));

        // uint balance0 = IERC20(_token0).balanceOf(address(this));
        // uint balance1 = IERC20(_token1).balanceOf(address(this));
        let (mut balance_0, mut balance_1) = (get_balance_0(&e), get_balance_1(&e));

        // uint liquidity = balanceOf[address(this)];
        // The contract's LP token balance; that should be the ones that the user sent
        let user_sent_shares = get_balance_shares(&e).checked_sub(MINIMUM_LIQUIDITY).unwrap();

        // bool feeOn = _mintFee(_reserve0, _reserve1);
        let fee_on: bool = mint_fee(&e, reserve_0, reserve_1);

        // uint _totalSupply = totalSupply; // gas savings, must be defined here since totalSupply can update in _mintFee
        let total_shares = get_total_shares(&e);

        // amount0 = liquidity.mul(balance0) / _totalSupply; // using balances ensures pro-rata distribution
        // amount1 = liquidity.mul(balance1) / _totalSupply; // using balances ensures pro-rata distribution
        let amount_0 = (balance_0.checked_mul(user_sent_shares).unwrap()).checked_div(total_shares).unwrap();
        let amount_1 = (balance_1.checked_mul(user_sent_shares).unwrap()).checked_div(total_shares).unwrap();

        // require(amount0 > 0 && amount1 > 0, 'UniswapV2: INSUFFICIENT_LIQUIDITY_BURNED');
        if amount_0 <= 0 || amount_1 <= 0 {
            panic!("SoroswapPair: insufficient liquidity burned");
        }

        // _burn(address(this), liquidity);
        burn_shares(&e, user_sent_shares);

        // _safeTransfer(_token0, to, amount0);
        // _safeTransfer(_token1, to, amount1);
        transfer_token_0_from_pair(&e, &to, amount_0);
        transfer_token_1_from_pair(&e, &to, amount_1);

        // The Pair balances have changed
        // balance0 = IERC20(_token0).balanceOf(address(this));
        // balance1 = IERC20(_token1).balanceOf(address(this));
        (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));

        // _update(balance0, balance1, _reserve0, _reserve1);
        update(&e, balance_0, balance_1, reserve_0.try_into().unwrap(), reserve_1.try_into().unwrap());

        // if (feeOn) kLast = uint(reserve0).mul(reserve1); // reserve0 and reserve1 are up-to-date
        if fee_on {
            put_klast(&e, reserve_0.checked_mul(reserve_1).unwrap());
        }
        // emit Burn(msg.sender, amount0, amount1, to);
        event::withdraw(&e, &to, user_sent_shares, amount_0, amount_1, &to);

        // returns (uint amount0, uint amount1)
        (amount_0,amount_1)
    }

    /// force balances to match reserves
    fn skim(e: Env, to: Address) {
        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));
        let (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        transfer_token_0_from_pair(&e, &to, balance_0.checked_sub(reserve_0).unwrap());
        transfer_token_1_from_pair(&e, &to, balance_1.checked_sub(reserve_1).unwrap());
    }

    /// force reserves to match balances
    fn sync(e: Env) {
        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));
        let (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        update(&e, balance_0, balance_1, reserve_0.try_into().unwrap(), reserve_1.try_into().unwrap());
    }

    fn get_reserves(e: Env) -> (i128, i128, u64) {
        (get_reserve_0(&e), get_reserve_1(&e), get_block_timestamp_last(&e))
    }

    fn total_shares(e: Env) -> i128 {
        get_total_shares(&e)
    }

    fn my_balance(e: Env, id: Address) -> i128 {
        SoroswapPairToken::balance(e.clone(), id)
    }

    fn k_last(e: Env) -> i128 {
        get_klast(&e)
    }

    fn price_0_cumulative_last(e: Env) -> u128 {
        get_price_0_cumulative_last(&e)
    }
    fn price_1_cumulative_last(e: Env) -> u128 {
        get_price_1_cumulative_last(&e)
    }
    
}



fn get_balance(e: &Env, contract_id: Address) -> i128 {
    // How many "contract_id" tokens does this contract holds?
    // We need to implement the token client
    any_token::TokenClient::new(e, &contract_id).balance(&e.current_contract_address())
}

fn get_balance_0(e: &Env) -> i128 {
    // How many "A TOKENS" does the Liquidity Pool holds?
    // How many "A TOKENS" does this contract holds?
    get_balance(e, get_token_0(e))
}

fn get_balance_1(e: &Env) -> i128 {
    get_balance(e, get_token_1(e))
}

fn get_balance_shares(e: &Env) -> i128 {
    // How many "SHARE" tokens does the Liquidity pool holds?
    // This shares should have been sent by the user when burning their LP positions (withdraw)
    SoroswapPairToken::balance(e.clone(), e.current_contract_address())
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
    //  address feeTo = IUniswapV2Factory(factory).feeTo();
    //  feeOn = feeTo != address(0);
    let fee_on = factory_client.fees_enabled();
    let klast = get_klast(&e);
     
    if fee_on{
        let fee_to: Address = factory_client.fee_to();

        if klast != 0 {
            let root_k = (reserve_0.checked_mul(reserve_1).unwrap()).sqrt();
            let root_klast = (klast).sqrt();
            if root_k > root_klast{
                // uint numerator = totalSupply.mul(rootK.sub(rootKLast));
                let total_shares = get_total_shares(&e);
                let numerator = total_shares.checked_mul(root_k.checked_sub(root_klast).unwrap()).unwrap();
        
                // uint denominator = rootK.mul(5).add(rootKLast);
                let denominator = root_k.checked_mul(5_i128).unwrap().checked_add(root_klast).unwrap();
                // uint liquidity = numerator / denominator;

                let liquidity_pool_shares_fees = numerator.checked_div(denominator).unwrap();

                // if (liquidity > 0) _mint(feeTo, liquidity);
                if liquidity_pool_shares_fees > 0 {
                    mint_shares(&e, &fee_to,    liquidity_pool_shares_fees);
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
        panic!("Soroswap: OVERFLOW")
    }
    if balance_1 > u_64_max_into_i128 {
        panic!("Soroswap: OVERFLOW")
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
