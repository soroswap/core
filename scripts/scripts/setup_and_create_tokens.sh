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

# Read the token_name_ideas.json file into a variable
TOKEN_NAME_JSON=$(cat /workspace/scripts/token_name_ideas.json)

# File handling
TOKENS_FILE="/workspace/.soroban/tokens.json"
TEMP_FILE=$(mktemp)

# Initialize tokens.json if it does not exist
if [[ ! -f "$TOKENS_FILE" ]]; then
    echo file not found
    echo "[]" > "$TOKENS_FILE"
fi

# If no existing tokens array was found for the network, initialize an empty array
NEW_TOKENS="[]"
echo NEW_TOKENS: $NEW_TOKENS

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
    echo "temp_token: $temp_token"

    # Add the new token to the existing tokens array
    NEW_TOKENS=$(jq --argjson new_token "$temp_token" '. += [$new_token]' <<<"$NEW_TOKENS")
    echo "NEW_TOKENS: $NEW_TOKENS"
done

echo NEW_TOKENS: $NEW_TOKENS
# Create the new network object with the updated tokens array
echo "Debug: NEW_TOKENS = $NEW_TOKENS"
NEW_NETWORK_OBJECT="{ \"network\": \"$NETWORK\", \"tokens\": $NEW_TOKENS }"
# NEW_NETWORK_OBJECT=$(jq --arg network "standalone" --argjson tokens '[{"token_id": "id1", "token_address": "address1", "token_name": "name1", "token_symbol": "symbol1"}, {"token_id": "id2", "token_address": "address2", "token_name": "name2", "token_symbol": "symbol2"}]' '[{network: $network, tokens: $tokens}]')
# NEW_NETWORK_OBJECT=$(jq --arg network "$NETWORK" --argjson tokens "$EXISTING_TOKENS" '[{network: $network, tokens: $tokens}]')
echo "New network object: $NEW_NETWORK_OBJECT"

touch $TOKENS_FILE
TOKEN_LIST=$(cat $TOKENS_FILE)
echo "TOKEN_LIST: $TOKEN_LIST"

# check if the network already exists in the list
exists=$(echo "$TOKEN_LIST" | jq '.[] | select(.network == "'$NETWORK'")')
echo "exists: $exists"


NEW_KEYS_JSON="{}"
if [[ -n "$TOKEN_LIST" ]]; then
    if [[ -n "$exists" ]]; then
        # if the network exists, update the tokens for that network
        echo network exists, replace
        TOKEN_LIST=$(echo "$TOKEN_LIST" | jq '
            map(if .network == "'$NETWORK'" then '"$NEW_NETWORK_OBJECT"' else . end)'
        )
    else
        # if the network doesn't exist, append the new object to the list
        echo network does not exist, append
        TOKEN_LIST=$(echo "$TOKEN_LIST" | jq '. += ['"$NEW_NETWORK_OBJECT"']')
    fi
else
    # if the file is empty, initialize with a new object
    echo "File is empty, initializing"
    TOKEN_LIST=$(echo '['"$NEW_NETWORK_OBJECT"']')
fi

echo "TOKEN_LIST: $TOKEN_LIST"
echo "$TOKEN_LIST" > "$TOKENS_FILE"

echo "end creating tokens" 