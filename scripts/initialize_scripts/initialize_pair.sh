#!/bin/bash

set -e

NETWORK="$1"

# If soroban-cli is called inside the soroban-preview docker containter,
# it can call the stellar standalone container just using its name "stellar"
# if [[ "$IS_USING_DOCKER" == "true" ]]; then
  SOROBAN_RPC_HOST="http://stellar:8000"
# else
  # SOROBAN_RPC_HOST="http://localhost:8000"
# fi

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
  echo "Using Futurenet network"
  SOROBAN_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
  FRIENDBOT_URL="https://friendbot.stellar.org/"
  ;;
*)
  echo "Usage: $0 standalone|futurenet|testnet"
  exit 1
  ;;
esac

#if !(soroban config network ls | grep "$NETWORK" 2>&1 >/dev/null); then
# Always set a net configuration 
  echo Add the $NETWORK network to cli client
  soroban config network add "$NETWORK" \
    --rpc-url "$SOROBAN_RPC_URL" \
    --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE"
#fi

if !(soroban config identity ls | grep token-admin 2>&1 >/dev/null); then
  echo Create the token-admin identity
  soroban keys generate --no-fund --network $NETWORK token-admin
fi
TOKEN_ADMIN_SECRET="$(soroban keys show token-admin)"
TOKEN_ADMIN_ADDRESS="$(soroban keys address token-admin)"

echo "We are using the following TOKEN_ADMIN_ADDRESS: $TOKEN_ADMIN_ADDRESS"
echo "--"
echo "--"
# TODO: Remove this once we can use `soroban config identity` from webpack.
echo "$TOKEN_ADMIN_SECRET" > /workspace/.soroban/token_admin_secret
echo "$TOKEN_ADMIN_ADDRESS" > /workspace/.soroban/token_admin_address

# This will fail if the account already exists, but it'll still be fine.
echo Fund token-admin account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$TOKEN_ADMIN_ADDRESS"

ARGS="--network $NETWORK --source token-admin"
echo "Using ARGS: $ARGS"
echo "--" 
echo "--"

mkdir -p /workspace/.soroban

TOKEN_WASM="/workspace/contracts/token/soroban_token_contract.wasm"
 
echo Deploying TOKEN_A

TOKEN_A_ID="$(
  soroban contract deploy $ARGS \
    --wasm $TOKEN_WASM
  )"
  echo "--"
  echo "--"

echo Initializing TOKEN_A
  echo "Executing: 
  fn initialize(  e: Env,
                  admin: Address,
                  decimal: u32,
                  name: Bytes,
                  symbol: Bytes) {
  "

  soroban contract invoke \
  $ARGS \
  --wasm $TOKEN_WASM \
  --id $TOKEN_A_ID \
  -- \
  initialize \
  --admin "$TOKEN_ADMIN_ADDRESS" \
  --decimal 7 \
  --name 'AA' \
  --symbol 'AA'
  echo "--"
  echo "--"


echo Deploying TOKEN_B
  TOKEN_B_ID="$(
  soroban contract deploy $ARGS \
    --wasm $TOKEN_WASM
  )"
  echo "--"
  echo "--"

echo Initializing TOKEN_B
  echo "Executing: 
  fn initialize(  e: Env,
                  admin: Address,
                  decimal: u32,
                  name: Bytes,
                  symbol: Bytes) {
  "

  soroban contract invoke \
  $ARGS \
  --wasm $TOKEN_WASM \
  --id $TOKEN_B_ID \
  -- \
  initialize \
  --admin "$TOKEN_ADMIN_ADDRESS" \
  --decimal 7 \
  --name 'BB' \
  --symbol 'BB'
  echo "--"
  echo "--"

echo Current TOKEN_A_ID: $TOKEN_A_ID
echo Current TOKEN_B_ID: $TOKEN_B_ID
if [[ "$TOKEN_B_ID" > "$TOKEN_A_ID" ]]; then
  echo "TOKEN_B_ID is greater than TOKEN_A_ID"
  echo "This is the correct order"
else
  echo "TOKEN_B_ID is less than or equal to TOKEN_A_ID"
  echo "We will invert the order of the tokens"
  TOKEN_A_ID_NEW=$TOKEN_B_ID
  TOKEN_B_ID=$TOKEN_A_ID
  TOKEN_A_ID=$TOKEN_A_ID_NEW

fi
echo Current TOKEN_A_ID: $TOKEN_A_ID
echo Current TOKEN_B_ID: $TOKEN_B_ID
  echo "--"
  echo "--"

# TODO, remove this when https://github.com/stellar/soroban-tools/issues/661 is resolved.
TOKEN_A_ADDRESS="$(node /workspace/scripts/address_workaround.js $TOKEN_A_ID)"
TOKEN_B_ADDRESS="$(node /workspace/scripts/address_workaround.js $TOKEN_B_ID)"

echo -n "$TOKEN_A_ID" > /workspace/.soroban/token_a_id
echo -n "$TOKEN_B_ID" > /workspace/.soroban/token_b_id
echo -n "$TOKEN_A_ADDRESS" > /workspace/.soroban/token_a_address
echo -n "$TOKEN_B_ADDRESS" > /workspace/.soroban/token_b_address

echo Build the SoroswapPair contract
cd /workspace/contracts/pair
make build
cd ..
cd /workspace/contracts/factory
make build
cd  ..
PAIR_WASM="/workspace/contracts/pair/target/wasm32v1-none/release/soroswap_pair.wasm"
FACTORY_WASM="/workspace/contracts/factory/target/wasm32v1-none/release/soroswap_factory.wasm"

echo "--"
echo "--"

echo Deploy the Pair 
PAIR_ID="$(
soroban contract deploy $ARGS \
  --wasm $PAIR_WASM
)"
echo "$PAIR_ID" > /workspace/.soroban/pair_wasm_hash
echo "SoroswapPair deployed succesfully with PAIR_ID: $PAIR_ID"
echo "--"
echo "--"

echo Deploy the Factory 
FACTORY_ID="$(
soroban contract deploy $ARGS \
  --wasm $FACTORY_WASM
)"

FACTORY_ADDRESS="$(node /workspace/scripts/address_workaround.js $FACTORY_ID)"

echo "$FACTORY_WASM" > /workspace/.soroban/factory_wasm_hash
echo "SoroswapFactory deployed succesfully with FACTORY_ID: $FACTORY_ID"
echo "SoroswapFactory deployed succesfully with FACTORY_ADDRESS: $FACTORY_ADDRESS"

echo "--"
echo "--"

echo "Initialize the Factory contract using the Admin address as Setter"
echo "Calling: 
fn initialize(e: Env,
                  setter: Address, 
                  pair_wasm_hash: BytesN<32>)
"
soroban contract invoke \
  $ARGS \
  --wasm $FACTORY_WASM \
  --id $FACTORY_ID \
  -- \
  initialize \
  --setter "$TOKEN_ADMIN_ADDRESS" \
  --pair_wasm_hash "$PAIR_ID"
echo "--"
echo "--"



echo "Initialize the Pair contract using the Admin address as Factory"
echo "Calling: 
fn initialize(e: Env,
                  factory: Address, 
                  token_a: Address, 
                  token_b: Address)
"

soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM \
  --id $PAIR_ID \
  -- \
  initialize \
  --factory "$FACTORY_ADDRESS" \
  --token_a "$TOKEN_A_ADDRESS" \
  --token_b "$TOKEN_B_ADDRESS" 
echo "--"
echo "--"

echo In the following we are going to use a new USER account:
  echo Creating the user identity
  soroban keys generate --no-fund --network $NETWORK user
  USER_SECRET="$(soroban keys show user)"
  USER_ADDRESS="$(soroban keys address user)"
  echo "We are using the following USER_ADDRESS: $USER_ADDRESS"
  echo "$USER_SECRET" > /workspace/.soroban/user_secret
  echo "$USER_ADDRESS" > /workspace/.soroban/user_address
  


echo "Mint 10000000000 units of token A user -- calling from TOKEN_ADMIN"

soroban contract invoke \
  $ARGS \
  --wasm $TOKEN_WASM \
  --id $TOKEN_A_ID \
  -- \
  mint \
  --to "$USER_ADDRESS" \
  --amount "10000000000" 

echo "Mint 10000000000 units of token B to user"

soroban contract invoke \
  $ARGS \
  --wasm $TOKEN_WASM \
  --id $TOKEN_B_ID \
  -- \
  mint \
  --to "$USER_ADDRESS" \
  --amount "10000000000" 


echo "Check that user has 10000000000 units of each token"
echo "Check TOKEN_A"
soroban contract invoke \
  $ARGS \
  --wasm $TOKEN_WASM\
  --id $TOKEN_A_ID \
  -- \
  balance \
  --id $USER_ADDRESS

echo "Check TOKEN_A"
soroban contract invoke \
  $ARGS \
  --wasm $TOKEN_WASM\
  --id $TOKEN_B_ID \
  -- \
  balance \
  --id $USER_ADDRESS


echo "test get_reserves"
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $PAIR_ID \
  -- \
  get_reserves


echo "Deposit these tokens into the Pool contract"
echo "This will be called by the user"
ARGS_USER="--network $NETWORK --source user"
echo "Hence we use ARG_USER: $ARGS_USER"

echo Fund user account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$USER_ADDRESS"


echo "
Calling:
    fn deposit( e: Env, 
                to: Address,
                desired_a: i128, 
                min_a: i128, 
                desired_b: i128, 
                min_b: i128) {


"


ARGS="--network $NETWORK --source token-admin"
PAIR_WASM="/workspace/contracts/pair/target/wasm32v1-none/release/soroswap_pair.wasm"
PAIR_ID=$(cat /workspace/.soroban/pair_wasm_hash)
TOKEN_ADMIN_ADDRESS=$(cat /workspace/.soroban/token_admin_address)
USER_ADDRESS=$(cat /workspace/.soroban/user_address)
TOKEN_A_ID=$(cat /workspace/.soroban/token_a_id)
TOKEN_B_ID=$(cat /workspace/.soroban/token_b_id)
ARGS_USER="--network $NETWORK --source user"


echo In the next we will use:
echo ARGS = $ARGS
echo ARGS_USER = $ARGS_USER
echo PAIR_WASM = $PAIR_WASM
echo PAIR_ID = $PAIR_ID
echo TOKEN_ADMIN_ADDRESS = $TOKEN_ADMIN_ADDRESS
echo USER_ADDRESS = $USER_ADDRESS
echo TOKEN_A_ID = $TOKEN_A_ID
echo TOKEN_B_ID = $TOKEN_B_ID
echo "--"
echo "--"

soroban contract invoke \
  $ARGS_USER \
  --wasm $PAIR_WASM \
  --id $PAIR_ID \
  -- \
  deposit \
  --to "$USER_ADDRESS" \
  --desired_a 1000000000 \
  --min_a 1000000000 \
  --desired_b 1000000000 \
  --min_b 1000000000

echo Check that the user pair tokens balance is 1000000000

soroban contract invoke \
  $ARGS \
  --wasm $FACTORY_WASM\
  --id $FACTORY_ID \
  -- \
  fees_enabled 

echo FACTORY_ID = $FACTORY_ID


echo "Check factory address in pair contract"
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $PAIR_ID \
  -- \
  factory 

echo "Check PAIR_ID"
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $PAIR_ID \
  -- \
  balance \
  --id $USER_ADDRESS

echo Now the user should have:
echo 900 units of TOKEN_A
echo "Check user\'s TOKEN_A balance"
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $TOKEN_A_ID \
  -- \
  balance \
  --id $USER_ADDRESS

echo 900 units of TOKEN_B
echo "Check user\'s TOKEN_B balance"
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $TOKEN_B_ID \
  -- \
  balance \
  --id $USER_ADDRESS

echo And the Pair contract should hold:
PAIR_CONTRACT_ADDRESS="{\"address\": {\"contract\":\"$PAIR_ID\"}}"

echo 1000000000 tokens of TOKEN_A
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $TOKEN_A_ID \
  -- \
  balance \
  --id "$PAIR_CONTRACT_ADDRESS"

echo 1000000000 tokens of TOKEN_B
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $TOKEN_B_ID \
  -- \
  balance \
  --id "$PAIR_CONTRACT_ADDRESS"


echo "The pair contract should hold 1,000 units of the pair token itself as minimum liquidity"
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $PAIR_ID \
  -- \
  balance \
  --id "$PAIR_CONTRACT_ADDRESS"
echo "--"
echo "--"
echo "--"
echo "--"

echo Now we will SWAP 

# If "buy_0" is true, the swap will buy token_a and sell token_b. This is flipped if "buy_0" is false.
# "out" is the amount being bought, with in_max being a safety to make sure you receive at least that amount.
#  swap will transfer the selling token "to" to this contract, and then the contract will transfer the buying token to "to".
#     fn swap(e: Env, to: Address, buy_0: bool, out: i128, in_max: i128);

# In this case we are selling token_a and buying token_b

soroban contract invoke \
  $ARGS_USER \
  --wasm $PAIR_WASM \
  --id $PAIR_ID \
  -- \
  swap \
  --to "$USER_ADDRESS" \
  --amount_out 490000000 \
  --amount_in_max 1000000000 

## Here we don't set any --buy_0 "false"... it will take it as false


echo Now the user should have:
echo 8036324660 units of TOKEN_A
echo "Check user\'s TOKEN_A balance"
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $TOKEN_A_ID \
  -- \
  balance \
  --id $USER_ADDRESS

echo 9490000000 units of TOKEN_B
echo "Check user\'s TOKEN_B balance"
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $TOKEN_B_ID \
  -- \
  balance \
  --id $USER_ADDRESS

echo And the Pair contract should hold:
PAIR_CONTRACT_ADDRESS="{\"address\": {\"contract\":\"$PAIR_ID\"}}"

echo 1963675340 tokens of TOKEN_A
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $TOKEN_A_ID \
  -- \
  balance \
  --id "$PAIR_CONTRACT_ADDRESS"

echo 510000000 tokens of TOKEN_B
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $TOKEN_B_ID \
  -- \
  balance \
  --id "$PAIR_CONTRACT_ADDRESS"


echo "---"
echo "---"
echo "---"
echo "---"
echo "WITHDRAW: The final step"

echo "Calling: 
    fn withdraw(  e: Env,
                  to: Address,
                  share_amount: i128, 
                  min_a: i128, 
                  min_b: i128) -> (i128, i128) {"

soroban contract invoke \
  $ARGS_USER \
  --wasm $PAIR_WASM \
  --id $PAIR_ID \
  -- \
  withdraw \
  --to "$USER_ADDRESS" \
  --share_amount 999999000 \
  --min_a 0 \
  --min_b 0 