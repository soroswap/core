#!/bin/bash

# This script takes two arguments:
# $1: The network to deploy the token to
# $2: The address of the token admin account
# $3: Token name
# $4: Token symbol
# Then it deploys the token contract to the network and initializes it
# Then it saves the token contract address and token id to a file on .soroban/temp_token.json

NETWORK="$1"
# TOKEN_ADMIN_ADDRESS="$2"
# TOKEN_A_ID="$3"
TOKEN_A_ID="CB7ZD5RFJKWPNEAXT7EVIQUNVACJTN5PRM76YI4RUZ2M36UIJ6IWUNU4"
TOKEN_B_ID="CA7X4NFKFRED2SY27L63IPOCZDWCUHWMHQGMQA2BWQTN5KKL2NUFQGR6"
# TOKEN_A_ID="GB6D5QHTBZEY3GWPADRFTRZB2Z6XCNH34LJW5L36W6Q7523QNBT2YVCV"

# Get the token admin address
TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"

TOKEN_WASM="/workspace/contracts/token/soroban_token_contract.wasm"

# BALANCE_BEFORE="$(soroban contract invoke \
#     --network $NETWORK \
#     --source token-admin \
#     --wasm $TOKEN_WASM \
#     --id $TOKEN_A_ID \
#     -- \
#     balance \
#     --id "$TOKEN_ADMIN_ADDRESS"
# )"
# echo "Balance before: $BALANCE_BEFORE"

MINT_RESULT="$(soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --wasm $TOKEN_WASM \
    --id $TOKEN_A_ID \
    -- \
    mint \
    --to "$TOKEN_ADMIN_ADDRESS" \
    --amount "25000000000000")"
echo "MINT_RESULT: $MINT_RESULT"

MINT_RESULT="$(soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --wasm $TOKEN_WASM \
    --id $TOKEN_B_ID \
    -- \
    mint \
    --to "$TOKEN_ADMIN_ADDRESS" \
    --amount "25000000000000")"
echo "MINT_RESULT: $MINT_RESULT"
