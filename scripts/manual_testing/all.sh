#NETWORK="$1"
#LOCAL_OR_PUBLIC="$2"

source /workspace/scripts/manual_testing/utils.sh

#Define network related constants
source /workspace/scripts/network_configs.sh
echo ===
echo "   "


bash /workspace/scripts/manual_testing/generate_user.sh $NETWORK
bash /workspace/scripts/manual_testing/mint.sh $NETWORK $MODE
bash /workspace/scripts/manual_testing/add_liquidity.sh $NETWORK $MODE
bash /workspace/scripts/manual_testing/swap.sh $NETWORK $MODE
bash /workspace/scripts/manual_testing/remove_liquidity.sh $NETWORK $MODE