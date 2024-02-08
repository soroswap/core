use soroban_sdk::{self, contracterror};
use soroswap_library::{SoroswapLibraryError};


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SoroswapAggregatorError {
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
pub enum CombinedAggregatorError {
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

impl From<SoroswapLibraryError> for CombinedAggregatorError {
    fn from(err: SoroswapLibraryError) -> Self {
        match err {
            SoroswapLibraryError::InsufficientAmount => CombinedAggregatorError::LibraryInsufficientAmount,
            SoroswapLibraryError::InsufficientLiquidity => CombinedAggregatorError::LibraryInsufficientLiquidity,
            SoroswapLibraryError::InsufficientInputAmount => CombinedAggregatorError::LibraryInsufficientInputAmount,
            SoroswapLibraryError::InsufficientOutputAmount => CombinedAggregatorError::LibraryInsufficientOutputAmount,
            SoroswapLibraryError::InvalidPath => CombinedAggregatorError::LibraryInvalidPath,
            SoroswapLibraryError::SortIdenticalTokens => CombinedAggregatorError::LibrarySortIdenticalTokens,
        }
    }
}

impl From<SoroswapAggregatorError> for CombinedAggregatorError {
    fn from(err: SoroswapAggregatorError) -> Self {
        match err {
            SoroswapAggregatorError::NotInitialized => CombinedAggregatorError::RouterNotInitialized,
            SoroswapAggregatorError::NegativeNotAllowed => CombinedAggregatorError::RouterNegativeNotAllowed,
            SoroswapAggregatorError::DeadlineExpired => CombinedAggregatorError::RouterDeadlineExpired,
            SoroswapAggregatorError::InitializeAlreadyInitialized => CombinedAggregatorError::RouterInitializeAlreadyInitialized,
            SoroswapAggregatorError::InsufficientAAmount => CombinedAggregatorError::RouterInsufficientAAmount,
            SoroswapAggregatorError::InsufficientBAmount => CombinedAggregatorError::RouterInsufficientBAmount,
            SoroswapAggregatorError::InsufficientOutputAmount => CombinedAggregatorError::RouterInsufficientOutputAmount,
            SoroswapAggregatorError::ExcessiveInputAmount => CombinedAggregatorError::RouterExcessiveInputAmount,
            SoroswapAggregatorError::PairDoesNotExist => CombinedAggregatorError::RouterPairDoesNotExist,
        }
    }
}
