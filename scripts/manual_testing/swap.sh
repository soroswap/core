NETWORK="$1"

SOROBAN_RPC_HOST="http://stellar:8000"

SOROBAN_RPC_URL="$SOROBAN_RPC_HOST/soroban/rpc"

source /workspace/scripts/manual_testing/utils.sh

display_colored_text PURPLE " === SWAP.SH === "

case "$1" in
standalone)
  echo "Using standalone network"
  FRIENDBOT_URL="$SOROBAN_RPC_HOST/friendbot"
  ;;
futurenet)
  echo "Using Futurenet network"
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
  TOKENS_FILE="/workspace/.soroban/tokens.json"
  ROUTER_FILE="/workspace/.soroban/router.json"
  PAIRS_FILE="/workspace/.soroban/pairs.json"

  ;;
public)
  echo "Using deployed contracts from /public folder"
  TOKENS_FILE="/workspace/public/tokens.json"
  ROUTER_FILE="/workspace/public/router.json"
  PAIRS_FILE="/workspace/public/pairs.json"

  ;;
*)
  echo "Usage: $0 local|public"
  echo "local: use contracts from the .soroban folder (local deployements)"
  echo "public: use contracts from the /public folder (addresses in production?)"
  exit 1
  ;;
esac


USER_PUBLIC=$(cat .soroban/user_public)
USER_SECRET=$(cat .soroban/user_secret)

echo USER_PUBLIC: $USER_PUBLIC
echo USER_SECRET: $USER_SECRET

echo Fund user account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$USER_PUBLIC"


TOKEN_0_ADDRESS=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[2].address' "$TOKENS_FILE")
TOKEN_1_ADDRESS=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[3].address' "$TOKENS_FILE")

TOKEN_0_HEX=$(node /workspace/scripts/address_to_hex.js $TOKEN_0_ADDRESS)
TOKEN_1_HEX=$(node /workspace/scripts/address_to_hex.js $TOKEN_1_ADDRESS)

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
echo TOKEN_0_HEX: $TOKEN_0_HEX
echo "..."
echo TOKEN_1_SYMBOL: $TOKEN_1_SYMBOL
echo TOKEN_1_ADDRESS: $TOKEN_1_ADDRESS
echo TOKEN_1_HEX: $TOKEN_1_HEX



echo "..."
echo "..."
echo "..."
echo "..."
echo We will execute swap_exact_tokens_for_tokens
echo To swap exact 100 units of token_0 to some amount of token_1
echo "..."

# fn swap_exact_tokens_for_tokens(
#         e: Env,
#         amount_in: i128,
#         amount_out_min: i128,
#         path: Vec<Address>,
#         to: Address,
#         deadline: u64,
#     ) -> Vec<i128>;

ROUTER_ADDRESS=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .router_address' "$ROUTER_FILE")
echo Using ROUTER_ADDRESS: $ROUTER_ADDRESS

ROUTER_WASM="/workspace/contracts/router/target/wasm32-unknown-unknown/release/soroswap_router.optimized.wasm"

    soroban contract invoke \
        --network $NETWORK \
        --source $USER_SECRET \
        --id $ROUTER_ADDRESS \
        -- \
        swap_exact_tokens_for_tokens \
        --amount_in 1000000000 \
        --amount_out_min 0 \
        --path "{\"vec\":[{\"address\": {\"contract\": \"$TOKEN_0_HEX\"}}, {\"address\": {\"contract\": \"$TOKEN_1_HEX\"}}]}" \
        --to $USER_PUBLIC \
        --deadline 9737055687 # year 2278


# "{\"vec\":[{\"address\": \"$TOKEN_0_ADDRESS\"}, {\"address\": \"$TOKEN_1_ADDRESS\"}]}"

#         error: parsing argument path: unknown variant `CBGIMY6IVEG73E4QFHSIFXOD4OTMHRAJEDOZS4VMUWGGQ7IKO46GHMEX`, expected `account` or `contract`