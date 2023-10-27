# Example usage:
#
# bash scripts/create_pair.sh standalone 21d393c5304fd3228da2fc84f03421c783e10e16da4ec70e873430f31392e2a3 aea545c2d0c818123261c60ed7126a21d3855fb17aae12e3332076cd29069555


NETWORK="$1"
TOKEN_A_ID="$2"
TOKEN_B_ID="$3"

TOKEN_A_ADDRESS="$(node /workspace/scripts/address_workaround.js $TOKEN_A_ID)"
TOKEN_B_ADDRESS="$(node /workspace/scripts/address_workaround.js $TOKEN_B_ID)"


ARGS="--network $NETWORK --source token-admin"
FACTORY_WASM="/workspace/contracts/factory/target/wasm32-unknown-unknown/release/soroswap_factory.wasm"

# Extract FACTORY_ID from JSON file
FACTORY_ID=$(jq -r --arg network "$NETWORK" '.[] | select(.network == $network) | .factory_id' /workspace/.soroban/factory.json)
echo $FACTORY_ID
echo "Create a pair using the SoroswapFactory contract, token_a:'$TOKEN_A_ADDRESS' and token_b:'$TOKEN_B_ADDRESS'"
echo "create pair"

PAIR_ID=$(soroban contract invoke \
  $ARGS \
  --wasm $FACTORY_WASM \
  --id $FACTORY_ID \
  -- \
  create_pair \
  --token_a "$TOKEN_A_ADDRESS" \
  --token_b "$TOKEN_B_ADDRESS" )
# Assuming the variable PAIR_ID contains the returned ID with apostrophes
PAIR_ID=$(echo $PAIR_ID | tr -d '"')
echo Pair created succesfully with PAIR_ID=$PAIR_ID
echo $PAIR_ID > .soroban/pair_id

