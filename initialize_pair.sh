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
*)
  echo "Usage: $0 standalone|futurenet"
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
  soroban config identity generate token-admin
fi
TOKEN_ADMIN_SECRET="$(soroban config identity show token-admin)"
TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"

echo "We are using the following TOKEN_ADMIN_ADDRESS: $TOKEN_ADMIN_ADDRESS"
echo "--"
echo "--"
# TODO: Remove this once we can use `soroban config identity` from webpack.
echo "$TOKEN_ADMIN_SECRET" > .soroban/token_admin_secret
echo "$TOKEN_ADMIN_ADDRESS" > .soroban/token_admin_address

# This will fail if the account already exists, but it'll still be fine.
echo Fund token-admin account from friendbot
curl  -X POST "$FRIENDBOT_URL?addr=$TOKEN_ADMIN_ADDRESS"

ARGS="--network $NETWORK --source token-admin"
echo "Using ARGS: $ARGS"
echo Wrap two Stellar asset
mkdir -p .soroban
echo "--"
echo "--"
echo "Wrapping TOKENA:$TOKEN_ADMIN_ADDRESS"
TOKEN_A_ID=$(soroban lab token wrap $ARGS --asset "GTTTA:$TOKEN_ADMIN_ADDRESS")
echo "token_a was wrapped succesfully with TOKEN_A_ID: $TOKEN_A_ID"
echo "--"
echo "--"

echo "Wrapping TOKENB:$TOKEN_ADMIN_ADDRESS"
TOKEN_B_ID=$(soroban lab token wrap $ARGS --asset "GTTTB:$TOKEN_ADMIN_ADDRESS")
echo "token_b was wrapped succesfully with TOKEN_B_ID: $TOKEN_B_ID"
echo "--"
echo "--"

echo -n "$TOKEN_A_ID" > .soroban/token_a_id
echo -n "$TOKEN_A_ID" > .soroban/token_b_id

echo Build the SoroswapPair contract
cd pair
make build
cd ..
PAIR_WASM="pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
echo "--"
echo "--"

echo Deploy the Pair 
PAIR_ID="$(
soroban contract deploy $ARGS \
  --wasm $PAIR_WASM
)"
echo "$PAIR_ID" > .soroban/pair_wasm_hash
echo "SoroswapPair deployed succesfully with PAIR_ID: $PAIR_ID"
echo "--"
echo "--"


echo "Initialize the Pair contract using the Admin address as Factory"
echo "Calling: 
fn initialize_pair( e: Env
                    factory: Address,
                    token_a: BytesN<32>,
                    token_b: BytesN<32>);
"
ARGS="--network standalone --source token-admin"
PAIR_WASM="pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
PAIR_ID=$(cat .soroban/pair_wasm_hash)
TOKEN_ADMIN_ADDRESS=$(cat .soroban/token_admin_address)
TOKEN_A_ID=$(cat .soroban/token_a_id)
TOKEN_B_ID=$(cat .soroban/token_b_id)


echo Using:
echo ARGS = $ARGS
echo PAIR_WASM = $PAIR_WASM
echo PAIR_ID = $PAIR_ID
echo TOKEN_ADMIN_ADDRESS = $TOKEN_ADMIN_ADDRESS
echo TOKEN_A_ID = $TOKEN_A_ID
echo TOKEN_B_ID = $TOKEN_B_ID


soroban contract invoke \
  $ARGS \
  --wasm $PAIR_WASM \
  --id $PAIR_ID \
  -- \
  initialize_pair \
  --factory "$TOKEN_ADMIN_ADDRESS" \
  --token_a "$TOKEN_A_ID" \
  --token_b "$TOKEN_B_ID" 

echo "--"
echo "--"
