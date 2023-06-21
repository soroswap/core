#![no_std]

mod test;
mod token;
mod create;
mod event;
mod factory;
mod uq64x64;

use num_integer::Roots;
use soroban_sdk::{contractimpl, Address, Bytes, ConversionError, Env, RawVal, TryFromVal};
use token::{Token, TokenTrait, TokenClient, internal_mint, internal_burn};
use factory::{FactoryClient};
use uq64x64::fraction;

static MINIMUM_LIQUIDITY: i128 = 1000;

#[derive(Clone, Copy)] 
#[repr(u32)]
/*
TODO: Analize UniswapV2
    // Use kind of SafeMath?
    // using UQ112x112 for uint224;
    // uint public constant MINIMUM_LIQUIDITY = 10**3;
    // bytes4 private constant SELECTOR = bytes4(keccak256(bytes('transfer(address,uint256)')));
    // uint32  private blockTimestampLast; // uses single storage slot, accessible via getReserves
    // uint public price0CumulativeLast;
    // uint public price1CumulativeLast;
    // uint public kLast; // reserve0 * reserve1, as of immediately after the most recent liquidity event

TODO: Analize reentrancy attack guard?

    uint private unlocked = 1;
    modifier lock() {
        require(unlocked == 1, 'UniswapV2: LOCKED');
        unlocked = 0;
        _;
        unlocked = 1;
    }
*/


pub enum DataKey {
    Token0 = 0, // address public token0;
    Token1 = 1, // address public token1;
    Reserve0 = 2, //uint112 private reserve0;
    Reserve1 = 3, // uint112 private reserve1;
    Factory = 4, 
    TotalShares = 5, // TODO: Delete when implementing the token interface,
    BlockTimestampLast = 6, // accessible via getReserves,
    Price0CumulativeLast = 7, // uint public price0CumulativeLast;
    Price1CumulativeLast = 8, // uint public price1CumulativeLast;
    KLast = 9

}


impl TryFromVal<Env, DataKey> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

fn get_factory(e: &Env) -> Address {
    e.storage().get_unchecked(&DataKey::Factory).unwrap()
}

fn get_token_0(e: &Env) -> Address {
    e.storage().get_unchecked(&DataKey::Token0).unwrap()
}

fn get_token_1(e: &Env) -> Address {
    e.storage().get_unchecked(&DataKey::Token1).unwrap()
}

fn get_total_shares(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::TotalShares).unwrap()
}

// // Get reserves functions
// function getReserves() public view returns (uint112 _reserve0, uint112 _reserve1, uint32 _blockTimestampLast) {
//     _reserve0 = reserve0;
//     _reserve1 = reserve1;
//     _blockTimestampLast = blockTimestampLast;
// }

fn get_reserve_0(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::Reserve0).unwrap()
}

fn get_reserve_1(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::Reserve1).unwrap()
}

fn get_block_timestamp_last(e: &Env) -> u64 {
    if let Some(block_timestamp_last) = e.storage().get(&DataKey::BlockTimestampLast) {
        block_timestamp_last.unwrap()
    } else {
        0
    }
}

fn get_price_0_cumulative_last(e: &Env) -> u128 {
    if let Some(price) = e.storage().get(&DataKey::Price0CumulativeLast) {
        price.unwrap()
    } else {
        0
    }
}

fn get_price_1_cumulative_last(e: &Env) -> u128 {
    if let Some(price) = e.storage().get(&DataKey::Price1CumulativeLast) {
        price.unwrap()
    } else {
        0
    }
}

fn get_klast(e: &Env) -> i128 {
    if let Some(klast) = e.storage().get(&DataKey::KLast) {
        klast.unwrap()
    } else {
        0
    }
}

fn get_balance(e: &Env, contract_id: Address) -> i128 {
    // How many "contract_id" tokens does this contract holds?
    // We need to implement the token client
    TokenClient::new(e, &contract_id).balance(&e.current_contract_address())
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
    Token::balance(e.clone(), e.current_contract_address())
}

fn put_factory(e: &Env, factory: Address) {
    e.storage().set(&DataKey::Factory, &factory);
}

fn put_token_0(e: &Env, contract_id: Address) {
    e.storage().set(&DataKey::Token0, &contract_id);
}

fn put_token_1(e: &Env, contract_id: Address) {
    e.storage().set(&DataKey::Token1, &contract_id);
}

fn put_total_shares(e: &Env, amount: i128) {
    e.storage().set(&DataKey::TotalShares, &amount)
}

fn put_reserve_0(e: &Env, amount: i128) {
    if amount < 0 {
        panic!("put_reserve_0: amount cannot be negative")
    }
    e.storage().set(&DataKey::Reserve0, &amount)
}

fn put_reserve_1(e: &Env, amount: i128) {
    if amount < 0 {
        panic!("put_reserve_1: amount cannot be negative")
    }
    e.storage().set(&DataKey::Reserve1, &amount)
}

fn put_block_timestamp_last(e: &Env, block_timestamp_last: u64) {
    e.storage().set(&DataKey::BlockTimestampLast, &block_timestamp_last);
}

fn put_price_0_cumulative_last(e: &Env, price_0_cumulative_last: u128) {
    e.storage().set(&DataKey::Price0CumulativeLast, &price_0_cumulative_last);
}

fn put_price_1_cumulative_last(e: &Env, price_1_cumulative_last: u128) {
    e.storage().set(&DataKey::Price1CumulativeLast, &price_1_cumulative_last);
}

fn put_klast(e: &Env, klast: i128) {
    e.storage().set(&DataKey::KLast, &klast);
}

fn burn_shares(e: &Env, amount: i128) {
    let total = get_total_shares(e);
    internal_burn(e.clone(), e.current_contract_address(), amount);
    put_total_shares(e, total.checked_sub(amount).unwrap());
}

fn mint_shares(e: &Env, to: Address, amount: i128) {
    let total = get_total_shares(e);
    internal_mint(e.clone(), to, amount);
    //put_total_shares(e, total + amount);
    put_total_shares(e, total.checked_add(amount).unwrap());
}


// // Safe transfer: Solidity Specific
// function _safeTransfer(address token, address to, uint value) private {
//     (bool success, bytes memory data) = token.call(abi.encodeWithSelector(SELECTOR, to, value));
//     require(success && (data.length == 0 || abi.decode(data, (bool))), 'UniswapV2: TRANSFER_FAILED');
// }

fn transfer(e: &Env, contract_id: Address, to: Address, amount: i128) {
    TokenClient::new(e, &contract_id).transfer(&e.current_contract_address(), &to, &amount);
}

fn transfer_token_0_from_pair(e: &Env, to: Address, amount: i128) {
    // Execute the transfer function in TOKEN_A to send "amount" of tokens from this Pair contract to "to"
    transfer(e, get_token_0(e), to, amount);
}

fn transfer_token_1_from_pair(e: &Env, to: Address, amount: i128) {
    transfer(e, get_token_1(e), to, amount);
}

fn get_deposit_amounts(
    desired_a: i128,
    min_a: i128,
    desired_b: i128,
    min_b: i128,
    reserve_0: i128,
    reserve_1: i128,
) -> (i128, i128) {
    // Compare it with UniswapV2 Router
    if reserve_0 == 0 && reserve_1 == 0 {
        return (desired_a, desired_b);
    }

    //let amount_b = desired_a * reserve_1
    let amount_b = desired_a.checked_mul(reserve_1).unwrap().checked_div(reserve_0).unwrap();
    if amount_b <= desired_b {
        if amount_b < min_b {
            panic!("amount_b less than min")
        }
        (desired_a, amount_b)
    } else {
        //let amount_a = desired_b * reserve_0 / reserve_1;
        let amount_a = desired_b.checked_mul(reserve_0).unwrap().checked_div(reserve_1).unwrap();
        if amount_a > desired_a || desired_a < min_a {
            panic!("amount_a invalid")
        }
        (amount_a, desired_b)
    }
}

/*
        accumulated fees are collected only when liquidity is deposited
        or withdrawn. The contract computes the accumulated fees, and mints new liquidity tokens
        to the fee beneficiary, immediately before any tokens are minted or burned 
*/

fn mint_fee(e: &Env, reserve_0: i128, reserve_1: i128) -> bool{
    let factory = get_factory(&e);
    let factory_client = FactoryClient::new(&e, &factory);
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
                    mint_shares(&e, fee_to,    liquidity_pool_shares_fees);
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

pub trait SoroswapPairTrait{
    // Sets the token contract addresses for this pool
    fn initialize_pair(e: Env, factory: Address, token_a: Address, token_b: Address);

    // Deposits token_a and token_b. Also mints pool shares for the "to" Identifier. The amount minted
    // is determined based on the difference between the reserves stored by this contract, and
    // the actual balance of token_a and token_b for this contract.
    fn deposit(e: Env, to: Address, desired_a: i128, min_a: i128, desired_b: i128, min_b: i128);

    // If "buy_a" is true, the swap will buy token_a and sell token_b. This is flipped if "buy_a" is false.
    // "out" is the amount being bought, with amount_in_max being a safety to make sure you receive at least that amount.
    // swap will transfer the selling token "to" to this contract, and then the contract will transfer the buying token to "to".
    fn swap(e: Env, to: Address, buy_a: bool, amount_out: i128, amount_in_max: i128);

    // transfers share_amount of pool share tokens to this contract, burns all pools share tokens in this contracts, and sends the
    // corresponding amount of token_a and token_b to "to".
    // Returns amount of both tokens withdrawn
    fn withdraw(e: Env, to: Address, share_amount: i128, min_a: i128, min_b: i128) -> (i128, i128);

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

struct SoroswapPair;

#[contractimpl]
impl SoroswapPairTrait for SoroswapPair {
    
    // TODO: Implement name for pairs depending on the tokens
    // TODO: This cannot be called again
    fn initialize_pair(e: Env, factory: Address, token_a: Address, token_b: Address) {
        if token_a >= token_b {
            panic!("token_a must be less than token_b");
        }
        put_factory(&e, factory);

        Token::initialize(
                e.clone(),
                e.current_contract_address(),
                7,
                Bytes::from_slice(&e, b"Soroswap Pair Token"),
                Bytes::from_slice(&e, b"SOROSWAP-LP"),
            );

        put_token_0(&e, token_a);
        put_token_1(&e, token_b);
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

    fn deposit(e: Env, to: Address, desired_a: i128, min_a: i128, desired_b: i128, min_b: i128) {
        // Depositor needs to authorize the deposit
        to.require_auth();
        let (mut reserve_0, mut reserve_1) = (get_reserve_0(&e), get_reserve_1(&e)); 

        // TODO: Implement this after creating the SoroswapRouter. For now tokens are being sent here using auth
        //     uint balance0 = IERC20(token0).balanceOf(address(this));
        //     uint balance1 = IERC20(token1).balanceOf(address(this));
        //     uint amount0 = balance0.sub(_reserve0);
        //     uint amount1 = balance1.sub(_reserve1);

        // Calculate deposit amounts --> compare it with UniswapV2Router
        let amounts = get_deposit_amounts(desired_a, min_a, desired_b, min_b, reserve_0, reserve_1);
        let token_a_client = TokenClient::new(&e, &get_token_0(&e));
        let token_b_client = TokenClient::new(&e, &get_token_1(&e));
        token_a_client.transfer(&to, &e.current_contract_address(), &amounts.0);
        token_b_client.transfer(&to, &e.current_contract_address(), &amounts.1); 

        let fee_on: bool = mint_fee(&e, reserve_0, reserve_1);
        let total_shares = get_total_shares(&e);

        // Now calculate how many new pool shares to mint
        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));
        let zero = 0; 
        let new_total_shares = if total_shares > zero {
            let shares_a = (balance_0.checked_mul(total_shares).unwrap()).checked_div(reserve_0).unwrap();
            let shares_b = (balance_1.checked_mul(total_shares).unwrap()).checked_div(reserve_1).unwrap();
            shares_a.min(shares_b)
        } else {
            // When the liquidity pool is being initialized, we block the minimum liquidity forever in this contract
            
            mint_shares(&e, e.current_contract_address(), MINIMUM_LIQUIDITY);    
            ((balance_0.checked_mul(balance_1).unwrap()).sqrt()).checked_sub(MINIMUM_LIQUIDITY).unwrap()
        };  

        mint_shares(&e, to.clone(), new_total_shares.checked_sub(total_shares).unwrap());
        update(&e, balance_0, balance_1, reserve_0.try_into().unwrap(), reserve_1.try_into().unwrap());
        
        (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e)); 
        if fee_on {
            put_klast(&e, reserve_0.checked_mul(reserve_1).unwrap());
        }
        event::deposit(&e, to, amounts.0, amounts.1);
    }


    fn swap(e: Env, to: Address, buy_0: bool, amount_out: i128, amount_in_max: i128) {
        to.require_auth();

        /*
        UniswapV2 implements 2 things that Soroswap it's not going to implement for now:
        1.- FlashSwaps. Soroban is not allowing reentrancy for the momennt. So no data as a parameter.
        2.- uint amount0Out as parameter. Soroswap will impleent all the logig in the Router contract.

        All this logic will change in this contract when the Router contract is implemented
        */
        
        if amount_out <= 0 { panic!("insufficient output amount") }
        if to == get_token_0(&e) || to == get_token_1(&e) {panic!("invalid to")}
        
        
        let (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        let (reserve_in, reserve_out) = if buy_0 {
            (reserve_1, reserve_0)
        } else {
            (reserve_0, reserve_1)
        };
        
        // First calculate how much needs to be sold to buy amount amount_out from the pool
        let n = reserve_in.checked_mul(amount_out).unwrap().checked_mul(1000).unwrap();
        let d = (reserve_out.checked_sub(amount_out).unwrap()).checked_mul(997).unwrap();
        let amount_in = (n.checked_div(d).unwrap()).checked_add(1).unwrap();

        if amount_in > amount_in_max {panic!("amount in is over max") }
        if amount_in <= 0 { panic!("insufficient input amount")}
        
        // Transfer the amount_in being sold to the contract
        let sell_token = if buy_0 { get_token_1(&e) } else { get_token_0(&e) };
        let sell_token_client = TokenClient::new(&e, &sell_token);
        sell_token_client.transfer(&to, &e.current_contract_address(), &amount_in);

        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));

        // residue_numerator and residue_denominator are the amount that the invariant considers after
        // deducting the fee, scaled up by 1000 to avoid fractions
        let residue_numerator: i128 = 997;
        let residue_denominator: i128 = 1000;
        let zero = 0;

        let new_invariant_factor = |balance: i128, reserve: i128, amount_out: i128| {
            let delta = balance.checked_sub(reserve).unwrap().checked_sub(amount_out).unwrap();
            let adj_delta = if delta > zero {
                //residue_numerator * delta
                residue_numerator.checked_mul(delta).unwrap()
            } else {
              //  residue_denominator * delta
                residue_denominator.checked_mul(delta).unwrap()
            };
            //residue_denominator * reserve + adj_delta
            residue_denominator.checked_mul(reserve).unwrap().checked_add(adj_delta).unwrap()
        };

        let (amount_0_in, amount_1_in) = if buy_0 { (0, amount_in) } else { (amount_in, 0) };
        let (amount_0_out, amount_1_out) = if buy_0 { (amount_out, 0) } else { (0, amount_out) };

        let new_inv_a = new_invariant_factor(balance_0, reserve_0, amount_0_out);
        let new_inv_b = new_invariant_factor(balance_1, reserve_1, amount_1_out);
        //let old_inv_a = residue_denominator * reserve_0;
        let old_inv_a = residue_denominator.checked_mul(reserve_0).unwrap();
        //let old_inv_b = residue_denominator * reserve_1;
        let old_inv_b = residue_denominator.checked_mul(reserve_1).unwrap();

        // if new_inv_a * new_inv_b < old_inv_a  * old_inv_b {
        if new_inv_a.checked_mul(new_inv_b).unwrap() < old_inv_a.checked_mul(old_inv_b).unwrap() {
            panic!("constant product invariant does not hold");
        }

        if buy_0 {
            transfer_token_0_from_pair(&e, to.clone(), amount_0_out);
        } else {
            transfer_token_1_from_pair(&e, to.clone(), amount_1_out);
        }

        let new_balance_0 = balance_0.checked_sub(amount_0_out).unwrap();
        let new_balance_1 = balance_1.checked_sub(amount_1_out).unwrap();
        update(&e, new_balance_0, new_balance_1, reserve_0.try_into().unwrap(), reserve_1.try_into().unwrap());
        event::swap(&e, to.clone(), amount_0_in, amount_1_in, amount_0_out, amount_1_out, to);
    }

    fn withdraw(e: Env, to: Address, share_amount: i128, min_a: i128, min_b: i128) -> (i128, i128) {
        to.require_auth();
        // We get the original reserves before the action:
        let (mut reserve_0, mut reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        
        /*
        For now we are sending the pair token to the contract here.
        This will change with a Router contract that will send the tokens to us.
        */
        Token::transfer(e.clone(), to.clone(), e.current_contract_address(), share_amount);
        // address _token0 = token0;                                // gas savings
        // address _token1 = token1;                                // gas savings
        // uint balance0 = IERC20(_token0).balanceOf(address(this));
        // uint balance1 = IERC20(_token1).balanceOf(address(this));
        // uint liquidity = balanceOf[address(this)];
        let (mut balance_0, mut balance_1) = (get_balance_0(&e), get_balance_1(&e));
        let user_sent_shares = get_balance_shares(&e).checked_sub(MINIMUM_LIQUIDITY).unwrap();
        
        // bool feeOn = _mintFee(_reserve0, _reserve1);
        let fee_on: bool = mint_fee(&e, reserve_0, reserve_1);

        // uint _totalSupply = totalSupply; // gas savings, must be defined here since totalSupply can update in _mintFee
        let total_shares = get_total_shares(&e);

        // Now calculate the withdraw amounts
        let out_0 = (balance_0.checked_mul(user_sent_shares).unwrap()).checked_div(total_shares).unwrap();
        let out_1 = (balance_1.checked_mul(user_sent_shares).unwrap()).checked_div(total_shares).unwrap();

        if out_0 <= 0 || out_1 <= 0 {
            panic!("insufficient amount_0 or amount_1");
        }

        // TODO: In the next iteration this should be in the Router contract
        if out_0 < min_a || out_1 < min_b {
            panic!("min not satisfied");
        }

        // _burn(address(this), liquidity);
        burn_shares(&e, user_sent_shares);
        transfer_token_0_from_pair(&e, to.clone(), out_0.clone());
        transfer_token_1_from_pair(&e, to.clone(), out_1.clone());
        (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));

        // _update(balance0, balance1, _reserve0, _reserve1);
        update(&e, balance_0, balance_1, reserve_0.try_into().unwrap(), reserve_1.try_into().unwrap());
        // Update reserve_0 and reserve_1 after being updated in update() function:
        (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e)); 
        // if (feeOn) kLast = uint(reserve0).mul(reserve1); // reserve0 and reserve1 are up-to-date
        if fee_on {
            put_klast(&e, reserve_0.checked_mul(reserve_1).unwrap());
        }

        event::withdraw(&e, to.clone(), user_sent_shares, out_0, out_1, to);
      
        (out_0, out_1)
    }

    // force balances to match reserves
    fn skim(e: Env, to: Address) {
        let (balance_0, balance_1) = (get_balance_0(&e), get_balance_1(&e));
        let (reserve_0, reserve_1) = (get_reserve_0(&e), get_reserve_1(&e));
        transfer_token_0_from_pair(&e, to.clone(), balance_0.checked_sub(reserve_0).unwrap());
        transfer_token_1_from_pair(&e, to, balance_1.checked_sub(reserve_1).unwrap());
    }

    // force reserves to match balances
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
        Token::balance(e.clone(), id)
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
