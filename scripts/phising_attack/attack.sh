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

TOKEN_ADMIN_SECRET="$(soroban config identity show token-admin)"
TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"

echo Create user identity
soroban config identity generate user
USER_ADDRESS="$(soroban config identity address user)"

echo Fund token-admin account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$TOKEN_ADMIN_ADDRESS" > /dev/null


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
    --id $TOKEN_0_ADDRESS \
    -- \
    balance \
    --id "$USER_ADDRESS"

echo Compile and deploy router contract
echo ".."