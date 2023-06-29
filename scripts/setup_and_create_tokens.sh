# Run script from project root

set -e

NETWORK="$1"
N_TOKENS="$2"

bash /workspace/scripts/setup.sh $1
TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"


# # Run the script create_token.sh to create the token
# bash /workspace/scripts/create_token.sh $NETWORK $TOKEN_ADMIN_ADDRESS
# # Get the token contract address and token id from the file on .soroban/temp_token.json


# Initialize an empty JSON array in tokens.json
touch /workspace/.soroban/tokens.json
echo "[]" > /workspace/.soroban/tokens.json

# Read token_name_ideas.json file into a variable
TOKEN_NAME_JSON=$(cat /workspace/scripts/token_name_ideas.json)


# Loop from 1 to N_TOKENS
for i in $(seq 1 $N_TOKENS); do

    # Extract symbol and name values for the current index
    SYMBOL=$(echo $TOKEN_NAME_JSON | jq -r ".tokens[$i-1].symbol")
    NAME=$(echo $TOKEN_NAME_JSON | jq -r ".tokens[$i-1].name")

    echo "Deploying token $i out of $N_TOKENS. Name: $NAME, Symbol: $SYMBOL"

    # Run the script that generates temp_token.json (replace ./your_script.sh with the actual script)
    bash /workspace/scripts/create_token.sh $NETWORK $TOKEN_ADMIN_ADDRESS $NAME $SYMBOL

    # Read the contents of temp_token.json
    temp_token=$(cat /workspace/.soroban/temp_token.json)

    # Add the contents of temp_token.json to the tokens.json array
    temp=$(mktemp)
    jq --argjson new_token "$temp_token" '. += [$new_token]' /workspace/.soroban/tokens.json > "$temp" && mv "$temp" /workspace/.soroban/tokens.json
done

# Display the final JSON file
echo Result available in /workspace/.soroban/tokens.json and localhost:8010
cat /workspace/.soroban/tokens.json

