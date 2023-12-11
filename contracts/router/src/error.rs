use soroban_sdk::{self, contracterror};
use soroswap_library::{SoroswapLibraryError};


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SoroswapRouterError {
    /// SoroswapRouter: not yet initialized
    NotInitialized = 401,

    /// SoroswapRouter: negative amount is not allowed
    NegativeNotAllowed = 402,

    /// SoroswapRouter: deadline expired
    DeadlineExpired = 403,
    
    /// SoroswapRouter: already initialized
    InitializeAlreadyInitialized = 404,

    /// SoroswapRouter: insufficient a amount
    InsufficientAAmount = 405,

    /// SoroswapRouter: insufficient b amount
    InsufficientBAmount = 406,

    /// SoroswapRouter: insufficient output amount
    InsufficientOutputAmount = 407,

    /// SoroswapRouter: excessive input amount
    ExcessiveInputAmount = 408,

    /// SoroswapRouter: pair does not exist
    PairDoesNotExist = 409,

}


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
// Define a new set of integer literals for the CombinedError enum
pub enum CombinedRouterError {
    RouterNotInitialized = 501,
    RouterNegativeNotAllowed = 502,
    RouterDeadlineExpired = 503,
    RouterInitializeAlreadyInitialized = 504,
    RouterInsufficientAAmount = 505,
    RouterInsufficientBAmount = 506,
    RouterInsufficientOutputAmount = 507,
    RouterExcessiveInputAmount = 508,
    RouterPairDoesNotExist = 509,

    LibraryInsufficientAmount = 510,
    LibraryInsufficientLiquidity = 511,
    LibraryInsufficientInputAmount = 512,
    LibraryInsufficientOutputAmount = 513,
    LibraryInvalidPath = 514,
    LibrarySortIdenticalTokens = 515,
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
