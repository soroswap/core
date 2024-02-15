#!/bin/bash

set -e

# Accept command-line arguments
NETWORK="$1"
RUN_SETUP="$2"

# Validate the input arguments
if [ -z "$NETWORK" ]; then
    echo "Error: Network name must be provided."
    echo "Usage: bash /path_to_script/deploy_contracts.sh <network> <run_setup>"
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
echo "Compile aggregator contract"
cd /workspace/contracts/aggregator
make build

echo "--"
echo "--"

# Define the paths to the compiled WASM files
AGGREGATOR_WASM="/workspace/contracts/aggregator/target/wasm32-unknown-unknown/release/soroswap_aggregator.optimized.wasm"

# Install the Pair contract WASM
echo "Install the Aggregator contract WASM"
echo "Install a WASM file to the ledger without creating a contract instance"

AGGREGATOR_WASM_HASH="$(
soroban contract install $ARGS \
  --wasm $AGGREGATOR_WASM
)"
echo "$AGGREGATOR_WASM_HASH" > /workspace/.soroban/aggregator_wasm_hash
echo "Aggregator contract installed successfully with hash: $AGGREGATOR_WASM_HASH"

echo "--"
echo "--"

# Deploy the Factory contract
echo "Deploy the Aggregator contract"
AGGREGATOR_ID="$(
  soroban contract deploy $ARGS \
    --wasm $AGGREGATOR_WASM
)"
echo "$AGGREGATOR_ID" > /workspace/.soroban/aggregator_id
echo "SoroswapAggregator deployed successfully with AGGREGATOR_ID: $AGGREGATOR_ID"

echo "--"
echo "--"

# Get the token admin address
TOKEN_ADMIN_ADDRESS="$(soroban keys address token-admin)"

ROUTER_FILE="/workspace/.soroban/router.json"
# Initialize factory.json if it does not exist
if [[ ! -f "$ROUTER_FILE" ]]; then
    echo file not found
    echo "[]" > "$ROUTER_FILE"
fi
CURRENT_ROUTER_JSON=$(cat $ROUTER_FILE)
ROUTER_ADDRESS=$(echo "$CURRENT_ROUTER_JSON" | jq --raw-output '.[] | select(.network == "'$NETWORK'").router_id')
echo $ROUTER_ADDRESS

AGGREGATOR_INITIALIZE_OBJECT=$(jq -n \
    --arg routerAddress "$ROUTER_ADDRESS" \
    '[ { "protocol_id": 0, "address": $routerAddress } ]')


echo $AGGREGATOR_INITIALIZE_OBJECT

echo "Initialize the SoroswapAggregator contract"
soroban contract invoke \
  $ARGS \
  --id $AGGREGATOR_ID \
  -- \
  initialize \
  --admin $TOKEN_ADMIN_ADDRESS \
  --protocol_addresses "$AGGREGATOR_INITIALIZE_OBJECT"

echo "--"
echo "--"

# Create the new AGGREGATOR object with the updated aggregator id and addresses
NEW_AGGREGATOR_OBJECT="{ \"network\": \"$NETWORK\", \"aggregator_id\": \"$AGGREGATOR_ID\", \"aggregator_address\": \"$AGGREGATOR_ID\" }"
echo "New aggregator object: $NEW_AGGREGATOR_OBJECT"

AGGREGATOR_FILE="/workspace/.soroban/aggregator.json"
# Initialize factory.json if it does not exist
if [[ ! -f "$AGGREGATOR_FILE" ]]; then
    echo file not found
    echo "[]" > "$AGGREGATOR_FILE"
fi


CURRENT_AGGREGATOR_JSON=$(cat $AGGREGATOR_FILE)
echo "CURRENT_AGGREGATOR_JSON: $CURRENT_AGGREGATOR_JSON"


# check if the network already exists in that json
exists=$(echo "$CURRENT_AGGREGATOR_JSON" | jq '.[] | select(.network == "'$NETWORK'")')
echo "This network already exist in the aggregator.json? : $exists"

NEW_AGGREGATOR_JSON="{}"
if [[ -n "$exists" ]]; then
    # if the network exists, update the factory for that network
    echo network exists, replace
    NEW_AGGREGATOR_JSON=$(echo "$CURRENT_AGGREGATOR_JSON" | jq '
        map(if .network == "'$NETWORK'" then '"$NEW_AGGREGATOR_OBJECT"' else . end)'
    )
else
    # if the network doesn't exist, append the new object to the list
    echo network does not exist, append
    NEW_AGGREGATOR_JSON=$(echo "$CURRENT_AGGREGATOR_JSON" | jq '. += ['"$NEW_AGGREGATOR_OBJECT"']')
fi

# echo "NEW_AGGREGATOR_JSON: $NEW_AGGREGATOR_JSON"
echo "$NEW_AGGREGATOR_JSON" > "$AGGREGATOR_FILE"

echo "end creating the factory" 

# Output the file path and contents
echo "Aggregator information available in /workspace/.soroban/aggregator.json"
cat /workspace/.soroban/aggregator.json
