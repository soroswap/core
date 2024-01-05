import { colors, tokens, loadAccounts, saveContracts} from "./utils/utils";
import { TransactionBuilder } from "./utils/TransactionBuilder";
import { testAccount, tokenContract, token } from './utils/types'

export const mint = async (txMaker: TransactionBuilder) => {
    console.log('')
    console.log(colors.purple, '===========')
    console.log(colors.purple, '= MINT.ts =')
    console.log(colors.purple, '===========')
    console.log('')

    console.log(colors.cyan, "Fetching accounts...")
    const accounts = loadAccounts() as testAccount[]
    const user = accounts[0] as testAccount
    const issuer = accounts[1] as testAccount
    const token_0 = tokens[6] as token
    const token_1 = tokens[7] as token

    console.log(colors.cyan, "Funding accounts...")
    await txMaker.fundAccount(issuer)
    await txMaker.fundAccount(user)

    const uploadTokenContractWasmResponse = await txMaker.uploadTokenContractWasm(issuer)
    if (uploadTokenContractWasmResponse.status == "SUCCESS") {
        console.log(colors.green, "Token contract wasm uploaded successfully")
    }
    if (uploadTokenContractWasmResponse.status == "error") console.log("uploadTokenContractWasmResponse.error:", uploadTokenContractWasmResponse.error)

    console.log(colors.cyan, `Creating token contract for ${token_0.symbol}...`)
    const tokenSorobanContractId1 = await txMaker.createTokenContract(issuer)

    if (typeof tokenSorobanContractId1 === "string") {
        console.log(`Initializing token contract for ${token_0.symbol} on ${tokenSorobanContractId1}...`)
        const initializeTokenContractResponse = await txMaker.initializeTokenContract({
            contractId: tokenSorobanContractId1 ?? "",
            source: issuer,
            name: token_0.name,
            symbol: token_0.symbol,
        })
        if (initializeTokenContractResponse.status == "SUCCESS") {
            console.log(colors.green, `Token ${token_0.symbol} initialized successfully`)
        }
        const mintTokensResponse = await txMaker.mintTokens({
            contractId: tokenSorobanContractId1 ?? "",
            source: issuer,
            amount: "1000200",
            destination: user.publicKey
        }).catch((error) => { console.log(error) })
        if(mintTokensResponse.status == "SUCCESS") {
            console.log(colors.green, `Token ${token_0.symbol} minted successfully`)
        }
    }
    else {
        console.log(colors.yellow,"tokenSorobanContractId1 is not a string")
        console.log(colors.yellow,"tokenSorobanContractId1:", tokenSorobanContractId1.status)
        return
    }

    console.log(colors.cyan, `Creating token contract for ${token_1.symbol}...`)
    const tokenSorobanContractId2 = await txMaker.createTokenContract(issuer)

    if (typeof tokenSorobanContractId2 === "string") {
        console.log(`Initializing token contract for ${token_1.symbol} on ${tokenSorobanContractId2}...`)
        const initializeTokenContractResponse = await txMaker.initializeTokenContract({
            contractId: tokenSorobanContractId2 ?? "",
            source: issuer,
            name: token_1.name,
            symbol: token_1.symbol,
        })
        if (initializeTokenContractResponse.status == "SUCCESS") {
            console.log(colors.green, `Token ${token_1.symbol} initialized successfully`)
        }
        const mintTokensResponse = await txMaker.mintTokens({
            contractId: tokenSorobanContractId2 ?? "",
            source: issuer,
            amount: "2500000",
            destination: user.publicKey
        }).catch((error) => { console.log(error) })
        if(mintTokensResponse.status == "SUCCESS") {
            console.log(colors.green, `Token ${token_1.symbol} minted successfully`)
        }
    }
    else {
        console.log(colors.yellow, "tokenSorobanContractId2 is not a string")
        console.log(colors.yellow, "tokenSorobanContractId2:", tokenSorobanContractId2.status)
        return
    }

    console.log(colors.cyan, "saving contracts...")
    const tokenContracts: tokenContract[] = [
        {
            address: token_0.address,
            name: token_0.name,
            contractId: tokenSorobanContractId1,
            symbol: token_0.symbol
        },
        {
            address: token_1.address,
            name: token_1.name,
            contractId: tokenSorobanContractId2,
            symbol: token_1.symbol
        }
    ]
    saveContracts(tokenContracts)
    console.log(colors.green, "contracts:", tokenContracts)
    console.log(colors.green, "saved successfully.")
    console.log('')

    console.log(colors.cyan, "Fetching token balances...")
    const balance = await txMaker.getTokenBalance({source: user, contractId: tokenSorobanContractId1})
    const balance2 = await txMaker.getTokenBalance({source: user, contractId: tokenSorobanContractId2})
    console.log(colors.cyan, `${token_0.symbol}_balance: `, balance)
    console.log(colors.cyan, `${token_1.symbol}_balance: `, balance2)
    
    console.log(colors.green, '- Done. -')
}
    