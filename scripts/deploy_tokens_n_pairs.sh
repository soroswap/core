#!/bin/bash

# deploy_tokens_n_pairs.sh
#
# This script automates the process of deploying tokens and initializing token pairs on the Soroban network.
# The steps involved include setting up the environment, deploying tokens, deploying the SoroswapFactory contract,
# and initializing token pairs using the deployed tokens.
#
# Usage:
#   bash /workspace/scripts/deploy_tokens_n_pairs.sh <network> <n_tokens>
#
# Arguments:
#   <network> : The name of the Soroban network to connect to (e.g. "standalone").
#   <n_tokens> : The number of tokens to deploy (should be an even number).
#
# Example:
#   bash /workspace/scripts/deploy_tokens_n_pairs.sh standalone 6
#
# Dependencies:
#   - Ensure the 'soroban' CLI tool is available.
#   - Ensure 'jq' (Command-line JSON processor) is installed.

# Enable the 'exit immediately' shell option
set -e

# Define constants for color-coded output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

# Validate the input arguments
if [ "$#" -ne 2 ]; then
    echo -e "${RED}Error: Invalid number of arguments.${NC}"
    echo "Usage: bash /workspace/scripts/deploy_tokens_n_pairs.sh <network> <n_tokens>"
    exit 1
fi

# Assign command-line arguments to variables
NETWORK="$1"
N_TOKENS="$2"

# Validate that the number of tokens is an even number
if [ $(($N_TOKENS % 2)) -ne 0 ]; then
    echo -e "${RED}Error: The number of tokens should be an even number.${NC}"
    exit 1
fi

# Display the starting message
echo -e "${GREEN}Starting the deployment process on network: $NETWORK with $N_TOKENS tokens.${NC}"

# Step 1: Setup and create tokens
echo -e "${GREEN}Running setup and creating tokens...${NC}"
bash /workspace/scripts/setup_and_create_tokens.sh $NETWORK $N_TOKENS

# Step 2: Deploy and initialize the factory
echo -e "${GREEN}Deploying and initializing the factory...${NC}"
bash /workspace/scripts/deploy_initialize_factory.sh $NETWORK $N_TOKENS false

# Step 3: Deploy and initialize pairs
echo -e "${GREEN}Deploying and initializing pairs...${NC}"
bash /workspace/scripts/deploy_pairs.sh $NETWORK $N_TOKENS false

# Step 4: Deploy router
echo -e "${GREEN}Deploying router...${NC}"
bash /workspace/scripts/deploy_initialize_router.sh $NETWORK

# Display the completion message
echo -e "${GREEN}Deployment process completed successfully.${NC}"
