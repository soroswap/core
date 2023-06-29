# Soroswap core Smart Contracts

## 0. Too long to read?
Install:
```
yarn
```
In one terminal
```
bash quickstart.sh standalone
```
In another terminal
```
bash run.sh
```

Create 8 tokens in the standalone network.
List will be available in .soroban/tokens.json
```
bash scripts/setup_and_create_tokens.sh standalone 8 ## Create 8 tokens in the standalone network
```

Serve the list of tokens at http://localhost:8010/api/tokens
```
node api/server.js #Serving file to :8010
```

Run any command you want. Here are some examples:
```
bash initialize.sh standalone
bash initialize_pair.sh standalone
bash initialize_factory.sh standalone
cd pair && make && make test
cd factory && make && make test
```

## 1. Environment Preparation:
 
1.- Run the Stellar Quicktart and the @esteblock/soroban-preview:9 Docker containers
Currently, Soroswap Protocol supports PREVIEW-9:
Here you can choose to use an `standalone` or `futurenet` instance 
```
bash quickstart.sh standalone
```
With this, a `stellar` container and a `soroban-preview-9` will run, sharing the `soroban-network` network in Docker

2.- Run a terminal of the soroban-preview
```
bash run.sh
```

## 2.- Compile
Inside the soroban-preview container, compile both the `core` and the `factory` contract
```
make build
```
If you ran this command in the `/workspace` path, this will compile both contracts

## 3.- Run rust test
1.- Run tests of the Pair contract:
```
cd pair
make test
```
2.- Run tests of the Factory contract:
```
cd factory
make test
```

## 4.- Experiment the Pair and the Factory contract using the soroban CLI

A full tutorial has been written showing in detail, step-by-step how to experiment with these contracts using the soroban CLI

Check it in: [docs.soroswap.finance](https://docs.soroswap.finance/), on a [6 chapters dev.to tutorial: ](https://dev.to/esteblock/series/22986) or directly on the [soroswap/docs repo](https://github.com/soroswap/docs)

If you want to go fast to the soroban CLI experiment, just run:

To test the Pair contract, inside the `soroban-preview-9` container run:
```bash
bash initialize_pair.sh standalone
```

To test the Pair contract, inside the `soroban-preview-9` container run:
```bash
bash initialize_factory.sh standalone
```
