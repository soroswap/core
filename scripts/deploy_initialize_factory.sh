# Run script from project root

set -e

NETWORK="$1"
N_TOKENS="$2"

bash /workspace/scripts/setup.sh $1


ARGS="--network $NETWORK --source token-admin"
echo "Using ARGS: $ARGS"
mkdir -p .soroban

echo "--"
echo "--"

echo Compile pair contract
cd /workspace/pair
make build
echo "--"
echo "--"


echo Compile factory contract
cd /workspace/factory
make build
echo "--"
echo "--"

FACTORY_WASM="/workspace/factory/target/wasm32-unknown-unknown/release/soroswap_factory_contract.wasm"
PAIR_WASM="/workspace/pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
TOKEN_WASM="/workspace/token/soroban_token_contract.wasm"



echo Install the Pair contract WASM
echo Install a WASM file to the ledger without creating a contract instance

PAIR_WASM_HASH="$(
soroban contract install $ARGS \
  --wasm $PAIR_WASM
)"
echo "$PAIR_WASM_HASH" > /workspace/.soroban/pair_wasm_hash
echo "SoroswapPair deployed succesfully with PAIR_WASM_HASH: $PAIR_WASM_HASH"
echo "--"
echo "--"


echo Deploy the Factory contract
FACTORY_ID="$(
  soroban contract deploy $ARGS \
    --wasm $FACTORY_WASM
)"
echo "$FACTORY_ID" > /workspace/.soroban/factory_id
echo "SoroswapFactory deployed succesfully with FACTORY_ID: $FACTORY_ID"
echo "--"
echo "--"



TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"

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

echo "{\"network\": \"$NETWORK\", \"factory\": \"$FACTORY_ID\"}" > /workspace/.soroban/factory.json


echo Factory available in  /workspace/.soroban/factory.json

cat /workspace/.soroban/factory.json

