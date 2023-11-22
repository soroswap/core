# Run script from project root

set -e

NETWORK="$1"

# Creating the .soroban folder if does not exist yet
mkdir -p .soroban


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
testnet)
  echo "Using Testnet network"
  SOROBAN_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
  FRIENDBOT_URL="https://friendbot.stellar.org/"
  SOROBAN_RPC_URL="https://soroban-testnet.stellar.org/"
  ;;
testnet-public)
  echo "Using Futurenet network with public RPC https://soroban-testnet.stellar.org/"
  SOROBAN_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
  FRIENDBOT_URL="https://friendbot.stellar.org/"
  SOROBAN_RPC_URL="https://soroban-testnet.stellar.org/"
  ;;
*)
  echo "Usage: $0 standalone|futurenet|testnet|testnet-public"
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

echo "$TOKEN_ADMIN_SECRET" > .soroban/token_admin_secret
echo "$TOKEN_ADMIN_ADDRESS" > .soroban/token_admin_address

NEW_KEYS_OBJECT="{ \"network\": \"$NETWORK\", \"admin_public\": \"$TOKEN_ADMIN_ADDRESS\", \"admin_secret\": \"$TOKEN_ADMIN_SECRET\" }"
echo "New keys object: $NEW_KEYS_OBJECT"

KEYS_FILE="/workspace/.soroban/token_admin_keys.json"
touch $KEYS_FILE
CURRENT_KEYS_JSON=$(cat $KEYS_FILE)
echo "CURRENT_KEYS_JSON: $CURRENT_KEYS_JSON"


# check if the network already exists in that json
exists=$(echo "$CURRENT_KEYS_JSON" | jq '.[] | select(.network == "'$NETWORK'")')
echo "This network already exist in the keys.json? : $exists"

NEW_KEYS_JSON="{}"
if [[ -n "$CURRENT_KEYS_JSON" ]]; then
    if [[ -n "$exists" ]]; then
        # if the network exists, update the keys for that network
        echo "Network exists, replacing"
        NEW_KEYS_JSON=$(echo "$CURRENT_KEYS_JSON" | jq '
            map(if .network == "'"$NETWORK"'" then '"$NEW_KEYS_OBJECT"' else . end)'
        )
    else
        # if the network doesn't exist, append the new object to the list
        echo "Network does not exist, appending"
        NEW_KEYS_JSON=$(echo "$CURRENT_KEYS_JSON" | jq '. + ['"$NEW_KEYS_OBJECT"']')
    fi
else
    # if the file is empty, initialize with a new object
    echo "File is empty, initializing"
    NEW_KEYS_JSON=$(echo '['"$NEW_KEYS_OBJECT"']')
fi

# echo "NEW_KEYS_JSON: $NEW_KEYS_JSON"
echo "$NEW_KEYS_JSON" > "$KEYS_FILE"

echo "Token admin information available in /workspace/.soroban/token_admin_keys.json"
cat /workspace/.soroban/token_admin_keys.json

echo "end creating the keys" 

# This will fail if the account already exists, but it'll still be fine.
echo Fund token-admin account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$TOKEN_ADMIN_ADDRESS"