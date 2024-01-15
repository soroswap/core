import * as sdk from 'stellar-sdk'
import { colors, loadContracts, loadAccounts } from "../manual_testing/refactor/utils/utils";
import { TransactionBuilder } from "../manual_testing/refactor/utils/TransactionBuilder";
import { testAccount, tokenContract } from '../manual_testing/refactor/utils/types'
const network = "standalone"
const rpcUri = "http://stellar"
const routerContractAddress = "CA45TW3JBZLBOD672XJ44MW3Y47LYTBK4JJHTX64BKDLEQXDLZFNPJAD"
const txBuilder = new TransactionBuilder(
    `${rpcUri}:8000`,
    `${rpcUri}:8000/soroban/rpc`,
    `${rpcUri}:8000/friendbot?addr=`,
    routerContractAddress,
    network
);
export const swap = async (txMaker: TransactionBuilder) => {
    console.log('')
    console.log(colors.purple, '===========')
    console.log(colors.purple, '= SWAP.ts =')
    console.log(colors.purple, '===========')
    console.log('')

    console.log(colors.cyan, "HELLO WORLD!")

    const contracts = await loadContracts() as tokenContract[]

    const token0 = contracts[0] as tokenContract
    const token1 = contracts[1] as tokenContract
    
    const accounts = loadAccounts() as testAccount[]

    /* console.log(colors.cyan, "Fetching accounts...")
    const accounts = await loadAccounts() as testAccount[]
    const user = accounts[0] as testAccount
    console.log(colors.green, `Loaded user`)
    console.log(user)

    console.log(colors.cyan, "Funding account...")
    await txMaker.fundAccount(user)
    
    console.log(colors.cyan, "Fetching tokens...")
    const contracts = await loadContracts() as tokenContract[]
    const token0 = contracts[0] as tokenContract
    const token1 = contracts[1] as tokenContract
    console.log(colors.green, `Loaded tokens: ${token0.symbol} and ${token1.symbol}`)

    console.log(colors.cyan, "Fetching token balances...")
    const token0FirstBalance = await txMaker.getTokenBalance({source: user, contractId: token0.contractId})
    const token1FirstBalance = await txMaker.getTokenBalance({source: user, contractId: token1.contractId})

    console.log(colors.green, `${token0.symbol}_Balance:`, token0FirstBalance)
    console.log(colors.green, `${token1.symbol}_Balance:`, token1FirstBalance)

    console.log(colors.cyan, "Swapping tokens...")
    const swapTokensResponse = await txMaker.swapExactTokensForTokensSoroswap({
        source: user,
        amountIn: "100000",
        amountOutMin: "0",
        path: [token0.contractId, token1.contractId],
        to: user,
    }).catch((error) => { console.error(colors.red, "ERROR: couldn't swap tokens", error) })
    if(swapTokensResponse?.status == "SUCCESS") {
        console.log(colors.green, `Tokens swapped successfully`)
    }

    console.log(colors.cyan, "Fetching new token balances...")
    const token0LastBalance = await txMaker.getTokenBalance({source: user, contractId: token0.contractId})
    const token1LastBalance = await txMaker.getTokenBalance({source: user, contractId: token1.contractId})
    console.log(colors.green, `${token0.symbol}_Balance:`, token0LastBalance)
    console.log(colors.green, `${token1.symbol}_Balance:`, token1LastBalance)
    console.log('')
    console.log(colors.green, '- Done. -') */
}
swap(txBuilder)