import { TransactionBuilder } from "./utils/TransactionBuilder";
import { colors, getRouterContractAddress, loadAccounts, loadContracts } from "./utils/utils";
import { testAccount, tokenContract } from './utils/types'

/*     const network1 = "standalone"
    const rpcUri1 = "http://stellar"
    const routerContractAddress = "CCCGECJFVPS4KDZUEXOSFKWEVX3QKDYATMWYSQ5WU6TOGYHHN6DMVYLT"
    const txBuilder = new TransactionBuilder(
        `${rpcUri1}:8000`,
        `${rpcUri1}:8000/soroban/rpc`,
        `${rpcUri1}:8000/friendbot?addr=`,
        routerContractAddress,
        network1
    ); */
export const add_liquidity = async (txMaker:TransactionBuilder) => {
    console.log('')
    console.log(colors.purple, '====================')
    console.log(colors.purple, '= ADD_LIQUIDITY.ts =')
    console.log(colors.purple, '====================')
    console.log('')

    console.log(colors.cyan, "Fetching user account...")
    const accounts = await loadAccounts() as testAccount[]
    const user = accounts[0] as testAccount
    console.log(colors.green, "User account:", user)

    console.log(colors.cyan, "Funding account...")
    await txMaker.fundAccount(user)

    console.log(colors.cyan, "Fetching tokens...")
    const contracts = await loadContracts() as tokenContract[]
    const token0 = contracts[0] as tokenContract
    const token1 = contracts[1] as tokenContract
    console.log(colors.green, "Token 0:", token0)
    console.log(colors.green, "Token 1:", token1)   

    console.log(colors.cyan, "Fetching token balances...")
    const token0FirstBalance = await txMaker.getTokenBalance({source: user, contractId: token0.contractId})
    const token1FirstBalance = await txMaker.getTokenBalance({source: user, contractId: token1.contractId})

    console.log(colors.green, `${token0.symbol}_Balance:`, token0FirstBalance)
    console.log(colors.green, `${token1.symbol}_Balance:`, token1FirstBalance)

    console.log(colors.cyan, "Adding liquidity...")
    console.log(colors.yellow, await txMaker.getRouterContractAddress())
    
    const addLiquidityResponse = await txMaker.addLiquiditySoroswap({
        tokenA: token0.contractId,
        tokenB: token1.contractId,
        amountADesired: "10500",
        amountBDesired: "10200",
        amountAMin: "0",
        amountBMin: "0",
        source: user,
        to: user,
    })
    .catch((error) => { 
        console.error(colors.red, "ERROR: couldn't add liquidity", error) 
    })
    if(addLiquidityResponse.status == "SUCCESS") {
        console.log(colors.green, `Liquidity added successfully`)
    }

    console.log(colors.cyan, "Fetching token balances...")
    const token0LastBalance = await txMaker.getTokenBalance({source: user, contractId: token0.contractId})
    const token1LastBalance = await txMaker.getTokenBalance({source: user, contractId: token1.contractId})
    console.log(colors.green, `${token0.symbol}_Balance:`, token0LastBalance)
    console.log(colors.green, `${token1.symbol}_Balance:`, token1LastBalance)
    console.log(colors.green, '- Done. -')
}
//add_liquidity(txBuilder)