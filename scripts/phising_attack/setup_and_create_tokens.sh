# Enable the 'exit immediately' shell option
set -e

# Accept command-line arguments
NETWORK="$1"

# Run the setup script
bash /workspace/scripts/phising_attack/setup.sh $NETWORK

# Get the token admin address
TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"

## TOKEN 0
