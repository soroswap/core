const Q64: u128 = 2u128.pow(64);


// encode a u64 as a UQ64x64
pub(crate) fn encode(y: u64) -> u128 {
    let y_into128: u128 = y.into();
    let z: u128 = y_into128 * Q64;
    z
}

// returns a UQ64x64 which represents the ratio of the x to y
pub(crate) fn fraction(x: u64, y: u64) -> u128 {
    if y == 0 {
        panic!("DIV_BY_ZERO")
    }
    uqdiv(encode(x),y)
}


// divide a UQ64x64 by a u64, returning a UQ64x64
pub(crate) fn uqdiv(x: u128, y: u64) -> u128 {
    if y == 0 {
        panic!("DIV_BY_ZERO")
    }

    let y_into128: u128 = y.into();
    let z: u128 = x / y_into128;
    z
}

pub(crate) fn integer_part(x: u128) -> u64 {
    (x >> 64) as u64
} 

/*
TODO: factional_part
"the lower 64 bits represent the fractional amount"
I got these hints... but not working yet
// let fractional_part: u128 = result_div & ((1<<64)-1);
// let fractional_part: u64 = (result_div & u64::MAX as u128) as u64;
*/


// decode a UQ112x112 into a u128 with 7 decimals of precision
pub(crate) fn decode_with_7_decimals(x: u128) -> u128 {
    /*
    Inspired by https://github.com/compound-finance/open-oracle/blob/d0a0d0301bff08457d9dfc5861080d3124d079cd/contracts/Uniswap/UniswapLib.sol#L27
    and https://ethereum.stackexchange.com/questions/113130/what-does-decode112with18-do
    
    to get close to: (x * 1e7) / 2^64 without risk of overflowing we do:
    = (x) * (2**log2(1e7)) / 2^64
    = (x) / (2 ** (64 - log2(1e7)))
    ≈ (x) / (1.8446744073709551616 × 10^12 )
    ≈ (x) / 1844674407370
    */

    x / 1844674407370
}

