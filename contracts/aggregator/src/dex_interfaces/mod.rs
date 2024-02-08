pub mod soroswap_interface;
pub mod phoenix_interface;

/// The `dex_constants` module assigns unique identifiers to supported DEXes.
/// These identifiers streamline the selection of DEX-specific swap functions
/// within the aggregator's swap execution logic, enhancing modularity and
/// simplifying the integration of new DEXes.
pub mod dex_constants {
  pub const SOROSWAP: i32 = 0;
  pub const PHOENIX: i32 = 1;
}