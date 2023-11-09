#!/bin/bash

TOKEN_ADMIN_ADDRESS="$(soroban config identity address token-admin)"
echo "TOKEN_ADMIN_ADDRESS: $TOKEN_ADMIN_ADDRESS"


XLM_CONTRACT_ID="CDHRSU6TM63VOL4ZZXQAQWTGLHORL2GCA2CEXJSKNMQTNNQLCP2N2B7V"
USDC_CONTRACT_ID="CDA72YPUOL3HCGARK5F33M5D475SBG23GTT73J5RU742UN3AUGF5CWON"

WALLET_TO_CHECK="GCHR5WWPDFF3U3HP2NA6TI6FCQPYEWS3UOPIPJKZLAAFM57CEG4ZYBWP"
PAIR_ADDRESS="CBMHTKDE3NYS7R5V5KQWQS5QSPE5GP3TPONWCPLVANFJWZCT3HYKWDG4"

TOKEN_WASM="/workspace/contracts/token/soroban_token_contract.wasm"
PAIR_WASM="/workspace/contracts/pair/target/wasm32-unknown-unknown/release/soroswap_pair.wasm"

NETWORK="standalone"

echo "XLM Balance of User: ${WALLET_TO_CHECK}"
soroban contract invoke \
  --network $NETWORK \
  --source token-admin \
  --wasm $TOKEN_WASM \
  --id $XLM_CONTRACT_ID \
  -- \
  balance \
  --id "$WALLET_TO_CHECK"   

# Balance of Pair address
echo "XLM Balance of Contract: ${PAIR_ADDRESS}"
soroban contract invoke \
  --network $NETWORK \
  --source token-admin \
  --wasm $TOKEN_WASM \
  --id $XLM_CONTRACT_ID \
  -- \
  balance \
  --id "$PAIR_ADDRESS"   

echo "USDC Balance of User: ${WALLET_TO_CHECK}"
soroban contract invoke \
  --network $NETWORK \
  --source token-admin \
  --wasm $TOKEN_WASM \
  --id $USDC_CONTRACT_ID \
  -- \
  balance \
  --id "$WALLET_TO_CHECK"   

# Balance of Pair address
echo "USDC Balance of Contract: ${PAIR_ADDRESS}"
soroban contract invoke \
  --network $NETWORK \
  --source token-admin \
  --wasm $TOKEN_WASM \
  --id $USDC_CONTRACT_ID \
  -- \
  balance \
  --id "$PAIR_ADDRESS"   

echo "Reserves of Pair Contract:  ${PAIR_ADDRESS}"
soroban contract invoke \
  --network $NETWORK \
  --source token-admin \
  --wasm $PAIR_WASM \
  --id $PAIR_ADDRESS \
  -- \
  get_reserves
