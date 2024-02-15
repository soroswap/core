#!/bin/bash
# Check if the argument is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <network>"
    exit 1
fi

NETWORK="$1"
echo "----------------------------------------"
AGGREGATOR_FILE="/workspace/.soroban/aggregator.json"
CURRENT_AGGREGATOR_JSON=$(cat $AGGREGATOR_FILE)
AGGREGATOR_ADDRESS=$(echo "$CURRENT_AGGREGATOR_JSON" | jq --raw-output '.[] | select(.network == "'$NETWORK'").aggregator_id')
echo Soroswap Aggregator address: $AGGREGATOR_ADDRESS

echo "----------------------------------------"

ARGS="--network $NETWORK --source token-admin"

echo "Getting the SoroswapAggregator Admin"
soroban contract invoke \
  $ARGS \
  --id $AGGREGATOR_ADDRESS \
  -- \
  get_admin