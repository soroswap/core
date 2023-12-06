#!/bin/bash

# This script takes two arguments:
NETWORK="$1"
DECIMAL=7
LOGO=""

# Validate the input arguments
if [ -z "$NETWORK" ]; then
    echo "Error: Network name must be provided."
    echo "Usage: bash /workspace/scripts/deploy_random_tokens.sh <network>"
    exit 1
fi

# Get the token admin address
TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"

# Arrays of common name syllables/parts
name_parts=("bel" "nar" "xis" "mik" "tar" "rin" "jas" "kel" "fen" "lor"
            "ana" "ser" "vin" "zel" "leo" "mia" "ara" "tia" "neo" "kai"
            "eva" "lio" "ria" "dor" "ael" "nia" "the" "sia" "cal" "ian"
            "ora" "ron" "lyn" "dan" "gav" "zoe" "axl" "nix" "kye" "rey")

echo Deploying tokens to network $NETWORK

for i in {1..4}
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
      --wasm $TOKEN_WASM \
      --id $TOKEN_ID \
      -- \
      initialize \
      --admin "$TOKEN_ADMIN_ADDRESS" \
      --decimal $DECIMAL \
      --name "$NAME" \
      --symbol "$SYMBOL"

    TOKEN_ADDRESS="$(node /workspace/scripts/address_workaround.js $TOKEN_ID)"

    # Save the token contract address and token id to a file on .soroban/temp_token.json
    echo "{\"address\": \"$TOKEN_ADDRESS\", \"name\": \"$NAME\", \"symbol\": \"$SYMBOL\", \"logoURI\": \"$LOGO\", \"decimals\": $DECIMAL}"

done