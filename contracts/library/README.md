# The Soroswap Library
Library that enables efficient and optimized code execution across different contracts on the Soroswap.Finance protocol

This can be used as a Library Contract or as a crate in your contract

## Usage as a crate

Add this to your Cargo.toml:

[dependencies]
soroswap-library = "<desired version>"

## How to publish:
https://doc.rust-lang.org/cargo/reference/publishing.html

```
cargo login
cargo publish --dry-run
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