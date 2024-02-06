# The Soroswap Library
Library that enables efficient and optimized code execution across different contracts on the Soroswap.Finance protocol.

This can be used as a Library Contract that can be deployed in the Soroban Blockchain or as a crate in your contract

Check https://crates.io/crates/soroswap-library


## Usage as a crate

1.- Add this to your Cargo.toml:

[dependencies]
soroswap-library = "<desired version>"

2.- Import it:
```rust
use soroswap_library;
```

3.- Use it:
```rust
let quote = soroswap_library::quote(amount_a, reserve_a, reserve_b)
```

## How to publish:
https://doc.rust-lang.org/cargo/reference/publishing.html

```
cargo login
cargo publish --dry-run
cargo publish
```

## Examples

## Acknowledgements

This library was inspired by the UniswapV2Library: 
https://github.com/Uniswap/v2-periphery/blob/master/contracts/libraries/UniswapV2Library.sol

## WASM

The WASM target wasm32-unknown-unknown is supported.

## Contributions

Contributions are welcome

## Licence
This library is released under the GPL-3.0   License.