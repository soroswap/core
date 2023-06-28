# Run script from project root

set -e

NETWORK="$1"
N_TOKENS="$2"
# Verifies that N_TOKENS should be an integer


# If soroban-cli is called inside the soroban-preview docker containter,
# it can call the stellar standalone container just using its name "stellar"
SOROBAN_RPC_HOST="http://stellar:8000"

SOROBAN_RPC_URL="$SOROBAN_RPC_HOST/soroban/rpc"

case "$1" in
standalone)
  echo "Using standalone network"
  SOROBAN_NETWORK_PASSPHRASE="Standalone Network ; February 2017"
  FRIENDBOT_URL="$SOROBAN_RPC_HOST/friendbot"
  ;;
futurenet)
  echo "Using Futurenet network"
  SOROBAN_NETWORK_PASSPHRASE="Test SDF Future Network ; October 2022"
  FRIENDBOT_URL="https://friendbot-futurenet.stellar.org/"
  ;;
*)
  echo "Usage: $0 standalone|futurenet"
  exit 1
  ;;
esac

# Always set a net configuration 
echo Add the $NETWORK network to cli client
soroban config network add "$NETWORK" \
  --rpc-url "$SOROBAN_RPC_URL" \
  --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE"

if !(soroban config identity ls | grep token-admin 2>&1 >/dev/null); then
  echo Create the token-admin identity
  soroban config identity generate token-admin
fi
TOKEN_ADMIN_SECRET="$(soroban config identity show token-admin)"
TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"

echo "We are using the following TOKEN_ADMIN_ADDRESS: $TOKEN_ADMIN_ADDRESS"


# This will fail if the account already exists, but it'll still be fine.
echo Fund token-admin account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$TOKEN_ADMIN_ADDRESS"

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
echo Result available in /workspace/.soroban/tokens.json
cat /workspace/.soroban/tokens.json

