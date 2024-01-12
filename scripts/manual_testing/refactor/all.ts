import {getRouterContractAddress, colors} from "./utils/utils";
import { TransactionBuilder } from "./utils/TransactionBuilder";
import {
    generateUsers,
    mint,
    addLiquidity,
    swap,
    removeLiquidity
} from "./index"
import { networkConfig } from "./utils/types";

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
    async function getNetworkValues(network: string): Promise<networkConfig | void> {
        let apiUri: string;
        let rpcUri: string;
        let routerContractAddress: string;
        let friendbotUri: string;

        switch (network) {
            case "standalone":
                apiUri = "http://soroswapCoreApi:8010";
                rpcUri = "http://stellar:8000";
                routerContractAddress = await getRouterContractAddress(apiUri, network);
                friendbotUri = `${rpcUri}/friendbot?addr=`;
                return { apiUri, rpcUri, routerContractAddress, friendbotUri };
            case "testnet":
                apiUri = "https://horizon-testnet.stellar.org";
                rpcUri = "https://soroban-testnet.stellar.org";
                routerContractAddress = await getRouterContractAddress(apiUri, network);
                friendbotUri = "https://friendbot.stellar.org/?addr=";
                return { apiUri, rpcUri, routerContractAddress, friendbotUri };
            default:
                console.log(colors.yellow, "Usage: ts-node all.ts standalone|testnet");
                throw new Error("Please pass your selected network as an argument");
        }
  
    }
    const networkValues = await getNetworkValues(network) as networkConfig;
    txMaker = new TransactionBuilder(
        networkValues.apiUri,
        `${networkValues.rpcUri}/soroban/rpc`,
        networkValues.friendbotUri,
        networkValues.routerContractAddress,
        network
    );
    
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
    console.log(colors.purple, networkValues)
    await txMaker.getRouterContractAddress()
    await generateUsers();
    await mint(txMaker, network);
    await addLiquidity(txMaker);
    await swap(txMaker);
    await removeLiquidity(txMaker);
}

testAll(network, mode)

