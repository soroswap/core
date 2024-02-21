# Ensure the script exits on any errors
source /workspace/scripts/network_configs.sh

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <network>"
    exit 1
fi

NETWORK=$1

echo "Build and optimize the contracts...";
cd /workspace/contracts/aggregator/protocols/phoenix
make build > /dev/null
cd target/wasm32-unknown-unknown/release

echo "Contracts compiled."
echo "Optimize contracts..."

soroban contract optimize --wasm soroban_token_contract.wasm
soroban contract optimize --wasm phoenix_factory.wasm
soroban contract optimize --wasm phoenix_pool.wasm
soroban contract optimize --wasm phoenix_stake.wasm
soroban contract optimize --wasm phoenix_multihop.wasm

echo "Contracts optimized."

if !(soroban config identity ls | grep phoenix-admin 2>&1 >/dev/null); then
  echo Create the phoenix-admin identity
  soroban keys generate --no-fund --network $NETWORK phoenix-admin
fi
# Fetch the admin's address
PHOENIX_ADMIN_ADDRESS=$(soroban config identity address phoenix-admin)
curl  -X POST "$FRIENDBOT_URL?addr=$PHOENIX_ADMIN_ADDRESS"


echo "Deploy the soroban_token_contract and capture its contract ID hash..."

TOKEN_ADDR1=$(soroban contract deploy \
    --wasm soroban_token_contract.optimized.wasm \
    --source phoenix-admin \
    --network $NETWORK)

TOKEN_ADDR2=$(soroban contract deploy \
    --wasm soroban_token_contract.optimized.wasm \
    --source phoenix-admin \
    --network $NETWORK)

FACTORY_ADDR=$(soroban contract deploy \
    --wasm phoenix_factory.optimized.wasm \
    --source phoenix-admin \
    --network $NETWORK)

MULTIHOP_ADDR=$(soroban contract deploy \
    --wasm phoenix_multihop.optimized.wasm \
    --source phoenix-admin \
    --network $NETWORK)

echo "Tokens, factory and multihop deployed."

echo "Install the soroban_token, phoenix_pair and phoenix_stake contracts..."

TOKEN_WASM_HASH=$(soroban contract install \
    --wasm soroban_token_contract.optimized.wasm \
    --source phoenix-admin \
    --network $NETWORK)

# Continue with the rest of the deployments
PAIR_WASM_HASH=$(soroban contract install \
    --wasm phoenix_pool.optimized.wasm \
    --source phoenix-admin \
    --network $NETWORK)

STAKE_WASM_HASH=$(soroban contract install \
    --wasm phoenix_stake.optimized.wasm \
    --source phoenix-admin \
    --network $NETWORK)

MULTIHOP_WASM_HASH=$(soroban contract install \
    --wasm phoenix_multihop.optimized.wasm \
    --source phoenix-admin \
    --network $NETWORK)

echo "Token, pair and stake contracts deployed."

# Sort the token addresses alphabetically
if [[ "$TOKEN_ADDR1" < "$TOKEN_ADDR2" ]]; then
    TOKEN_ID1=$TOKEN_ADDR1
    TOKEN_ID2=$TOKEN_ADDR2
else
    TOKEN_ID1=$TOKEN_ADDR2
    TOKEN_ID2=$TOKEN_ADDR1
fi

echo "Initialize multihop..."

soroban contract invoke \
    --id $MULTIHOP_ADDR \
    --source phoenix-admin \
    --network $NETWORK \
    -- \
    initialize \
    --admin $PHOENIX_ADMIN_ADDRESS \
    --factory $FACTORY_ADDR

echo "Multihop initialized."

echo "Initialize factory..."

# ADMIN_ADDRESS_HEX=$(node scripts/address_to_hex.js $PHOENIX_ADMIN_ADDRESS)

# echo "Admin address: $PHOENIX_ADMIN_ADDRESS"
# echo "Admin address Hex: $ADMIN_ADDRESS_HEX"

soroban contract invoke \
    --id $FACTORY_ADDR \
    --source phoenix-admin \
    --network $NETWORK \
    -- \
    initialize \
    --admin $PHOENIX_ADMIN_ADDRESS \
    --multihop_wasm_hash $MULTIHOP_WASM_HASH \
    --lp_wasm_hash $PAIR_WASM_HASH \
    --stake_wasm_hash $STAKE_WASM_HASH \
    --token_wasm_hash $TOKEN_WASM_HASH \
    --whitelisted_accounts "{\"vec\":[{\"address\": \"$PHOENIX_ADMIN_ADDRESS\"}]}"

echo "Factory initialized."

echo "Initialize the token contracts..."

soroban contract invoke \
    --id $TOKEN_ID1 \
    --source phoenix-admin \
    --network $NETWORK \
    -- \
    initialize \
    --admin $PHOENIX_ADMIN_ADDRESS \
    --decimal 7 \
    --name TOKEN \
    --symbol TOK

soroban contract invoke \
    --id $TOKEN_ID2 \
    --source phoenix-admin \
    --network $NETWORK \
    -- \
    initialize \
    --admin $PHOENIX_ADMIN_ADDRESS \
    --decimal 7 \
    --name PHOENIX \
    --symbol PHO

echo "Tokens initialized."

echo "Initialize pair using the previously fetched hashes through factory..."

soroban contract invoke \
    --id $FACTORY_ADDR \
    --source phoenix-admin \
    --network $NETWORK \
    -- \
    create_liquidity_pool \
    --lp_init_info "{ \"admin\": \"${PHOENIX_ADMIN_ADDRESS}\", \"lp_wasm_hash\": \"${PAIR_WASM_HASH}\", \"share_token_decimals\": 7, \"swap_fee_bps\": 1000, \"fee_recipient\": \"${PHOENIX_ADMIN_ADDRESS}\", \"max_allowed_slippage_bps\": 10000, \"max_allowed_spread_bps\": 10000, \"max_referral_bps\": 10000, \"token_init_info\": { \"token_wasm_hash\": \"${TOKEN_WASM_HASH}\", \"token_a\": \"${TOKEN_ID1}\", \"token_b\": \"${TOKEN_ID2}\" }, \"stake_init_info\": { \"stake_wasm_hash\": \"${STAKE_WASM_HASH}\", \"min_bond\": \"100\", \"min_reward\": \"100\", \"max_distributions\": 3 } }" \
    --caller $PHOENIX_ADMIN_ADDRESS

PAIR_ADDR=$(soroban contract invoke \
    --id $FACTORY_ADDR \
    --source phoenix-admin \
    --network $NETWORK --fee 100 \
    -- \
    query_pools | jq -r '.[0]')

echo "Pair contract initialized."

echo "Mint both tokens to the admin and provide liquidity..."
soroban contract invoke \
    --id $TOKEN_ID1 \
    --source phoenix-admin \
    --network $NETWORK \
    -- \
    mint --to $PHOENIX_ADMIN_ADDRESS --amount 100000000000

soroban contract invoke \
    --id $TOKEN_ID2 \
    --source phoenix-admin \
    --network $NETWORK \
    -- \
    mint --to $PHOENIX_ADMIN_ADDRESS --amount 100000000000

# Provide liquidity in 2:1 ratio to the pool
soroban contract invoke \
    --id $PAIR_ADDR \
    --source phoenix-admin \
    --network $NETWORK --fee 10000000 \
    -- \
    provide_liquidity --sender $PHOENIX_ADMIN_ADDRESS --desired_a 100000000000 --desired_b 50000000000

echo "Liquidity provided."

# Continue with the rest of the commands
echo "Bond tokens to stake contract..."

STAKE_ADDR=$(soroban contract invoke \
    --id $PAIR_ADDR \
    --source phoenix-admin \
    --network $NETWORK --fee 10000000 \
    -- \
    query_stake_contract_address | jq -r '.')

# Bond token in stake contract
soroban contract invoke \
    --id $STAKE_ADDR \
    --source phoenix-admin \
    --network $NETWORK \
    -- \
    bond --sender $PHOENIX_ADMIN_ADDRESS --tokens 70000000000

echo "Tokens bonded."

echo "Initialization complete!"
echo "Token Contract 1 address: $TOKEN_ID1"
echo "Token Contract 2 address: $TOKEN_ID2"
echo "Pair Contract address: $PAIR_ADDR"
echo "Stake Contract address: $STAKE_ADDR"
echo "Factory Contract address: $FACTORY_ADDR"

NEW_PHOENIX_OBJECT="{ \"network\": \"$NETWORK\", \"multihop_address\": \"$MULTIHOP_ADDR\", \"factory_address\": \"$FACTORY_ADDR\", \"token_a\": \"$TOKEN_ID1\", \"token_b\": \"$TOKEN_ID2\" }"
echo "New aggregator object: $NEW_PHOENIX_OBJECT"

PHOENIX_FILE="/workspace/.soroban/phoenix_protocol.json"
# Initialize factory.json if it does not exist
if [[ ! -f "$PHOENIX_FILE" ]]; then
    echo file not found
    echo "[]" > "$PHOENIX_FILE"
fi


CURRENT_PHOENIX_JSON=$(cat $PHOENIX_FILE)
echo "CURRENT_PHOENIX_JSON: $CURRENT_PHOENIX_JSON"


# check if the network already exists in that json
exists=$(echo "$CURRENT_PHOENIX_JSON" | jq '.[] | select(.network == "'$NETWORK'")')
echo "This network already exist in the phoenix_protocol.json? : $exists"

NEW_PHOENIX_JSON="{}"
if [[ -n "$exists" ]]; then
    # if the network exists, update the factory for that network
    echo network exists, replace
    NEW_PHOENIX_JSON=$(echo "$CURRENT_PHOENIX_JSON" | jq '
        map(if .network == "'$NETWORK'" then '"$NEW_PHOENIX_OBJECT"' else . end)'
    )
else
    # if the network doesn't exist, append the new object to the list
    echo network does not exist, append
    NEW_PHOENIX_JSON=$(echo "$CURRENT_PHOENIX_JSON" | jq '. += ['"$NEW_PHOENIX_OBJECT"']')
fi

# echo "NEW_PHOENIX_JSON: $NEW_PHOENIX_JSON"
echo "$NEW_PHOENIX_JSON" > "$PHOENIX_FILE"

echo "end creating the phoenix_protocol.json" 

# Output the file path and contents
echo "Aggregator information available in /workspace/.soroban/phoenix_protocol.json"
cat /workspace/.soroban/phoenix_protocol.json
