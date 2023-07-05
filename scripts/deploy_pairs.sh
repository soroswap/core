#!/bin/bash

# deploy_pairs.sh
# This script deploys pairs of tokens on the Soroban network using the SoroswapFactory contract.
# For each token pair (token0_token1, token2_token3, ...), the script deploys and initializes a pair contract.
# All the pair addresses are saved to /workspace/.soroban/pairs.json
#
# Usage:
# bash /workspace/scripts/deploy_pairs.sh <network> <n_tokens> <run_setup>
#
# <network>: Name of the Soroban network to connect to.
# <n_tokens>: The number of tokens to pair (should be even).
# <run_setup>: Set to "false" to skip running the setup script, any other value will run setup.
#
# Example:
# bash /workspace/scripts/deploy_pairs.sh standalone 5 false
#
# Dependencies:
# - soroban: Make sure the 'soroban' CLI tool is available.
# - jq: Command-line JSON processor.

# Enable the 'exit immediately' shell option
set -e

# Accept command-line arguments
NETWORK="$1"
N_TOKENS="$2"
RUN_SETUP="$3"

# Validate the input arguments
if [ -z "$NETWORK" ]; then
    echo "Error: Network name must be provided."
    echo "Usage: bash /workspace/scripts/deploy_pairs.sh <network> <n_tokens> <run_setup>"
    exit 1
fi

# Run the setup script if RUN_SETUP is not set to "false"
if [ "$RUN_SETUP" != "false" ]; then
    bash /workspace/scripts/setup.sh $NETWORK
fi

# Get token array from tokens.json
TOKENS=$(cat /workspace/.soroban/tokens.json | jq -r --arg network "$NETWORK" '.[] | select(.network == $network) | .tokens')
echo "TOKENS: $TOKENS"

# Initialize an empty array to store pair addresses
PAIR_ADDRESSES=[]

# Loop through the tokens and deploy pairs
for i in $(seq 0 2 $(($N_TOKENS-1))); do
    echo "Deploying pair for tokens $i and $(($i+1))"
    TOKEN_A_ID=$(echo $TOKENS | jq -r ".[$i].token_id")
    TOKEN_B_ID=$(echo $TOKENS | jq -r ".[$(($i+1))].token_id")
    
    echo "TOKEN_A_ID: $TOKEN_A_ID"
    echo "TOKEN_B_ID: $TOKEN_B_ID"
    # Use the create_pair.sh script to deploy a new pair contract
    bash /workspace/scripts/create_pair.sh $NETWORK $TOKEN_A_ID $TOKEN_B_ID
    
done

