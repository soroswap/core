#!/bin/bash

# This script takes two arguments:
# $1: The network to deploy the token to
# $2: The address of the token admin account
# $3: Token name
# $4: Token symbol
# Then it deploys the token contract to the network and initializes it
# Then it saves the token contract address and token id to a file on .soroban/temp_token.json

NETWORK="$1"
TOKEN_ADMIN_ADDRESS="$2"
NAME="$3"
SYMBOL="$4"
LOGO="$5"
DECIMAL=7

mkdir -p /workspace/.soroban

# Compile the token contract
echo "Compile token contract"
cd /workspace/contracts/token
make build
cd /workspace/
TOKEN_WASM="/workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm"

echo Deploying token to network $NETWORK
echo $NETWORK
echo $TOKEN_WASM
echo Will deploy the token now
echo using WASM: $TOKEN_WASM
TOKEN_A_ID="$(
  soroban contract deploy \
  --wasm /workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm \
  --source token-admin \
  --network $NETWORK
  )"
TOKEN_A_ADDRESS="$(node ./scripts/address_workaround.js $TOKEN_A_ID)"

# echo TOKEN_A_ID: $TOKEN_A_ID

# echo Initializing TOKEN_A
# echo "Executing: 
#   fn initialize(  e: Env,
#                   admin: Address,
#                   decimal: u32,
#                   name: Bytes,
#                   symbol: Bytes) {
#   "
echo "--"
echo Initializing token with NAME $NAME and SYMBOL $SYMBOL


soroban contract invoke \
  --network $NETWORK --source token-admin \
  --id $TOKEN_A_ID \
  -- \
  initialize \
  --admin "$TOKEN_ADMIN_ADDRESS" \
  --decimal $DECIMAL \
  --name "$NAME" \
  --symbol "$SYMBOL"

TOKEN_ADDRESS="$(node /workspace/scripts/address_workaround.js $TOKEN_A_ID)"
echo "Saving to temp_token.json"
# Save the token contract address and token id to a file on .soroban/temp_token.json
echo "{\"address\": \"$TOKEN_ADDRESS\", \"name\": \"$NAME\", \"symbol\": \"$SYMBOL\", \"logoURI\": \"$LOGO\", \"decimals\": $DECIMAL}" > /workspace/.soroban/temp_token.json

