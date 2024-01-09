import { TransactionBuilder } from "./utils/TransactionBuilder";
import { colors, loadContracts, loadAccounts, getRouterContractAddress} from "./utils/utils";
import { testAccount, tokenContract } from './utils/types'

/*     const network1 = "standalone"
    const rpcUri1 = "http://stellar"
    const routerContractAddress = "CDPOUV6Q2DLFOQZ3TA5ACUADUWB56HCQAAJWLVV27INLWC7MVAJXY3EU"
    const txBuilder = new TransactionBuilder(
        `${rpcUri1}:8000`,
        `${rpcUri1}:8000/soroban/rpc`,
        `${rpcUri1}:8000/friendbot?addr=`,
        routerContractAddress,
        network1
    ); */
/**
 * Removes liquidity from a Soroban pool.
 * 
 * @param txMaker The transaction builder object.
 * @returns A promise that resolves when the liquidity is successfully removed.
 */
export const removeLiquidity = async (txMaker:TransactionBuilder) => {
    
    console.log('')
    console.log(colors.purple, '=======================')
    console.log(colors.purple, '= REMOVE_LIQUIDITY.ts =')
    console.log(colors.purple, '=======================')
    console.log('')

    console.log(colors.cyan, "Fetching user account...")
    const accounts = await loadAccounts() as testAccount[]
    const user = accounts[0] as testAccount
    console.log(colors.green, "User account:", user)

    console.log(colors.cyan, "Funding account...")
    await txMaker.fundAccount(user)
    .catch((error) => { 
        console.error(colors.red, "ERROR: couldn't fund account", error) 
    })

    console.log(colors.cyan, "Fetching tokens...")
    const contracts = await loadContracts() as tokenContract[]
    const token0 = contracts[0] as tokenContract
    const token1 = contracts[1] as tokenContract

    console.log(colors.cyan, "Fetching token balances...")
    const token0FirstBalance = await txMaker.getTokenBalance({source: user, contractId: token0.contractId})
    const token1FirstBalance = await txMaker.getTokenBalance({source: user, contractId: token1.contractId})

    console.log(colors.green, `${token0.symbol}_Balance:`, token0FirstBalance)
    console.log(colors.green, `${token1.symbol}_Balance:`, token1FirstBalance)


    console.log(colors.cyan, "Removing liquidity...")
    const removeLiquidityResponse = await txMaker.removeLiquiditySoroswap({
        tokenA: token0.contractId,
        tokenB: token1.contractId,
        liquidity: "1000",
        amountAMin: "0",
        amountBMin: "0",
        source: user,
        to: user,
    }).catch((error) => { console.error(colors.red, "ERROR: couldn't remove liquidity", error) })
    if(removeLiquidityResponse?.status == "SUCCESS") {
        console.log(colors.green, `Liquidity removed successfully`)
    }

    console.log(colors.cyan, "Fetching token balances...")
    const token0LastBalance = await txMaker.getTokenBalance({source: user, contractId: token0.contractId})
    const token1LastBalance = await txMaker.getTokenBalance({source: user, contractId: token1.contractId})
    console.log(colors.green, `${token0.symbol}_Balance:`, token0LastBalance)
    console.log(colors.green, `${token1.symbol}_Balance:`, token1LastBalance)
    console.log(colors.green, '- Done. -')
}


//remove_liquidity(txBuilder)