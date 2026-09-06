# Router Module

> **Living document.** Read this before modifying the module. Update it in the same change whenever the module's behavior, public functions, storage, errors, events, or library dependency change.

**Source:** `contracts/router/` · **Crate:** `soroswap-router` 0.0.1, `cdylib` (`Cargo.toml:2`, `:16`) · **Last verified:** 2026-09-06

## Purpose

The only contract end users and integrators are meant to call. It wraps the pair's positional, unauthenticated primitives in a safe flow: authorize the user, check the deadline, compute amounts, move tokens, then call the pair. It also re-exports the library math as contract functions so off chain callers can simulate a quote without a second contract.

## Structure

| File | Purpose |
|---|---|
| `src/lib.rs` | `SoroswapRouter` contract, `SoroswapRouterTrait`, and the private `add_liquidity_amounts` and `swap` helpers. |
| `src/storage.rs` | Single instance key `Factory`, TTL. |
| `src/factory.rs` | Compile time import of the optimized factory WASM, `SoroswapFactoryClient`. |
| `src/pair.rs` | Compile time import of the pair WASM, `SoroswapPairClient`. |
| `src/event.rs` | Event payload structs and publishers. |
| `src/error.rs` | `SoroswapRouterError` 401 to 409 and the `CombinedRouterError` 501 to 515 actually returned. |
| `src/test/` | Suites: `initialize`, `add_liquidity`, `remove_liquidity`, `library_functions`, `swap_tokens_for_exact_tokens`, `swap_exact_tokens_for_tokens`, `events`, `budget` (`src/test.rs:141`). |

## Public surface

Every function returns `Result<_, CombinedRouterError>`. Trait at `src/lib.rs:177`.

| Function | Signature | Auth |
|---|---|---|
| `initialize` | `(e: Env, factory: Address)` (`src/lib.rs:390`) | none, first caller wins |
| `add_liquidity` | `(e, token_a, token_b, amount_a_desired: i128, amount_b_desired: i128, amount_a_min: i128, amount_b_min: i128, to: Address, deadline: u64) -> (i128, i128, i128)` (`src/lib.rs:417`) | `to.require_auth()` (`:434`) |
| `remove_liquidity` | `(e, token_a, token_b, liquidity: i128, amount_a_min: i128, amount_b_min: i128, to: Address, deadline: u64) -> (i128, i128)` (`src/lib.rs:492`) | `to.require_auth()` (`:507`) |
| `swap_exact_tokens_for_tokens` | `(e, amount_in: i128, amount_out_min: i128, path: Vec<Address>, to: Address, deadline: u64) -> Vec<i128>` (`src/lib.rs:577`) | `to.require_auth()` (`:589`) |
| `swap_tokens_for_exact_tokens` | `(e, amount_out: i128, amount_in_max: i128, path: Vec<Address>, to: Address, deadline: u64) -> Vec<i128>` (`src/lib.rs:646`) | `to.require_auth()` (`:658`) |
| `get_factory` | `(e: Env) -> Address` (`src/lib.rs:709`) | none |
| `router_pair_for` | `(e, token_a, token_b) -> Address` (`src/lib.rs:729`) | none |
| `router_quote` | `(amount_a, reserve_a, reserve_b) -> i128` (`src/lib.rs:751`) | none |
| `router_get_amount_out` | `(amount_in, reserve_in, reserve_out) -> i128` (`src/lib.rs:766`) | none |
| `router_get_amount_in` | `(amount_out, reserve_in, reserve_out) -> i128` (`src/lib.rs:781`) | none |
| `router_get_amounts_out` | `(e, amount_in: i128, path: Vec<Address>) -> Vec<i128>` (`src/lib.rs:797`) | none |
| `router_get_amounts_in` | `(e, amount_out: i128, path: Vec<Address>) -> Vec<i128>` (`src/lib.rs:815`) | none |

The `router_*` functions are thin pass throughs to `soroswap_library` (`src/lib.rs:752`, `:767`, `:782`, `:801`, `:819`).

## Key methods

- **Guard order in every state changing function** (`src/lib.rs:428` and the same block at `:502`, `:585`, `:654`): `check_initialized`, then `check_nonnegative_amount` on each amount, then `extend_instance_ttl`, then `to.require_auth()`, then `ensure_deadline`. Keep this order when adding a function; authorization comes before the deadline check, and both come before any token movement.
- **`add_liquidity_amounts`** (`src/lib.rs:68`): creates the pair if `factory.pair_exists` is false (`:80`), reads reserves, and on an empty pool takes the desired amounts as is (`:92`). Otherwise it quotes B from A, falling back to quoting A from B, and errors with `InsufficientBAmount` or `InsufficientAAmount` if the optimal amount is below the minimum.
- **`add_liquidity`** (`src/lib.rs:417`): transfers both tokens from `to` straight to the pair (`:457`), then calls `pair.deposit(to)` (`:460`). The router never holds user funds between operations.
- **`remove_liquidity`** (`src/lib.rs:492`): requires the pair to exist (`:514`), transfers the LP tokens from `to` to the pair (`:527`), calls `pair.withdraw(to)` (`:530`), reorders the returned amounts to match the caller's token order (`:533`), then enforces both minimums.
- **`swap` helper** (`src/lib.rs:133`): walks the path, and for every hop except the last it sends the output to the *next* pair rather than to the user (`:150`). That chaining is why the initial transfer at `:617` only funds the first pair.
- **`swap_exact_tokens_for_tokens`** (`src/lib.rs:577`): computes the whole amount vector with `get_amounts_out`, checks the final amount against `amount_out_min` (`:602`), funds the first pair, then runs the hops.
- **`swap_tokens_for_exact_tokens`** (`src/lib.rs:646`): same shape with `get_amounts_in` and an `amount_in_max` ceiling check (`:671`).

## Storage layout and TTL

One instance key, `DataKey::Factory` (`src/storage.rs:6`), written once by `initialize`. `has_factory` (`src/storage.rs:24`) is the initialized flag. Instance TTL bump 30 days of ledgers, threshold 29 (`src/storage.rs:10`), extended by `extend_instance_ttl` inside every entrypoint.

## Errors

Two enums in `src/error.rs`:

- `SoroswapRouterError` 401 to 409 (`src/error.rs:8`) is internal only.
- **`CombinedRouterError` 501 to 515 (`src/error.rs:43`) is what the contract actually returns.** `Router*` variants 501 to 509 mirror the router errors, `Library*` variants 510 to 515 wrap `SoroswapLibraryError` 301 to 306. The `From` impls at `src/error.rs:62` and `:75` do the mapping.

An integrator decoding router failures must map 5xx codes, not 4xx. Pair and factory errors bubble up unchanged from the sub-invocation.

## Events

Topic tuple `("SoroswapRouter", <symbol>)`: `init` (`src/event.rs:16`), `add` (`src/event.rs:64`), `remove` (`src/event.rs:115`), `swap` (`src/event.rs:150`). The swap event carries the full `path` and `amounts` vectors plus `to` (`src/event.rs:123`).

## Dependencies

- `soroban-sdk` 20.2.0 (`Cargo.toml:19`), `num-integer` (`Cargo.toml:20`).
- **`soroswap-library` 0.3.0 from crates.io** (`Cargo.toml:23`, locked at `Cargo.lock:1117`). This is *not* the in repo `contracts/library` crate, which is at version 2.0.0 (`contracts/library/Cargo.toml:3`).
- Factory client compiled in from `../factory/target/wasm32-unknown-unknown/release/soroswap_factory.optimized.wasm` (`src/factory.rs:2`), so the factory must be built *and optimized* before the router compiles.
- Pair client compiled in from `../pair/target/wasm32-unknown-unknown/release/soroswap_pair.wasm` (`src/pair.rs:2`), the unoptimized build.
- Token transfers through `soroban_sdk::token::Client` (`src/lib.rs:2`).
- `contracts/router/Makefile:9` builds `../token`, `../pair` and `../factory` first.

## Gotchas and invariants

- **The router uses the published `soroswap-library` 0.3.0, not `contracts/library`.** `src/lib.rs:84` calls `soroswap_library::get_reserves`, a name the in repo library does not export; it exports `get_reserves_with_factory` and `get_reserves_with_pair` (`contracts/library/src/lib.rs:19`). Editing `contracts/library` therefore has no effect on the router until the crate is published and the version bumped here.
- `assert!(amount_a_optimal <= amount_a_desired)` (`src/lib.rs:114`) panics instead of returning a contract error. A panic gives integrators an opaque failure, unlike the 5xx codes.
- Neither swap function checks that the first pair exists, deliberately, to save a cross contract call (comment at `src/lib.rs:615` and `:683`). The failure surfaces from inside the token transfer instead of as `PairDoesNotExist`.
- `add_liquidity` silently creates the pair when it does not exist (`src/lib.rs:80`), so the first liquidity provider also pays for the pair deployment.
- `initialize` has no `require_auth` (`src/lib.rs:390`) and the factory address can never be changed afterwards. There is no setter.
- `ensure_deadline` rejects `ledger_timestamp >= deadline` (`src/lib.rs:34`), so a deadline exactly equal to the current ledger time fails.
- `path` must have at least two entries; the length check lives in the library's `get_amounts_out` and `get_amounts_in`, and surfaces as `LibraryInvalidPath` 514.

## Testing

`make test` in `contracts/router` builds `../token`, `../pair` and `../factory` first, then runs `cargo test` (`contracts/router/Makefile:5`). `src/test/budget.rs` measures CPU and memory with `env.budget`; run it with `cargo test budget -- --nocapture` from `contracts/router` (repo root `README.md:179`). Note the root README points at `/contracts/router/budget.rs` (root `README.md:182`); the file is `contracts/router/src/test/budget.rs`. `src/test/swap.rs` exists but its `mod` declaration is commented out (`src/test.rs:143`), so it does not run.
