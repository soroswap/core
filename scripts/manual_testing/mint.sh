NETWORK="$1"

source /workspace/scripts/manual_testing/utils.sh

display_colored_text PURPLE " === MINT.SH === "

#Define network related constants
source /workspace/scripts/network_configs.sh

echo We are going to mint 2 test tokens
echo We are going to use the admin private key and the user public key

ADMIN_SECRET=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .admin_secret' $ADMIN_KEYS_FILE)
USER_PUBLIC=$(cat .soroban/user_public)

echo ADMIN_SECRET: $ADMIN_SECRET
echo USER_PUBLIC: $USER_PUBLIC

TOKEN_0_ADDRESS=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[6].address' "$TOKENS_FILE")
TOKEN_1_ADDRESS=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[7].address' "$TOKENS_FILE")

TOKEN_0_SYMBOL=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[6].symbol' "$TOKENS_FILE")
TOKEN_1_SYMBOL=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[7].symbol' "$TOKENS_FILE")

echo "..."
echo "..."
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
echo We will mint you 25,000,000 units ..plus 7 decimals.. of each token 
echo "..."
TOKEN_WASM="/workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm"
echo "Minting TOKEN_0:"
soroban contract invoke \
  --network $NETWORK --source token-admin \
  --id $TOKEN_0_ADDRESS \
  -- \
  mint \
  --to "$USER_PUBLIC" \
  --amount 250000000000000 

echo "..."
echo "Minting TOKEN_1:"
soroban contract invoke \
  --network $NETWORK --source token-admin \
  --id $TOKEN_1_ADDRESS \
  -- \
  mint \
  --to "$USER_PUBLIC" \
  --amount 250000000000000 

  # Here we will deploy Stellar assets and Tokens from another account

  # Fund the asset_deployer account
  ASSET_DEPLOYER_PUBLIC="$(soroban keys address asset_deployer)"
  echo "Funding the asset_deployer account: $ASSET_DEPLOYER_PUBLIC"
  curl  -X POST "$FRIENDBOT_URL?addr=$ASSET_DEPLOYER_PUBLIC"
  echo "Funding the user account:  $USER_PUBLIC"
  curl  -X POST "$FRIENDBOT_URL?addr=$USER_PUBLIC"


  SOROBAN_TOKEN_A_ID="$(
  soroban contract deploy --network $NETWORK --source asset_deployer \
    --wasm $TOKEN_WASM
  )"
  echo SOROBAN_TOKEN_A_ID: $SOROBAN_TOKEN_A_ID
  SOROBAN_TOKEN_B_ID="$(STELLAR_ASSET_CONTRACT_ID="$(
  soroban lab token id \
  --network standalone \
  --source asset_deployer \
  --asset "AstroDollar:$ASSET_DEPLOYER_PUBLIC"
)"
  soroban contract deploy --network $NETWORK --source asset_deployer \
    --wasm $TOKEN_WASM
  )"
  echo SOROBAN_TOKEN_B_ID: $SOROBAN_TOKEN_B_ID

mkdir -p $SOROBAN_TOKENS_FOLDER
echo "Created folder: $SOROBAN_TOKENS_FOLDER"

echo "Saving token addresses to $SOROBAN_TOKENS_FOLDER"
echo $SOROBAN_TOKEN_A_ID > $SOROBAN_TOKENS_FOLDER/token_a_id
echo $SOROBAN_TOKEN_B_ID > $SOROBAN_TOKENS_FOLDER/token_b_id

echo "Initializing tokens"
soroban contract invoke \
  --network $NETWORK --source asset_deployer \
  --id $SOROBAN_TOKEN_A_ID \
  -- \
  initialize \
  --name "Soroban Token A" \
  --symbol "SOROBA" \
  --decimal 7 \
  --admin "$ASSET_DEPLOYER_PUBLIC"

soroban contract invoke \
  --network $NETWORK --source asset_deployer \
  --id $SOROBAN_TOKEN_B_ID \
  -- \
  initialize \
  --name "Soroban Token B" \
  --symbol "SOROBB" \
  --decimal 7 \
  --admin "$ASSET_DEPLOYER_PUBLIC"

echo "Minting tokens"
soroban contract invoke \
  --network $NETWORK --source asset_deployer \
  --id $SOROBAN_TOKEN_A_ID \
  -- \
  mint \
  --to "$USER_PUBLIC" \
  --amount 250000000000000

soroban contract invoke \
  --network $NETWORK --source asset_deployer \
  --id $SOROBAN_TOKEN_B_ID \
  -- \
  mint \
  --to "$USER_PUBLIC" \
  --amount 250000000000000

USER_SECRET=$(cat .soroban/user_secret)
ASSET_DEPLOYER_SECRET=$(cat .soroban/asset_deployer_secret)

#Print balances for every token and verify users balance
node /workspace/scripts/manual_testing/deployStellarAsset.js $NETWORK $USER_SECRET $ASSET_DEPLOYER_SECRET

TOKEN_0_BALANCE_OF_USER="$(soroban contract invoke \
  --network $NETWORK \
  --source asset_deployer \
  --id $TOKEN_0_ADDRESS \
  -- \
  balance \
  --id "$USER_PUBLIC"   )"

TOKEN_1_BALANCE_OF_USER="$(soroban contract invoke \
  --network $NETWORK \
  --source asset_deployer \
  --id $TOKEN_1_ADDRESS \
  -- \
  balance \
  --id "$USER_PUBLIC"   )"

printTokensTable $TOKEN_0_SYMBOL $TOKEN_0_ADDRESS $TOKEN_1_SYMBOL $TOKEN_1_ADDRESS