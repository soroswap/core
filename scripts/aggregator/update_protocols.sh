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

PHOENIX_FILE="/workspace/.soroban/phoenix_protocol.json"
CURRENT_PHOENIX_JSON=$(cat $PHOENIX_FILE)
PHOENIX_MULTIHOP_ADDRESS=$(echo "$CURRENT_PHOENIX_JSON" | jq --raw-output '.[] | select(.network == "'$NETWORK'").multihop_address')
echo Phoenix Multihop address: $PHOENIX_MULTIHOP_ADDRESS

echo "----------------------------------------"
echo "Update Aggregator protocols to include Phoenix"

ARGS="--network $NETWORK --source token-admin"

AGGREGATOR_UPDATE_OBJECT=$(jq -n \
    --arg phoenixAddress "$PHOENIX_MULTIHOP_ADDRESS" \
    '[ { "protocol_id": 1, "address": $phoenixAddress } ]')

echo "Updating the SoroswapAggregator protocols"
soroban contract invoke \
  $ARGS \
  --id $AGGREGATOR_ADDRESS \
  -- \
  update_protocols \
  --protocol_addresses "$AGGREGATOR_UPDATE_OBJECT"