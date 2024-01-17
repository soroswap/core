#!/bin/bash

# This script takes two arguments:
NETWORK="$1"
N_TOKENS=${2:-4}
DECIMAL=7
LOGO=""
JSON_FILE="/workspace/.soroban/random_tokens.json"
# Using /var/tmp which is specific to the Docker container
FLAG_FILE="/var/tmp/soroban_first_run_completed"

# Stellar Assets config
GENERATED_STELLAR_ASSETS="/workspace/.soroban/generated_stellar_assets.json"

# Validate the input arguments
if [ -z "$NETWORK" ]; then
    echo "Error: Network name must be provided."
    echo "Usage: bash /workspace/scripts/deploy_random_tokens.sh <network> <number_of_tokens(optional)>"
    exit 1
fi

# Initialize or read the existing JSON structure
if [ -f "$JSON_FILE" ]; then
    JSON_DATA=$(cat "$JSON_FILE")
else
    JSON_DATA='[]'
fi

# Check if the script has run before in this Docker instance
if [ -f "$FLAG_FILE" ]; then
    # Script has run before in this Docker instance
    FIRST_RUN=false
else
    # First run of the script in this Docker instance
    FIRST_RUN=true
    touch "$FLAG_FILE"
fi

# Get the token admin address
TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"

# Arrays of common name syllables/parts
name_parts=("bel" "nar" "xis" "mik" "tar" "rin" "jas" "kel" "fen" "lor"
            "ana" "ser" "vin" "zel" "leo" "mia" "ara" "tia" "neo" "kai"
            "eva" "lio" "ria" "dor" "ael" "nia" "the" "sia" "cal" "ian"
            "ora" "ron" "lyn" "dan" "gav" "zoe" "axl" "nix" "kye" "rey")

echo Deploying $N_TOKENS tokens to network $NETWORK

TOKENS_ARRAY="[]"
for ((i=1; i<=N_TOKENS; i++))
do
    # Generate a random name
    part1=${name_parts[$RANDOM % ${#name_parts[@]}]}
    part2=${name_parts[$RANDOM % ${#name_parts[@]}]}
    NAME="${part1}${part2}"

    # Extract the first 4 letters of the name for the shorter version and uppercase
    SYMBOL=$(echo ${NAME:0:4} | tr '[:lower:]' '[:upper:]')

    TOKEN_WASM="/workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm"

    TOKEN_ID="$(
      soroban contract deploy --network $NETWORK --source token-admin \
        --wasm $TOKEN_WASM
      )"

    soroban contract invoke \
      --network $NETWORK --source token-admin \
      --id $TOKEN_ID \
      -- \
      initialize \
      --admin "$TOKEN_ADMIN_ADDRESS" \
      --decimal $DECIMAL \
      --name "$NAME" \
      --symbol "$SYMBOL"

    TOKEN_ADDRESS="$(node /workspace/scripts/address_workaround.js $TOKEN_ID)"

    TOKEN_JSON="{\"address\": \"$TOKEN_ADDRESS\", \"name\": \"$NAME\", \"symbol\": \"$SYMBOL\", \"logoURI\": \"$LOGO\", \"decimals\": $DECIMAL}"
    TOKENS_ARRAY=$(echo $TOKENS_ARRAY | jq ". += [$TOKEN_JSON]")
    echo $TOKEN_JSON
done

node /workspace/scripts/issue_stellar_assets.js $NETWORK $N_TOKENS
GENERATED_ASSETS_JSON=$(jq '.tokens' "$GENERATED_STELLAR_ASSETS")
for ((i=1; i<=N_TOKENS; i++)) do
    ASSET_SYMBOL=$(echo "$GENERATED_ASSETS_JSON" | jq -r ".[$i-1].symbol")
    ASSET_NAME=$(echo "$GENERATED_ASSETS_JSON" | jq -r ".[$i-1].asset")

    node /workspace/scripts/stellar_mint_asset_test.js $NETWORK $ASSET_NAME

    ASSET_JSON="{\"asset\": \"$ASSET_NAME\", \"symbol\": \"$ASSET_SYMBOL\"}"
    TOKENS_ARRAY=$(echo $TOKENS_ARRAY | jq ". += [$ASSET_JSON]")
    echo $ASSET_JSON
done

# Check if the network object already exists in the JSON data
NETWORK_EXISTS=$(echo $JSON_DATA | jq "any(.[]; .network == \"$NETWORK\")")

if [ "$NETWORK_EXISTS" = "true" ]; then
    if [ "$NETWORK" = "standalone" ] && [ "$FIRST_RUN" = true ]; then
        # For the first run on standalone, replace the tokens array
        JSON_DATA=$(echo $JSON_DATA | jq "map(if .network == \"$NETWORK\" then .tokens = $TOKENS_ARRAY else . end)")
    else
        # Append new tokens to the existing array
        JSON_DATA=$(echo $JSON_DATA | jq "map(if .network == \"$NETWORK\" then .tokens += $TOKENS_ARRAY else . end)")
    fi
else
    # Add a new network object
    NEW_NETWORK_JSON="{\"network\": \"$NETWORK\", \"tokens\": $TOKENS_ARRAY}"
    JSON_DATA=$(echo $JSON_DATA | jq ". += [$NEW_NETWORK_JSON]")
fi

# Write the updated JSON back to the file
echo $JSON_DATA | jq '.' > "$JSON_FILE"