#NETWORK="$1"

source /workspace/scripts/manual_testing/utils.sh

display_colored_text PURPLE " === REMOVE LIQUIDITY.SH === "

#Define network related constants
source /workspace/scripts/network_configs.sh

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



