#!/bin/bash

set -e

NETWORK="futurenet"
FACTORY_ID="1486c77ba3a9639226dbbe64d1ac6f73a7dd6122201c6bc335d382981ec566b4"

# If soroban-cli is called inside the soroban-preview docker containter,
# it can call the stellar standalone container just using its name "stellar"
# if [[ "$IS_USING_DOCKER" == "true" ]]; then
  SOROBAN_RPC_HOST="http://stellar:8000"
# else
  # SOROBAN_RPC_HOST="http://localhost:8000"
# fi

SOROBAN_RPC_URL="$SOROBAN_RPC_HOST/soroban/rpc"

echo "Using Futurenet network"
SOROBAN_NETWORK_PASSPHRASE="Test SDF Future Network ; October 2022"
FRIENDBOT_URL="https://friendbot-futurenet.stellar.org/"

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
echo "$TOKEN_ADMIN_SECRET" > .soroban/token_admin_secret
echo "$TOKEN_ADMIN_ADDRESS" > .soroban/token_admin_address

# This will fail if the account already exists, but it'll still be fine.
echo Fund token-admin account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$TOKEN_ADMIN_ADDRESS"

ARGS="--network $NETWORK --source token-admin"
echo "Using ARGS: $ARGS"
echo Wrap two Stellar asset
mkdir -p .soroban

echo "--"
echo "--"

TOKEN_WASM="token/soroban_token_contract.wasm"


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


echo -n "$TOKEN_A_ID" > .soroban/token_a_id
echo -n "$TOKEN_B_ID" > .soroban/token_b_id

echo Build the SoroswapPair and SoroswapFactory contract
  make build
  FACTORY_WASM="/workspace/contracts/factory/target/wasm32v1-none/release/soroswap_factory.wasm"
  PAIR_WASM="/workspace/contracts/pair/target/wasm32v1-none/release/soroswap_pair.wasm"
  echo "--"
  echo "--"

  echo "--"
  echo "--"

  echo "--"
  echo "--"

  echo "--"
  echo "--"

  echo "--"
  echo "--"

  echo "--"
  echo "--"



echo "Create a pair using the SoroswapFactory contract, token_a and token_b"
PAIR_ID=$(soroban contract invoke \
  $ARGS \
  --wasm $FACTORY_WASM \
  --id $FACTORY_ID \
  -- \
  create_pair \
  --token_a "$TOKEN_A_ID" \
  --token_b "$TOKEN_B_ID" )
# Assuming the variable PAIR_ID contains the returned ID with apostrophes
PAIR_ID=$(echo $PAIR_ID | tr -d '"')
echo Pair created succesfully with PAIR_ID=$PAIR_ID
echo $PAIR_ID > .soroban/pair_id

echo "--"
echo "--"


echo How many pairs already exist??
soroban contract invoke \
  $ARGS \
  --wasm $FACTORY_WASM \
  --id $FACTORY_ID \
  -- \
  all_pairs_length \

echo We should be able to get the same PAIR_ID calling to get_pair function:
soroban contract invoke \
  $ARGS \
  --wasm $FACTORY_WASM \
  --id $FACTORY_ID \
  -- \
  get_pair \
  --token_a "$TOKEN_A_ID" \
  --token_b "$TOKEN_B_ID" 

echo Also if we ask for the inverse order

soroban contract invoke \
  $ARGS \
  --wasm $FACTORY_WASM \
  --id $FACTORY_ID \
  -- \
  get_pair \
  --token_a "$TOKEN_B_ID" \
  --token_b "$TOKEN_A_ID" 

echo "---"
echo "---"
echo "---"
echo "---"
echo "-- Now we will test functions calling the Pair contract"

echo Lets see if the token adddresses in the Pair contract are correct
echo Lets ask for token_0 -- TOKEN_A_ID -- 
echo We have TOKEN_A_ID = $TOKEN_A_ID

soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM \
  --id $PAIR_ID \
  -- \
  token_0 

echo "---"
echo "---"


echo Lets see if the token adddresses in the Pair contract are correct
echo Lets ask for token_1 -- TOKEN_B_ID -- 
echo We have TOKEN_B_ID = $TOKEN_B_ID

soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM \
  --id $PAIR_ID \
  -- \
  token_1 

echo "---"
echo "---"
echo Now lets test the created Pair. 


echo "---"
echo "---"


echo Finally::: check that if we try to create the same pair, the contract will panic:
soroban contract invoke \
  $ARGS \
  --wasm $FACTORY_WASM \
  --id $FACTORY_ID \
  -- \
  create_pair \
  --token_a "$TOKEN_A_ID" \
  --token_b "$TOKEN_B_ID" 

soroban contract invoke \
  $ARGS \
  --wasm $FACTORY_WASM \
  --id $FACTORY_ID \
  -- \
  create_pair \
  --token_a "$TOKEN_B_ID" \
  --token_b "$TOKEN_A_ID" 




echo "---"
echo "---"

echo "---"
echo "---"

echo "---"
echo "---"



echo In the following we are going to use a new USER account:
  echo Creating the user identity
  soroban keys generate --no-fund --network $NETWORK user
  USER_SECRET="$(soroban keys show user)"
  USER_ADDRESS="$(soroban keys address user)"
  echo "We are using the following USER_ADDRESS: $USER_ADDRESS"
  echo "$USER_SECRET" > .soroban/user_secret
  echo "$USER_ADDRESS" > .soroban/user_address
  



echo "Mint 1000 units of token A user -- calling from TOKEN_ADMIN"

soroban contract invoke \
  $ARGS \
  --wasm $TOKEN_WASM \
  --id $TOKEN_A_ID \
  -- \
  mint \
  --admin "$TOKEN_ADMIN_ADDRESS" \
  --to "$USER_ADDRESS" \
  --amount "1000" 

echo "Mint 1000 units of token B to user"

soroban contract invoke \
  $ARGS \
  --wasm $TOKEN_WASM \
  --id $TOKEN_B_ID \
  -- \
  mint \
  --admin "$TOKEN_ADMIN_ADDRESS" \
  --to "$USER_ADDRESS" \
  --amount "1000" 


echo "Check that user has 1000 units of each token"
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


echo "test get_rsrvs"
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $PAIR_ID \
  -- \
  get_rsrvs


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
  --desired_a 100 \
  --min_a 100 \
  --desired_b 100 \
  --min_b 100

echo Check that the user pair tokens balance is 100


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

echo 100 tokens of TOKEN_A
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $TOKEN_A_ID \
  -- \
  balance \
  --id "$PAIR_CONTRACT_ADDRESS"

echo 100 tokens of TOKEN_B
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $TOKEN_B_ID \
  -- \
  balance \
  --id "$PAIR_CONTRACT_ADDRESS"


echo And none of its own tokens -- the pair tokens --
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

# If "buy_a" is true, the swap will buy token_a and sell token_b. This is flipped if "buy_a" is false.
# "out" is the amount being bought, with in_max being a safety to make sure you receive at least that amount.
#  swap will transfer the selling token "to" to this contract, and then the contract will transfer the buying token to "to".
#     fn swap(e: Env, to: Address, buy_a: bool, out: i128, in_max: i128);

# In this case we are selling token_a and buying token_b

soroban contract invoke \
  $ARGS_USER \
  --wasm $PAIR_WASM \
  --id $PAIR_ID \
  -- \
  swap \
  --to "$USER_ADDRESS" \
  --out 49 \
  --in_max 100 



echo Now the user should have:
echo 803 units of TOKEN_A
echo "Check user\'s TOKEN_A balance"
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $TOKEN_A_ID \
  -- \
  balance \
  --id $USER_ADDRESS

echo 949 units of TOKEN_B
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

echo 197 tokens of TOKEN_A
soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM\
  --id $TOKEN_A_ID \
  -- \
  balance \
  --id "$PAIR_CONTRACT_ADDRESS"

echo 51 tokens of TOKEN_B
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
  --share_amount 100 \
  --min_a 197 \
  --min_b 51 
