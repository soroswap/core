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



# # Run the script create_token.sh to create the token
# bash /workspace/scripts/create_token.sh $NETWORK $TOKEN_ADMIN_ADDRESS
# # Get the token contract address and token id from the file on .soroban/temp_token.json


# Initialize an empty JSON array in all_tokens.json
touch /workspace/.soroban/all_tokens.json
echo "[]" > /workspace/.soroban/all_tokens.json

# Loop from 1 to N_TOKENS
for i in $(seq 1 $N_TOKENS); do
    # Run the script that generates temp_token.json (replace ./your_script.sh with the actual script)
    bash /workspace/scripts/create_token.sh $NETWORK $TOKEN_ADMIN_ADDRESS

    # Read the contents of temp_token.json
    temp_token=$(cat /workspace/.soroban/temp_token.json)

    # Add the contents of temp_token.json to the all_tokens.json array
    temp=$(mktemp)
    jq --argjson new_token "$temp_token" '. += [$new_token]' /workspace/.soroban/all_tokens.json > "$temp" && mv "$temp" /workspace/.soroban/all_tokens.json
done

# Display the final JSON file
cat /workspace/.soroban/all_tokens.json

