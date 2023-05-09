# Soroswap core Smart Contracts

## 1. Environment Preparation:
 
1.- Run the Stellar Quicktart and the @esteblock/soroban-preview:8 Docker containers
```
bash quickstart.sh
```
With this, a `stellar` container and a `soroban-preview-8` will run, sharing the `soroban-network` network in Docker

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



## Upgrade from stellar remote
```
git remote add stellar https://github.com/stellar/soroban-examples
git pull stellar main
```

## Use Soroban Preview #8
```
currentDir=$(pwd)

docker run --volume  ${currentDir}:/workspace \
           --name soroban-preview-8 \
           --interactive \
           --tty \
           -p 8001:8000 \
           --detach \
           --ipc=host \
           --network soroban-network \
           esteblock/soroban-preview:8

```

```bash
  docker exec soroban-preview-8 make build
```

Or enter inside the docker container and run commands there
```bash
docker exec -it soroban-preview-8 bash
```