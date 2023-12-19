NETWORK="$1"
echo "=== MINT.SH ==="

case "$1" in
standalone)
  echo "Using standalone network"
  SOROBAN_NETWORK_PASSPHRASE="Standalone Network ; February 2017"
  SOROBAN_RPC_HOST="http://stellar:8000"
  FRIENDBOT_URL="$SOROBAN_RPC_HOST/friendbot"
  ;;
futurenet)
  echo "Using Futurenet network"
  SOROBAN_NETWORK_PASSPHRASE="Test SDF Future Network ; October 2022"
  FRIENDBOT_URL="https://friendbot-futurenet.stellar.org/"
  ;;
testnet)
  echo "Using Testnet network"
  FRIENDBOT_URL="https://friendbot.stellar.org/"
  ## TODO: Remove when solving the rpc problem:_
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


case "$2" in
local)
  echo "Using deployed contracts from .soroban folder"
  ADMIN_KEYS_FILE="/workspace/.soroban/token_admin_keys.json"
  TOKENS_FILE="/workspace/.soroban/tokens.json"
  ;;
public)
  echo "Using deployed contracts from /public folder"
  ADMIN_KEYS_FILE="/workspace/public/token_admin_keys.json"
  TOKENS_FILE="/workspace/public/tokens.json"
  ;;
*)
  echo "Usage: $0 local|public"
  echo "local: use contracts from the .soroban folder (local deployements)"
  echo "public: use contracts from the /public folder (addresses in production?)"
  exit 1
  ;;
esac


echo We are going to mint 2 test tokens
echo We are going to use the admin private key and the user public key

ADMIN_SECRET=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .admin_secret' $ADMIN_KEYS_FILE)
USER_PUBLIC=$(cat .soroban/user_public)

echo ADMIN_SECRET: $ADMIN_SECRET
echo USER_PUBLIC: $USER_PUBLIC

TOKEN_0_ADDRESS=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[2].address' "$TOKENS_FILE")
TOKEN_1_ADDRESS=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[3].address' "$TOKENS_FILE")

TOKEN_0_SYMBOL=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[2].symbol' "$TOKENS_FILE")
TOKEN_1_SYMBOL=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[3].symbol' "$TOKENS_FILE")

echo "..."
echo "..."
echo "..."
echo "..."
echo We will use the following test tokens in the $NETWORK network
echo "..."
echo TOKEN_0_SYMBOL: $TOKEN_0_SYMBOL
echo TOKEN_0_ADDRESS: $TOKEN_0_ADDRESS
echo "..."
echo TOKEN_1_SYMBOL: $TOKEN_1_SYMBOL
echo TOKEN_1_ADDRESS: $TOKEN_1_ADDRESS




echo "..."
echo "..."
echo "..."
echo "..."
echo We will mint you 25,000,000 units ..plus 7 decimals.. of each token 
echo "..."
TOKEN_WASM="/workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm"
echo "Minting TOKEN_0:"
soroban contract invoke \
  --network $NETWORK --source token-admin \
  --wasm $TOKEN_WASM \
  --id $TOKEN_0_ADDRESS \
  -- \
  mint \
  --to "$USER_PUBLIC" \
  --amount 250000000000000 

echo "..."
echo "Minting TOKEN_1:"
soroban contract invoke \
  --network $NETWORK --source token-admin \
  --wasm $TOKEN_WASM \
  --id $TOKEN_1_ADDRESS \
  -- \
  mint \
  --to "$USER_PUBLIC" \
  --amount 250000000000000 