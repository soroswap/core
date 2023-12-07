use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FactoryError {
    /// SoroswapFactory: not yet initialized
    NotInitialized = 1,

    /// SoroswapFactory: token_a and token_b have identical addresses
    CreatePairIdenticalTokens = 2,
    /// SoroswapFactory: pair already exists between token_a and token_b
    CreatePairAlreadyExists = 3,

    /// SoroswapFactory: already initialized
    InitializeAlreadyInitialized = 4,

    /// SoroswapFactory: pair does not exist
    PairDoesNotExist = 5,

    /// SoroswapFactory: index does not exist
    IndexDoesNotExist = 6,
}

