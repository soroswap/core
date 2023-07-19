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
PAIRS_ARRAY=[]

# Loop through the tokens and deploy pairs
for i in $(seq 0 2 $(($N_TOKENS-1))); do
    echo "Deploying pair for tokens $i and $(($i+1))"
    TOKEN_A_ID=$(echo $TOKENS | jq -r ".[$i].token_id")
    TOKEN_B_ID=$(echo $TOKENS | jq -r ".[$(($i+1))].token_id")
    
    echo "TOKEN_A_ID: $TOKEN_A_ID"
    echo "TOKEN_B_ID: $TOKEN_B_ID"

    # Use the create_pair.sh script to deploy a new pair contract
    bash /workspace/scripts/create_pair.sh $NETWORK $TOKEN_A_ID $TOKEN_B_ID

    TOKEN_A_ADDRESS="$(node /workspace/scripts/address_workaround.js $TOKEN_A_ID)"
    TOKEN_B_ADDRESS="$(node /workspace/scripts/address_workaround.js $TOKEN_B_ID)"

    PAIR_ID=$(cat /workspace/.soroban/pair_id)
    echo "PAIR_ID: $PAIR_ID"
    # Construct the pair entry JSON object
    PAIR_ENTRY=$(printf '{"token_a_id": "%s", "token_b_id": "%s", "pair_address": "%s", "token_a_address":"%s", "token_b_address":"%s"}' "$TOKEN_A_ID" "$TOKEN_B_ID" "$PAIR_ID" "$TOKEN_A_ADDRESS" "$TOKEN_B_ADDRESS")
    echo "PAIR_ENTRY: $PAIR_ENTRY"
    # Add the pair entry to PAIRS_ARRAY for the specific network
    PAIRS_ARRAY=$(echo "$PAIRS_ARRAY" | jq --argjson obj "$PAIR_ENTRY" '. += [$obj]')
    echo "PAIRS_ARRAY: $PAIRS_ARRAY"

done

NEW_PAIRS_OBJECT=$(jq -n --arg NETWORK "$NETWORK" --argjson PAIRS_ARRAY "$PAIRS_ARRAY" '{"network": $NETWORK, "pairs": $PAIRS_ARRAY}')
echo NEW_PAIRS_OBJECT: $NEW_PAIRS_OBJECT

PAIRS_FILE="/workspace/.soroban/pairs.json"
# Initialize pairs.json if it does not exist
if [[ ! -f "$PAIRS_FILE" ]]; then
    echo file not found
    echo "[]" > "$PAIRS_FILE"
fi


CURRENT_PAIRS_JSON=$(cat $PAIRS_FILE)
echo "CURRENT_PAIRS_JSON: $CURRENT_PAIRS_JSON"


# check if the network already exists in that json
exists=$(echo "$CURRENT_PAIRS_JSON" | jq '.[] | select(.network == "'$NETWORK'")')
echo "This network already exist in the factory.json? : $exists"

NEW_PAIRS_JSON="{}"
if [[ -n "$exists" ]]; then
    # if the network exists, update the factory for that network
    echo network exists, replace
    NEW_PAIRS_JSON=$(echo "$CURRENT_PAIRS_JSON" | jq '
        map(if .network == "'$NETWORK'" then '"$NEW_PAIRS_OBJECT"' else . end)'
    )
else
    # if the network doesn't exist, append the new object to the list
    echo network does not exist, append
    NEW_PAIRS_JSON=$(echo "$CURRENT_PAIRS_JSON" | jq '. += ['"$NEW_PAIRS_OBJECT"']')
fi

# echo "NEW_PAIRS_JSON: $NEW_PAIRS_JSON"
echo "$NEW_PAIRS_JSON" > "$PAIRS_FILE"

# Output the file path and contents
echo "Factory information available in /workspace/.soroban/pairs.json"
cat /workspace/.soroban/pairs.json





# # Save the PAIRS_ARRAY array to a temporal .json file to add the network
# temp=$(mktemp)
# temp2=$(mktemp)
# echo "$PAIRS_ARRAY" > "$temp"
# # Add networks to the JSON file
# jq '. | [{network: "standalone", pairs: .}, {network: "futurenet", pairs: []}]' "$temp" > "$temp_2" && mv "$temp" /workspace/.soroban/pairs.json


# '[
#   {
#     "token_a_id": "a95d6b0d22af71b856435e3a81247475a14da1f1ce114c10f631f8c7bd036fb8",
#     "token_b_id": "cf90347844cd8a2139d62f1f4062142e9a52d0e178d957657773ae41c6db450c",
#     "pair_address": "CBLZNBZBOTN442FM3UTNYJY2KI6ACSCLUAWC24MXI3UILS4ALKU27J4W",
#     "token_a_address": "CCUV22YNEKXXDOCWINPDVAJEOR22CTNB6HHBCTAQ6YY7RR55ANX3QSXS",
#     "token_b_address": "CDHZANDYITGYUIJZ2YXR6QDCCQXJUUWQ4F4NSV3FO5Z24QOG3NCQZYFI"
#   },
#   {
#     "token_a_id": "3997190bc4c809adb259e47a53f719bb590cb7c861a9a0fe6003b413ad3d1cab",
#     "token_b_id": "084befbce079b0aa3bbc115acce408ba3c85a6d28598631f621d0c75d129f3e0",
#     "pair_address": "CBTXO4GVPMFJW7FNYMGHTLUZJ5GHLSZ7UJS7SY3GGBWFCAH6TUW4ZPIO",
#     "token_a_address": "CA4ZOGILYTEATLNSLHSHUU7XDG5VSDFXZBQ2TIH6MAB3IE5NHUOKXSWK",
#     "token_b_address": "CAEEX3544B43BKR3XQIVVTHEBC5DZBNG2KCZQYY7MIOQY5ORFHZ6BFYG"
#   }
# ]'
