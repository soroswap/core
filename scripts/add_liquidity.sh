#!/bin/bash

TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"
echo "TOKEN_ADMIN_ADDRESS: $TOKEN_ADMIN_ADDRESS"


ROUTER_CONTRACT="CCSP6PGXMA26QRCOCVPILATP2MTY75HGQDVEAB3CYCUAMKFGBIGCD5CQ"

ROUTER_WASM="/workspace/contracts/router/target/wasm32-unknown-unknown/release/soroswap_router.optimized.wasm"

NETWORK="standalone"

now=$(date +%s)


TOKEN_A_ADDRESS="CB7ZD5RFJKWPNEAXT7EVIQUNVACJTN5PRM76YI4RUZ2M36UIJ6IWUNU4"
TOKEN_B_ADDRESS="CA7X4NFKFRED2SY27L63IPOCZDWCUHWMHQGMQA2BWQTN5KKL2NUFQGR6"
AMOUNT_A_DESIRED=500000
AMOUNT_B_DESIRED=500000
AMOUNT_A_MIN=0
AMOUNT_B_MIN=0
TO=$TOKEN_ADMIN_ADDRESS
DEADLINE=$(date -d "+1 hour" +%s)
echo "deadline $DEADLINE"

soroban contract invoke \
  --network $NETWORK \
  --source token-admin \
  # --wasm $ROUTER_WASM \
  --id $ROUTER_CONTRACT \
  -- \
  add_liquidity \
  --token_a "$TOKEN_A_ADDRESS" \
  --token_b "$TOKEN_B_ADDRESS" \
  --amount_a_desired "$AMOUNT_A_DESIRED" \
  --amount_b_desired "$AMOUNT_B_DESIRED" \
  --amount_a_min "$AMOUNT_A_MIN" \
  --amount_b_min "$AMOUNT_B_MIN" \
  --to "$TO" \
  --deadline "$DEADLINE" 
