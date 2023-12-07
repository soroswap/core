use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// SoroswapPair: already initialized
    InitializeAlreadyInitialized = 1,
    /// SoroswapPair: not yet initialized
    NotInitialized = 2,

    /// SoroswapPair: token_0 must be less than token_1 while initializing
    InitializeTokenOrderInvalid = 3,

    /// SoroswapPair: insufficient amount of token 0 sent while doing deposit
    DepositInsufficientAmountToken0 = 4,
    /// SoroswapPair: insufficient amount of token 1 sent while doing deposit
    DepositInsufficientAmountToken1 = 5,
    /// SoroswapPair: insufficient first liquidity minted while doing deposit
    DepositInsufficientFirstLiquidity = 6,
    /// SoroswapPair: insufficient liquidity minted while doing deposit
    DepositInsufficientLiquidityMinted = 7,
    /// SoroswapPair: insufficient output amount while doing deposDepositit

    SwapInsufficientOutputAmount = 8,
    /// SoroswapPair: negatives amounts out dont supported while doing swap
    SwapNegativesOutNotSupported = 9,
    /// SoroswapPair: insufficient liquidity to do the swap
    SwapInsufficientLiquidity = 10,
    /// SoroswapPair: invalid to to do the swap
    SwapInvalidTo = 11,
    /// SoroswapPair: insufficient input amount while doing swap
    SwapInsufficientInputAmount = 12,
    /// SoroswapPair: negatives amounts in dont supported while doing swap
    SwapNegativesInNotSupported = 13,
    /// SoroswapPair: K constant is not met while doing swap
    SwapKConstantNotMet = 14,

    /// SoroswapPair: liquidity was not initialized yet while doing withdraw
    WithdrawLiquidityNotInitialized = 15,
    /// SoroswapPair: insufficient sent shares while doing withdraw
    WithdrawInsufficientSentShares = 16,
    /// SoroswapPair: insufficient liquidity burned while doing withdraw
    WithdrawInsufficientLiquidityBurned = 17,

    /// SoroswapPair: OVERFLOW while updating
    UpdateOverflow = 18,
}


