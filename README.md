# Soroswap core Smart Contracts
You'll need node, yarn and Docker installed


Check the documentation in
- https://github.com/soroswap/docs/
- https://docs.soroswap.finance/

## TLDR;
### 1. Setup 
1.1. Clone this repo
```
git clone http://github.com/soroswap/core.git
```
1.2 yarn install
```
yarn 
```
1.3 In one terminal
```
bash quickstart.sh standalone # or futurenet
```
1.4. In another terminal
```
bash run.sh
```

### 2. Create N tokens, deploy factory and 4 pairs.

This will create `.soroban/tokens.json`, `.soroban/factory.json`, `.soroban/pairs.json` and `.soroban/token_admin_keys.json`
```
bash scripts/deploy_tokens_n_pairs.sh standalone 8 # put a even number to not to breack the pair creation
```

### 3. (For local development): Serve those .json files 

In a new terminal run

```
bash serve_with_docker.sh
```
This will serve:
- List of tokens at http://localhost:8010/api/tokens
- Factory addresses http://localhost:8010/api/factory
- Admin keys http://localhost:8010/api/keys
- Created pairs http://localhost:8010/api/keys

The created pairs won't be readed by the front-end, however will be useful to debug

#### 5. (For production): Public those .json files and serve them using Vercel
From project root:
```
bash run.sh
bash scripts/upload_addresses.sh
```
Make sure that the origin is the soroswap/core.git ... Otherwise the only thing to do is to update the files on public and push them to main.

If everything goes right. Vercel will serve the created .json files in the following API's:

https://api.soroswap.finance/api/factory
https://api.soroswap.finance/api/keys
https://api.soroswap.finance/api/tokens
https://api.soroswap.finance/api/pairs

____
____
____


# Environment Preparation:
 
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
