# Factory Module

> **Living document.** Read this before modifying the module. Update it in the same change whenever the module's behavior, public functions, storage, errors, events, or dependencies change.

**Source:** `contracts/factory/` · **Crate:** `soroswap-factory` 0.0.2, `cdylib` (`contracts/factory/Cargo.toml:2`, `:9`) · **Last verified:** 2026-09-06

## Purpose

Deploys SoroswapPair instances at addresses derived deterministically from the token pair, keeps the registry of every pair created, and owns the protocol fee switch (`fee_to`, `fee_to_setter`, `fees_enabled`). Every pair reads the fee switch from here on each deposit and withdraw, and the router asks the factory whether a pair exists before creating one, so a change here has protocol wide blast radius.

## Structure

| File | Purpose |
|---|---|
| `src/lib.rs` | The `SoroswapFactory` contract, implementing `SoroswapFactoryTrait`. |
| `src/storage.rs` | `DataKey` enum, TTL constants, storage accessors. |
| `src/pair.rs` | Compile time import of the pair WASM, the `Pair` sorted token tuple, salt, and `create_contract`. |
| `src/event.rs` | Event payload structs and publishers. |
| `src/test/` | Test suites: `initialize`, `fee_to_setter`, `pairs`, `events`, `deterministic` (`src/test.rs:107`). |

## Public surface

All functions return `Result<_, FactoryError>` and are declared in the interface crate (`contracts/factory-interface/src/lib.rs:15`).

| Function | Signature | Auth |
|---|---|---|
| `fee_to` | `(e: Env) -> Result<Address, FactoryError>` (`src/lib.rs:44`) | none |
| `fee_to_setter` | `(e: Env) -> Result<Address, FactoryError>` (`src/lib.rs:61`) | none |
| `fees_enabled` | `(e: Env) -> Result<bool, FactoryError>` (`src/lib.rs:78`) | none |
| `all_pairs_length` | `(e: Env) -> Result<u32, FactoryError>` (`src/lib.rs:95`) | none |
| `get_pair` | `(e: Env, token_a: Address, token_b: Address) -> Result<Address, FactoryError>` (`src/lib.rs:114`) | none |
| `all_pairs` | `(e: Env, n: u32) -> Result<Address, FactoryError>` (`src/lib.rs:133`) | none |
| `pair_exists` | `(e: Env, token_a: Address, token_b: Address) -> Result<bool, FactoryError>` (`src/lib.rs:152`) | none |
| `initialize` | `(e: Env, setter: Address, pair_wasm_hash: BytesN<32>) -> Result<(), FactoryError>` (`src/lib.rs:178`) | **none, see gotchas** |
| `set_fee_to` | `(e: Env, to: Address) -> Result<(), FactoryError>` (`src/lib.rs:201`) | `fee_to_setter.require_auth()` (`src/lib.rs:208`) |
| `set_fee_to_setter` | `(e: Env, new_setter: Address) -> Result<(), FactoryError>` (`src/lib.rs:226`) | `fee_to_setter.require_auth()` (`src/lib.rs:233`) |
| `set_fees_enabled` | `(e: Env, is_enabled: bool) -> Result<(), FactoryError>` (`src/lib.rs:250`) | `fee_to_setter.require_auth()` (`src/lib.rs:257`) |
| `create_pair` | `(e: Env, token_a: Address, token_b: Address) -> Result<Address, FactoryError>` (`src/lib.rs:275`) | none, permissionless |

## Key methods

- **`create_pair`** (`src/lib.rs:275`): normalizes the tokens through `Pair::new`, rejects an existing pair with `CreatePairAlreadyExists`, deploys the pair WASM with the pair salt, then calls `pair.initialize(factory, token_0, token_1)` (`src/lib.rs:290`) and records the address in both indexes (`:296`, `:297`). Permissionless by design, anyone can list a pair.
- **`Pair::new`** (`src/pair.rs:22`): rejects identical tokens, otherwise sorts the two addresses ascending. This is why `(a, b)` and `(b, a)` resolve to the same storage key and the same deployed pair.
- **`Pair::salt`** (`src/pair.rs:33`): `sha256(token_0.to_xdr() || token_1.to_xdr())`. The comment at `src/pair.rs:37` warns that simplifying the XDR encoding would change the hash and therefore every pair address. `contracts/library` reproduces this exact salt off chain (`contracts/library/src/tokens.rs:16`).
- **`create_contract`** (`src/pair.rs:54`): `e.deployer().with_current_contract(salt).deploy(pair_wasm_hash)`, so the pair address depends on the factory address, the salt, and nothing else.
- **`add_pair_to_all_pairs`** (`src/storage.rs:139`): writes `PairAddressesNIndexed(total_pairs)` then increments, keeping the index 0 based.

## Storage layout and TTL

`DataKey` at `src/storage.rs:11`:

| Key | Storage | Type |
|---|---|---|
| `FeeTo` | instance | `Address` |
| `FeeToSetter` | instance | `Address` |
| `FeesEnabled` | instance | `bool` |
| `TotalPairs` | instance | `u32`, also the "is initialized" flag |
| `PairWasmHash` | persistent | `BytesN<32>` (`src/storage.rs:131`) |
| `PairAddressesNIndexed(u32)` | persistent | `Address` (`src/storage.rs:144`) |
| `PairAddressesByTokens(Pair)` | persistent | `Address` (`src/storage.rs:67`) |

TTL constants at `src/storage.rs:22`: instance bump 30 days of ledgers, threshold 29 days; persistent bump 60 days, threshold 59 days. Every entrypoint calls `extend_instance_ttl` (`src/storage.rs:29`). Persistent reads bump their own key through `get_persistent_extend_or_error` (`src/storage.rs:36`), and `get_pair_exists` bumps on a hit (`src/storage.rs:81`).

## Errors

`FactoryError` lives in the interface crate (`contracts/factory-interface/src/error.rs:6`): `NotInitialized` 201, `CreatePairIdenticalTokens` 202, `CreatePairAlreadyExists` 203, `InitializeAlreadyInitialized` 204, `PairDoesNotExist` 205, `IndexDoesNotExist` 206.

A local `PairError::CreatePairIdenticalTokens = 901` (`src/pair.rs:12`) is raised by `Pair::new` and converted to 202 by the `From` impl at `src/lib.rs:17`, so 901 never surfaces on chain.

## Events

All published under the topic tuple `("SoroswapFactory", <symbol>)`:

| Symbol | Payload struct | Emitted by |
|---|---|---|
| `init` | `InitializedEvent { setter }` | `src/event.rs:16` |
| `new_pair` | `NewPairEvent { token_0, token_1, pair, new_pairs_length }` | `src/event.rs:42` |
| `fee_to` | `FeeToSettedEvent { setter, old, new }` | `src/event.rs:67` |
| `setter` | `NewSetterEvent { old, new }` | `src/event.rs:88` |
| `fees` | `NewFeesEnabledEvent { fees_enabled }` | `src/event.rs:107` |

## Dependencies

- `soroswap-factory-interface` by path (`Cargo.toml:14`) for the trait, the client, and `FactoryError`.
- `soroban-sdk` 20.2.0 (`Cargo.toml:12`), `num-integer` (`Cargo.toml:13`).
- Compile time import of `../pair/target/wasm32-unknown-unknown/release/soroswap_pair.wasm` (`src/pair.rs:5`), so the pair must be built first. `contracts/factory/Makefile:9` builds `../token` and `../pair` before this crate.
- Consumers: the pair reads `fees_enabled` and `fee_to` from the factory (`contracts/pair/src/lib.rs:444`), the router calls `pair_exists` and `create_pair` (`contracts/router/src/lib.rs:80`) and `pair_exists` again in `remove_liquidity` (`contracts/router/src/lib.rs:514`).

## Gotchas and invariants

- **`initialize` has no `require_auth`** (`src/lib.rs:178`). The only guard is `has_total_pairs`, so on a fresh deployment the first caller wins. The deploy script initializes it in the same run as the deploy (`scripts/deploy.ts:37`).
- `initialize` sets `fee_to = setter` (`src/lib.rs:183`), so the deployer receives protocol fees until `set_fee_to` is called.
- `TotalPairs` doubles as the initialized flag (`src/storage.rs:61`). Removing or renaming that key breaks every entrypoint's guard.
- `get_fee_to` and `get_fee_to_setter` `.unwrap()` (`src/storage.rs:96`, `:110`). They are only safe because every caller checks `has_total_pairs` first. Keep that ordering.
- `fees_enabled` defaults to `false` when the key is unset (`src/storage.rs:100`), so the protocol fee is off until explicitly enabled.
- The pair WASM hash is a stored value, not the imported WASM. Upgrading the pair means calling `initialize` on a new factory, there is no setter for `PairWasmHash`.
- Rebuilding the pair changes its WASM and therefore the hash embedded in tests, but not the addresses of pairs already deployed.

## Testing

`make test` in `contracts/factory` runs `cargo build --target wasm32-unknown-unknown --release` for `../token` and `../pair` first, then `cargo test` (`contracts/factory/Makefile:5`). Tests register the real pair and token WASM rather than mocks (`src/test.rs:11`, `:26`, `:34`). `src/test/deterministic.rs` locks in the salt to address derivation, so a change to `Pair::salt` will fail there first.
