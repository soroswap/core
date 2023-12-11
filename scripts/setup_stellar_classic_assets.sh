#!/bin/bash

#soroban lab token id --asset native --network testnet
#soroban lab token wrap --network standalone --source-account token-admin --asset native

NETWORK="$1"
DECIMAL=7 # Is 7 the default on stellar network?

ASSETS_DIRECTORY="/workspace/scripts/stellar_classic_assets.json"
CLASSIC_ASSETS_JSON=$(cat $ASSETS_DIRECTORY)
N_TOKENS=$(jq '.tokens | length' "$ASSETS_DIRECTORY")

# Directory of the tokens.json file
TOKENS_DIRECTORY="/workspace/.soroban/tokens.json"
# Read the entire tokens.json file into a variable
TOKENS_JSON=$(cat "$TOKENS_DIRECTORY")

if [ "$NETWORK" == "standalone" ]; then
    # Attempt to run the command and capture its exit status
    soroban lab token wrap --asset native --network $NETWORK --source-account token-admin
    EXIT_STATUS=$?

    # Check if the command failed (non-zero exit status)
    if [ $EXIT_STATUS -ne 0 ]; then
        echo "Notice: 'soroban lab token wrap' command already executed or failed with status $EXIT_STATUS. Continuing..."
    fi
fi

for i in $(seq 1 $N_TOKENS); do
    # Extract symbol, name, logoURI, and asset values for the current index
    SYMBOL=$(echo "$CLASSIC_ASSETS_JSON" | jq -r ".tokens[$i-1].symbol")
    NAME=$(echo "$CLASSIC_ASSETS_JSON" | jq -r ".tokens[$i-1].name")
    LOGO=$(echo "$CLASSIC_ASSETS_JSON" | jq -r ".tokens[$i-1].logoURI")
    ASSET=$(echo "$CLASSIC_ASSETS_JSON" | jq -r ".tokens[$i-1].asset")

    TOKEN_ADDRESS=$(soroban lab token id --network "$NETWORK" --asset "$ASSET")

    # Create TEMP_WRAPPED as a valid JSON object
    TEMP_WRAPPED=$(jq -n \
                    --arg address "$TOKEN_ADDRESS" \
                    --arg name "$NAME" \
                    --arg symbol "$SYMBOL" \
                    --arg logoURI "$LOGO" \
                    --argjson decimals "$DECIMAL" \
                    '{address: $address, name: $name, symbol: $symbol, logoURI: $logoURI, decimals: $decimals}')
    echo "Token ID $SYMBOL - $TOKEN_ADDRESS"
    # Check if a token with the same address already exists in the network array
    # If it does, replace it; otherwise, prepend it to the top of the list
    UPDATED_TOKENS_JSON=$(echo "$TOKENS_JSON" | jq --argjson newToken "$TEMP_WRAPPED" --arg network "$NETWORK" '
      map(
        if .network == $network then
          if any(.tokens[]; .address == $newToken.address) then
            .tokens |= map(if .address == $newToken.address then $newToken else . end)
          else
            .tokens = [$newToken] + .tokens
          end
        else
          .
        end
      )'
    )

    echo "$UPDATED_TOKENS_JSON" > "$TOKENS_DIRECTORY"
    echo "Added Stellar assets"
done
