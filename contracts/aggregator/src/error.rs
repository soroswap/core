use soroban_sdk::{self, contracterror};
use soroswap_library::{SoroswapLibraryError};


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SoroswapAggregatorError {
    /// SoroswapAggregator: not yet initialized
    NotInitialized = 401,

    /// SoroswapAggregator: negative amount is not allowed
    NegativeNotAllowed = 402,

    /// SoroswapAggregator: deadline expired
    DeadlineExpired = 403,
    
    /// SoroswapAggregator: already initialized
    InitializeAlreadyInitialized = 404,

    /// SoroswapAggregator: insufficient a amount
    InsufficientAAmount = 405,

    /// SoroswapAggregator: insufficient b amount
    InsufficientBAmount = 406,

    /// SoroswapAggregator: insufficient output amount
    InsufficientOutputAmount = 407,

    /// SoroswapAggregator: excessive input amount
    ExcessiveInputAmount = 408,

    /// SoroswapAggregator: Unsupported protocol
    UnsupportedProtocol = 409,

}


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
// Define a new set of integer literals for the CombinedError enum
pub enum CombinedAggregatorError {
    AggregatorNotInitialized = 501,
    AggregatorNegativeNotAllowed = 502,
    AggregatorDeadlineExpired = 503,
    AggregatorInitializeAlreadyInitialized = 504,
    AggregatorInsufficientAAmount = 505,
    AggregatorInsufficientBAmount = 506,
    AggregatorInsufficientOutputAmount = 507,
    AggregatorExcessiveInputAmount = 508,
    AggregatorUnsupportedProtocol = 509,

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
            SoroswapAggregatorError::NotInitialized => CombinedAggregatorError::AggregatorNotInitialized,
            SoroswapAggregatorError::NegativeNotAllowed => CombinedAggregatorError::AggregatorNegativeNotAllowed,
            SoroswapAggregatorError::DeadlineExpired => CombinedAggregatorError::AggregatorDeadlineExpired,
            SoroswapAggregatorError::InitializeAlreadyInitialized => CombinedAggregatorError::AggregatorInitializeAlreadyInitialized,
            SoroswapAggregatorError::InsufficientAAmount => CombinedAggregatorError::AggregatorInsufficientAAmount,
            SoroswapAggregatorError::InsufficientBAmount => CombinedAggregatorError::AggregatorInsufficientBAmount,
            SoroswapAggregatorError::InsufficientOutputAmount => CombinedAggregatorError::AggregatorInsufficientOutputAmount,
            SoroswapAggregatorError::ExcessiveInputAmount => CombinedAggregatorError::AggregatorExcessiveInputAmount,
            SoroswapAggregatorError::UnsupportedProtocol => CombinedAggregatorError::AggregatorUnsupportedProtocol,
        }
    }
}
