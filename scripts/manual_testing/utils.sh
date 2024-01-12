#!/bin/bash
# Declare colors
YELLOW="\033[43;30m"
GREEN="\033[42;30m"
RED="\033[41;30m"
BLUE="\033[44;30m"
PURPLE="\033[48;5;54m"
RESET="\033[0m"

# Function to display colored text
display_colored_text() {
    local color="$1"
    local text="$2"
    echo -e "\033[1m${!color}${text}${RESET}"
}

NETWORK="$1"
USER_PUBLIC=$(cat /workspace/.soroban/user_public)
TOKENS_FILE="/workspace/.soroban/tokens.json"
USER_SECRET=$(cat /workspace/.soroban/user_secret)


# Function to get the balance of a token for a given token address
# Parameters:
#   - tokenAddress: The address of the token
# Returns:
#   - The token balance as an integer
getTokenBalance() {
    local tokenAddress="$1"
    local TOKEN_BALANCE="$(soroban contract invoke \
    --network $NETWORK --source $USER_SECRET \
    --id $tokenAddress \
    -- \
    balance \
    --id "$USER_PUBLIC"   )"
    TOKEN_BALANCE_INT=${TOKEN_BALANCE//[!0-9]/}
    echo $TOKEN_BALANCE_INT
}

TOKEN_0_ADDRESS=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[2].address' "$TOKENS_FILE")
TOKEN_0_SYMBOL=$(jq -r --arg NETWORK "$NETWORK" '.[] | select(.network == $NETWORK) | .tokens[2].symbol' "$TOKENS_FILE")

# Function to print a table of tokens and their balances
# Arguments:
#   - List of token names
#   - List of token balances
#to make it work, we need to pass the token symbol and the token address in that specific order
#printTokensTable $TOKEN_0_SYMBOL $TOKEN_0_ADDRESS $TOKEN_1_SYMBOL $TOKEN_1_ADDRESS ...
printTokensTable() {
    echo ""
    if (( $# % 2 == 0 )); then
        local i=0
        local balance=""
        local symbol=""
        display_colored_text BLUE " --------------------------------------- "
        display_colored_text BLUE " | Token Name | Balance                | "
        display_colored_text BLUE " --------------------------------------- "

        for param in "$@"; do
            if ((i % 2 == 0)); then
                symbol=$param
            elif ((i % 2 == 1)); then
                balance=$(getTokenBalance $param)
                printf "\033[1;44;30m | %-10s | %-22s | \033[0m\n" "$symbol" "$balance"
                display_colored_text BLUE " --------------------------------------- "
            fi
            ((i++))
        done
    fi
    echo ""
}

# Function: printTokensBalanceDiff
# Description: Prints the difference in token balances before and after a specified operation.
# Parameters:
#   - operation: The operation performed ('add_liquidity', 'remove_liquidity', 'swap').
#   - token_symbol: The symbol of the token.
#   - token_address: The address of the token.
#   - token_balance_before: The token balance before the operation.
# Returns: None

printTokensBalanceDiff(){
        local count=0
        local token_symbol=""
        local token_balance_before=""
        local token_balance_after=""
        local delta_balance=""
        echo ""
        display_colored_text BLUE " ------------------------------------------------------------------------------------- "
        printf "\033[1;44;30m |  %-10s |  %-10s  |  %-15s  | %-27s | \033[0m\n" "Token Name" "Initial Balance" "Actual Balance" "Diff after $1"
        display_colored_text BLUE " ------------------------------------------------------------------------------------- "
        for ((i=2; i<=$#; i++)); do
            case $count in
                0)
                        token_symbol="${!i}"
                        ;;
                1)
                        token="${!i}"
                        token_balance_after=$(getTokenBalance "$token")
                        ;;
                2)
                        token_balance_before="${!i}"
                        ;;
            esac
            ((count++))
            if ((count % 3 == 0)); then
                delta_balance=$(($token_balance_after - $token_balance_before))
                if (($delta_balance > 1)); then
                        delta_balance="+$delta_balance"
                fi
                printf "\033[1;44;30m | %-10s  | %-16s  |  %-15s  | %-27s | \033[0m\n" "$token_symbol" "$token_balance_before" "$token_balance_after" "$delta_balance"
                display_colored_text BLUE " ------------------------------------------------------------------------------------- " 
                count=0
            fi
        done
}