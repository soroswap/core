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

mkdir -p /workspace/.soroban

TOKEN_WASM="/workspace/token/soroban_token_contract.wasm"

# echo $NETWORK
# echo $TOKEN_WASM

TOKEN_A_ID="$(
  soroban contract deploy --network $NETWORK --source token-admin \
    --wasm $TOKEN_WASM
  )"
# echo TOKEN_A_ID: $TOKEN_A_ID

# echo Initializing TOKEN_A
# echo "Executing: 
#   fn initialize(  e: Env,
#                   admin: Address,
#                   decimal: u32,
#                   name: Bytes,
#                   symbol: Bytes) {
#   "
# echo Initializing token with NAME $NAME and SYMBOL $SYMBOL


soroban contract invoke \
  --network $NETWORK --source token-admin \
  --wasm $TOKEN_WASM \
  --id $TOKEN_A_ID \
  -- \
  initialize \
  --admin "$TOKEN_ADMIN_ADDRESS" \
  --decimal 7 \
  --name "$NAME" \
  --symbol "$SYMBOL"

# Save the token contract address and token id to a file on .soroban/temp_token.json
echo "{\"token_id\": \"$TOKEN_A_ID\", \"token_address\": \"$TOKEN_ADMIN_ADDRESS\", \"token_name\": \"$NAME\", \"token_symbol\": \"$SYMBOL\"}" > /workspace/.soroban/temp_token.json
