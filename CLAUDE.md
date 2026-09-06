# Soroswap Core

## Entry summary

- Soroswap's AMM protocol on Stellar Soroban: Rust contracts in `contracts/`, TypeScript deploy tooling in `scripts/` and `utils/`, address JSON served from `api/` and `public/`.
- Exposes three deployed contracts. **Router** is the integration surface: `initialize`, `add_liquidity`, `remove_liquidity`, `swap_exact_tokens_for_tokens`, `swap_tokens_for_exact_tokens`, `get_factory`, plus `router_quote` / `router_get_amount(s)_in|out` / `router_pair_for` read helpers (`contracts/router/src/lib.rs:177`). Errors are `CombinedRouterError` 501 to 515 (`contracts/router/src/error.rs:43`).
- **Factory**: `create_pair`, `get_pair`, `pair_exists`, `all_pairs`, `all_pairs_length`, `fee_to`, `fee_to_setter`, `fees_enabled`, `set_fee_to`, `set_fee_to_setter`, `set_fees_enabled`, `initialize` (`contracts/factory-interface/src/lib.rs:15`). Errors 201 to 206.
- **Pair**, deployed only by the factory: `deposit`, `swap`, `withdraw`, `skim`, `sync`, `get_reserves`, `k_last`, `token_0`, `token_1`, `factory` (`contracts/pair/src/lib.rs:55`), plus the full LP token interface on the same address. Errors 101 to 118. Swap fee 0.3%.
- Mainnet factory `CA4HEQTL2WPEUYKYKCDOHCDNIV4QHNJ7EL4J4NQ6VADP7SYHVRYZ7AW2`, router `CAG5LRYQ5JVEUI5TEID72EYOVX44TTUJT5BQR2J6J77FH65PCCFAJDDH` (`public/mainnet.contracts.json:2`).
- Testnet factory `CDP3HMUH6SMS3S7NPGNDJLULCOXXEPSHY4JKUKMBNQMATHDHWXRRJTBY`, router `CCJUD55AG6W5HAI5LRVNKAE5WDP5XGZBUDS5WNTIVDU7O264UZZE7BRD` (`public/testnet.contracts.json:2`). Pair addresses are derived, never listed.
- Also publishes the `soroswap-library` crate to crates.io (`contracts/library/Cargo.toml:12`) and serves addresses and token lists as JSON on Vercel (`api/server.js`, `vercel.json`).
- Consumes: Stellar RPC and Horizon per network (`configs.json:4`), the `soroswap/token-list` repo, and `soroswap-library` 0.3.0 from crates.io.

## Workspace layout

| Path | What it is |
|---|---|
| `contracts/factory/` | SoroswapFactory contract |
| `contracts/factory-interface/` | Shared trait, client and `FactoryError` rlib |
| `contracts/pair/` | SoroswapPair pool and its embedded LP token |
| `contracts/router/` | SoroswapRouter, the user facing contract |
| `contracts/library/` | `soroswap-library` crate and deployable helper contract |
| `contracts/token/` | Standard token used only as a test token |
| `scripts/`, `utils/` | TypeScript deploy, seed and manual test pipeline |
| `api/`, `public/`, `vercel.json` | Address and token list JSON service |
| `audits/` | OtterSec audit, 2024-02-22 |
| `docs/modules/` | Per module living docs, index at [docs/modules/README.md](docs/modules/README.md) |

**Each contract is a standalone crate with its own `Cargo.lock`. There is no cargo workspace.** `contracts/Makefile:3` drives them in order: `token pair factory library router`.

## Build, test, deploy

Commands verified in the repo. Do not invent variants.

- Build all contracts: `make build` from `contracts/` (`contracts/Makefile:8`). Per contract: `make build` in that directory, which also runs `soroban contract optimize` (`contracts/pair/Makefile:11`).
- Test all contracts: `make test` from `contracts/` (`contracts/Makefile:13`), which builds first. Per contract: `make test`, that is `cargo test` (`contracts/factory/Makefile:5`).
- Budget profiling: `cargo test budget -- --nocapture` from `contracts/router` (`README.md:179`), running `contracts/router/src/test/budget.rs`.
- Local network: `bash scripts/quickstart.sh standalone|futurenet|testnet` then `bash scripts/run.sh` (`README.md:43`, `:49`), which shells into the pinned `soroban-preview` container.
- Node deps: `yarn` at the repo root, and separately inside `api/` (`scripts/serve_with_docker.sh:42`).
- Deploy: `yarn deploy <network>` (`package.json:10`), which is `yarn build && node dist/scripts/deploy.js`. Networks: `mainnet`, `standalone`, `futurenet`, `testnet`, `testnet-public` (`configs.json:4`).
- Publish addresses: `yarn upload <network>` (`package.json:11`), then commit `public/`.
- End to end manual test: `yarn test <network> [public]` (`package.json:9`).
- Trustlines: `yarn trustline <network> <folder>` and `yarn seeTrustline <network>` (`package.json:12`, `:13`).

## Key entry points

- `contracts/router/src/lib.rs:388`: the contract every integrator calls.
- `contracts/factory/src/lib.rs:275`: `create_pair`, where pairs come from.
- `contracts/pair/src/lib.rs:239`: `swap`, the K invariant and the 0.3% fee.
- `contracts/library/src/tokens.rs:62`: `pair_for`, the deterministic pair address.
- `scripts/deploy.ts:12`: the full deployment sequence.
- `utils/address_book.ts:8`: how addresses and WASM hashes are recorded.
- `api/server.js:40`: how addresses are published.

## Gotchas

- **The router does not use `contracts/library`.** It pins `soroswap-library` 0.3.0 from crates.io (`contracts/router/Cargo.toml:23`, `contracts/router/Cargo.lock:1117`) while the in repo crate is 2.0.0 with a different API. Editing `contracts/library` changes nothing on chain until it is published and the pin is bumped.
- SDK skew: `contracts/library` is on soroban-sdk 22.x (`contracts/library/Cargo.toml:18`), every other crate is on 20.2.0.
- Contracts import each other's built WASM at compile time (`contracts/router/src/factory.rs:2`, `contracts/pair/src/lib.rs:20`). Build order matters, and the router and library need the `*.optimized.wasm` outputs.
- The pair never pulls tokens. `deposit`, `swap` and `withdraw` assume the caller already transferred value in and have **no** `require_auth`. Go through the router.
- `initialize` on both the factory (`contracts/factory/src/lib.rs:178`) and the router (`contracts/router/src/lib.rs:390`) is unauthenticated. First caller wins, and neither address can be changed afterwards.
- README drift: it claims test token and liquidity seeding is skipped on mainnet (`README.md:88`), but `scripts/deploy.ts:70` runs it unconditionally. It documents `bash scripts/manual_testing/all.sh` (`README.md:191`), which does not exist. Its "SorobanExp" link for the factory (`README.md:8`) points at the router address. Verify README claims against the code before relying on them.
- Never commit `.env`, secret keys or mnemonics. `.soroban/` and `.env` are gitignored (`.gitignore:7`, `:9`); `public/` is committed on purpose.

## Cross-repo dependencies

Outbound:

- **this repo -> `soroswap/token-list`**: fetches `https://raw.githubusercontent.com/soroswap/token-list/main/tokenList.json` to set and inspect trustlines. Evidence `scripts/set_trustlines.ts:66`, `scripts/see_trustlines.ts:41`.
- **this repo -> `soroswap-library` on crates.io**: the router depends on the published 0.3.0 build of this repo's own library crate. Evidence `contracts/router/Cargo.toml:23`, `contracts/router/Cargo.lock:1117`.
- **this repo -> `paltalabs/deterministic-address-soroban`**: reference for the deterministic pair address scheme. Evidence `contracts/library/src/tokens.rs:50`, `contracts/library/src/lib.rs:51`.
- **this repo -> `stellar-expert/soroban-build-workflow`**: reusable GitHub Actions workflow that builds and releases all five contracts. Evidence `.github/workflows/release.yml:11`, `:21`, `:31`, `:41`, `:51`.
- **this repo -> `esteblock/soroban-preview` and `stellar/quickstart` docker images**: pinned by digest for the local dev network. Evidence `configs.json:2`, `:3`, `scripts/quickstart.sh:72`, `:83`.
- **this repo -> `docs.soroswap.finance`**: protocol documentation. Evidence `README.md:16`.

Inbound:

- **this repo -> Vercel address API**: `public/*.contracts.json`, `public/tokens.json` and `public/random_tokens.json` are served as an HTTP address registry for consumers outside this repo. Evidence `api/server.js:40`, `vercel.json:16`, `README.md:109`. The repo records no consumer, so which Soroswap or PaltaLabs services read it **cannot be verified from this repo**. No reference to an aggregator, SDK, API or DeFindex repo exists in this source tree.

## Module Documentation Convention (MANDATORY)

Every module has a living doc at `docs/modules/<module>.md` (flat file, one per module). `docs/modules/README.md` is the index that routes a module's source path to its doc. These are the fast on-ramp for anyone, human or agent, touching a module.

**Progressive disclosure, do NOT load all docs at once.** When you are about to touch a module, open `docs/modules/README.md`, find the ONE doc matching the code you are changing, and read only that. Never pull the whole `docs/modules/` folder into context.

**The workflow rule:**

1. **Before modifying a module, read its `docs/modules/<module>.md` first.** It holds the file map, public functions with `file:line`, auth model, storage and TTL, errors, events, dependencies, and gotchas.
2. **After modifying a module, update its doc in the same change.** New or removed contract functions, changed auth, new storage keys, new error codes or events, changed fee math, dependency changes: all go into the doc before the work is done. Bump the "Last verified" date.
3. Doc claims must be verified against source and cite `file:line`. Never document something you have not confirmed exists. If you cannot verify it, write `_TBD, unverified_`.
4. **Adding a new module?** Create its `docs/modules/<module>.md` and add a row to `docs/modules/README.md` in the same change.

Docs follow a shared template: Purpose, Structure, Public surface, Key methods (`file:line`), Storage and TTL, Errors, Events, Dependencies, Gotchas and invariants, Testing.
