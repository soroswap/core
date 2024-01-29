#!/bin/bash

#Define network related constants
source /workspace/scripts/network_configs.sh

#Define constants for color-coded output
RED='\033[1;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m'

#Define error handling
set -e
set -u

#Get the router contract address
ROUTER_CONTRACT=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .router_address' "$ROUTER_FILE")
echo ""
echo -e "${BLUE}Router address: $ROUTER_CONTRACT${NC}"
echo ""

# Get the token admin address
TOKEN_ADMIN_ADDRESS="$(soroban keys address token-admin)"

#Found tokenAdmin account
echo Fund tokenAdmin account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$TOKEN_ADMIN_ADDRESS"
echo ""
echo ""
echo -e "${GREEN}funded $TOKEN_ADMIN_ADDRESS${NC}"
echo ""

#Read the tokens array from the tokens.json file
TOKENS_ADRESSES_RAW+=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[].address' "$TOKENS_FILE")
TOKENS_SYMBOL_RAW+=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[].symbol' "$TOKENS_FILE")

TOKENS+=($TOKENS_ADRESSES_RAW)
TOKENS_SYMBOL+=($TOKENS_SYMBOL_RAW)

#Verify that the tokens array has the expected length
EXPECTED_LENGTH=12

if [ ${#TOKENS[@]} -le $EXPECTED_LENGTH ]; then
    echo -e "${RED}The tokens.json file must have at least ${EXPECTED_LENGTH} token addresses to run this script.${NC}"
    exit 1
fi

#Get the tokens to use on this operation
TOKEN_A=${TOKENS[6]}
TOKEN_B=${TOKENS[11]}
TOKEN_C=${TOKENS[8]}
TOKEN_D=${TOKENS[9]}
TOKEN_E=${TOKENS[7]}

#Get the token symbols
TOKEN_A_SYMBOL=${TOKENS_SYMBOL[6]}
TOKEN_B_SYMBOL=${TOKENS_SYMBOL[11]}
TOKEN_C_SYMBOL=${TOKENS_SYMBOL[8]}
TOKEN_D_SYMBOL=${TOKENS_SYMBOL[9]}
TOKEN_E_SYMBOL=${TOKENS_SYMBOL[7]}

# Create an array with all tokens
ALL_TOKENS=($TOKEN_A $TOKEN_B $TOKEN_C $TOKEN_D $TOKEN_E)
ALL_TOKENS_SYMBOL=($TOKEN_A_SYMBOL $TOKEN_B_SYMBOL $TOKEN_C_SYMBOL $TOKEN_D_SYMBOL $TOKEN_E_SYMBOL)

#Get the token A decimals 

echo -e "${BLUE}Using the following tokens:${NC}"
echo $"$TOKEN_A_SYMBOL:" $TOKEN_A
echo $"$TOKEN_B_SYMBOL:" $TOKEN_B
echo $"$TOKEN_C_SYMBOL:" $TOKEN_C
echo $"$TOKEN_D_SYMBOL:" $TOKEN_D
echo $"$TOKEN_E_SYMBOL:" $TOKEN_E

echo ""

#Mint the tokens to the token admin account
echo -e "${BLUE}Minting tokens to the token admin account${NC}"

# Loop through each token and mint it
i=0
for TOKEN in "${ALL_TOKENS[@]}"
do  
    echo -e "${BLUE}Minting ${ALL_TOKENS_SYMBOL[$i]}${NC}"
    i=$((i+1))
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
PATH_1=($TOKEN_A $TOKEN_E)
PATH_1_SYMBOL=($TOKEN_A_SYMBOL $TOKEN_E_SYMBOL)

#Path 2: XTAR - XRP - USDC
PATH_2=($TOKEN_A $TOKEN_B $TOKEN_E)
PATH_2_SYMBOL=($TOKEN_A_SYMBOL $TOKEN_B_SYMBOL $TOKEN_E_SYMBOL)

#Path 3: XTAR - ARST - EURc - USDC
PATH_3=($TOKEN_A $TOKEN_C $TOKEN_D $TOKEN_E)
PATH_3_SYMBOL=($TOKEN_A_SYMBOL $TOKEN_C_SYMBOL $TOKEN_D_SYMBOL $TOKEN_E_SYMBOL)

echo ""
echo -e "${BLUE}Adding liquidity to the following pairs${NC}"
echo ""

    echo "-------------------"
for ((i=0; i<${#PATH_1[@]}-1; i++))
do
    printf "| %6s & %-6s | \033[0m\n" ${PATH_1_SYMBOL[i]} ${PATH_1_SYMBOL[i+1]}
done
    echo "-------------------"


    echo "-------------------"
for ((i=0; i<${#PATH_2[@]}-1; i++))
do
    printf "| %6s & %-6s | \033[0m\n" ${PATH_2_SYMBOL[i]} ${PATH_2_SYMBOL[i+1]}
done
    echo "-------------------"

    echo "-------------------"
for ((i=0; i<${#PATH_3[@]}-1; i++))
do
    printf "| %6s & %-6s | \033[0m\n" ${PATH_3_SYMBOL[i]} ${PATH_3_SYMBOL[i+1]}
done
    echo "-------------------"


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
    echo -e "${YELLOW}Adding 20000 to ${PATH_1_SYMBOL[i]} and 19500 to ${PATH_1_SYMBOL[i+1]}${NC}"
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
    echo -e "${YELLOW}Adding ${AMOUNT_A_DESIRED_VALUES[i]} to ${PATH_2_SYMBOL[i]} and ${AMOUNT_B_DESIRED_VALUES[i]} to ${PATH_2_SYMBOL[i+1]}${NC}"
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
    echo -e "${YELLOW}Adding ${AMOUNT_A_DESIRED_VALUES[i]} to ${PATH_3_SYMBOL[i]} and ${AMOUNT_B_DESIRED_VALUES[i]} to ${PATH_3_SYMBOL[i+1]}${NC}"
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