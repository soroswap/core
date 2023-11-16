soroban config identity generate user
USER_SECRET=$(soroban config identity show user)
USER_PUBLIC="$(soroban config identity address user)"

echo USER_SECRET: $USER_SECRET
echo USER_PUBLIC: $USER_PUBLIC

echo "$USER_SECRET" > .soroban/user_secret
echo "$USER_PUBLIC" > .soroban/user_public

NEW_KEYS_OBJECT="{ \"user_public\": \"$USER_PUBLIC\", \"user_secret\": \"$USER_SECRET\" }"

KEYS_FILE="/workspace/.soroban/user_keys.json"
touch $KEYS_FILE
echo "$NEW_KEYS_OBJECT" > "$KEYS_FILE"
CURRENT_KEYS_JSON=$(cat $KEYS_FILE)
echo "Created a new user_keys.json file: $CURRENT_KEYS_JSON"

