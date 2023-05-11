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
TOKEN_A_ID=$(soroban lab token wrap $ARGS --asset "FTTTA:$TOKEN_ADMIN_ADDRESS")
echo "token_0 was wrapped succesfully with TOKEN_A_ID: $TOKEN_A_ID"
echo "--"
echo "--"

echo "Wrapping TOKENB:$TOKEN_ADMIN_ADDRESS"
TOKEN_B_ID=$(soroban lab token wrap $ARGS --asset "FTTTB:$TOKEN_ADMIN_ADDRESS")
echo "token_1 was wrapped succesfully with TOKEN_B_ID: $TOKEN_B_ID"
echo "--"
echo "--"

echo -n "$TOKEN_0_ID" > .soroban/token_0_id
echo -n "$TOKEN_0_ID" > .soroban/token_1_id

echo Build the SoroswapPair and SoroswapFactory contract
make build
FACTORY_WASM="factory/target/wasm32-unknown-unknown/release/soroswap_factory_contract.wasm"
PAIR_WASM="factory/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
echo "--"
echo "--"
echo Install the Pair contract WASM
echo Install a WASM file to the ledger without creating a contract instance

PAIR_WASM_HASH="$(
soroban contract deploy $ARGS \
  --wasm $PAIR_WASM
)"
echo "$PAIR_WASM_HASH" > .soroban/pair_wasm_hash
echo "SoroswapPair deployed succesfully with PAIR_WASM_HASH: $PAIR_WASM_HASH"
echo "--"
echo "--"

echo Deploy the Factory contract
FACTORY_ID="$(
  soroban contract deploy $ARGS \
    --wasm $FACTORY_WASM
)"
echo "$FACTORY_ID" > .soroban/factory_id
echo "SoroswapFactory deployed succesfully with FACTORY_ID: $FACTORY_ID"
echo "--"
echo "--"

echo "Initialize the SoroswapFactory contract"
soroban contract invoke \
  $ARGS \
  --wasm $FACTORY_WASM \
  --id $FACTORY_ID \
  -- \
  initialize \
  --setter "$TOKEN_ADMIN_ADDRESS" \
  --pair_wasm_hash "$PAIR_WASM_HASH" 

echo "--"
echo "--"

echo "Create a pair using the SoroswapFactory contract, token_a and token_b"
soroban contract invoke \
  $ARGS \
  --wasm $FACTORY_WASM \
  --id $FACTORY_ID \
  -- \
  create_pair \
  --token_a "$TOKEN_A_ID" \
  --token_b "$TOKEN_B_ID" 

echo "--"
echo "--"