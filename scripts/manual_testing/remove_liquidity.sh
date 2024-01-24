NETWORK="$1"

SOROBAN_RPC_HOST="http://stellar:8000"

SOROBAN_RPC_URL="$SOROBAN_RPC_HOST/soroban/rpc"

source /workspace/scripts/manual_testing/utils.sh

display_colored_text PURPLE " === REMOVE LIQUIDITY.SH === "

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


TOKEN_0_ADDRESS=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[6].address' "$TOKENS_FILE")
TOKEN_1_ADDRESS=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[7].address' "$TOKENS_FILE")

TOKEN_0_SYMBOL=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[6].symbol' "$TOKENS_FILE")
TOKEN_1_SYMBOL=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[7].symbol' "$TOKENS_FILE")

TOKEN_0_FIRST_BALANCE=$(getTokenBalance $TOKEN_0_ADDRESS)
TOKEN_1_FIRST_BALANCE=$(getTokenBalance $TOKEN_1_ADDRESS)
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
echo We will add remove liquidity, burning the 1000 units of LP tokens that we got before:
echo "..."
echo "..."
echo "..."
echo "..."
echo But before we will check how many tokens of each TOKEN_0, TOKEN_1 and LP...  we have:
echo "..."
echo "..."
echo "..."
TOKEN_WASM="/workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm"
echo TOKEN_0 balance:
soroban contract invoke \
  --network $NETWORK --source $USER_SECRET \
  --id $TOKEN_0_ADDRESS \
  -- \
  balance \
  --id "$USER_PUBLIC" 

echo "..."
echo "..."
echo "..."

echo TOKEN_1 balance:
soroban contract invoke \
  --network $NETWORK --source $USER_SECRET \
  --id $TOKEN_1_ADDRESS \
  -- \
  balance \
  --id "$USER_PUBLIC" 
echo "..."
echo "..."
echo "..."

PAIR_ADDRESS=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .pairs[0].pair_address' "$PAIRS_FILE")


echo "..."
echo "..."
echo "..."
PAIR_WASM="/workspace/contracts/pair/target/wasm32-unknown-unknown/release/soroswap_pair.optimized.wasm"

LP_BALANCE=$(soroban contract invoke \
  --network $NETWORK --source $USER_SECRET \
  --id $PAIR_ADDRESS \
  -- \
  balance \
  --id "$USER_PUBLIC")
echo "..."
echo "..."
echo "..."
LP_BALANCE=$(echo $LP_BALANCE | tr -d '"')
echo LP BALANCE: $LP_BALANCE
echo "..."
echo "..."
echo We will burn all the $LP_BALANCE LP tokens:


echo "..."
ROUTER_ADDRESS=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .router_address' "$ROUTER_FILE")
echo Using ROUTER_ADDRESS: $ROUTER_ADDRESS

ROUTER_WASM="/workspace/contracts/router/target/wasm32-unknown-unknown/release/soroswap_router.optimized.wasm"

soroban contract invoke \
    --network $NETWORK \
    --source $USER_SECRET \
    --id $ROUTER_ADDRESS \
    -- \
    remove_liquidity \
    --token_a $TOKEN_1_ADDRESS \
    --token_b $TOKEN_0_ADDRESS \
    --liquidity $LP_BALANCE \
    --amount_a_min 0 \
    --amount_b_min 0 \
    --to $USER_PUBLIC \
    --deadline 9737055687 # year 2278


echo "..."
echo "..."
echo "..."


echo Lets check new TOKEN_0, TOKEN_1 and LP...  balances
echo "..."
echo "..."
echo "..."
TOKEN_WASM="/workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm"
echo TOKEN_0 balance:
soroban contract invoke \
  --network $NETWORK --source $USER_SECRET \
  --id $TOKEN_0_ADDRESS \
  -- \
  balance \
  --id "$USER_PUBLIC" 

echo "..."
echo "..."
echo "..."

echo TOKEN_1 balance:
soroban contract invoke \
  --network $NETWORK --source $USER_SECRET \
  --id $TOKEN_1_ADDRESS \
  -- \
  balance \
  --id "$USER_PUBLIC" 
echo "..."
echo "..."
echo "..."

echo LP balance:
soroban contract invoke \
  --network $NETWORK --source $USER_SECRET \
  --id $PAIR_ADDRESS \
  -- \
  balance \
  --id "$USER_PUBLIC"

printTokensBalanceDiff "Remove_liquidity" $TOKEN_0_SYMBOL $TOKEN_0_ADDRESS $TOKEN_0_FIRST_BALANCE $TOKEN_1_SYMBOL $TOKEN_1_ADDRESS $TOKEN_1_FIRST_BALANCE "LP Balance" $PAIR_ADDRESS $LP_BALANCE



