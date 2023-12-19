#!/bin/bash

# This script checks the number of token pairs in a smart contract on the Soroban network.
#
# Usage:
# ./scripts/check_how_many_pairs.sh <network>
#
# <network> : Name of the Soroban network to connect to.
#
# Example:
# ./scripts/check_how_many_pairs.sh standalone
#
# Dependencies:
# - jq: This script requires 'jq' for parsing JSON files. Make sure it is installed on your system.

# Accept the network name as the first command-line argument
NETWORK="$1"

# Validate that NETWORK argument is provided
if [ -z "$NETWORK" ]; then
    echo "Error: Network name must be provided."
    echo "Usage: ./scripts/check_how_many_pairs.sh <network>"
    exit 1
fi

# Define the arguments to be passed to the 'soroban contract invoke' command
ARGS="--network $NETWORK --source token-admin"

# Define the path to the compiled Soroban Factory Contract WebAssembly (WASM) file
FACTORY_WASM="/workspace/contracts/factory/target/wasm32-unknown-unknown/release/soroswap_factory.wasm"

# Extract the factory ID from the JSON file using 'jq'
# The '.factory' syntax specifies the 'factory' key in the JSON file
FACTORY_ID=$(jq -r '.factory' .soroban/factory.json)

# Check if jq command failed and FACTORY_ID is empty
if [ -z "$FACTORY_ID" ]; then
    echo "Error: Failed to extract FACTORY_ID from the JSON file."
    exit 1
fi

# Invoke the Soroban smart contract with the provided arguments, WASM file path, and factory ID
# The 'all_pairs_length' function is called to retrieve the number of token pairs
soroban contract invoke \
  $ARGS \
  --id $FACTORY_ID \
  -- \
  all_pairs_length \
