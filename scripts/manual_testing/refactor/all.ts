import {getRouterContractAddress, colors} from "./utils/utils";
import { TransactionBuilder } from "./utils/TransactionBuilder";
import {
    generateUsers,
    mint,
    add_liquidity,
    swap,
    remove_liquidity
} from "./index"

// Get the arguments passed to the script
const args = process.argv.slice(2);

// Access the individual arguments
/**
 * Represents the network used for testing.
 * @type {string}
 */
const network = args[0];

const mode = args[1];

console.log("network:", network)
console.log("running on:", mode)
let txMaker: TransactionBuilder;



const testAll = async (network: string, mode: string) => {
    let apiUri: string;
    let rpcUri: string;
    let routerContractAddress: string;
    switch (network) {
        case "standalone":
            console.log(colors.green, 'Using standalone network')
            apiUri = "http://soroswapCoreApi:8010"
            rpcUri = "http://stellar"
            routerContractAddress = await getRouterContractAddress(apiUri, network)

            txMaker = new TransactionBuilder(
                `${rpcUri}:8000`,
                `${rpcUri}:8000/soroban/rpc`,
                `${rpcUri}:8000/friendbot?addr=`,
                routerContractAddress,
                network
            );
        break;
        case "testnet":
            console.log(colors.green, 'Using testnet network')
            apiUri = "https://horizon-testnet.stellar.org"
            rpcUri = "http://stellar"
            routerContractAddress = await getRouterContractAddress(apiUri, network)
            txMaker = new TransactionBuilder(
                `${rpcUri}`,
                `${rpcUri}/soroban/rpc`,
                "https://friendbot.stellar.org/?addr=",
                routerContractAddress,
                network
            );
            break;
        default:
            console.log(colors.yellow, "Please pass your selected network as an argument")
            console.log(colors.yellow, "Usage: ts-node all.ts standalone|testnet")
    }
    switch (mode) {
        case "local":
            console.log("Using deployed contracts from .soroban folder")
            break;
        case "public":
            console.log("Using deployed contracts from /public folder")
            break;
        default:
            console.log("Usage: local|public")
            console.log("local: use contracts from the .soroban folder (local deployements)")
            console.log("public: use contracts from the /public folder (addresses in production)")
            break;
    }
    
    await txMaker.getRouterContractAddress()
    await generateUsers();
    await mint(txMaker, network);
    await add_liquidity(txMaker);
    await swap(txMaker);
    await remove_liquidity(txMaker);
}

testAll(network, mode)

