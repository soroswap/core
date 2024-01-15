#!/bin/bash
NETWORK="$1"

#Define consts and necesary files
source /workspace/scripts/manual_testing/utils.sh
echo "Using deployed contracts from .soroban folder"
TOKENS_FILE="/workspace/.soroban/tokens.json"
ROUTER_FILE="/workspace/.soroban/router.json"
SOROBAN_TOKENS_FOLDER="/workspace/.soroban/soroban_tokens/"
PAIRS_FILE="/workspace/.soroban/pairs.json"
ROUTER_WASM="/workspace/contracts/router/target/wasm32-unknown-unknown/release/soroswap_router.optimized.wasm"
SOROBAN_RPC_HOST="http://stellar:8000"
FRIENDBOT_URL="$SOROBAN_RPC_HOST/friendbot"

#Get the router contract address
ROUTER_CONTRACT=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .router_address' "$ROUTER_FILE")
echo ""
display_colored_text GREEN "ROUTER_CONTRACT: $ROUTER_CONTRACT"

# Get the token admin address
TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"

#Found tokenAdmin account
echo Fund tokenAdmin account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$TOKEN_ADMIN_ADDRESS"
echo ""
display_colored_text BLUE "funded $TOKEN_ADMIN_ADDRESS"
echo ""
#Read the tokens array from the tokens.json file
TOKENS=($(jq -r '.[].tokens[].address' "$TOKENS_FILE"))

#Get the tokens to use on this operation
TOKEN_A=${TOKENS[6]} #XTAR
TOKEN_F=${TOKENS[7]} #USDC

TOKEN_B=${TOKENS[8]} #XRP
TOKEN_C=${TOKENS[9]} #ARST
TOKEN_D=${TOKENS[10]} #AQUA
TOKEN_E=${TOKENS[11]} #EURC

#Get the token A decimals 

display_colored_text BLUE "TOKENS:"
echo "XTAR:" $TOKEN_A
echo "XRP:" $TOKEN_B
echo "ARST:" $TOKEN_C
echo "AQUA:" $TOKEN_D
echo "EURC:" $TOKEN_E
echo "XTAR:" $TOKEN_F

echo ""


#Acceder a un valor espefifico del array
#"${TOKENS[6]}"

# Create an array with all tokens
ALL_TOKENS=($TOKEN_A $TOKEN_B $TOKEN_C $TOKEN_D $TOKEN_E $TOKEN_F)

#Mint the tokens to the token admin account
display_colored_text BLUE "Minting tokens to the token admin account"

# Loop through each token and mint it
for TOKEN in "${ALL_TOKENS[@]}"
do
    soroban contract invoke \
  --network $NETWORK --source token-admin \
  --id $TOKEN \
  -- \
  mint \
  --to token-admin \
  --amount 250000000
done

#Create different paths to add liquidity

#Path 1: XTAR - USDC
PATH_1=($TOKEN_A $TOKEN_F)

#Path 2: XTAR - XRP - USDC
PATH_2=($TOKEN_A $TOKEN_B $TOKEN_F)

#Path 3: XTAR - ARST - EURc - USDC
PATH_3=($TOKEN_A $TOKEN_C $TOKEN_E $TOKEN_F)

echo ""
display_colored_text BLUE "PATHS:"
echo ""
display_colored_text YELLOW "---------------------------------------------------------------------------------------------------------------------"

for ((i=0; i<${#PATH_1[@]}-1; i++))
do
    echo "${PATH_1[i]} and ${PATH_1[i+1]}"
done

display_colored_text YELLOW "---------------------------------------------------------------------------------------------------------------------"

for ((i=0; i<${#PATH_2[@]}-1; i++))
do
    echo "${PATH_2[i]} and ${PATH_2[i+1]}"
done

display_colored_text YELLOW "---------------------------------------------------------------------------------------------------------------------"

for ((i=0; i<${#PATH_3[@]}-1; i++))
do
    echo "${PATH_3[i]} and ${PATH_3[i+1]}"
done

display_colored_text YELLOW "---------------------------------------------------------------------------------------------------------------------"

#Define the constants for the add_liquidity function
AMOUNT_A_MIN=0
AMOUNT_B_MIN=0
TO=$TOKEN_ADMIN_ADDRESS
DEADLINE=$(date -d "+1 hour" +%s)

#--------------------------------------------------------------------------------------------------------------------------------------
#Add liquidity to the pairs in path 1
echo ""
display_colored_text BLUE "Adding liquidity to path 1"
echo ""
for ((i=0; i<${#PATH_1[@]}-1; i++))
do
    display_colored_text GREEN "Adding 20000 to ${TOKENS[i]} and 19500 ${TOKENS[i+1]}"
    soroban contract invoke \
        --network $NETWORK \
        --source token-admin \
        --id $ROUTER_CONTRACT \
        -- \
        add_liquidity \
        --token_a "${PATH_1[i]}" \
        --token_b "${PATH_1[i+1]}" \
        --amount_a_desired 20000 \
        --amount_b_desired 19500 \
        --amount_a_min "$AMOUNT_A_MIN" \
        --amount_b_min "$AMOUNT_B_MIN" \
        --to "$TO" \
        --deadline "$DEADLINE" 
done

#--------------------------------------------------------------------------------------------------------------------------------------
#Define the desired values for the add_liquidity function on path 2
AMOUNT_A_DESIRED_VALUES=(75000 70000)
AMOUNT_B_DESIRED_VALUES=(74000 70000)

#Add liquidity to the pairs in path 2
echo ""
display_colored_text BLUE "Adding liquidity to path 2"
echo ""
for ((i=0; i<${#PATH_2[@]}-1; i++))
do
    display_colored_text GREEN "Adding ${AMOUNT_A_DESIRED_VALUES[i]} to ${TOKENS[i]} and ${AMOUNT_B_DESIRED_VALUES[i]} ${TOKENS[i+1]}"
    soroban contract invoke \
        --network $NETWORK \
        --source token-admin \
        --id $ROUTER_CONTRACT \
        -- \
        add_liquidity \
        --token_a "${PATH_2[i]}" \
        --token_b "${PATH_2[i+1]}" \
        --amount_a_desired "${AMOUNT_A_DESIRED_VALUES[i]}" \
        --amount_b_desired "${AMOUNT_B_DESIRED_VALUES[i]}" \
        --amount_a_min "$AMOUNT_A_MIN" \
        --amount_b_min "$AMOUNT_B_MIN" \
        --to "$TO" \
        --deadline "$DEADLINE" 
done

#--------------------------------------------------------------------------------------------------------------------------------------
#Define the desired values for the add_liquidity function on path 3
AMOUNT_A_DESIRED_VALUES=(60000 55000 80000)
AMOUNT_B_DESIRED_VALUES=(61000 53500 80500)
#Add liquidity to the pairs in path 3
echo ""
display_colored_text BLUE "Adding liquidity to path 3"
echo ""
for ((i=0; i<${#PATH_3[@]}-1; i++))
do
    display_colored_text GREEN "Adding ${AMOUNT_A_DESIRED_VALUES[i]} to ${TOKENS[i]} and ${AMOUNT_B_DESIRED_VALUES[i]} ${TOKENS[i+1]}"
    soroban contract invoke \
        --network $NETWORK \
        --source token-admin \
        --id $ROUTER_CONTRACT \
        -- \
        add_liquidity \
        --token_a "${PATH_3[i]}" \
        --token_b "${PATH_3[i+1]}" \
        --amount_a_desired "${AMOUNT_A_DESIRED_VALUES[i]}" \
        --amount_b_desired "${AMOUNT_B_DESIRED_VALUES[i]}" \
        --amount_a_min "$AMOUNT_A_MIN" \
        --amount_b_min "$AMOUNT_B_MIN" \
        --to "$TO" \
        --deadline "$DEADLINE"
done