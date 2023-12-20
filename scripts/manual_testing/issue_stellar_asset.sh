NETWORK="$1"
USER_SECRET=$(cat .soroban/user_secret)
ASSET_DEPLOYER_SECRET=$(cat .soroban/asset_deployer_secret)
node /workspace/scripts/manual_testing/deployStellarAsset.js $NETWORK $USER_SECRET $ASSET_DEPLOYER_SECRET