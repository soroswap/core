import * as sdk from "stellar-sdk";
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
let network = args[0];
const mode = args[1];

console.log("network:", network)
console.log("running on:", mode)
let txMaker: TransactionBuilder;
const networkPassphrase = network === "testnet" ?
    sdk.Networks.TESTNET : network == "standalone" ?
        sdk.Networks.STANDALONE :
        sdk.Networks.PUBLIC;


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
        console.log("public: use contracts from the /public folder (addresses in production?)")
        break;
}

const testAll = async () => {
    let apiUri = "http://soroswapCoreApi:8010"
    let rpcUri = "http://stellar"
    let routerContractAddress = await getRouterContractAddress(apiUri, network)

    txMaker = new TransactionBuilder(
        `${rpcUri}:8000`,
        `${rpcUri}:8000/soroban/rpc`,
        `${rpcUri}:8000/friendbot?addr=`,
        routerContractAddress,
        network
    );
    await txMaker.getRouterContractAddress()
    await generateUsers();
    await mint(txMaker);
    await add_liquidity(txMaker);
    await swap(txMaker);
    await remove_liquidity(txMaker);
}

testAll()

