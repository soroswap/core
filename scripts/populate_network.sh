#Define network related constants
source /workspace/scripts/network_configs.sh

# Define constants for color-coded output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

#Step 1: Deploy 8 tokens and pairs
echo -e "${GREEN}Deploying tokens...${NC}"
bash /workspace/scripts/deploy_tokens_n_pairs.sh $NETWORK 8

#Step 2: Deploy and Initialize Aggregator
echo -e "${GREEN}Deploying and initializing Aggregator...${NC}"
bash /workspace/scripts/deploy_initialize_aggregator.sh $NETWORK false

#Step 3: Add Liquidity to multiple pairs
echo -e "${GREEN}Adding liquidity to multiple pairs...${NC}"
bash /workspace/scripts/multi_add_liquidity.sh $NETWORK $MODE