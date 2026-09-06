# Address API Module

> **Living document.** Read this before modifying the module. Update it in the same change whenever the routes, the served files, or the Vercel config change.

**Source:** `api/`, `public/`, `vercel.json` · **Last verified:** 2026-09-06

## Purpose

A minimal Express service that publishes the deployed contract addresses and token lists as JSON over HTTP. It is the protocol's address registry for anything outside this repo. It has no database and no logic beyond reading files: the data comes from `public/`, which is populated by `yarn upload` (see [deploy-tooling.md](deploy-tooling.md)).

## Structure

| File | Purpose |
|---|---|
| `api/server.js` | The whole service, CommonJS Express app. |
| `api/package.json` | Its own dependency set, separate from the repo root. |
| `vercel.json` | Vercel build and routing config. |
| `public/<network>.contracts.json` | `{ ids, hashes }` per network, currently `mainnet` and `testnet`. |
| `public/tokens.json`, `public/random_tokens.json` | Token lists served verbatim. |
| `public/backup-<date>/` | Dated snapshots written by `yarn upload`. |
| `public/mainnet-deployment-2024-03/` | Archived optimized WASM for the mainnet deployment. |

## Endpoints

| Method | Path | Handler | Notes |
|---|---|---|---|
| GET | `/` | `api/server.js:16` | Returns `healthy`. |
| GET | `/api/tokens` | `api/server.js:20` | Sends `tokens.json`, 404 if missing. |
| GET | `/api/random_tokens` | `api/server.js:30` | Sends `random_tokens.json`, 404 if missing. |
| GET | `/api/:network/:contractName` | `api/server.js:40` | Reads `<network>.contracts.json` and returns `{ address }` from its `ids` map (`:51`). 404 if the file or the name is missing. |

All JSON responses set `Cache-Control: no-store` (`api/server.js:23`, `:33`, `:49`). CORS is open to all origins (`api/server.js:14`).

## Key behavior

- **Directory switch** (`api/server.js:8`): when `process.env.VERCEL === "1"` it serves from `<repo>/public`, otherwise from the absolute path `/workspace/.soroban`. That second path only exists inside the docker container from `scripts/quickstart.sh`.
- **Vercel routing** (`vercel.json:10`): `/static/(.*)` maps to `/public/$1`, everything else to `api/server.js`. Only `api/*.js` is built (`vercel.json:5`).
- **Local serving**: `scripts/serve_with_docker.sh` runs a `node:18.18.2` container named `soroswapCoreApi` on the `soroban-network` docker network, publishes port 8010, installs `api/` dependencies and starts the server (`scripts/serve_with_docker.sh:27`, `:42`).

## Deployed addresses currently recorded

From `public/mainnet.contracts.json:2`: factory `CA4HEQTL2WPEUYKYKCDOHCDNIV4QHNJ7EL4J4NQ6VADP7SYHVRYZ7AW2`, router `CAG5LRYQ5JVEUI5TEID72EYOVX44TTUJT5BQR2J6J77FH65PCCFAJDDH`, plus WASM hashes for pair, factory, router and token (`:6`).

From `public/testnet.contracts.json:2`: factory `CDP3HMUH6SMS3S7NPGNDJLULCOXXEPSHY4JKUKMBNQMATHDHWXRRJTBY`, router `CCJUD55AG6W5HAI5LRVNKAE5WDP5XGZBUDS5WNTIVDU7O264UZZE7BRD`.

Pair addresses are not recorded here. They are derived deterministically, see `contracts/library/src/tokens.rs:62`.

## Dependencies

- `api/package.json:8` declares this service's own dependencies (`cors`, `express`, `dotenv`, `vercel`), so `yarn` must be run inside `api/` as well as at the repo root (`scripts/serve_with_docker.sh:42`).
- Data files come from `scripts/upload_addresses.ts`.
- Consumers: anything outside this repo that needs Soroswap's mainnet or testnet addresses. The repo itself records no consumer, so which Soroswap services read it cannot be verified from here.

## Gotchas and invariants

- **`public/` is committed on purpose.** `README.md:125` states the files must be committed for Vercel to serve them. Editing `public/` by hand desynchronizes it from `.soroban/`; use `yarn upload <network>`.
- `/api/:network/:contractName` reads only the `ids` map, never `hashes` (`api/server.js:48`). There is no route that exposes WASM hashes.
- Local mode is hardcoded to `/workspace/.soroban` (`api/server.js:11`), so running `node api/server.js` on a host machine serves 404s for everything.
- Both `package.json:8` and `api/package.json:5` define `"start": "node scripts/api/server.js"`, a path that does not exist. The real entry point is `api/server.js`.
- `README.md:111` documents `/api/<network>/factory` and `/api/<network>/router`; those work because `factory` and `router` are the keys in the `ids` map, not because the route special cases them.
- `vercel.json:3` still carries the placeholder name `my-express-api`.

## Testing

No tests. Verify by starting the container with `bash scripts/serve_with_docker.sh` and requesting the four routes on port 8010 (`README.md:104`).
