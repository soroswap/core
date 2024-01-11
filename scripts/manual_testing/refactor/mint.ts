import { colors, loadAccounts, loadTokens, saveContracts} from "./utils/utils";
import { TransactionBuilder } from "./utils/TransactionBuilder";
import { testAccount, tokenContract, token } from './utils/types'


/* const network1 = "standalone"
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
 * Mints tokens for a given network.
 * 
 * @param txMaker The transaction builder object.
 * @param network The network to mint tokens on.
 */
export const mint = async (txMaker: TransactionBuilder, network: string) => {
    console.log('')
    console.log(colors.purple, '===========')
    console.log(colors.purple, '= MINT.ts =')
    console.log(colors.purple, '===========')
    console.log('')

    console.log(colors.cyan, "Fetching accounts...")
    const accounts = loadAccounts() as testAccount[]
    const user = accounts[0] as testAccount
    const issuer = accounts[1] as testAccount

    console.log(colors.green, "User account:", user)
    console.log(colors.green, "Issuer account:", issuer)
    
    console.log(colors.cyan, "Funding accounts...")
    await txMaker.fundAccount(issuer)
    await txMaker.fundAccount(user)

    console.log(colors.cyan, "Loading tokens...")
    const tokensFile =  loadTokens(network) as any
    const token0 = tokensFile[0].tokens[6] as token
    const token1 = tokensFile[0].tokens[7] as token

    console.log(colors.cyan, "Uploading token contract wasm...")
    const uploadTokenContractWasmResponse = await txMaker.uploadTokenContractWasm(issuer)
    if (uploadTokenContractWasmResponse.status == "SUCCESS") {
        console.log(colors.green, "Token contract wasm uploaded successfully")
    }
    if (uploadTokenContractWasmResponse.status == "error") console.log("uploadTokenContractWasmResponse.error:", uploadTokenContractWasmResponse.error)

    console.log(colors.cyan, `Creating token contract for ${token0.symbol}...`)
    const tokenSorobanContractId1 = await txMaker.createTokenContract(issuer)

    if (typeof tokenSorobanContractId1 === "string") {
        console.log(`Initializing token contract for ${token0.symbol} on ${tokenSorobanContractId1}...`)
        const initializeTokenContractResponse = await txMaker.initializeTokenContract({
            contractId: tokenSorobanContractId1 ?? "",
            source: issuer,
            name: token0.name,
            symbol: token0.symbol,
        })
        if (initializeTokenContractResponse.status == "SUCCESS") {
            console.log(colors.green, `Token ${token0.symbol} initialized successfully`)
        }
        const mintTokensResponse = await txMaker.mintTokens({
            contractId: tokenSorobanContractId1 ?? "",
            source: issuer,
            amount: "1000200",
            destination: user.publicKey
        }).catch((error) => { console.log(error) })
        if(mintTokensResponse.status == "SUCCESS") {
            console.log(colors.green, `Token ${token0.symbol} minted successfully`)
        }
    }
    else {
        console.error(colors.red,"tokenSorobanContractId1 is not a string")
        console.log(colors.yellow,"tokenSorobanContractId1:", tokenSorobanContractId1.status)
        return
    }

    console.log(colors.cyan, `Creating token contract for ${token1.symbol}...`)
    const tokenSorobanContractId2 = await txMaker.createTokenContract(issuer)

    if (typeof tokenSorobanContractId2 === "string") {
        console.log(`Initializing token contract for ${token1.symbol} on ${tokenSorobanContractId2}...`)
        const initializeTokenContractResponse = await txMaker.initializeTokenContract({
            contractId: tokenSorobanContractId2 ?? "",
            source: issuer,
            name: token1.name,
            symbol: token1.symbol,
        })
        if (initializeTokenContractResponse.status == "SUCCESS") {
            console.log(colors.green, `Token ${token1.symbol} initialized successfully`)
        }
        const mintTokensResponse = await txMaker.mintTokens({
            contractId: tokenSorobanContractId2 ?? "",
            source: issuer,
            amount: "2500000",
            destination: user.publicKey
        }).catch((error) => { console.log(error) })
        if(mintTokensResponse.status == "SUCCESS") {
            console.log(colors.green, `Token ${token1.symbol} minted successfully`)
        }
    }
    else {
        console.log(colors.red, "tokenSorobanContractId2 is not a string")
        console.log(colors.yellow, "tokenSorobanContractId2:", tokenSorobanContractId2.status)
        return
    }

    console.log(colors.cyan, "saving contracts...")
    const tokenContracts: tokenContract[] = [
        {
            address: token0.address,
            name: token0.name,
            contractId: tokenSorobanContractId1,
            symbol: token0.symbol,
        
        },
        {
            address: token1.address,
            name: token1.name,
            contractId: tokenSorobanContractId2,
            symbol: token1.symbol,
        
        }
    ]
    console.log(saveContracts(tokenContracts))
    console.log(colors.green, "contracts:", tokenContracts)
    console.log(colors.green, "saved successfully.")
    console.log('')

    console.log(colors.cyan, "Fetching token balances...")
    const balance = await txMaker.getTokenBalance({source: user, contractId: tokenSorobanContractId1})
    const balance2 = await txMaker.getTokenBalance({source: user, contractId: tokenSorobanContractId2})
    console.log(colors.cyan, `${token0.symbol}_balance: `, balance)
    console.log(colors.cyan, `${token1.symbol}_balance: `, balance2)
    console.log(colors.green, '- Done. -')
}

//mint(txBuilder)