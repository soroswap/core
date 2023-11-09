#!/bin/bash

# Setup
TOKEN_ADMIN_SECRET="SCLPVGNVME5OJKOMPRPKDQSMKWC52RLK63T5IGMMCA52KL64WDK3MZZD"
LOCAL_USER_ADDRESS="$(soroban config identity address token-admin)"
echo "LOCAL_USER_ADDRESS: $LOCAL_USER_ADDRESS"

NETWORK="testnet"

TOKEN_WASM="/workspace/contracts/token/soroban_token_contract.wasm"
PAIR_WASM="/workspace/contracts/pair/target/wasm32-unknown-unknown/release/soroswap_pair.wasm"
ROUTER_WASM="/workspace/contracts/router/target/wasm32-unknown-unknown/release/soroswap_router.wasm"
FACTORY_WASM="/workspace/contracts/factory/target/wasm32-unknown-unknown/release/soroswap_factory.wasm"

XLM_CONTRACT_ID="CACEEMMWGVDM6RZD7ZL6Z75Y32MI5ZWBGVTXTSCLCXXOW57OD63KKDTD"
USDC_CONTRACT_ID="CDMOQLZXRDQMQBJDKFNPE3ORBUXZ7PY6JMN2XFL4TVASPFK4BG65TKQP"

ROUTER_CONTRACT_ID="CCPY4Q24CWFCNZYEGUZ3RFHQS4PTDX3LTJRVEUNXGIST46RMSO4ENWF3"

# WALLET_TO_CHECK="GCHR5WWPDFF3U3HP2NA6TI6FCQPYEWS3UOPIPJKZLAAFM57CEG4ZYBWP"
WALLET_TO_CHECK="$LOCAL_USER_ADDRESS"

# Mint test tokens
CONTRACT_IDS=("$XLM_CONTRACT_ID" "$USDC_CONTRACT_ID")

## Loop through each contract ID and mint tokens
# for CONTRACT_ID in "${CONTRACT_IDS[@]}"; do
#     echo "Minting tokens for contract $CONTRACT_ID..."

#     MINT_RESULT="$(soroban contract invoke \
#         --network $NETWORK \
#         --source-account $TOKEN_ADMIN_SECRET \
#         --wasm $TOKEN_WASM \
#         --id $CONTRACT_ID \
#         -- \
#         mint \
#         --to "$LOCAL_USER_ADDRESS" \
#         --amount "25000000000000")"

#     echo "Mint result: $MINT_RESULT"
# done

# ## Check balances
# # Loop through each contract ID and get token balances
# for CONTRACT_ID in "${CONTRACT_IDS[@]}"; do
#     echo "Checking token balance for contract $CONTRACT_ID..."

#     soroban contract invoke \
#         --network $NETWORK \
#         --source-account $TOKEN_ADMIN_SECRET \
#         --wasm $TOKEN_WASM \
#         --id $CONTRACT_ID \
#         -- \
#         balance \
#         --id "$LOCAL_USER_ADDRESS" 
# done


# Add liquidity
    # fn add_liquidity(
    #     e: Env,
    #     token_a: Address,
    #     token_b: Address,
    #     amount_a_desired: i128,
    #     amount_b_desired: i128,
    #     amount_a_min: i128,
    #     amount_b_min: i128,
    #     to: Address,
    #     deadline: u64,
    # ) -> (i128, i128, i128);
echo "Adding liquidity"
# soroban contract invoke \
#         --network $NETWORK \
#         --source token-admin \
#         --wasm $ROUTER_WASM \
#         --id $ROUTER_CONTRACT_ID \
#         -- \
#         add_liquidity \
#         --token_a "$XLM_CONTRACT_ID" \
#         --token_b "$USDC_CONTRACT_ID" \
#         --amount_a_desired 10000000000000\
#         --amount_b_desired 1000000000000\
#         --amount_a_min 1000000000000\
#         --amount_b_min 100000000000\
#         --to $LOCAL_USER_ADDRESS\
#         --deadline 1699721331

## Reading some data
# soroban contract invoke \
#         --network $NETWORK \
#         --source token-admin \
#         --wasm $ROUTER_WASM \
#         --id $ROUTER_CONTRACT_ID \
#         -- \
#         get_factory

    # fn swap_exact_tokens_for_tokens(
    #     e: Env,
    #     amount_in: i128,
    #     amount_out_min: i128,
    #     path: Vec<Address>,
    #     to: Address,
    #     deadline: u64,
    # ) -> Vec<i128>;

PATH_ARRAY="[$XLM_CONTRACT_ID,$USDC_CONTRACT_ID]"

soroban contract invoke \
        --network $NETWORK \
        --source token-admin \
        --wasm $ROUTER_WASM \
        --id $ROUTER_CONTRACT_ID \
        -- \
        swap_exact_tokens_for_tokens \
        --amount_in 0 \
        --amount_out_min 0 \
        --path "CACEEMMWGVDM6RZD7ZL6Z75Y32MI5ZWBGVTXTSCLCXXOW57OD63KKDTD,CDMOQLZXRDQMQBJDKFNPE3ORBUXZ7PY6JMN2XFL4TVASPFK4BG65TKQP" \
        --to $LOCAL_USER_ADDRESS \
        --deadline 1699721331
