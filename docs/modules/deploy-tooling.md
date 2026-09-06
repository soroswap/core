# Deploy Tooling Module

> **Living document.** Read this before modifying the module. Update it in the same change whenever the scripts, the yarn commands, the address book format, or the required env vars change.

**Source:** `scripts/`, `utils/`, `configs.json`, `tsconfig.json` · **Last verified:** 2026-09-06

## Purpose

The TypeScript pipeline that installs the contract WASMs, deploys and initializes the factory and router, seeds test tokens and liquidity, and records every address and hash in `.soroban/<network>.contracts.json`. It is the only thing in the repo that talks to a live network, and its output files are what the public API and every downstream consumer read.

## Structure

| File | Purpose |
|---|---|
| `scripts/deploy.ts` | Full deployment: install, deploy, initialize, seed. Entry for `yarn deploy`. |
| `scripts/deploy_soroban_test_tokens.ts`, `deploy_token.ts`, `deploy_random_tokens.ts`, `deploy_stellar_test_tokens.ts`, `mint_token.ts` | Test token creation and minting. |
| `scripts/multi_add_liquidity.ts` | Builds N token paths and adds liquidity to each hop through the router. |
| `scripts/setup_native_token.ts` | Prepends wrapped native XLM to the tokens book and deploys the Stellar asset contract off mainnet (`scripts/setup_native_token.ts:22`, `:25`). |
| `scripts/upload_addresses.ts` | Backs up `public/` then copies `.soroban/` outputs into it. Entry for `yarn upload`. |
| `scripts/set_trustlines.ts`, `see_trustlines.ts` | Trustline management against the Soroswap token list. |
| `scripts/manual_testing/` | `all.ts` plus `mint`, `addLiquidity`, `swap`, `removeLiquidity`. Entry for `yarn test`. |
| `scripts/quickstart.sh`, `run.sh`, `serve_with_docker.sh` | Docker orchestration. |
| `scripts/old_bash/` | Superseded bash implementations, kept for reference. Do not extend. |
| `utils/address_book.ts` | `AddressBook`, the ids and hashes registry. |
| `utils/tokens_book.ts` | `TokensBook`, per network token list. |
| `utils/env_config.ts` | `EnvConfig`, RPC and keypair loading. |
| `utils/contract.ts` | Install, deploy, invoke, Stellar asset deploy, balance helpers. |
| `utils/tx.ts` | Transaction building and submission, `getCurrentTimePlusOneHour`. |
| `configs.json` | Docker image digests and per network RPC and passphrase config. |

## Commands

From `package.json:6`:

| Command | Runs |
|---|---|
| `yarn build` | `tsc` (`package.json:7`), output to `./dist` (`tsconfig.json`). |
| `yarn deploy <network>` | `yarn build && node dist/scripts/deploy.js` (`package.json:10`). |
| `yarn test <network> [public]` | `yarn build && node dist/scripts/manual_testing/all.js` (`package.json:9`). |
| `yarn upload <network>` | `node dist/scripts/upload_addresses.js` (`package.json:11`). |
| `yarn trustline <network> <folder>` | `yarn build && node dist/scripts/set_trustlines.js` (`package.json:12`). |
| `yarn seeTrustline <network>` | `yarn build && node dist/scripts/see_trustlines.js` (`package.json:13`). |

Networks accepted by `EnvConfig.loadFromFile`: `mainnet`, `standalone`, `futurenet`, `testnet`, `testnet-public` (`configs.json:6`, `:11`, `:17`, `:23`, `:29`). An unknown value throws (`utils/env_config.ts:57`).

Contracts are built separately with `make build` in `contracts/` (`contracts/Makefile:8`), which iterates `token pair factory library router` (`contracts/Makefile:3`).

## Key methods

- **`deployAndInitContracts`** (`scripts/deploy.ts:12`): installs pair, factory and router WASM (`:23`, `:27`, `:31`), deploys the factory and initializes it with the admin as `fee_to_setter` plus the pair WASM hash (`:41`), deploys the router and initializes it with the factory address (`:54`), then seeds 8 Soroban test tokens, 4 Stellar test tokens, 8 random tokens and liquidity across 3 paths (`:70` to `:83`). Ends by writing the address book (`:98`).
- **`installContract`** (`utils/contract.ts:26`): hashes the optimized WASM, records the hash under `hashes`, then uploads it. The WASM paths are hardcoded at `utils/contract.ts:12` and all point at `*.optimized.wasm`.
- **`deployContract`** (`utils/contract.ts:41`): derives the contract id from a **random** 32 byte salt (`:47`) and the source account, so every redeploy yields a new address. Only pairs are deterministic, and they are deployed by the factory, not here.
- **`AddressBook`** (`utils/address_book.ts:8`): `loadFromFile(network, folder = '.soroban')` (`:25`), `writeToFile()` always writes into `.soroban/` (`:45`), `getContractId` / `setContractId` (`:73`, `:89`), `getWasmHash` / `setWasmHash` (`:98`, `:114`). The on disk shape is `{ ids: {...}, hashes: {...} }`, see `public/mainnet.contracts.json`.
- **`EnvConfig.loadFromFile`** (`utils/env_config.ts:50`): reads `configs.json`, and for mainnet takes the RPC URL from `MAINNET_RPC_URL` instead of the config file (`:63`). `getUser(key)` (`:100`) pulls any other secret key by env var name.
- **`upload_addresses.ts` `main`** (`scripts/upload_addresses.ts:16`): creates `public/backup-<YYYY-MM-DD>/`, copies the existing `public/` files there, then copies `<network>.contracts.json`, `tokens.json` and `random_tokens.json` from `.soroban/` (`:34`).

## Dependencies

- `@stellar/stellar-sdk` 14 (`package.json:29`) plus `cors`, `date-fns`, `dotenv`, `express`, `soroban-client` and `vercel` (`package.json:28`).
- Env vars, read by name only: `SOROSWAP_ADMIN_SECRET_KEY` (`utils/env_config.ts:73`), `TEST_TOKENS_ADMIN_SECRET_KEY` (`scripts/deploy.ts:60`), `TESTING_ACCOUNT_SECRET_KEY` (`scripts/manual_testing/all.ts:11`), `TRUSTLINE_WALLET_SECRET_KEY` (`scripts/set_trustlines.ts:24`), `MAINNET_RPC_URL` (`utils/env_config.ts:63`). Template at `.env.example`.
- Docker images pinned by digest in `configs.json:2` and `:3`, consumed by `scripts/quickstart.sh:3` and `scripts/run.sh:1`.
- External: the Soroswap token list at `https://raw.githubusercontent.com/soroswap/token-list/main/tokenList.json` (`scripts/set_trustlines.ts:66`, `scripts/see_trustlines.ts:41`).

## Gotchas and invariants

- **`utils/contract.ts:20` reads `process.argv[2]` as the network at import time.** Any script that imports it, directly or transitively, must be invoked with the network as its first argument or `config()` throws. This is why every entry script re-reads `process.argv[2]` too.
- Scripts run from `dist/`, not from source. `utils/env_config.ts:9` resolves `.env` as `../../.env` relative to `__dirname`, which only lands on the repo root when running the compiled `dist/utils/env_config.js`. Running the TypeScript directly will not find the `.env`.
- `AddressBook.loadFromFile` takes a folder argument so you can read `public/`, but `writeToFile` is hardcoded to `.soroban/` (`utils/address_book.ts:45`). You cannot write back to `public/` except through `yarn upload`.
- `installContract` and `deployContract` need `*.optimized.wasm` (`utils/contract.ts:12`), which only exists after `soroban contract optimize` has run. `make build` does it per contract; a bare `cargo build` does not.
- `scripts/deploy.ts:14` and `:61` airdrop only when the network is not mainnet. A mainnet run assumes both accounts are already funded.
- The whole test token, random token and liquidity seeding block runs unconditionally in `deployAndInitContracts` (`scripts/deploy.ts:70` to `:83`), including on mainnet. The README claims it is skipped for mainnet (`README.md:88`); the code does not skip it.
- `.soroban/` and `.env` are gitignored (`.gitignore:7`, `:9`). Deployment output is only durable once `yarn upload` copies it into `public/` and it is committed.
- Never put secret keys into `configs.json` or `public/`. Only `.env` holds them.

## Testing

There is no unit test suite here. `yarn test <network> [public]` runs `scripts/manual_testing/all.ts` end to end against a real network: mint, add liquidity, swap, remove liquidity (`scripts/manual_testing/all.ts:16`). Passing `public` as the second argument reads addresses from `public/` instead of `.soroban/` (`scripts/manual_testing/all.ts:27`). Note `README.md:191` documents `bash scripts/manual_testing/all.sh`, a file that does not exist; the current entry point is the yarn script.
