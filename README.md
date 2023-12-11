# Soroswap core Smart Contracts

Make sure to compile contracts in the right order: token, pair, factory, library, router. If you just do `cd contracts && make build`, this will be done in the correct order ;)

Check the documentation in

- https://github.com/soroswap/docs/
- https://docs.soroswap.finance/

## TLDR;

### 0. Prerequisites

jq, docker, docker-compose, node, yarn

### 1. Setup

1.1. Clone this repo

```
git clone http://github.com/soroswap/core.git
```

1.2 yarn install

```
yarn
```

1.3 In one terminal: (choose standalone, futurenet or testnet)

```
bash scripts/quickstart.sh standalone # or futurenet or testnet
```

1.4. In another terminal

```
bash scripts/run.sh
```

### 2. Create N tokens, deploy SoroswapFactory, SoroswapRouter and create N^2 pairs.

This will create `.soroban/tokens.json`, `.soroban/factory.json`, `.soroban/pairs.json` and `.soroban/token_admin_keys.json`

Remember here to choose standalone, testnet or futurenet
```
bash scripts/deploy_tokens_n_pairs.sh standalone 8 # put a even number to not to breack the pair creation
```

This will:

- Create 8 test tokens.
- Build and install the SoroswapPair contract in Soroban.
- Build and Deploy the SoroswapFactory contract and initialize it with the installed SoroswapPair WASM.
- Build and Deploy the SoroswapRouter contract and initialize it with the deployed Factoy address.
- Create N^2 Pairs (all combinations between pairs) using the Router contract

This will create the `.soroban` folder with a lot of useful `.json` files with the contract and admin addresses.

### 3. (For local development): Serve those .json files

In a new terminal run

```
bash scripts/serve_with_docker.sh
```

This will serve:

- List of tokens at http://localhost:8010/api/tokens
- Factory addresses http://localhost:8010/api/factory
- Router addresses http://localhost:8010/api/router
- Admin keys http://localhost:8010/api/keys
- Created pairs http://localhost:8010/api/keys

The created pairs won't be readed by the front-end, however will be useful to debug

### 4. (For production): Public those .json files and serve them using Vercel

From project root:

```
bash scripts/run.sh
bash scripts/upload_addresses.sh
```

Make sure that the origin is the soroswap/core.git ... Otherwise the only thing to do is to update the files on ./public and push them to main.

If everything goes right. Vercel will serve the created .json files in the following API's:

https://api.soroswap.finance/api/factory
https://api.soroswap.finance/api/keys
https://api.soroswap.finance/api/tokens
https://api.soroswap.finance/api/pairs

#### Note:

If you want to deploy both in standalone an futurenet you can deploy first on futurenet and then on standalone. Then your dapp will connect to standalone using your quickstart containter and to futurenet using the public RPC.

---

# Compile and Test the contracts

## Prepare

1.- Run the Stellar Quickstart and the @esteblock/soroban-preview Docker containers
Here you can choose to use an `standalone` or `futurenet` instance

```
bash scripts/quickstart.sh standalone
```

2.- Run a terminal with the `@esteblock/soroban-preview` image container

```
bash scripts/run.sh
```

## Compile

3.- Inside the soroban-preview container, compile all contracts:

```
cd contracts
make build
```

If you ran this command in the `/workspace` path, this will compile both contracts

## Test

4.- Run tests in all contracts

```
make test
```

## Check budget usage

If you want to know how the memory and CPU instructions usage, you can go, for each contract and do:

```
cd router
cargo test budget -- --nocapture
```

This will run the `/contracts/router/budget.rs` test that by using `env.budget`, calculates the budget.

## Experiment using the soroban CLI

We have some tutorials about this. Check [docs.soroswap.finance](https://docs.soroswap.finance/), [6 chapters dev.to tutorial: ](https://dev.to/esteblock/series/22986) or the [soroswap/docs repo](https://github.com/soroswap/docs)

If you want to go fast to the soroban CLI "manual experiment":

```bash
bash scripts/manual_testing/all.sh standalone local
```

This will take all standalone deployments available in `.soroban` (local) and will do 1) Mint test tokens, 2) Provide Liqudiity 3) Swap 4) Remove Liquidity.

You can change `standalone` for `testnet` and use the `testnet` deployed address.
You can change `local` for `public` in order to take addresses from the `./public` folder

## Stellar Assets

The script `scripts/setup_stellar_classic_assets.sh` gets all the Stellar Assets from `stellar_classic_assets.json` and puts them into the tokens.json file for the API

This will get the token id and wrap the native (XLM) token if the network is standalone

```bash
#This gets the address of any Stellar assets --asset can be either native or <TOKEN:ISSUER>
soroban lab token id --network standalone --asset native

#If using standalone the native asset needs to be wrapped for it to work
soroban lab token wrap --asset native --network standalone --source-account my-account

#PS: If it gives an error... or the token is already wrapped or your source account is not funded
```

## `deploy_random_tokens.sh` Script Documentation

#### Overview

The `deploy_random_tokens.sh` script deploys a specified number of random test tokens to a selected blockchain network.

#### Usage

```bash
scripts/deploy_random_tokens.sh <network> [<number_of_tokens>]
```

- `<network>`: Choose from standalone, futurenet, or testnet.
- `<number_of_tokens>`: Optional, defaults to 4 if not specified.

#### Functionality

- Creates a specified number of test tokens.
- Saves token details in .soroban/random_tokens.json.
- Appends new tokens on futurenet and testnet.
- Replaces or appends tokens for standalone based on prior runs.

#### Handling Network Resets

Manually update or delete random_tokens.json post-reset on futurenet or testnet.
