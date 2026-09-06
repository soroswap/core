# Module Documentation Index

Living docs, one per module. **Read the relevant doc before modifying a module; update it in the same change.** See "Module Documentation Convention" in `CLAUDE.md` for the workflow.

Each Rust contract under `contracts/` is its own crate with its own `Cargo.lock`. There is no cargo workspace.

## Contracts and crates

| Doc | Module | One-liner |
|---|---|---|
| [factory.md](factory.md) | `contracts/factory/` | Deploys and registers SoroswapPair instances at deterministic addresses, holds the protocol fee switch. |
| [factory-interface.md](factory-interface.md) | `contracts/factory-interface/` | Shared rlib crate with the `SoroswapFactoryTrait`, its generated client, and `FactoryError`. |
| [pair.md](pair.md) | `contracts/pair/` | Constant product AMM pool with a built in LP token, 0.3% swap fee, optional protocol fee. |
| [router.md](router.md) | `contracts/router/` | User facing entrypoint for add/remove liquidity and multi hop swaps, with deadline and slippage checks. |
| [library.md](library.md) | `contracts/library/` | Published `soroswap-library` crate plus deployable contract: token sorting, deterministic pair address, quote math. |
| [token.md](token.md) | `contracts/token/` | Standard Soroban token used only as a test token, not part of the protocol. |

## Tooling

| Doc | Module | One-liner |
|---|---|---|
| [deploy-tooling.md](deploy-tooling.md) | `scripts/`, `utils/`, `configs.json` | TypeScript install/deploy/initialize pipeline, address book, manual testing flows. |
| [address-api.md](address-api.md) | `api/`, `public/`, `vercel.json` | Express service on Vercel that serves the deployed contract addresses and token lists as JSON. |
