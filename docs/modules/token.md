# Token Module

> **Living document.** Read this before modifying the module. Update it in the same change whenever the public functions, storage, or TTL constants change.

**Source:** `contracts/token/` · **Crate:** `soroban-token-contract` 0.0.6, `cdylib` (`Cargo.toml:2`, `:8`) · **Last verified:** 2026-09-06

## Purpose

A standard, mintable Soroban token. It is **not part of the AMM protocol**. It exists so the deploy pipeline can create test tokens on standalone, futurenet and testnet, and so the factory, pair, router and library test suites have a real token WASM to register. Nothing in the protocol imports this crate as a Rust dependency; the other contracts import its compiled WASM.

## Structure

| File | Purpose |
|---|---|
| `src/lib.rs` | Crate root, wires the modules together. |
| `src/contract.rs` | The `Token` contract: admin functions plus `soroban_sdk::token::Interface`. |
| `src/admin.rs` | Admin read and write helpers. |
| `src/balance.rs`, `src/allowance.rs`, `src/metadata.rs` | Storage accessors. |
| `src/storage_types.rs` | `DataKey`, TTL constants. |
| `src/test.rs` | Single test module covering mint, transfer, allowance, burn, admin. |

## Public surface

| Function | Signature | Auth |
|---|---|---|
| `initialize` | `(e: Env, admin: Address, decimal: u32, name: String, symbol: String)` (`src/contract.rs:26`) | none, panics if an admin already exists (`:27`) |
| `mint` | `(e: Env, to: Address, amount: i128)` (`src/contract.rs:45`) | `admin.require_auth()` (`:48`) |
| `set_admin` | `(e: Env, new_admin: Address)` (`src/contract.rs:58`) | `admin.require_auth()` (`:60`) |
| `allowance` | `(e, from, spender) -> i128` (`src/contract.rs:80`) | none |
| `approve` | `(e, from, spender, amount, expiration_ledger: u32)` (`src/contract.rs:87`) | `from.require_auth()` (`:88`) |
| `balance` | `(e, id) -> i128` (`src/contract.rs:102`) | none |
| `transfer` | `(e, from, to, amount)` (`src/contract.rs:109`) | `from.require_auth()` (`:110`) |
| `transfer_from` | `(e, spender, from, to, amount)` (`src/contract.rs:123`) | `spender.require_auth()` (`:124`) |
| `burn` | `(e, from, amount)` (`src/contract.rs:138`) | `from.require_auth()` (`:139`) |
| `burn_from` | `(e, spender, from, amount)` (`src/contract.rs:151`) | `spender.require_auth()` (`:152`) |
| `decimals` / `name` / `symbol` | `(e) -> u32 / String / String` (`src/contract.rs:165`, `:169`, `:173`) | none |

`get_allowance` (`src/contract.rs:71`) is behind `#[cfg(test)]` (`:70`) and is not part of the deployed ABI.

## Storage layout and TTL

`DataKey` (`src/storage_types.rs:25`): `Allowance(AllowanceDataKey)`, `Balance(Address)`, `Nonce(Address)`, `State(Address)`, `Admin`. `Nonce` and `State` are declared but unused in this crate. `Admin` is instance storage (`src/admin.rs:6`).

TTL constants (`src/storage_types.rs:4`): instance bump 7 days of ledgers, threshold 6 days; balance bump 30 days, threshold 29 days. **These differ from the pair's LP token**, which uses 30 and 120 days (`contracts/pair/src/soroswap_pair_token/storage_types.rs:4`).

## Errors and events

No `contracterror` enum. Failures panic: `"already initialized"` (`src/contract.rs:28`), `"Decimal must fit in a u8"` (`:32`), and the non-negative amount check (`:17`). Events are the standard Soroban token events emitted through `TokenUtils`, including `mint` (`:55`) and `set_admin` (`:67`).

## Dependencies

- `soroban-sdk` 20.2.0 and `soroban-token-sdk` 20.2.0 (`Cargo.toml:11`, `:12`).
- Its compiled WASM is imported by the pair (`contracts/pair/src/lib.rs:20`), and registered in the factory, pair, router and library test suites.
- The deploy pipeline installs and deploys it: `utils/contract.ts:17` names the optimized WASM path, `scripts/deploy_soroban_test_tokens.ts:28` installs it, `scripts/deploy_token.ts` deploys and initializes each one.

## Gotchas and invariants

- `contracts/Makefile:3` lists `token` first in `SUBDIRS`, and factory, pair, router and library Makefiles all build `../token` first. It is the root of the build graph even though it is not part of the protocol.
- Do not treat this as the token interface the protocol targets. The pair imports this WASM only because there is no shared interface crate yet, flagged by the TODO at `contracts/pair/src/lib.rs:18`.
- `initialize` is unauthenticated. On a public network the first caller sets the admin.
- Its Makefile uses `soroban contract build` (`Makefile:9`) while every other contract Makefile uses `cargo build --target wasm32-unknown-unknown --release`.

## Testing

`make test` in `contracts/token` (`Makefile:5`). `src/test.rs` is a single module that also asserts the exact `e.auths()` shape for `mint` (`src/test.rs:30`), so changing the admin auth model breaks it immediately.
