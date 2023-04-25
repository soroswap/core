# Soroswap core Smart Contracts

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