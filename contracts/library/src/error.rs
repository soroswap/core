use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SoroswapLibraryError {
    /// SoroswapLibrary: insufficient amount
    InsufficientAmount = 301,

    /// SoroswapLibrary: insufficient liquidity
    InsufficientLiquidity = 302,

    /// SoroswapLibrary: insufficient input amount
    InsufficientInputAmount = 303,

    /// SoroswapLibrary: insufficient output amount
    InsufficientOutputAmount = 304,

    /// SoroswapLibrary: invalid path
    InvalidPath = 305,

    /// SoroswapLibrary: token_a and token_b have identical addresses
    SortIdenticalTokens = 306,
}
