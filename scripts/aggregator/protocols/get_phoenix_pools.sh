#!/bin/bash
# Check if the argument is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <network>"
    exit 1
fi

NETWORK="$1"
echo "----------------------------------------"
PHOENIX_FILE="/workspace/.soroban/phoenix_protocol.json"
CURRENT_PHOENIX_JSON=$(cat $PHOENIX_FILE)
PHOENIX_FACTORY_ADDRESS=$(echo "$CURRENT_PHOENIX_JSON" | jq --raw-output '.[] | select(.network == "'$NETWORK'").factory_address')

echo Phoenix Aggregator address: $PHOENIX_FACTORY_ADDRESS

echo "----------------------------------------"

ARGS="--network $NETWORK --source phoenix-admin"

echo "Getting Phoenix pools"
soroban contract invoke \
  $ARGS \
  --id $PHOENIX_FACTORY_ADDRESS \
  -- \
  query_pools