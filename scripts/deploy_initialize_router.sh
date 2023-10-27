#!/bin/bash

# This script is used for setting up and deploying smart contracts to the Soroban network.
# It should be run from the project root directory.
#
# Usage:
# bash /path_to_script/deploy_contracts.sh <network> <n_tokens> <run_setup>
#
# <network>: Name of the Soroban network to connect to.
# <n_tokens>: The number of tokens (this argument is not used in the script but kept for compatibility).
# <run_setup>: Set to "false" to skip running the setup script, any other value will run setup.
#
# Example:
# bash /path_to_script/deploy_contracts.sh standalone 5 false
#
# Dependencies:
# - soroban: Make sure the 'soroban' CLI tool is available.
# - make: Ensure that 'make' is available for building contracts.

# Enable the 'exit immediately' shell option
set -e

# Accept command-line arguments
NETWORK="$1"
RUN_SETUP="$2"

# Validate the input arguments
if [ -z "$NETWORK" ]; then
    echo "Error: Network name must be provided."
    echo "Usage: bash /path_to_script/deploy_contracts.sh <network> <n_tokens> <run_setup>"
    exit 1
fi

# Run the setup script if RUN_SETUP is not set to "false"
if [ "$RUN_SETUP" != "false" ]; then
    bash /workspace/scripts/setup.sh $NETWORK
fi

# Define the arguments to be passed to the 'soroban contract' commands
ARGS="--network $NETWORK --source token-admin"
echo "Using ARGS: $ARGS"

# Create a directory for Soroban files if it doesn't exist
mkdir -p .soroban

echo "--"
echo "--"

# Compile the pair contract
echo "Compile router contract"
cd /workspace/contracts/router
make build

echo "--"
echo "--"

# Define the paths to the compiled WASM files
ROUTER_WASM="/workspace/contracts/router/target/wasm32-unknown-unknown/release/soroswap_router.wasm"

# Deploy the Router contract
echo "Deploy the router contract"
ROUTER_ID="$(
  soroban contract deploy $ARGS \
    --wasm $ROUTER_WASM
)"
echo "$ROUTER_ID" > /workspace/.soroban/router_id
echo "SoroswapRouter deployed successfully with ROUTER_ID: $ROUTER_ID"

echo "--"
echo "--"

# Get the factory address
FACTORY_FILE="/workspace/.soroban/factory_id"
FACTORY_ID=$(cat $FACTORY_FILE)

FACTORY_ADDRESS="$(node /workspace/scripts/address_workaround.js $FACTORY_ID)"

# Initialize the SoroswapRouter contract
echo "Initialize the SoroswapRouter contract"
soroban contract invoke \
  $ARGS \
  --wasm $ROUTER_WASM \
  --id $ROUTER_ID \
  -- \
  initialize \
  --factory "$FACTORY_ADDRESS" \

echo "--"
echo "--"

ROUTER_ADDRESS="$(node /workspace/scripts/address_workaround.js $ROUTER_ID)"

# Create the new ROUTER object with the updated router id and addresses
NEW_ROUTER_OBJECT="{ \"network\": \"$NETWORK\", \"router_id\": \"$ROUTER_ID\", \"router_address\": \"$ROUTER_ADDRESS\" }"
echo "New router object: $NEW_ROUTER_OBJECT"
# NEW_ROUTER_OBJECT="{ \"network\": \"futurenet\", \"factory_id\": \"5adb2e4748f175bcc1ab4e11c0f03bc275701ef556cd9d2b10becb37ea6a33c9\", \"factory_address\": \"CBNNWLSHJDYXLPGBVNHBDQHQHPBHK4A66VLM3HJLCC7MWN7KNIZ4SLNG\"}"

ROUTER_FILE="/workspace/.soroban/router.json"
# Initialize router.json if it does not exist
if [[ ! -f "$ROUTER_FILE" ]]; then
    echo file not found
    echo "[]" > "$ROUTER_FILE"
fi


CURRENT_ROUTER_JSON=$(cat $ROUTER_FILE)
echo "CURRENT_ROUTER_JSON: $CURRENT_ROUTER_JSON"


# check if the network already exists in that json
exists=$(echo "$CURRENT_ROUTER_JSON" | jq '.[] | select(.network == "'$NETWORK'")')
echo "This network already exist in the router.json? : $exists"

NEW_ROUTER_JSON="{}"
if [[ -n "$exists" ]]; then
    # if the network exists, update the factory for that network
    echo network exists, replace
    NEW_ROUTER_JSON=$(echo "$CURRENT_ROUTER_JSON" | jq '
        map(if .network == "'$NETWORK'" then '"$NEW_ROUTER_OBJECT"' else . end)'
    )
else
    # if the network doesn't exist, append the new object to the list
    echo network does not exist, append
    NEW_ROUTER_JSON=$(echo "$CURRENT_ROUTER_JSON" | jq '. += ['"$NEW_ROUTER_OBJECT"']')
fi

# echo "NEW_ROUTER_JSON: $NEW_ROUTER_JSON"
echo "$NEW_ROUTER_JSON" > "$ROUTER_FILE"

echo "end creating the router" 

# Output the file path and contents
echo "Router information available in /workspace/.soroban/router.json"
cat /workspace/.soroban/router.json
