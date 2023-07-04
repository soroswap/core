#!/bin/bash

# This script sets up a workspace and deploys tokens to the Soroban network.
# Make sure to run this script from the project root directory.
#
# Usage:
# bash /workspace/scripts/setup_and_create_tokens.sh <network> <n_tokens>
#
# <network> : Name of the Soroban network to connect to.
# <n_tokens> : The number of tokens to deploy.
#
# Example:
# bash /workspace/scripts/setup_and_create_tokens.sh standalone 5
#
# Dependencies:
# - jq: This script requires 'jq' for parsing JSON files. Make sure it is installed on your system.
# - soroban: Make sure the 'soroban' CLI tool is available.

# Enable the 'exit immediately' shell option
set -e

# Accept command-line arguments
NETWORK="$1"
N_TOKENS="$2"

# Validate the input arguments
if [ -z "$NETWORK" ] || [ -z "$N_TOKENS" ] || ! [[ "$N_TOKENS" =~ ^[0-9]+$ ]]; then
    echo "Error: Both network name and number of tokens must be provided, and number of tokens must be an integer."
    echo "Usage: bash /workspace/scripts/setup_and_create_tokens.sh <network> <n_tokens>"
    exit 1
fi

# Run the setup script
bash /workspace/scripts/setup.sh $NETWORK

# Get the token admin address
TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"

# Initialize an empty JSON array in tokens.json
touch /workspace/.soroban/tokens.json
echo "[]" > /workspace/.soroban/tokens.json

# Read the token_name_ideas.json file into a variable
TOKEN_NAME_JSON=$(cat /workspace/scripts/token_name_ideas.json)

# Loop from 1 to N_TOKENS
for i in $(seq 1 $N_TOKENS); do
    # Extract symbol and name values for the current index
    SYMBOL=$(echo $TOKEN_NAME_JSON | jq -r ".tokens[$i-1].symbol")
    NAME=$(echo $TOKEN_NAME_JSON | jq -r ".tokens[$i-1].name")

    echo "Deploying token $i out of $N_TOKENS. Name: $NAME, Symbol: $SYMBOL"

    # Run the script that generates temp_token.json (make sure create_token.sh is an existing script)
    bash /workspace/scripts/create_token.sh $NETWORK $TOKEN_ADMIN_ADDRESS $NAME $SYMBOL

    # Read the contents of temp_token.json
    temp_token=$(cat /workspace/.soroban/temp_token.json)

    # Add the contents of temp_token.json to the tokens.json array
    temp=$(mktemp)
    jq --argjson new_token "$temp_token" '. += [$new_token]' /workspace/.soroban/tokens.json > "$temp" && mv "$temp" /workspace/.soroban/tokens.json
done

# Add networks to the JSON file
jq '. | [{network: "standalone", tokens: .}, {network: "futurenet", tokens: []}]' /workspace/.soroban/tokens.json > "$temp" && mv "$temp" /workspace/.soroban/tokens.json

# Display the final JSON file
echo "Result available in /workspace/.soroban/tokens.json and localhost:8010"
cat /workspace/.soroban/tokens.json
