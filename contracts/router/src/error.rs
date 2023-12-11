use soroban_sdk::{self, contracterror};
use soroswap_library::{SoroswapLibraryError};


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SoroswapRouterError {
    /// SoroswapRouter: not yet initialized
    NotInitialized = 1,

    /// SoroswapRouter: negative amount is not allowed
    NegativeNotAllowed = 2,

    /// SoroswapRouter: deadline expired
    DeadlineExpired = 3,
    
    /// SoroswapRouter: already initialized
    InitializeAlreadyInitialized = 4,

    /// SoroswapRouter: insufficient a amount
    InsufficientAAmount = 5,

    /// SoroswapRouter: insufficient b amount
    InsufficientBAmount = 6,

    /// SoroswapRouter: insufficient output amount
    InsufficientOutputAmount = 7,

    /// SoroswapRouter: excessive input amount
    ExcessiveInputAmount = 8,

    /// SoroswapRouter: pair does not exist
    PairDoesNotExist = 9,

}


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
// Define a new set of integer literals for the CombinedError enum
pub enum CombinedRouterError {
    RouterNotInitialized = 101,
    RouterNegativeNotAllowed = 102,
    RouterDeadlineExpired = 103,
    RouterInitializeAlreadyInitialized = 104,
    RouterInsufficientAAmount = 105,
    RouterInsufficientBAmount = 106,
    RouterInsufficientOutputAmount = 107,
    RouterExcessiveInputAmount = 108,
    RouterPairDoesNotExist = 109,

    LibraryInsufficientAmount = 110,
    LibraryInsufficientLiquidity = 111,
    LibraryInsufficientInputAmount = 112,
    LibraryInsufficientOutputAmount = 113,
    LibraryInvalidPath = 114,
    LibrarySortIdenticalTokens = 115,
}

impl From<SoroswapLibraryError> for CombinedRouterError {
    fn from(err: SoroswapLibraryError) -> Self {
        match err {
            SoroswapLibraryError::InsufficientAmount => CombinedRouterError::LibraryInsufficientAmount,
            SoroswapLibraryError::InsufficientLiquidity => CombinedRouterError::LibraryInsufficientLiquidity,
            SoroswapLibraryError::InsufficientInputAmount => CombinedRouterError::LibraryInsufficientInputAmount,
            SoroswapLibraryError::InsufficientOutputAmount => CombinedRouterError::LibraryInsufficientOutputAmount,
            SoroswapLibraryError::InvalidPath => CombinedRouterError::LibraryInvalidPath,
            SoroswapLibraryError::SortIdenticalTokens => CombinedRouterError::LibrarySortIdenticalTokens,
        }
    }
}

impl From<SoroswapRouterError> for CombinedRouterError {
    fn from(err: SoroswapRouterError) -> Self {
        match err {
            SoroswapRouterError::NotInitialized => CombinedRouterError::RouterNotInitialized,
            SoroswapRouterError::NegativeNotAllowed => CombinedRouterError::RouterNegativeNotAllowed,
            SoroswapRouterError::DeadlineExpired => CombinedRouterError::RouterDeadlineExpired,
            SoroswapRouterError::InitializeAlreadyInitialized => CombinedRouterError::RouterInitializeAlreadyInitialized,
            SoroswapRouterError::InsufficientAAmount => CombinedRouterError::RouterInsufficientAAmount,
            SoroswapRouterError::InsufficientBAmount => CombinedRouterError::RouterInsufficientBAmount,
            SoroswapRouterError::InsufficientOutputAmount => CombinedRouterError::RouterInsufficientOutputAmount,
            SoroswapRouterError::ExcessiveInputAmount => CombinedRouterError::RouterExcessiveInputAmount,
            SoroswapRouterError::PairDoesNotExist => CombinedRouterError::RouterPairDoesNotExist,
        }
    }
}
