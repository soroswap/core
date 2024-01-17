#!/bin/bash
NETWORK="$1"

#Define consts and necesary files
source /workspace/scripts/manual_testing/utils.sh
echo "Using deployed contracts from .soroban folder"
TOKENS_FILE="/workspace/.soroban/tokens.json"
ROUTER_FILE="/workspace/.soroban/router.json"
SOROBAN_RPC_HOST="http://stellar:8000"
FRIENDBOT_URL="$SOROBAN_RPC_HOST/friendbot"

# Define constants for color-coded output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
NC='\033[0m'

#Get the router contract address
ROUTER_CONTRACT=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .router_address' "$ROUTER_FILE")
echo ""
echo -e "${PURPLE}ROUTER_CONTRACT: $ROUTER_CONTRACT${NC}"

# Get the token admin address
TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"

#Found tokenAdmin account
echo Fund tokenAdmin account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$TOKEN_ADMIN_ADDRESS"
echo ""
echo -e "${GREEN}funded $TOKEN_ADMIN_ADDRESS${NC}"
echo ""
#Read the tokens array from the tokens.json file
TOKENS=($(jq -r '.[].tokens[].address' "$TOKENS_FILE"))

#Get the tokens to use on this operation
TOKEN_A=${TOKENS[6]} #XTAR
TOKEN_F=${TOKENS[7]} #USDC
TOKEN_B=${TOKENS[8]} #XRP
TOKEN_C=${TOKENS[9]} #ARST
TOKEN_E=${TOKENS[11]} #EURC

#Get the token A decimals 

echo -e "${BLUE}Usign the following tokens:${NC}"
echo "XTAR:" $TOKEN_A
echo "XRP:" $TOKEN_B
echo "ARST:" $TOKEN_C
echo "EURC:" $TOKEN_E
echo "XTAR:" $TOKEN_F

echo ""

# Create an array with all tokens
ALL_TOKENS=($TOKEN_A $TOKEN_B $TOKEN_C $TOKEN_E $TOKEN_F)

#Mint the tokens to the token admin account
echo -e "${BLUE}Minting tokens to the token admin account${NC}"

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
echo -e "${BLUE}Adding liquidity to the following pairs${NC}"
echo ""
echo "---------------------------------------------------------------------------------------------------------------------"

for ((i=0; i<${#PATH_1[@]}-1; i++))
do
    echo "${PATH_1[i]} and ${PATH_1[i+1]}"
done

echo "---------------------------------------------------------------------------------------------------------------------"

for ((i=0; i<${#PATH_2[@]}-1; i++))
do
    echo "${PATH_2[i]} and ${PATH_2[i+1]}"
done

echo "---------------------------------------------------------------------------------------------------------------------"

for ((i=0; i<${#PATH_3[@]}-1; i++))
do
    echo "${PATH_3[i]} and ${PATH_3[i+1]}"
done

echo "---------------------------------------------------------------------------------------------------------------------"

#Define the constants for the add_liquidity function
AMOUNT_A_MIN=0
AMOUNT_B_MIN=0
TO=$TOKEN_ADMIN_ADDRESS
DEADLINE=$(date -d "+1 hour" +%s)

#--------------------------------------------------------------------------------------------------------------------------------------
#Add liquidity to the pairs in path 1
echo ""
echo -e "${BLUE}Adding liquidity to path 1${NC}"
echo ""
for ((i=0; i<${#PATH_1[@]}-1; i++))
do
    echo -e "${YELLOW}Adding 20000 to ${TOKENS[i]} and 19500 to ${TOKENS[i+1]}${NC}"
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
echo -e "${BLUE}Adding liquidity to path 2${NC}"
echo ""
for ((i=0; i<${#PATH_2[@]}-1; i++))
do
    echo -e "${YELLOW}Adding ${AMOUNT_A_DESIRED_VALUES[i]} to ${TOKENS[i]} and ${AMOUNT_B_DESIRED_VALUES[i]} to ${TOKENS[i+1]}${NC}"
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
echo -e "${BLUE}Adding liquidity to path 3${NC}"
echo ""
for ((i=0; i<${#PATH_3[@]}-1; i++))
do
    echo -e "${YELLOW}Adding ${AMOUNT_A_DESIRED_VALUES[i]} to ${TOKENS[i]} and ${AMOUNT_B_DESIRED_VALUES[i]} to ${TOKENS[i+1]}${NC}"
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

echo ""
echo -e "${GREEN}Liquidity added successfully.${NC}"