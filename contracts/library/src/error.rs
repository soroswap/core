use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SoroswapLibraryError {
    /// SoroswapLibrary: insufficient amount 
    InsufficientAmount = 1,

    /// SoroswapLibrary: insufficient liquidity
    InsufficientLiquidity = 2,

    /// SoroswapLibrary: insufficient input amount
    InsufficientInputAmount = 3,

    /// SoroswapLibrary: insufficient output amount
    InsufficientOutputAmount = 4,

    /// SoroswapLibrary: invalid path
    InvalidPath = 5,

    /// SoroswapLibrary: token_a and token_b have identical addresses
    SortIdenticalTokens = 6,
}