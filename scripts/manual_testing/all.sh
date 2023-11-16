NETWORK="$1"
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

bash /workspace/scripts/manual_testing/generate_user.sh
bash /workspace/scripts/manual_testing/mint.sh $NETWORK
bash /workspace/scripts/manual_testing/add_liquidity.sh $NETWORK
#bash /workspace/scripts/manual_testing/swap.sh $NETWORK
bash /workspace/scripts/manual_testing/remove_liquidity.sh $NETWORK