NETWORK="$1"
LOCAL_OR_PUBLIC="$2"

source /workspace/scripts/manual_testing/utils.sh

case "$1" in
standalone)
  display_colored_text GREEN " Using standalone network "
  ;;
futurenet)
  display_colored_text GREEN " Using Futurenet network "
  ;;
testnet)
  display_colored_text GREEN " Using Futurenet network "
  ;;
testnet-public)
  display_colored_text GREEN " Using Futurenet network with public RPC https://soroban-testnet.stellar.org/ "
  ;;
*)
  display_colored_text YELLOW " Usage: $0 standalone|futurenet|testnet|testnet-public "
  exit 1
  ;;
esac

case "$2" in
local)
  display_colored_text GREEN " Using deployed contracts from .soroban folder "
  ;;
public)
  display_colored_text GREEN " Using deployed contracts from /public folder "
  ;;
*)
  display_colored_text YELLOW " Usage: $0 local|public                        "
  display_colored_text YELLOW " local: use contracts from the .soroban folder (local deployements)       "
  display_colored_text YELLOW " public: use contracts from the /public folder (addresses in production?) "
  exit 1
  ;;
esac
echo ===
echo "   "


bash /workspace/scripts/manual_testing/generate_user.sh $NETWORK
bash /workspace/scripts/manual_testing/mint.sh $NETWORK $LOCAL_OR_PUBLIC
bash /workspace/scripts/manual_testing/add_liquidity.sh $NETWORK $LOCAL_OR_PUBLIC
bash /workspace/scripts/manual_testing/swap.sh $NETWORK $LOCAL_OR_PUBLIC
bash /workspace/scripts/manual_testing/remove_liquidity.sh $NETWORK $LOCAL_OR_PUBLIC