use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SoroswapPairError {
    /// SoroswapPair: already initialized
    InitializeAlreadyInitialized = 101,
    /// SoroswapPair: not yet initialized
    NotInitialized = 102,

    /// SoroswapPair: token_0 must be less than token_1 while initializing 
    InitializeTokenOrderInvalid = 103,

    /// SoroswapPair: insufficient amount of token 0 sent while doing deposit
    DepositInsufficientAmountToken0 = 104,
    /// SoroswapPair: insufficient amount of token 1 sent while doing deposit
    DepositInsufficientAmountToken1 = 105,
    /// SoroswapPair: insufficient first liquidity minted while doing deposit
    DepositInsufficientFirstLiquidity = 106,
    /// SoroswapPair: insufficient liquidity minted while doing deposit
    DepositInsufficientLiquidityMinted = 107,
    /// SoroswapPair: insufficient output amount while doing deposDepositit

    SwapInsufficientOutputAmount = 108,
    /// SoroswapPair: negatives amounts out dont supported while doing swap
    SwapNegativesOutNotSupported = 109,
    /// SoroswapPair: insufficient liquidity to do the swap
    SwapInsufficientLiquidity = 110,
    /// SoroswapPair: invalid to to do the swap
    SwapInvalidTo = 111,
    /// SoroswapPair: insufficient input amount while doing swap
    SwapInsufficientInputAmount = 112,
    /// SoroswapPair: negatives amounts in dont supported while doing swap
    SwapNegativesInNotSupported = 113,
    /// SoroswapPair: K constant is not met while doing swap
    SwapKConstantNotMet = 114,

    /// SoroswapPair: liquidity was not initialized yet while doing withdraw
    WithdrawLiquidityNotInitialized = 115,
    /// SoroswapPair: insufficient sent shares while doing withdraw
    WithdrawInsufficientSentShares = 116,
    /// SoroswapPair: insufficient liquidity burned while doing withdraw
    WithdrawInsufficientLiquidityBurned = 117,

    /// SoroswapPair: OVERFLOW while updating
    UpdateOverflow = 118,

    /// SoroswapPairToken: expiration_ledger is less than ledger seq when amount > 0
    TokenWriteAllowanceExpirationLedgerExpired = 119,

    /// SoroswapPairToken: insufficient allowance when spending allowance
    TokenSpendAllowanceInsufficientAllowance = 120,
    
    /// SoroswapPairToken: integer overflow occurred while adding balance
    TokenAddBalanceOverflow = 121,
    
    /// SoroswapPairToken: insufficient balance
    TokenSpendBalanceInsufficient = 122,

    /// SoroswapPairToken: negative amount is not allowed
    TokenNegativeAmountNotAllowed = 123,

    /// SoroswapPairToken: already initialized
    TokenInitializeAlreadyInitialized = 124,

    /// SoroswapPairToken: Decimal must fit in a u8
    TokenDecimalNotAllowed = 125,

    /// Total Supply: integer overflow occurred while increasing total supply
    TokenTotalSupplyIncreaseOverflow = 126,

    /// Total Supply: insufficient total supply while decreasing total supply
    TokenTotalSupplyInsufficient = 127,

    /// Total Supply: Integer underflow occurred while decreasing total supply
    TokenTotalSupplyDecreaseUnderflow = 128,
}

