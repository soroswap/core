import { TransactionBuilder } from "./utils/TransactionBuilder";
import { colors, tokenContracts, loadAccounts, getRouterContractAddress} from "./utils/utils";
import { testAccount, tokenContract } from './utils/types'

    const network1 = "standalone"
    const rpcUri1 = "http://stellar"
    const routerContractAddress = "CDPOUV6Q2DLFOQZ3TA5ACUADUWB56HCQAAJWLVV27INLWC7MVAJXY3EU"
    const txBuilder = new TransactionBuilder(
        `${rpcUri1}:8000`,
        `${rpcUri1}:8000/soroban/rpc`,
        `${rpcUri1}:8000/friendbot?addr=`,
        routerContractAddress,
        network1
    );
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
    const token_0 = tokenContracts[0] as tokenContract
    const token_1 = tokenContracts[1] as tokenContract 

    console.log(colors.cyan, "Fetching token balances...")
    const token_0_first_balance = await txMaker.getTokenBalance({source: user, contractId: token_0.contractId})
    const token_1_first_balance = await txMaker.getTokenBalance({source: user, contractId: token_1.contractId})

    console.log(colors.green, `${token_0.symbol}_Balance:`, token_0_first_balance)
    console.log(colors.green, `${token_1.symbol}_Balance:`, token_1_first_balance)


    console.log(colors.cyan, "Adding liquidity...")
    const addLiquidityResponse = await txMaker.addLiquiditySoroswap({
        tokenA: token_0.contractId,
        tokenB: token_1.contractId,
        amountADesired: "10500",
        amountBDesired: "10200",
        amountAMin: "0",
        amountBMin: "0",
        source: user,
        to: user,
    }).catch((error) => { console.error(colors.red, "ERROR: couldn't add liquidity", error) })
    if(addLiquidityResponse.status == "SUCCESS") {
        console.log(colors.green, `Liquidity added successfully`)
    }

    console.log(colors.cyan, "Fetching token balances...")
    const token_0_last_balance = await txMaker.getTokenBalance({source: user, contractId: token_0.contractId})
    const token_1_last_balance = await txMaker.getTokenBalance({source: user, contractId: token_1.contractId})
    console.log(colors.green, `${token_0.symbol}_Balance:`, token_0_last_balance)
    console.log(colors.green, `${token_1.symbol}_Balance:`, token_1_last_balance)
    console.log(colors.green, '- Done. -')
}

//add_liquidity(txBuilder)