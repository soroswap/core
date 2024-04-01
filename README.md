# Soroswap.Finance core Smart Contracts.

Soroswap.Finance is live on **Mainnet**
- Deployer Address: [`GAYPUMZFDKUEUJ4LPTHVXVG2GD5B6AV5GGLYDMSZXCSI4QILQKSY25JI`](https://stellar.expert/explorer/public/account/GAYPUMZFDKUEUJ4LPTHVXVG2GD5B6AV5GGLYDMSZXCSI4QILQKSY25JI)
- SoroswapFactory: `CA4HEQTL2WPEUYKYKCDOHCDNIV4QHNJ7EL4J4NQ6VADP7SYHVRYZ7AW2`
    - [See in Stellar.Expert](https://stellar.expert/explorer/public/contract/CA4HEQTL2WPEUYKYKCDOHCDNIV4QHNJ7EL4J4NQ6VADP7SYHVRYZ7AW2)
    - [Se in StellarChain](https://stellarchain.io/contracts/3872426bd59e4a61585086e3886d457903b53f22f89e361ea806ffcb07ac719f)
    - [See in SorobanExp](https://www.sorobanexp.com/blockchain/ct/contract/CAG5LRYQ5JVEUI5TEID72EYOVX44TTUJT5BQR2J6J77FH65PCCFAJDDH)


- SoroswapRouter: `CAG5LRYQ5JVEUI5TEID72EYOVX44TTUJT5BQR2J6J77FH65PCCFAJDDH`
    - [See in Stellar.Expert](https://stellar.expert/explorer/public/contract/CAG5LRYQ5JVEUI5TEID72EYOVX44TTUJT5BQR2J6J77FH65PCCFAJDDH)
    - [See in StellarChain](https://stellarchain.io/contracts/0dd5c710ea6a4a23b32207fd130eadf9c9ce899f4308e93e4ffe53fbaf108a04)
    - [See in SorobanExp](https://www.sorobanexp.com/blockchain/ct/contract/CAG5LRYQ5JVEUI5TEID72EYOVX44TTUJT5BQR2J6J77FH65PCCFAJDDH)

Check the documentation in https://docs.soroswap.finance/ and find the Audit Report by [OtterSec](https://osec.io/) in [./audits/2024-02-22_soroswap_ottersec_audit.pdf](audits/2024-02-22_soroswap_ottersec_audit.pdf)

## TLDR;

### 0. Prerequisites

jq, docker, docker-compose, node, yarn

### 1. Setup

1.1. Clone this repo

```
git clone http://github.com/soroswap/core.git
```

1.2 Copy the .env.example file to create a new .env file:

```
cp .env.example .env
```
Now, edit the .env file and provide the `SOROSWAP_ADMIN_SECRET_KEY`, `TEST_TOKENS_ADMIN_SECRET_KEY`, `TESTING_ACCOUNT_SECRET_KEY` and `MAINNET_RPC_URL` variables. This will be used to deploy the contracts


1.3 In one terminal: (choose standalone, futurenet or testnet)

```
bash scripts/quickstart.sh standalone # or futurenet or testnet
```

1.4 In another terminal

```
bash scripts/run.sh
```

1.5 yarn install

```
yarn
```

1.6 Build contracts

```
cd /workspace/contracts
make build
```


### 2. Deploy populated network

In the same terminal mentioned before, run:

>[!Note]
>- Accepted values for network are: `standalone | testnet | futurenet`
  

```bash
    yarn build
    # After building you can run:
    yarn deploy <network>

```


This will:  
- Install SoroswapPair. 
- Install SoroswapFactory.
- Install SoroswapRouter.
- Deploy and Initialize SoroswapFactory.
- Deploy and Initialize SoroswapRouter.
- If network is not Mainnet:
    - Deploy 8 test tokens
    - Add liquidity to 3 paths.
    - Deploy 4 Stellar Test Tokens
    - Deploy 8 Random Tokens
- Wrap Native XLM if does not exist.
- Create:
    - `.soroban/tokens.json`
    - `./soroban/<NETWORK>.contracts.json`.
    - `.soroban/random_tokens.json`

### 3. (For local development): Serve those .json files

In a new terminal run

```
bash scripts/serve_with_docker.sh
```

This will serve:

- List of tokens at `http://localhost:8010/api/tokens`
- List of random tokens at `http://localhost:8010/api/random_tokens`
- Factory address `http://localhost:8010/api/<network>/factory`
- Factory/Router addresses `http://localhost:8010/api/<network>/router`

### 4. (For production): Public those .json files and serve them using Vercel

From project root:

```
bash scripts/run.sh
yarn publish <network>
```

Then you will need to commit this changes to /public directory

Make sure that the origin is the soroswap/core.git ... Otherwise the only thing to do is to update the files on ./public and push them to main.

If everything goes right. Vercel will serve the created .json files

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

The script `scripts/setup_stellar_classic_assets.sh` gets all the Stellar Assets from `known_stellar_classic_assets.json` and puts them into the tokens.json file for the API

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

# Manual Testing

The following example showcase a local manual testing.
Make sure you have deployed the contracts to the network and saved it locally 


We provide code for manual testing of the contracts using `typescript`. This will allow us to play aroung without the need of an User Interface.

To run all the transsactions we could do on soroswap protocol just run (inside soroban-preview image):

```bash
# Usage:
# yarn test <network> <public>
# network: Type of the network to configure (standalone, futurenet, testnet, testnet-public)
# public: OPTIONAL: If you type `public` as a 2nd argument, you will run test agains the addresses in the public folder

yarn build #optional, if you have already built everything is not needed
yarn test standalone

```

To run tests against contracts addresses in your local `.soroban` folder, do:
```bash
yarn test standalone
```

To run tests agains contracts addresses in the public `public` folder, do:
```bash
yarn test public
```

# Set trustlines to all tokens from token-list

You can run the following script to set trustlines to all tokens from the token-list

```bash
yarn trustline <network> <folder>
```
where `network` is the network(it could be mainnet, testnet or standalone) to set the trustlines and `folder` is the folder where the token-list is located (public or .soroban). If you want to set the trustlines to the tokens in the public folder, you can run:

```bash
yarn trustline testnet public
```

however, if you use mainnet, folder is not used and the token list is the official soroswap token list.

In order to check the existing trustlines of the token list, you can run:

```bash
yarn seeTrustline mainnet
```