NETWORK="$1"
LOCAL_OR_PUBLIC="$2"
case "$1" in
standalone)
  echo "Using standalone network"
  ;;
futurenet)
  echo "Using Futurenet network"
  ;;
testnet)
  echo "Using Futurenet network"
  ;;
testnet-public)
  echo "Using Futurenet network with public RPC https://soroban-testnet.stellar.org/"
  ;;
*)
  echo "Usage: $0 standalone|futurenet|testnet|testnet-public"
  exit 1
  ;;
esac

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


bash /workspace/scripts/manual_testing/generate_user.sh 
bash /workspace/scripts/manual_testing/mint.sh $NETWORK $LOCAL_OR_PUBLIC
bash /workspace/scripts/manual_testing/add_liquidity.sh $NETWORK $LOCAL_OR_PUBLIC
bash /workspace/scripts/manual_testing/swap.sh $NETWORK $LOCAL_OR_PUBLIC
bash /workspace/scripts/manual_testing/remove_liquidity.sh $NETWORK $LOCAL_OR_PUBLIC