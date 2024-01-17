#!/bin/bash

#soroban lab token id --asset native --network testnet
#soroban lab token wrap --network standalone --source-account token-admin --asset native

NETWORK="$1"
DECIMAL=7 # Is 7 the default on stellar network?

ASSETS_DIRECTORY="/workspace/scripts/known_stellar_classic_assets.json"
GENERATED_STELLAR_ASSETS="/workspace/.soroban/generated_stellar_assets.json"

CLASSIC_ASSETS_JSON=$(jq '.tokens' "$ASSETS_DIRECTORY")
GENERATED_ASSETS_JSON=$(jq '.tokens' "$GENERATED_STELLAR_ASSETS")

MERGED_TOKENS=$(jq -s '.[0] + .[1]' <(echo "$CLASSIC_ASSETS_JSON") <(echo "$GENERATED_ASSETS_JSON"))
FINAL_TOKENS_JSON=$(jq -n --argjson tokens "$MERGED_TOKENS" '{"tokens": $tokens}')
N_TOKENS=$(echo "$FINAL_TOKENS_JSON" | jq '.tokens | length')

# Directory of the tokens.json file
TOKENS_DIRECTORY="/workspace/.soroban/tokens.json"

for i in $(seq 1 $N_TOKENS); do
    # Reload the tokens.json file to get the latest state
    TOKENS_JSON=$(cat "$TOKENS_DIRECTORY")
    
    # Attempt to run the command and capture its exit status
    echo "ASSET: $ASSET"
    soroban lab token wrap --asset "$ASSET" --network "$NETWORK" --source-account token-admin
    EXIT_STATUS=$?

    # Check if the command failed (non-zero exit status)
    if [ $EXIT_STATUS -ne 0 ]; then
        echo "Notice: 'soroban lab token wrap' command already executed or failed with status $EXIT_STATUS. Continuing..."
    fi

    # Extract symbol, name, logoURI, and asset values for the current index
    SYMBOL=$(echo "$FINAL_TOKENS_JSON" | jq -r ".tokens[$i-1].symbol")
    NAME=$(echo "$FINAL_TOKENS_JSON" | jq -r ".tokens[$i-1].name")
    LOGO=$(echo "$FINAL_TOKENS_JSON" | jq -r ".tokens[$i-1].logoURI")
    ASSET=$(echo "$FINAL_TOKENS_JSON" | jq -r ".tokens[$i-1].asset")

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
