NETWORK="$1"

source /workspace/scripts/manual_testing/utils.sh

display_colored_text PURPLE " === GENERATE_USER.SH === "

soroban keys generate --no-fund --network $NETWORK user
USER_SECRET=$(soroban keys show user)
USER_PUBLIC="$(soroban keys address user)"

display_colored_text GREEN " Generated user keys "
echo USER_SECRET: $USER_SECRET
echo USER_PUBLIC: $USER_PUBLIC


echo "$USER_SECRET" > .soroban/user_secret
echo "$USER_PUBLIC" > .soroban/user_public

NEW_KEYS_OBJECT="{ \"user_public\": \"$USER_PUBLIC\", \"user_secret\": \"$USER_SECRET\" }"

# KEYS_FILE="/workspace/public/user_keys.json"
# touch $KEYS_FILE
# echo "$NEW_KEYS_OBJECT" > "$KEYS_FILE"
# CURRENT_KEYS_JSON=$(cat $KEYS_FILE)
# echo "Created a new user_keys.json file: $CURRENT_KEYS_JSON"

soroban keys generate --no-fund --network $NETWORK asset_deployer
ASSET_DEPLOYER_SECRET=$(soroban keys show asset_deployer)
ASSET_DEPLOYER_PUBLIC=$(soroban keys address asset_deployer)

echo "$ASSET_DEPLOYER_SECRET" > .soroban/asset_deployer_secret
echo "$ASSET_DEPLOYER_PUBLIC" > .soroban/asset_deployer_public


echo "   "
echo "   "

