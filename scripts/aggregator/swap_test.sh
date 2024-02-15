#!/bin/bash
source /workspace/scripts/network_configs.sh

if !(soroban config identity ls | grep alice 2>&1 >/dev/null); then
  echo Create the alice identity
  soroban keys generate --no-fund --network $NETWORK alice
fi
# Fetch the admin's address
USER_ADDRESS=$(soroban config identity address alice)
curl  -X POST "$FRIENDBOT_URL?addr=$USER_ADDRESS"

AGGREGATOR_FILE="/workspace/.soroban/aggregator.json"
CURRENT_AGGREGATOR_JSON=$(cat $AGGREGATOR_FILE)
AGGREGATOR_ADDRESS=$(echo "$CURRENT_AGGREGATOR_JSON" | jq --raw-output '.[] | select(.network == "'$NETWORK'").aggregator_id')
echo Soroswap Aggregator address: $AGGREGATOR_ADDRESS

echo "----------------------------------------"
echo "Swap Test on Aggregator"

ARGS="--network $NETWORK --source alice"

# PHOENIX TOKENS
PHOENIX_FILE="/workspace/.soroban/phoenix_protocol.json"
CURRENT_PHOENIX_JSON=$(cat $PHOENIX_FILE)
PHOENIX_TOKEN_A=$(echo "$CURRENT_PHOENIX_JSON" | jq --raw-output '.[] | select(.network == "'$NETWORK'").token_a')
PHOENIX_TOKEN_B=$(echo "$CURRENT_PHOENIX_JSON" | jq --raw-output '.[] | select(.network == "'$NETWORK'").token_b')
echo tokenA: $PHOENIX_TOKEN_A
echo tokenB: $PHOENIX_TOKEN_B

# Create the JSON object
SWAP_OBJECT=$(jq -n \
    --arg phoenixTokenA "$PHOENIX_TOKEN_A" \
    --arg phoenixTokenB "$PHOENIX_TOKEN_B" \
    '[ 
        {
            "index": 1, 
            "path": [$phoenixTokenA, $phoenixTokenB], 
            "parts": "3"
        }
    ]')

echo "Minting tokens for the user"

soroban contract invoke \
  --network $NETWORK --source phoenix-admin \
  --id $PHOENIX_TOKEN_A \
  -- \
  mint \
  --to "$USER_ADDRESS" \
  --amount 250000000000000

soroban contract invoke \
  --network $NETWORK --source phoenix-admin \
  --id $PHOENIX_TOKEN_B \
  -- \
  mint \
  --to "$USER_ADDRESS" \
  --amount 250000000000000


echo "Getting token Balances before swap"

soroban contract invoke \
  $ARGS \
  --id $PHOENIX_TOKEN_A \
  -- \
  balance \
  --id "$USER_ADDRESS"

soroban contract invoke \
  $ARGS \
  --id $PHOENIX_TOKEN_B \
  -- \
  balance \
  --id "$USER_ADDRESS"

echo "Swapping tokens"

DEADLINE=$(date -d "+1 hour" +%s)

soroban contract invoke \
  $ARGS \
  --id $AGGREGATOR_ADDRESS \
  -- \
  swap \
  --from_token "$PHOENIX_TOKEN_A" \
  --dest_token "$PHOENIX_TOKEN_B" \
  --amount 250000000000000 \
  --amount_out_min 0 \
  --distribution "$SWAP_OBJECT" \
  --to "$USER_ADDRESS" \
  --deadline $DEADLINE

echo "Getting token Balances after swap"

soroban contract invoke \
  $ARGS \
  --id $PHOENIX_TOKEN_A \
  -- \
  balance \
  --id "$USER_ADDRESS"

soroban contract invoke \
  $ARGS \
  --id $PHOENIX_TOKEN_B \
  -- \
  balance \
  --id "$USER_ADDRESS"