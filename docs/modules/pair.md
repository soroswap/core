# Pair Module

> **Living document.** Read this before modifying the module. Update it in the same change whenever the module's behavior, public functions, storage, errors, events, or fee math change.

**Source:** `contracts/pair/` · **Crate:** `soroswap-pair` 0.0.1, `cdylib` (`Cargo.toml:2`, `:9`) · **Last verified:** 2026-09-06

## Purpose

The constant product liquidity pool. It holds two token reserves, mints and burns its own LP token, and enforces the K invariant on every swap. It is deployed only by the factory, never directly, and it holds all protocol funds, so this is the highest risk crate in the repo.

## Structure

| File | Purpose |
|---|---|
| `src/lib.rs` | `SoroswapPair` contract, `mint_fee`, `update`, token transfer helpers, LP name and symbol construction. |
| `src/storage.rs` | Instance `DataKey` enum, TTL, reserve and token accessors. |
| `src/balances.rs` | Reads the pair's own token and LP share balances. |
| `src/math.rs` | `CheckedCeilingDiv` for the swap fee. |
| `src/strings.rs` | `take_first_n_chars` and `concat` used to build the LP token name and symbol. |
| `src/event.rs` | Event payload structs and publishers. |
| `src/error.rs` | `SoroswapPairError`, codes 101 to 118. |
| `src/soroswap_pair_token/` | The embedded LP token: `contract.rs`, `balance.rs`, `allowance.rs`, `metadata.rs`, `total_supply.rs`, `storage_types.rs`. |
| `src/test/` | Suites: `initialize`, `deposit`, `swap`, `withdraw`, `fee`, `skim`, `sync`, `events`, `soroswap_pair_token` (`src/test.rs:129`). |

## Public surface

Pool functions, declared in `SoroswapPairTrait` (`src/lib.rs:55`):

| Function | Signature | Auth |
|---|---|---|
| `initialize` | `(e: Env, factory: Address, token_0: Address, token_1: Address) -> Result<(), SoroswapPairError>` (`src/lib.rs:99`) | none, guarded by `has_token_0` |
| `deposit` | `(e: Env, to: Address) -> Result<i128, SoroswapPairError>` (`src/lib.rs:168`) | none |
| `swap` | `(e: Env, amount_0_out: i128, amount_1_out: i128, to: Address) -> Result<(), SoroswapPairError>` (`src/lib.rs:239`) | none |
| `withdraw` | `(e: Env, to: Address) -> Result<(i128, i128), SoroswapPairError>` (`src/lib.rs:315`) | none |
| `skim` | `(e: Env, to: Address)` (`src/lib.rs:369`) | none |
| `sync` | `(e: Env)` (`src/lib.rs:385`) | none |
| `token_0` / `token_1` / `factory` | `(e: Env) -> Address` (`src/lib.rs:136`, `:142`, `:148`) | none |
| `get_reserves` | `(e: Env) -> (i128, i128)` (`src/lib.rs:399`) | none |
| `k_last` | `(e: Env) -> i128` (`src/lib.rs:413`) | none |

LP token functions on the same contract, `soroban_sdk::token::Interface` (`src/soroswap_pair_token/contract.rs:67`): `allowance` `:68`, `approve` `:75` (`from.require_auth()` `:76`), `balance` `:90`, `transfer` `:97` (`:98`), `transfer_from` `:111` (`spender.require_auth()` `:112`), `burn` `:126` (`:127`), `burn_from` `:131` (`:132`), `decimals` `:147`, `name` `:151`, `symbol` `:155`. Plus `total_supply(e) -> i128` (`:54`).

## Key methods

- **`deposit`** (`src/lib.rs:168`): the caller must already have transferred both tokens to the pair. Amounts are derived as balance minus reserve (`:177`). On the first deposit it permanently mints `MINIMUM_LIQUIDITY = 1000` to the pair itself (`src/lib.rs:31`, `:193`) and returns `sqrt(amount_0 * amount_1) - 1000`; afterwards it mints `min(amount_0 * supply / reserve_0, amount_1 * supply / reserve_1)` (`:200`). Calls `mint_fee` before minting (`:188`), then `update`, then stores `k_last` if the fee is on (`:213`).
- **`swap`** (`src/lib.rs:239`): sends the requested outputs first (`:261`), then infers the inputs from the resulting balances (`:270`). The 0.3% fee is `ceil(amount_in * 3 / 1000)` per side (`:288`), and the invariant check is `(balance_0 - fee_0) * (balance_1 - fee_1) >= reserve_0 * reserve_1` (`:294`). `to` may not be either pool token (`:257`).
- **`withdraw`** (`src/lib.rs:315`): burns the LP shares the caller already transferred into the pair, minus the locked minimum (`:329`), and pays out pro rata against the pair's actual balances (`:339`). The locked 1000 shares are never burned.
- **`mint_fee`** (`src/lib.rs:435`): reads `fees_enabled` and `fee_to` from the factory through `SoroswapFactoryClient` (`:444`). When on and `k_last != 0`, it mints `total_supply * (sqrt(k) - sqrt(k_last)) / (sqrt(k) * 5 + sqrt(k_last))` to `fee_to` (`:456`), which is one sixth of the growth in K. When off it clears `k_last` (`:465`). Returns whether the fee is on, and callers use that to decide whether to store the new `k_last`.
- **`create_name` / `create_symbol`** (`src/lib.rs:41`, `:33`): LP metadata is `"<SYM0>-<SYM1> Soroswap LP Token"` and `"<SYM0>-<SYM1>-SOROSWAP-LP"`, each underlying symbol truncated to six characters.

## Storage layout and TTL

Pool state, all **instance** storage, `DataKey` as `repr(u32)` 0 through 5 (`src/storage.rs:7`): `Token0`, `Token1`, `Reserve0`, `Reserve1`, `Factory`, `KLast`. The key encodes as a bare `u32` through the manual `TryFromVal` impl (`src/storage.rs:23`), not as a symbol, so the on chain keys are integers.

LP token state (`src/soroswap_pair_token/storage_types.rs:25`): `Allowance(AllowanceDataKey)` in **temporary** storage (`src/soroswap_pair_token/allowance.rs:6`), `Balance(Address)` in **persistent** storage (`src/soroswap_pair_token/balance.rs:6`), `TotalSupply` in **instance** storage (`src/soroswap_pair_token/total_supply.rs:6`).

TTL: pool instance bump 30 days of ledgers, threshold 29 (`src/storage.rs:19`), extended by `extend_instance_ttl` at the top of every entrypoint. LP token instance 30 days, balances 120 days (`src/soroswap_pair_token/storage_types.rs:4`, `:7`). Balances bump on every read and write (`src/soroswap_pair_token/balance.rs:7`, `:20`).

## Errors

`SoroswapPairError` (`src/error.rs:6`), codes 101 to 118: `InitializeAlreadyInitialized` 101, `NotInitialized` 102, `InitializeTokenOrderInvalid` 103, `DepositInsufficientAmountToken0` 104, `DepositInsufficientAmountToken1` 105, `DepositInsufficientFirstLiquidity` 106, `DepositInsufficientLiquidityMinted` 107, `SwapInsufficientOutputAmount` 108, `SwapNegativesOutNotSupported` 109, `SwapInsufficientLiquidity` 110, `SwapInvalidTo` 111, `SwapInsufficientInputAmount` 112, `SwapNegativesInNotSupported` 113, `SwapKConstantNotMet` 114, `WithdrawLiquidityNotInitialized` 115, `WithdrawInsufficientSentShares` 116, `WithdrawInsufficientLiquidityBurned` 117, `UpdateOverflow` 118.

`UpdateOverflow` (118) is declared and referenced in a doc comment (`src/lib.rs:167`) but never constructed anywhere in the crate. Do not document it to integrators as reachable.

## Events

Topic tuple `("SoroswapPair", <symbol>)`:

| Symbol | Payload | Publisher |
|---|---|---|
| `deposit` | `DepositEvent { to, amount_0, amount_1, liquidity, new_reserve_0, new_reserve_1 }` | `src/event.rs:33` |
| `swap` | `SwapEvent { to, amount_0_in, amount_1_in, amount_0_out, amount_1_out }` | `src/event.rs:64` |
| `withdraw` | `WithdrawEvent { to, liquidity, amount_0, amount_1, new_reserve_0, new_reserve_1 }` | `src/event.rs:98` |
| `sync` | `SyncEvent { new_reserve_0, new_reserve_1 }` | `src/event.rs:115` |
| `skim` | `SkimEvent { skimmed_0, skimmed_1 }` | `src/event.rs:133` |

A `sync` event fires from `update` (`src/lib.rs:475`), so every deposit, withdraw, swap and sync emits one. LP token operations emit the standard SEP token events through `TokenUtils` (`src/soroswap_pair_token/contract.rs:31`, `:44`, `:108`).

## Cross-contract calls

- Factory: `fees_enabled` and `fee_to` on every `mint_fee` (`src/lib.rs:445`, `:449`), so every deposit and withdraw costs two cross contract reads.
- Underlying tokens: `symbol()` at initialize (`src/lib.rs:110`), `balance()` on every balance read (`src/balances.rs:10`), `transfer()` on payouts (`src/lib.rs:423`). The client comes from a compile time import of the test token WASM (`src/lib.rs:20`) with a TODO to replace it with a generic interface (`src/lib.rs:18`).

## Gotchas and invariants

- **The pair never pulls tokens.** `deposit`, `swap` and `withdraw` all assume the caller already transferred value in. Calling them directly without a preceding transfer either errors or lets someone else claim the funds. The router is the safe caller.
- **No `require_auth` on any pool function.** Access control is entirely positional: whoever moved tokens in first gets the result. Do not add a pool function that transfers value based on an argument alone.
- `initialize` requires `token_0 < token_1` (`src/lib.rs:104`). The factory always satisfies this through `Pair::new` (`contracts/factory/src/pair.rs:22`).
- `get_reserves` returns only the two reserves, no timestamp, despite the doc comment at `src/lib.rs:392` saying "and the last block timestamp". There is no price accumulator or TWAP in this contract.
- LP token decimals are hardcoded to 7 (`src/lib.rs:113`) regardless of the underlying tokens' decimals.
- `src/strings.rs` uses fixed stack buffers: 100 bytes in `take_first_n_chars` (`:12`) and 35 bytes in `concat` (`:22`). A pool token whose symbol exceeds 100 bytes, or a longer LP suffix, panics at `initialize`. The current suffixes fit: 6 + 1 + 6 + 12 for the symbol and 6 + 1 + 6 + 18 for the name.
- `put_reserve_0` and `put_reserve_1` panic on a negative amount (`src/storage.rs:94`, `:102`) rather than returning an error.
- The 1000 locked LP shares mean total supply never returns to zero once a pool is seeded, which is what blocks the first depositor inflation attack.
- The fee is charged with a ceiling division (`src/math.rs:6`), rounding in the pool's favor, and it is applied per input side rather than only on the input token.

## Testing

`make test` in `contracts/pair` builds `../token` first, then runs `cargo test` (`contracts/pair/Makefile:5`). Tests register the real token and factory WASM (`src/test.rs:19`, `:31`) so a stale `../factory/target` build makes the pair suite test old factory behavior. `src/test/fee.rs` covers the protocol fee path, `src/test/soroswap_pair_token.rs` the LP token.
