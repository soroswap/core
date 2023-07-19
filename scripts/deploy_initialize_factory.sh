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
N_TOKENS="$2"
RUN_SETUP="$3"

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
echo "Compile pair contract"
cd /workspace/contracts/pair
make build

echo "--"
echo "--"

# Compile the factory contract
echo "Compile factory contract"
cd /workspace/contracts/factory
make build

echo "--"
echo "--"

# Define the paths to the compiled WASM files
FACTORY_WASM="/workspace/contracts/factory/target/wasm32-unknown-unknown/release/soroswap_factory_contract.wasm"
PAIR_WASM="/workspace/contracts/pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
TOKEN_WASM="/workspace/contracts/token/soroban_token_contract.wasm"

# Install the Pair contract WASM
echo "Install the Pair contract WASM"
echo "Install a WASM file to the ledger without creating a contract instance"

PAIR_WASM_HASH="$(
soroban contract install $ARGS \
  --wasm $PAIR_WASM
)"
echo "$PAIR_WASM_HASH" > /workspace/.soroban/pair_wasm_hash
echo "Pair contract installed successfully with hash: $PAIR_WASM_HASH"

echo "--"
echo "--"

# Deploy the Factory contract
echo "Deploy the Factory contract"
FACTORY_ID="$(
  soroban contract deploy $ARGS \
    --wasm $FACTORY_WASM
)"
echo "$FACTORY_ID" > /workspace/.soroban/factory_id
echo "SoroswapFactory deployed successfully with FACTORY_ID: $FACTORY_ID"

echo "--"
echo "--"

# Get the token admin address
TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"

# Initialize the SoroswapFactory contract
echo "Initialize the SoroswapFactory contract"
soroban contract invoke \
  $ARGS \
  --wasm $FACTORY_WASM \
  --id $FACTORY_ID \
  -- \
  initialize \
  --setter "$TOKEN_ADMIN_ADDRESS" \
  --pair_wasm_hash "$PAIR_WASM_HASH"

echo "--"
echo "--"

FACTORY_ADDRESS="$(node /workspace/scripts/address_workaround.js $FACTORY_ID)"

# Create the new FACTORY object with the updated factory id and addresses
NEW_FACTORY_OBJECT="{ \"network\": \"$NETWORK\", \"factory_id\": \"$FACTORY_ID\", \"factory_address\": \"$FACTORY_ADDRESS\" }"
echo "New factory object: $NEW_FACTORY_OBJECT"
# NEW_FACTORY_OBJECT="{ \"network\": \"futurenet\", \"factory_id\": \"5adb2e4748f175bcc1ab4e11c0f03bc275701ef556cd9d2b10becb37ea6a33c9\", \"factory_address\": \"CBNNWLSHJDYXLPGBVNHBDQHQHPBHK4A66VLM3HJLCC7MWN7KNIZ4SLNG\"}"

FACTORY_FILE="/workspace/.soroban/factory.json"
# Initialize factory.json if it does not exist
if [[ ! -f "$FACTORY_FILE" ]]; then
    echo file not found
    echo "[]" > "$FACTORY_FILE"
fi


CURRENT_FACTORY_JSON=$(cat $FACTORY_FILE)
echo "CURRENT_FACTORY_JSON: $CURRENT_FACTORY_JSON"


# check if the network already exists in that json
exists=$(echo "$CURRENT_FACTORY_JSON" | jq '.[] | select(.network == "'$NETWORK'")')
echo "This network already exist in the factory.json? : $exists"

NEW_FACTORY_JSON="{}"
if [[ -n "$exists" ]]; then
    # if the network exists, update the factory for that network
    echo network exists, replace
    NEW_FACTORY_JSON=$(echo "$CURRENT_FACTORY_JSON" | jq '
        map(if .network == "'$NETWORK'" then '"$NEW_FACTORY_OBJECT"' else . end)'
    )
else
    # if the network doesn't exist, append the new object to the list
    echo network does not exist, append
    NEW_FACTORY_JSON=$(echo "$CURRENT_FACTORY_JSON" | jq '. += ['"$NEW_FACTORY_OBJECT"']')
fi

# echo "NEW_FACTORY_JSON: $NEW_FACTORY_JSON"
echo "$NEW_FACTORY_JSON" > "$FACTORY_FILE"

echo "end creating the factory" 


# # Save the network and factory information in a JSON file
# jq -n \
#   --arg network "$NETWORK" \
#   --arg factory_id "$FACTORY_ID" \
#   --arg factory_address "$FACTORY_ADDRESS" \
#   '[{"network": $network, "factory_id": $factory_id, "factory_address": $factory_address}]' \
#   > /workspace/.soroban/factory.json



# Output the file path and contents
echo "Factory information available in /workspace/.soroban/factory.json"
cat /workspace/.soroban/factory.json
