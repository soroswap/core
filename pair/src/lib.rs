#![no_std]

mod test;
mod token;
mod create;
mod event;

use num_integer::Roots;
use soroban_sdk::{contractimpl, Address, Bytes, BytesN, ConversionError, Env, RawVal, TryFromVal};
use token::{Token, TokenTrait, TokenClient, internal_mint, internal_burn};


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
    BlockTimestampLast = 6, // accessible via getReserves

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

fn get_token_0(e: &Env) -> BytesN<32> {
    e.storage().get_unchecked(&DataKey::Token0).unwrap()
}

fn get_token_1(e: &Env) -> BytesN<32> {
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

fn get_reserve_a(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::Reserve0).unwrap()
}

fn get_reserve_b(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::Reserve1).unwrap()
}

fn get_block_timestamp_last(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::BlockTimestampLast).unwrap()
}

fn get_balance(e: &Env, contract_id: BytesN<32>) -> i128 {
    // How many "contract_id" tokens does this contract holds?
    // We need to implement the token client
    TokenClient::new(e, &contract_id).balance(&e.current_contract_address())
}

fn get_balance_a(e: &Env) -> i128 {
    // How many "A TOKENS" does the Liquidity Pool holds?
    // How many "A TOKENS" does this contract holds?
    get_balance(e, get_token_0(e))
}

fn get_balance_b(e: &Env) -> i128 {
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

fn put_token_a(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(&DataKey::Token0, &contract_id);
}

fn put_token_b(e: &Env, contract_id: BytesN<32>) {
    e.storage().set(&DataKey::Token1, &contract_id);
}

fn put_total_shares(e: &Env, amount: i128) {
    e.storage().set(&DataKey::TotalShares, &amount)
}

fn put_reserve_a(e: &Env, amount: i128) {
    if amount < 0 {
        panic!("put_reserve_a: amount cannot be negative")
    }
    e.storage().set(&DataKey::Reserve0, &amount)
}

fn put_reserve_b(e: &Env, amount: i128) {
    if amount < 0 {
        panic!("put_reserve_a: amount cannot be negative")
    }
    e.storage().set(&DataKey::Reserve1, &amount)
}

fn put_block_timestamp_last(e: &Env) {
    e.storage().set(&DataKey::BlockTimestampLast, &e.ledger().timestamp());
}

fn burn_shares(e: &Env, amount: i128) {
    let total = get_total_shares(e);
    internal_burn(e.clone(), e.current_contract_address(), amount);
    put_total_shares(e, total - amount);
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

fn transfer(e: &Env, contract_id: BytesN<32>, to: Address, amount: i128) {
    TokenClient::new(e, &contract_id).xfer(&e.current_contract_address(), &to, &amount);
}

fn transfer_a(e: &Env, to: Address, amount: i128) {
    // Execute the transfer function in TOKEN_A to send "amount" of tokens from this Pair contract to "to"
    transfer(e, get_token_0(e), to, amount);
}

fn transfer_b(e: &Env, to: Address, amount: i128) {
    transfer(e, get_token_1(e), to, amount);
}

fn get_deposit_amounts(
    desired_a: i128,
    min_a: i128,
    desired_b: i128,
    min_b: i128,
    reserve_a: i128,
    reserve_b: i128,
) -> (i128, i128) {
    if reserve_a == 0 && reserve_b == 0 {
        return (desired_a, desired_b);
    }

    //let amount_b = desired_a * reserve_b
    let amount_b = desired_a.checked_mul(reserve_b).unwrap().checked_div(reserve_a).unwrap();
    if amount_b <= desired_b {
        if amount_b < min_b {
            panic!("amount_b less than min")
        }
        (desired_a, amount_b)
    } else {
        //let amount_a = desired_b * reserve_a / reserve_b;
        let amount_a = desired_b.checked_mul(reserve_a).unwrap().checked_div(reserve_b).unwrap();
        if amount_a > desired_a || desired_a < min_a {
            panic!("amount_a invalid")
        }
        (amount_a, desired_b)
    }
}

pub trait SoroswapPairTrait{
    // Sets the token contract addresses for this pool
    fn initialize_pair(e: Env, factory: Address, token_a: BytesN<32>, token_b: BytesN<32>);

    fn token_0(e: Env) -> BytesN<32>;
    fn token_1(e: Env) -> BytesN<32>;

    // Deposits token_a and token_b. Also mints pool shares for the "to" Identifier. The amount minted
    // is determined based on the difference between the reserves stored by this contract, and
    // the actual balance of token_a and token_b for this contract.
    fn deposit(e: Env, to: Address, desired_a: i128, min_a: i128, desired_b: i128, min_b: i128);

    // If "buy_a" is true, the swap will buy token_a and sell token_b. This is flipped if "buy_a" is false.
    // "out" is the amount being bought, with in_max being a safety to make sure you receive at least that amount.
    // swap will transfer the selling token "to" to this contract, and then the contract will transfer the buying token to "to".
    fn swap(e: Env, to: Address, buy_a: bool, out: i128, in_max: i128);

    // transfers share_amount of pool share tokens to this contract, burns all pools share tokens in this contracts, and sends the
    // corresponding amount of token_a and token_b to "to".
    // Returns amount of both tokens withdrawn
    fn withdraw(e: Env, to: Address, share_amount: i128, min_a: i128, min_b: i128) -> (i128, i128);

    fn get_reserves(e: Env) -> (i128, i128, i128);

    fn my_balance(e: Env, id: Address) -> i128;

    fn factory(e: Env) -> Address;
}

struct SoroswapPair;

#[contractimpl]
impl SoroswapPairTrait for SoroswapPair {
    // initialize
    // // Constructor. Can be constructed my any contract
    // constructor() public {
    //     factory = msg.sender;
    // }
    // // called once by the factory at time of deployment
    // function initialize(address _token0, address _token1) external {
    //     require(msg.sender == factory, 'UniswapV2: FORBIDDEN'); // sufficient check
    //     token0 = _token0;
    //     token1 = _token1; 
    // }

    // TODO: Implement name for pairs depending on the tokens
    // TODO: This cannot be called again
    fn initialize_pair(e: Env, factory: Address, token_a: BytesN<32>, token_b: BytesN<32>) {
        if token_a >= token_b {
            panic!("token_a must be less than token_b");
        }
        put_factory(&e, factory);

        Token::initialize(
                e.clone(),
                e.current_contract_address(),
                7,
                Bytes::from_slice(&e, b"Pool Share Token"),
                Bytes::from_slice(&e, b"POOL"),
            );

        put_token_a(&e, token_a);
        put_token_b(&e, token_b);
        put_total_shares(&e, 0);
        put_reserve_a(&e, 0);
        put_reserve_b(&e, 0);


    }


    fn token_0(e: Env) -> BytesN<32> {
        get_token_0(&e)
    }

    fn token_1(e: Env) -> BytesN<32> {
        get_token_1(&e)
    }

    

    fn factory(e: Env) -> Address {
        get_factory(&e)
    }

// TODO: Analize Uniswap V2
// // update reserves and, on the first call per block, price accumulators
// function _update(uint balance0, uint balance1, uint112 _reserve0, uint112 _reserve1) private {
//     require(balance0 <= uint112(-1) && balance1 <= uint112(-1), 'UniswapV2: OVERFLOW');
//     uint32 blockTimestamp = uint32(block.timestamp % 2**32);
//     uint32 timeElapsed = blockTimestamp - blockTimestampLast; // overflow is desired
//     if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) {
//         // * never overflows, and + overflow is desired
//         price0CumulativeLast += uint(UQ112x112.encode(_reserve1).uqdiv(_reserve0)) * timeElapsed;
//         price1CumulativeLast += uint(UQ112x112.encode(_reserve0).uqdiv(_reserve1)) * timeElapsed;
//     }
//     reserve0 = uint112(balance0);
//     reserve1 = uint112(balance1);
//     blockTimestampLast = blockTimestamp;
//     emit Sync(reserve0, reserve1);
// }

// // if fee is on, mint liquidity equivalent to 1/6th of the growth in sqrt(k)
// function _mintFee(uint112 _reserve0, uint112 _reserve1) private returns (bool feeOn) {
//     address feeTo = IUniswapV2Factory(factory).feeTo();
//     feeOn = feeTo != address(0);
//     uint _kLast = kLast; // gas savings
//     if (feeOn) {
//         if (_kLast != 0) {
//             uint rootK = Math.sqrt(uint(_reserve0).mul(_reserve1));
//             uint rootKLast = Math.sqrt(_kLast);
//             if (rootK > rootKLast) {
//                 uint numerator = totalSupply.mul(rootK.sub(rootKLast));
//                 uint denominator = rootK.mul(5).add(rootKLast);
//                 uint liquidity = numerator / denominator;
//                 if (liquidity > 0) _mint(feeTo, liquidity);
//             }
//         }
//     } else if (_kLast != 0) {
//         kLast = 0;
//     }
// }


// The deposit function in UniswapV2Pair is called "mint"

//  // this low-level function should be called from a contract which performs important safety checks
//  function mint(address to) external lock returns (uint liquidity) {
//     (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings
//     uint balance0 = IERC20(token0).balanceOf(address(this));
//     uint balance1 = IERC20(token1).balanceOf(address(this));
//     uint amount0 = balance0.sub(_reserve0);
//     uint amount1 = balance1.sub(_reserve1);

//     bool feeOn = _mintFee(_reserve0, _reserve1);
//     uint _totalSupply = totalSupply; // gas savings, must be defined here since totalSupply can update in _mintFee
//     if (_totalSupply == 0) {
//         liquidity = Math.sqrt(amount0.mul(amount1)).sub(MINIMUM_LIQUIDITY);
//        _mint(address(0), MINIMUM_LIQUIDITY); // permanently lock the first MINIMUM_LIQUIDITY tokens
//     } else {
//         liquidity = Math.min(amount0.mul(_totalSupply) / _reserve0, amount1.mul(_totalSupply) / _reserve1);
//     }
//     require(liquidity > 0, 'UniswapV2: INSUFFICIENT_LIQUIDITY_MINTED');
//     _mint(to, liquidity);

//     _update(balance0, balance1, _reserve0, _reserve1);
//     if (feeOn) kLast = uint(reserve0).mul(reserve1); // reserve0 and reserve1 are up-to-date
//     emit Mint(msg.sender, amount0, amount1);
// }


    fn deposit(e: Env, to: Address, desired_a: i128, min_a: i128, desired_b: i128, min_b: i128) {
        // Depositor needs to authorize the deposit
        to.require_auth();

        let (reserve_a, reserve_b) = (get_reserve_a(&e), get_reserve_b(&e));

        // Calculate deposit amounts
        let amounts = get_deposit_amounts(desired_a, min_a, desired_b, min_b, reserve_a, reserve_b);

        // TOKEN: Token Client
        let token_a_client = TokenClient::new(&e, &get_token_0(&e));
        let token_b_client = TokenClient::new(&e, &get_token_1(&e));

        token_a_client.xfer(&to, &e.current_contract_address(), &amounts.0);
        token_b_client.xfer(&to, &e.current_contract_address(), &amounts.1);

        // Now calculate how many new pool shares to mint
        let (balance_a, balance_b) = (get_balance_a(&e), get_balance_b(&e));
        let total_shares = get_total_shares(&e);

        let zero = 0;
        let new_total_shares = if reserve_a > zero && reserve_b > zero {
            // let shares_a = (balance_a * total_shares) / reserve_a;
            // let shares_b = (balance_b * total_shares) / reserve_b;
            let shares_a = (balance_a.checked_mul(total_shares).unwrap()).checked_div(reserve_a).unwrap();
            let shares_b = (balance_b.checked_mul(total_shares).unwrap()).checked_div(reserve_b).unwrap();
            shares_a.min(shares_b)
        } else {
            (balance_a.checked_mul(balance_b).unwrap()).sqrt()
        };

        mint_shares(&e, to.clone(), new_total_shares - total_shares);
        put_reserve_a(&e, balance_a);
        put_reserve_b(&e, balance_b);

        event::deposit(&e, to, amounts.0, amounts.1);
    }

// Check UniswapV2 swap function
    fn swap(e: Env, to: Address, buy_a: bool, out: i128, in_max: i128) {
        to.require_auth();

        let (reserve_a, reserve_b) = (get_reserve_a(&e), get_reserve_b(&e));
        let (reserve_sell, reserve_buy) = if buy_a {
            (reserve_b, reserve_a)
        } else {
            (reserve_a, reserve_b)
        };

        // First calculate how much needs to be sold to buy amount out from the pool
        let n = reserve_sell.checked_mul(out).unwrap().checked_mul(1000).unwrap();
        let d = (reserve_buy - out).checked_mul(997).unwrap();
        let sell_amount = (n.checked_div(d).unwrap()).checked_add(1).unwrap();
        if sell_amount > in_max {
            panic!("in amount is over max")
        }

        // Transfer the amount being sold to the contract
        let sell_token = if buy_a {
            get_token_1(&e)
        } else {
            get_token_0(&e)
        };
        // TOKEN: Token Client
        let sell_token_client = TokenClient::new(&e, &sell_token);
        sell_token_client.xfer(&to, &e.current_contract_address(), &sell_amount);

        let (balance_a, balance_b) = (get_balance_a(&e), get_balance_b(&e));

        // residue_numerator and residue_denominator are the amount that the invariant considers after
        // deducting the fee, scaled up by 1000 to avoid fractions
        let residue_numerator: i128 = 997;
        let residue_denominator: i128 = 1000;
        let zero = 0;

        let new_invariant_factor = |balance: i128, reserve: i128, out: i128| {
            let delta = balance - reserve - out;
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

        let (out_a, out_b) = if buy_a { (out, 0) } else { (0, out) };

        let new_inv_a = new_invariant_factor(balance_a, reserve_a, out_a);
        let new_inv_b = new_invariant_factor(balance_b, reserve_b, out_b);
        //let old_inv_a = residue_denominator * reserve_a;
        let old_inv_a = residue_denominator.checked_mul(reserve_a).unwrap();
        //let old_inv_b = residue_denominator * reserve_b;
        let old_inv_b = residue_denominator.checked_mul(reserve_b).unwrap();

        // if new_inv_a * new_inv_b < old_inv_a  * old_inv_b {
        if new_inv_a.checked_mul(new_inv_b).unwrap() < old_inv_a.checked_mul(old_inv_b).unwrap() {
            panic!("constant product invariant does not hold");
        }

        if buy_a {
            transfer_a(&e, to, out_a);
        } else {
            transfer_b(&e, to, out_b);
        }

        // Checks if not negative in put_reserve_a and put_reserve_b
        put_reserve_a(&e, balance_a - out_a);
        put_reserve_b(&e, balance_b - out_b);
    }

// Check UniswapV2 burn function
// TODO: In UniswapV2 this is called burn
    fn withdraw(e: Env, to: Address, share_amount: i128, min_a: i128, min_b: i128) -> (i128, i128) {
        to.require_auth();

        // First transfer the pool shares that need to be redeemed:
        // Transfer from the user the "share_amounts" pool shares that it needs to be redeeemed.

        Token::xfer(e.clone(), to.clone(), e.current_contract_address(), share_amount);

        let (balance_a, balance_b) = (get_balance_a(&e), get_balance_b(&e));
        let balance_shares = get_balance_shares(&e);

        let total_shares = get_total_shares(&e);

        // Now calculate the withdraw amounts
        // let out_a = (balance_a * balance_shares) / total_shares;
        // let out_b = (balance_b * balance_shares) / total_shares;
        let out_a = (balance_a.checked_mul(balance_shares).unwrap()).checked_div(total_shares).unwrap();
        let out_b = (balance_b.checked_mul(balance_shares).unwrap()).checked_div(total_shares).unwrap();

        if out_a < min_a || out_b < min_b {
            panic!("min not satisfied");
        }

        burn_shares(&e, balance_shares);
        transfer_a(&e, to.clone(), out_a.clone());
        transfer_b(&e, to.clone(), out_b.clone());
        // Checks if not negative in put_reserve_a and put_reserve_b
        put_reserve_a(&e, balance_a - out_a);
        put_reserve_b(&e, balance_b - out_b);

        event::withdraw(&e, to.clone(), out_a, out_b, to);

        (out_a, out_b)
    }

    fn get_reserves(e: Env) -> (i128, i128, i128) {
        (get_reserve_a(&e), get_reserve_b(&e), get_block_timestamp_last(&e))
    }

    fn my_balance(e: Env, id: Address) -> i128 {
        Token::balance(e.clone(), id)
    }

}

// TODO: Analize if we should add UniswapV2 lock guard function:

    // // Reentrancy attack guard
    // uint private unlocked = 1;
    // modifier lock() {
    //     require(unlocked == 1, 'UniswapV2: LOCKED');
    //     unlocked = 0;
    //     _;
    //     unlocked = 1;
    // }

// TODO: Analize if we should add UniswapV2 Events: 
// event Mint(address indexed sender, uint amount0, uint amount1);
// event Burn(address indexed sender, uint amount0, uint amount1, address indexed to);
// event Swap(
//     address indexed sender,
//     uint amount0In,
//     uint amount1In,
//     uint amount0Out,
//     uint amount1Out,
//     address indexed to
// );
// event Sync(uint112 reserve0, uint112 reserve1);


// TODO: Analize if we should add UniswapV2 skim and sync functions:
    // // force balances to match reserves
    // function skim(address to) external lock {
    //     address _token0 = token0; // gas savings
    //     address _token1 = token1; // gas savings
    //     _safeTransfer(_token0, to, IERC20(_token0).balanceOf(address(this)).sub(reserve0));
    //     _safeTransfer(_token1, to, IERC20(_token1).balanceOf(address(this)).sub(reserve1));
    // }

    // // force reserves to match balances
    // function sync() external lock {
    //     _update(IERC20(token0).balanceOf(address(this)), IERC20(token1).balanceOf(address(this)), reserve0, reserve1);
    // }