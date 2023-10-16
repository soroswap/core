# fixed-point-math
A light fixed-point math library for Rust. Written specifically to be used in [Soroban](https://soroban.stellar.org/) and other WASM based blockchain environments.

## Safety
This is **experimental software** and is provided on an "as is" and "as available" basis.

We do **not give any warranties** and **will not be liable for any loss** incurred through any use of this codebase.

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
fixed-point-math = "<desired version>"
```

### Examples
Fixed-point math support is currently added to both `i128` and `u64` types. For any supported number, you can perform fixed-point operations like this:

```rust
use fixed_point_math::{STROOP, FixedPoint};

let x: u64 = 1_5000000;
let y: u64 = 2_0000000;
assert_eq!(x.fixed_mul_floor(y, STROOP).unwrap(), 3_0000000);
```

## Overflow
Overflowing results are handled in the same manner as Rust's built-in "checked" math, by returning `None`.

Fixed-point math also deals with phantom overflows, where an intermediary computation overflows but the expected result would be within bounds. This library manages this differently for each supported type:
* i128
    * No extra handling is done. `i128` is large enough to support most computation with 7/9 decimal values. However, its likely 18-decimal math will encounter overflows.
* u64
    * The intermediary computation gets scaled to `u128`, and it is attempted again.

## Acknowledgements
This library was inspired by or directly modified from many sources, primary:
- [Solmate](https://github.com/transmissions11/solmate)
- [OpenZeppelin](https://github.com/OpenZeppelin/openzeppelin-contracts)

## WASM
The WASM target `wasm32-unknown-unknown` is supported.

## Contributions
Contributions are welcome. Please check out the contribution guide (TODO)!

## License
This library is released under the [MIT License](../LICENSE).
