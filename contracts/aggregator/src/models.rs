/// `DexDistribution` specifies how to distribute a swap operation across multiple DEX protocols.
/// It includes the index identifying the DEX, the token swap path required for the swap, and the
/// portion of the total swap amount allocated to this DEX. This struct enables the aggregator to
/// execute swaps with optimized routes and amounts, reducing slippage and maximizing efficiency.
///
/// # Fields
/// * `index`: An integer representing the specific DEX protocol to use. This index correlates to
///   predefined constants that map to specific DEXes (e.g., `SOROSWAP = 0`, `PHOENIX = 1`).
/// * `path`: A vector of `Address`es representing the token swap path. The first element is the
///   input token, the last is the output token, and any intermediate elements represent tokens to
///   trade through (important for multi-hop swaps).
/// * `parts`: The portion of the total swap amount to be executed through this DEX. The swap
///   amount for this DEX is calculated as `(total_swap_amount * parts) / total_parts`.
///
/// # Considerations
/// - Should we implement a set of contract functions for updating and retrieving the contract addresses of
///   the different DEX protocols, allowing these addresses to be updated by an admin? This approach would
///   centralize the management of DEX contract addresses within the contract itself, enhancing security and
///   governance by restricting updates to authorized administrators. It would also negate the need to pass
///   contract addresses through function parameters, streamlining the swap execution process.
///   Alternatively, should we include the contract address directly as a parameter in the `DexDistribution`
///   struct? This method would offer flexibility in specifying DEX addresses on a per-swap basis but might
///   increase complexity and reduce the ease of managing DEX addresses centrally.
///
/// This approach allows for dynamic and flexible distribution of swap amounts across various
/// DEX protocols, accommodating complex swapping strategies that may involve multi-hop paths
/// and varying liquidity sources.
use soroban_sdk::{contracttype, Vec, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DexDistribution {
    pub index: i32,
    pub path: Vec<Address>,
    pub parts: i128,
}