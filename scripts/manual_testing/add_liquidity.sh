NETWORK="$1"

SOROBAN_RPC_HOST="http://stellar:8000"

SOROBAN_RPC_URL="$SOROBAN_RPC_HOST/soroban/rpc"


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
  echo "Using Testnet network with public RPC https://soroban-testnet.stellar.org/"
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
  SOROBAN_TOKENS_FOLDER="/workspace/.soroban/soroban_tokens/"
  ;;
public)
  echo "Using deployed contracts from /public folder"
  TOKENS_FILE="/workspace/public/tokens.json"
  ROUTER_FILE="/workspace/public/router.json"
  SOROBAN_TOKENS_FOLDER="/workspace/public/soroban_tokens/"

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

TOKEN30_SYMBOL=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[2].symbol' "$TOKENS_FILE")
TOKEN41_SYMBOL=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[3].symbol' "$TOKENS_FILE")

echo3"..."
echo4"..."
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
echo We will add liquidity: 1,000 units of each token
echo "..."
ROUTER_ADDRESS=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .router_address' "$ROUTER_FILE")
echo Using ROUTER_ADDRESS: $ROUTER_ADDRESS

ROUTER_WASM="/workspace/contracts/router/target/wasm32-unknown-unknown/release/soroswap_router.optimized.wasm"

    soroban contract invoke \
        --network $NETWORK \
        --source $USER_SECRET \
        --id $ROUTER_ADDRESS \
        -- \
        add_liquidity \
        --token_a $TOKEN_1_ADDRESS \
        --token_b $TOKEN_0_ADDRESS \
        --amount_a_desired 10000000000 \
        --amount_b_desired 10000000000 \
        --amount_a_min 0 \
        --amount_b_min 0 \
        --to $USER_PUBLIC \
        --deadline 9737055687 # year 2278

# TOKEN_WASM="/workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm"
# ADMIN_PUBLIC=$(cat .soroban/token_admin_public)


# soroban contract invoke \
#     --network $NETWORK \
#     --source $USER_SECRET \
#     --wasm $TOKEN_WASM \
#     --id $TOKEN_0_ADDRESS \
#     -- \
#     transfer \
#     --from "$USER_PUBLIC" \
#     --to "$ADMIN_PUBLIC" \
#     --amount 10000000000

# #!/bin/bash

# # Setup
# TOKEN_ADMIN_SECRET="SCLPVGNVME5OJKOMPRPKDQSMKWC52RLK63T5IGMMCA52KL64WDK3MZZD"
# LOCAL_USER_ADDRESS="$(soroban config identity address token-admin)"
# echo "LOCAL_USER_ADDRESS: $LOCAL_USER_ADDRESS"

# NETWORK="testnet"

# TOKEN_WASM="/workspace/contracts/token/soroban_token_contract.optimized.wasm"
# PAIR_WASM="/workspace/contracts/pair/target/wasm32-unknown-unknown/release/soroswap_pair.optimized.wasm"
# ROUTER_WASM="/workspace/contracts/router/target/wasm32-unknown-unknown/release/soroswap_router.optimized.wasm"
# FACTORY_WASM="/workspace/contracts/factory/target/wasm32-unknown-unknown/release/soroswap_factory.optimized.wasm"

# XLM_CONTRACT_ID="CACEEMMWGVDM6RZD7ZL6Z75Y32MI5ZWBGVTXTSCLCXXOW57OD63KKDTD"
# USDC_CONTRACT_ID="CDMOQLZXRDQMQBJDKFNPE3ORBUXZ7PY6JMN2XFL4TVASPFK4BG65TKQP"

# ROUTER_CONTRACT_ID="CCPY4Q24CWFCNZYEGUZ3RFHQS4PTDX3LTJRVEUNXGIST46RMSO4ENWF3"

# # WALLET_TO_CHECK="GCHR5WWPDFF3U3HP2NA6TI6FCQPYEWS3UOPIPJKZLAAFM57CEG4ZYBWP"
# WALLET_TO_CHECK="$LOCAL_USER_ADDRESS"

# # Mint test tokens
# CONTRACT_IDS=("$XLM_CONTRACT_ID" "$USDC_CONTRACT_ID")

# ## Loop through each contract ID and mint tokens
# # for CONTRACT_ID in "${CONTRACT_IDS[@]}"; do
# #     echo "Minting tokens for contract $CONTRACT_ID..."

# #     MINT_RESULT="$(soroban contract invoke \
# #         --network $NETWORK \
# #         --source-account $TOKEN_ADMIN_SECRET \
# #         --wasm $TOKEN_WASM \
# #         --id $CONTRACT_ID \
# #         -- \
# #         mint \
# #         --to "$LOCAL_USER_ADDRESS" \
# #         --amount "25000000000000")"

# #     echo "Mint result: $MINT_RESULT"
# # done

# # ## Check balances
# # # Loop through each contract ID and get token balances
# # for CONTRACT_ID in "${CONTRACT_IDS[@]}"; do
# #     echo "Checking token balance for contract $CONTRACT_ID..."

# #     soroban contract invoke \
# #         --network $NETWORK \
# #         --source-account $TOKEN_ADMIN_SECRET \
# #         --wasm $TOKEN_WASM \
# #         --id $CONTRACT_ID \
# #         -- \
# #         balance \
# #         --id "$LOCAL_USER_ADDRESS" 
# # done


# # Add liquidity
#     # fn add_liquidity(
#     #     e: Env,
#     #     token_a: Address,
#     #     token_b: Address,
#     #     amount_a_desired: i128,
#     #     amount_b_desired: i128,
#     #     amount_a_min: i128,
#     #     amount_b_min: i128,
#     #     to: Address,
#     #     deadline: u64,
#     # ) -> (i128, i128, i128);
# echo "Adding liquidity"
# # soroban contract invoke \
# #         --network $NETWORK \
# #         --source token-admin \
# #         --wasm $ROUTER_WASM \
# #         --id $ROUTER_CONTRACT_ID \
# #         -- \
# #         add_liquidity \
# #         --token_a "$XLM_CONTRACT_ID" \
# #         --token_b "$USDC_CONTRACT_ID" \
# #         --amount_a_desired 10000000000000\
# #         --amount_b_desired 1000000000000\
# #         --amount_a_min 1000000000000\
# #         --amount_b_min 100000000000\
# #         --to $LOCAL_USER_ADDRESS\
# #         --deadline 1699721331

# ## Reading some data
# # soroban contract invoke \
# #         --network $NETWORK \
# #         --source token-admin \
# #         --wasm $ROUTER_WASM \
# #         --id $ROUTER_CONTRACT_ID \
# #         -- \
# #         get_factory

#     # fn swap_exact_tokens_for_tokens(
#     #     e: Env,
#     #     amount_in: i128,
#     #     amount_out_min: i128,
#     #     path: Vec<Address>,
#     #     to: Address,
#     #     deadline: u64,
#     # ) -> Vec<i128>;

# PATH_ARRAY="[$XLM_CONTRACT_ID,$USDC_CONTRACT_ID]"

# soroban contract invoke \
#         --network $NETWORK \
#         --source token-admin \
#         --wasm $ROUTER_WASM \
#         --id $ROUTER_CONTRACT_ID \
#         -- \
#         swap_exact_tokens_for_tokens \
#         --amount_in 0 \
#         --amount_out_min 0 \
#         --path "CACEEMMWGVDM6RZD7ZL6Z75Y32MI5ZWBGVTXTSCLCXXOW57OD63KKDTD,CDMOQLZXRDQMQBJDKFNPE3ORBUXZ7PY6JMN2XFL4TVASPFK4BG65TKQP" \
#         --to $LOCAL_USER_ADDRESS \
#         --deadline 1699721331

echo "..."
echo "..."
echo "..."
echo "..."
echo We will add liquidity for stellar asset
echo "..."

echo "we will first wrap the token"  
ASSET_DEPLOYER_PUBLIC="$(soroban config identity address asset_deployer)"

soroban lab token wrap \
  --network standalone \
  --source asset_deployer \
  --asset "AstroDollar:$ASSET_DEPLOYER_PUBLIC"

STELLAR_ASSET_CONTRACT_ID="$(
  soroban lab token id \
  --network standalone \
  --source asset_deployer \
  --asset "AstroDollar:$ASSET_DEPLOYER_PUBLIC"
)"
echo "STELLAR_ASSET_CONTRACT_ID: $STELLAR_ASSET_CONTRACT_ID"
echo "We verify the asset is wrapped and balances are OK"

TOKEN_WASM="/workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm"

STELLAR_ASSET_BALANCE_OF_USER="$(soroban contract invoke \
  --network $NETWORK \
  --source asset_deployer \
  --wasm $TOKEN_WASM \
  --id $STELLAR_ASSET_CONTRACT_ID \
  -- \
  balance \
  --id "$USER_PUBLIC"   )"
TOKEN_1_BALANCE_OF_USER="$(soroban contract invoke \
  --network $NETWORK \
  --source asset_deployer \
  --wasm $TOKEN_WASM \
  --id $TOKEN_1_ADDRESS \
  -- \
  balance \
  --id "$USER_PUBLIC"   )"
echo "Balances of User: ${USER_PUBLIC}, 
  Stellar Asset: ${STELLAR_ASSET_BALANCE_OF_USER}
  Token 1: ${TOKEN_1_BALANCE_OF_USER}"


echo "Then, we will add liquidity with one of the previous tokens"

soroban contract invoke \
    --network $NETWORK \
    --source $USER_SECRET \
    --wasm $ROUTER_WASM \
    --id $ROUTER_ADDRESS \
    -- \
    add_liquidity \
    --token_a $TOKEN_1_ADDRESS \
    --token_b $STELLAR_ASSET_CONTRACT_ID \
    --amount_a_desired 10000000000 \
    --amount_b_desired 10000000000 \
    --amount_a_min 0 \
    --amount_b_min 0 \
    --to $USER_PUBLIC \
    --deadline 9737055687 # year 2278

echo "Then, we will add liquidity with the tokens deployed not by token-admin"

SOROBAN_TOKEN_A_ID=$(cat $SOROBAN_TOKENS_FOLDER/token_a_id)
SOROBAN_TOKEN_B_ID=$(cat $SOROBAN_TOKENS_FOLDER/token_b_id)
echo "SOROBAN_TOKEN_A_ID: $SOROBAN_TOKEN_A_ID"
echo "SOROBAN_TOKEN_B_ID: $SOROBAN_TOKEN_B_ID"


SOROBAN_TOKEN_A_ID_BALANCE_OF_USER="$(soroban contract invoke \
  --network $NETWORK \
  --source asset_deployer \
  --wasm $TOKEN_WASM \
  --id $SOROBAN_TOKEN_A_ID \
  -- \
  balance \
  --id "$USER_PUBLIC"   )"
SOROBAN_TOKEN_B_ID_BALANCE_OF_USER="$(soroban contract invoke \
  --network $NETWORK \
  --source asset_deployer \
  --wasm $TOKEN_WASM \
  --id $SOROBAN_TOKEN_B_ID \
  -- \
  balance \
  --id "$USER_PUBLIC"   )"
echo "We check balances..."
echo "Balances of User: ${USER_PUBLIC}, 
soroban token A: ${SOROBAN_TOKEN_A_ID_BALANCE_OF_USER}
soroban token B: ${SOROBAN_TOKEN_B_ID_BALANCE_OF_USER}"


soroban contract invoke \
    --network $NETWORK \
    --source $USER_SECRET \
    --wasm $ROUTER_WASM \
    --id $ROUTER_ADDRESS \
    -- \
    add_liquidity \
    --token_a $SOROBAN_TOKEN_A_ID \
    --token_b $SOROBAN_TOKEN_B_ID \
    --amount_a_desired 10000000000 \
    --amount_b_desired 10000000000 \
    --amount_a_min 0 \
    --amount_b_min 0 \
    --to $USER_PUBLIC \
    --deadline 9737055687 # year 2278
