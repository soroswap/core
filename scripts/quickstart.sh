#!/bin/bash

previewVersion=$(jq -r '.previewVersion' preview_version.json)
quickstartHash=$(jq -r '.quickstartHash' preview_version.json)
 
set -e

case "$1" in
standalone)
    echo "Using standalone network"
    ARGS="--local --enable-soroban-diagnostic-events"
    ;;
futurenet)
    echo "Using Futurenet network"
    ARGS="--futurenet"
    ;;
testnet)
    echo "Using Testnet network"
    ARGS="--testnet"
    ;;
*)
    echo "Usage: $0 standalone|futurenet|testnet"
    exit 1
    ;;
esac

shift

echo "1. Creating docker soroban network"
(docker network inspect soroban-network -f '{{.Id}}' 2>/dev/null) \
  || docker network create soroban-network

echo "  "
echo "  "

echo "2. Running a soroban-precview docker container"
echo "  "
echo "  "

echo "3. Searching for a previous soroban-preview docker container"
containerID=$(docker ps --filter=`name=soroban-preview-${previewVersion}` --all --quiet)
if [[ ${containerID} ]]; then
    echo "Start removing soroban-preview-${previewVersion}  container."
    docker rm --force soroban-preview-${previewVersion}
    echo "Finished removing soroban-preview-${previewVersion} container."
else
    echo "No previous soroban-preview-${previewVersion} container was found"
fi
echo "  "
echo "  "

echo "4. Searching for a previous stellar container"
containerID=$(docker ps --filter=`name=stellar` --all --quiet)
if [[ ${containerID} ]]; then
    echo "Start removing stellar container."
    docker rm --force stellar
    echo "Finished removing stellar container."
else
    echo "No previous stellar container was found"
fi
echo "  "
echo "  "

currentDir=$(pwd)
docker run -dti \
  --volume ${currentDir}:/workspace \
  --name soroban-preview-${previewVersion} \
  -p 8001:8000 \
  --ipc=host \
  --network soroban-network \
  esteblock/soroban-preview:${previewVersion}

# Run the stellar quickstart image
docker run --rm -ti \
  --name stellar \
  --network soroban-network \
  -p 8000:8000 \
  stellar/quickstart:${quickstartHash} \
  $ARGS \
  --enable-soroban-rpc \
  --protocol-version 20 \
  --enable-soroban-diagnostic-events \
  "$@" # Pass through args from the CLI
