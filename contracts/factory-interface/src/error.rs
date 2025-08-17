use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FactoryError {
    /// SoroswapFactory: not yet initialized
    NotInitialized = 201,

    /// SoroswapFactory: token_a and token_b have identical addresses
    CreatePairIdenticalTokens = 202,
    /// SoroswapFactory: pair already exists between token_a and token_b
    CreatePairAlreadyExists = 203,

    /// SoroswapFactory: already initialized
    InitializeAlreadyInitialized = 204,

    /// SoroswapFactory: pair does not exist
    PairDoesNotExist = 205,

    /// SoroswapFactory: index does not exist
    IndexDoesNotExist = 206,
}
