// decode a UQ112x112 into a u128 with 7 decimals of precision
fn decode_uq64x64_with_7_decimals(x: u128) -> u128 {
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