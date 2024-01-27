NETWORK=$1
MODE=$2

RED='\033[1;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'
#Validate the input arguments
#if [ "$#" -ne 2 ]; then
#    echo -e "${RED}Error: Invalid number of arguments.${NC}"
#    echo -e "${YELLOW}Usage: bash /workspace/scripts/populate_network.sh <standalone|futurenet|testnet|testnet-public> <local|public>${NC}"
#    exit 1
#fi

#Verify that the network is valid
case "$1" in
standalone)
    echo "Using Standalone network"
  ;;
futurenet)
    echo "Using Futurenet network"
  ;;
testnet)
  echo "Using Testnet network"
  ;;
testnet-public)
  echo "Using Testnet network with public RPC https://soroban-testnet.stellar.org/"
  ;;
*)
  echo "Usage: $0 standalone|futurenet|testnet|testnet-public"
  exit 1
  ;;
esac

if [ ${#2} -gt 1 ]; then    
    case "$2" in
    local)
        echo "Using deployed contracts from .soroban folder"
    ;;
    public)
        echo "Using deployed contracts from /public folder"
    ;;
    *)
        echo "Usage: $0 local|public"
        echo "local: use contracts from the .soroban folder (local deployements)"
        echo "public: use contracts from the /public folder (addresses in production?)"
        exit 1
    ;;
    esac    
fi

CONFIGS_FILE="/workspace/configs.json"

FRIENDBOT_URL=$(jq -r --arg NETWORK "$NETWORK" '.networkConfig[] | select(.network == $NETWORK) | .friendbot_url' "$CONFIGS_FILE")
SOROBAN_RPC_URL=$(jq -r --arg NETWORK "$NETWORK" '.networkConfig[] | select(.network == $NETWORK) | .soroban_rpc_url' "$CONFIGS_FILE")
SOROBAN_NETWORK_PASSPHRASE=$(jq -r --arg NETWORK "$NETWORK" '.networkConfig[] | select(.network == $NETWORK) | .soroban_network_passphrase' "$CONFIGS_FILE")

TOKENS_FILE=$(jq -r --arg MODE "$MODE" '.networkConfig[] | select(.mode == $MODE) | .tokens_file' "$CONFIGS_FILE")
ROUTER_FILE=$(jq -r --arg MODE "$MODE" '.networkConfig[] | select(.mode == $MODE) | .router_file' "$CONFIGS_FILE")
