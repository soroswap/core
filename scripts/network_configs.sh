NETWORK=$1
MODE=$2
echo $NETWORK $MODE

echo Using $NETWORK network
echo Using $MODE files

CONFIGS_FILE="/workspace/preview_version.json"

FRIENDBOT_URL=$(jq -r --arg NETWORK "$NETWORK" '.networkConfig[] | select(.network == $NETWORK) | .friendbot_url' "$CONFIGS_FILE")
SOROBAN_RPC_URL=$(jq -r --arg NETWORK "$NETWORK" '.networkConfig[] | select(.network == $NETWORK) | .soroban_rpc_url' "$CONFIGS_FILE")

TOKENS_FILE=$(jq -r --arg MODE "$MODE" '.networkConfig[] | select(.mode == $MODE) | .tokens_file' "$CONFIGS_FILE")
ROUTER_FILE=$(jq -r --arg MODE "$MODE" '.networkConfig[] | select(.mode == $MODE) | .router_file' "$CONFIGS_FILE")
