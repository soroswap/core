import * as sdk from "stellar-sdk";
import fs from "fs";
import {
        colors, 
        getNetworkPassphrase, 
        waitForConfirmation,
        showErrorResultCodes,
        hexToByte,
        getCurrentTimePlusOneHour,
} from "./utils";
import { 
    mintTokensArgs,
    testAccount,
    initializeTokenContractArgs,
    addLiquiditySoroswapArgs,
    removeLiquiditySoroswapArgs
} from "./types";


export class TransactionBuilder {
    private horizonServer: sdk.Horizon.Server;
    private sorobanServer: sdk.SorobanRpc.Server;
    private friendbotURI: string;
    private routerContractAddress: string;
    private network: string;
    /**
     * Constructs a new instance of the `TxMaker` class.
     * @param horizonServer The Horizon server URL used for interacting with the Stellar network.
     * @param sorobanServer The Soroban server URL used for interacting with the Soroban network.
     * @param friendbotURI The URI for the friendbot service used for funding test accounts.
     * @param routerContractAddress The address of the router contract.
     * @param network The network used: Standalone or Testnet.
     */
    constructor(
            horizonServer: string,
            sorobanServer: string,
            friendbotURI: string, 
            routerContractAddress: string, 
            network: string
        ) {
        this.horizonServer = new sdk.Horizon.Server(horizonServer, {
            allowHttp: true
        });
        this.sorobanServer = new sdk.SorobanRpc.Server(sorobanServer, {
            allowHttp: true
        });
        this.friendbotURI = friendbotURI;
        this.routerContractAddress = routerContractAddress;
        this.network = network;
    }

    /**
     * Builds a transaction with the given source account, signer keypair, and operations.
     * @param source The source account for the transaction.
     * @param signer The signer keypair for the transaction.
     * @param operations The operations to be added to the transaction.
     * @returns The built transaction.
     */
    buildTransaction(source: sdk.Account, signer: sdk.Keypair, ...operations: sdk.xdr.Operation[]): sdk.Transaction {
        const transaction: sdk.TransactionBuilder = new sdk.TransactionBuilder(source, {
            fee: sdk.BASE_FEE,
            networkPassphrase: getNetworkPassphrase(this.network)
        })
        operations.forEach(op => {transaction.addOperation(op)});
        const builtTransaction: sdk.Transaction = transaction.setTimeout(30).build();
        builtTransaction.sign(signer);

        return builtTransaction;
    }

    /**
     * Mints tokens and sends them to a destination address.
     * 
     * @param args - The arguments for minting tokens.
     * @returns A promise that resolves with the confirmation of the minting transaction.
     */
    async mintTokens(args: mintTokensArgs): Promise<any> {
        const source = await this.sorobanServer.getAccount(args.source.publicKey);
        const sourceKeypair = sdk.Keypair.fromSecret(args.source.privateKey);
        const mintTokenArgs = [
            new sdk.Address(args.destination).toScVal(),
            sdk.nativeToScVal(Number(args.amount), { type: "i128" }),
        ];
        const op = sdk.Operation.invokeContractFunction({
            contract: args.contractId,
            function: "mint",
            args: mintTokenArgs,
        });
        let tx = this.buildTransaction(source, sourceKeypair, op);
        const preparedTransaction = await this.sorobanServer.prepareTransaction(tx);
        preparedTransaction.sign(sourceKeypair);
        try {
            const txRes = await this.sorobanServer.sendTransaction(preparedTransaction);
            const confirmation = await waitForConfirmation(txRes.hash, this.sorobanServer);
            return confirmation;
        } catch (error) {
            showErrorResultCodes(error);
            console.log("error:", error);
            return { status: "error", error: error };
        }
    }

    /**
     * Retrieves the token balance for a given source account and contract ID.
     * @param args - The arguments for retrieving the token balance.
     * @param args.source - The source account.
     * @param args.contractId - The contract ID.
     * @returns The token balance as a number.
     * @throws Error if there is an error getting the token balance.
     */
    async getTokenBalance(args: {source: testAccount, contractId: string}) {

        const tokenContract = new sdk.Contract(args.contractId);
        const op = tokenContract.call("balance", new sdk.Address(args.source.publicKey).toScVal());
        const source = await this.sorobanServer.getAccount(args.source.publicKey);
        const sourceKeypair = sdk.Keypair.fromSecret(args.source.privateKey);
        
        const transaction = this.buildTransaction(source, sourceKeypair, op);
        
        const preparedTransaction = await this.sorobanServer.prepareTransaction(transaction);
        const simulatedTransaction = await this.sorobanServer.simulateTransaction(preparedTransaction) as any;
        
        const parsedResult = sdk.scValToNative(simulatedTransaction.result.retval).toString();
        if(!parsedResult) {
            throw new Error("The operation has no result.");
        }
        if(parsedResult == 0) {
            return parsedResult
        }
        const resultNumber = parseInt(parsedResult.slice(0, -1));
        if (isNaN(resultNumber)) {
            console.error(colors.red, "Error, balance is unreachable.")
            throw new Error("Error getting token balance.");
        }
        return resultNumber;
    }

       /**
     * Funds an account by requesting testnet lumens from the friendbot service.
     * @param account The test account to fund.
     * @returns A promise that resolves when the account is successfully funded.
     */
    async fundAccount(account: testAccount): Promise<void> {
        try {
            const response = await fetch(
                `${this.friendbotURI}${encodeURIComponent(
                    account.publicKey,
                )}`,
            );
            const responseJSON = await response.json();
            if (responseJSON.successful) {
                console.log("SUCCESS! You have a new account :)\n");
            } else {
                if (
                    responseJSON.detail ===
                    "createAccountAlreadyExist (AAAAAAAAAGT/////AAAAAQAAAAAAAAAA/////AAAAAA=)"
                ) {
                    console.log("Account already exists");
                } else {
                    console.error("ERROR! :(\n", responseJSON);
                }
            }
        } catch (error) {
            console.error("ERROR!", error);
            showErrorResultCodes(error);
        }
    }

    /**
     * Uploads a token contract WebAssembly (Wasm) file to the Soroban network.
     * @param signer The test account used to sign the transaction.
     * @returns A promise that resolves to the confirmation of the transaction.
     */
    async uploadTokenContractWasm(signer: testAccount): Promise<any> {
        // Read the Wasm file
        const wasmBuffer = fs.readFileSync("/workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm");

        // Create the operation to upload the contract Wasm
        const op = sdk.Operation.uploadContractWasm({ wasm: wasmBuffer });

        // Get the source account and keypair
        const source = await this.sorobanServer.getAccount(signer.publicKey);
        const sourceKeypair = sdk.Keypair.fromSecret(signer.privateKey);

        // Build the transaction
        let tx = this.buildTransaction(source, sourceKeypair, op);

        try {
            // Prepare and sign the transaction
            const preparedTransaction = await this.sorobanServer.prepareTransaction(tx);
            preparedTransaction.sign(sourceKeypair);

            // Send the transaction and wait for confirmation
            const submitTransactionResponse = await this.sorobanServer.sendTransaction(preparedTransaction);
            const confirmation = await waitForConfirmation(submitTransactionResponse.hash, this.sorobanServer);
            return confirmation;
        } catch (error) {
            console.error("ERROR!", error);
            showErrorResultCodes(error);
            return { status: "error", error: error };
        }
    }

    /**
     * Creates a token contract on the Soroban network.
     * @param signer The test account used to sign the transaction.
     * @returns A promise that resolves to the contract address of the created token contract.
     */
    async createTokenContract(signer: testAccount): Promise<string | { status: string; error: any }> {
        // Read the Wasm file
        const wasmBuffer = fs.readFileSync("/workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.optimized.wasm");

        // Calculate the hash of the Wasm file
        const hash = sdk.hash(wasmBuffer);

        // Create the operation to create the custom contract
        const op = sdk.Operation.createCustomContract({
            address: new sdk.Address(signer.publicKey),
            wasmHash: hash,
        });

        // Get the source account and keypair
        const source = await this.sorobanServer.getAccount(signer.publicKey);
        const sourceKeypair = sdk.Keypair.fromSecret(signer.privateKey);

        // Build the transaction
        let tx = this.buildTransaction(source, sourceKeypair, op);

        try {
            // Prepare and sign the transaction
            const preparedTransaction = await this.sorobanServer.prepareTransaction(tx);
            preparedTransaction.sign(sourceKeypair);

            // Send the transaction and wait for confirmation
            const submitTransactionResponse = await this.sorobanServer.sendTransaction(preparedTransaction);
            const confirmation = await waitForConfirmation(submitTransactionResponse.hash, this.sorobanServer);

            if (confirmation.resultMetaXdr) {
                // Extract the contract ID from the transaction metadata
                const buff = Buffer.from(confirmation.resultMetaXdr.toXDR("base64"), "base64");
                const txMeta = sdk.xdr.TransactionMeta.fromXDR(buff);
                const contractId = txMeta
                    .v3()
                    .sorobanMeta()
                    ?.returnValue()
                    .address()
                    .contractId()
                    .toString("hex") || "";

                // Encode the contract ID as a Stellar contract address
                return sdk.StrKey.encodeContract(Buffer.from(hexToByte(contractId)));
            }
            else {
                return { status: "error", error: "No resultMetaXdr when creating token contract" };
            }
        } catch (error) {
            console.error("ERROR!", error);
            showErrorResultCodes(error);
            return { status: "error", error: error };
        }
    }

    /**
     * Initializes the token contract.
     * 
     * @param args - The arguments for initializing the token contract.
     * @returns A promise that resolves to the result of the initialization.
     */
    async initializeTokenContract(args: initializeTokenContractArgs): Promise<any> {
        const source = await this.sorobanServer.getAccount(args.source.publicKey);
        const sourceKeypair = sdk.Keypair.fromSecret(args.source.privateKey);
        const initializeArgs = [
            new sdk.Address(args.source.publicKey).toScVal(),
            sdk.nativeToScVal(7, { type: "u32" }),
            sdk.nativeToScVal(args.name, { type: "string" }),
            sdk.nativeToScVal(args.symbol, { type: "string" }),
        ];
        const op = sdk.Operation.invokeContractFunction({
            contract: args.contractId,
            function: "initialize",
            args: initializeArgs,
        });

        let tx = this.buildTransaction(source, sourceKeypair, op);
        const preparedTransaction = await this.sorobanServer.prepareTransaction(tx);
        preparedTransaction.sign(sourceKeypair);
        try {
            const txRes = await this.sorobanServer.sendTransaction(preparedTransaction);
            const confirmation = await waitForConfirmation(txRes.hash, this.sorobanServer);
            return confirmation;
        } catch (error) {
            showErrorResultCodes(error);
            console.log("error:", error);
            return { status: "error", error: error };
        }
    }

    /**
     * Executes the add_liquidity function of the soroswap router contract to add liquidity to a pool.
     * @param args The arguments for adding liquidity.
     *             - source: The source account's keypair and public key.
     *             - tokenA: The address of the first token in the pool.
     *             - tokenB: The address of the second token in the pool.
     *             - amountADesired: The desired amount of token A to add to the pool.
     *             - amountBDesired: The desired amount of token B to add to the pool.
     *             - amountAMin: The minimum amount of token A to add to the pool.
     *             - amountBMin: The minimum amount of token B to add to the pool.
     *             - to: The destination account's keypair and public key.
     * @returns A promise that resolves to the confirmation of the transaction.
     */
    async addLiquiditySoroswap(args: addLiquiditySoroswapArgs): Promise<any> {
        const account = await this.sorobanServer.getAccount(args.source.publicKey);
        const sourceKeypair = sdk.Keypair.fromSecret(args.source.privateKey);

        const routerContract = new sdk.Contract(this.routerContractAddress);

        const scValParams = [
            new sdk.Address(args.tokenA).toScVal(),
            new sdk.Address(args.tokenB).toScVal(),
            sdk.nativeToScVal(Number(args.amountADesired), { type: "i128" }),
            sdk.nativeToScVal(Number(args.amountBDesired), { type: "i128" }),
            sdk.nativeToScVal(Number(args.amountAMin), { type: "i128" }),
            sdk.nativeToScVal(Number(args.amountBMin), { type: "i128" }),
            new sdk.Address(args.to.publicKey).toScVal(),
            sdk.nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }),
        ];

        const op = routerContract.call("add_liquidity", ...scValParams);

        const transaction = this.buildTransaction(account, sourceKeypair, op);

        const preparedTransaction = await this.sorobanServer.prepareTransaction(transaction);
        preparedTransaction.sign(sourceKeypair);

        try {
            const txRes = await this.sorobanServer.sendTransaction(preparedTransaction);
            const confirmation = await waitForConfirmation(txRes.hash, this.sorobanServer);
            return confirmation;
        } catch (error) {
            showErrorResultCodes(error);
            console.error(error);
        }
    }

    /**
     * Executes the swap exact tokens for tokens function of the soroswap router contract to swap tokens.
     * @param args The arguments for swapping tokens.
     *             - source: The source account's keypair and public key.
     *             - amountIn: The amount of token A to swap.
     *             - amountOutMin: The minimum amount of token B to receive.
     *             - path: A vector representing the trading route, where the first element is the input token. The destination account's keypair and public key.and the last is the output token. Intermediate elements represent pairs to trade through.
     *             - to: The destination account's keypair and public key.
     * @returns A promise that resolves to the confirmation of the transaction.
     */
    async swapExactTokensForTokensSoroswap(args: {
        source: testAccount,
        amountIn: string,
        amountOutMin: string,
        path: string[],
        to: testAccount
    }): Promise<any> {
        const account = await this.sorobanServer.getAccount(args.source.publicKey);
        const sourceKeypair = sdk.Keypair.fromSecret(args.source.privateKey);

        const routerContract = new sdk.Contract(this.routerContractAddress);
        const path = args.path.map((token) => new sdk.Address(token));
        const scValParams = [
            sdk.nativeToScVal(Number(args.amountIn), { type: "i128" }),
            sdk.nativeToScVal(Number(args.amountOutMin), { type: "i128" }),
            sdk.nativeToScVal(path, { type: "Vec" }),
            new sdk.Address(args.to.publicKey).toScVal(),
            sdk.nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }),
        ];

        const op = routerContract.call("swap_exact_tokens_for_tokens", ...scValParams);

        const transaction = this.buildTransaction(account, sourceKeypair, op);

        const preparedTransaction = await this.sorobanServer.prepareTransaction(transaction);
        preparedTransaction.sign(sourceKeypair);

        try {
            const txRes = await this.sorobanServer.sendTransaction(preparedTransaction);
            const confirmation = await waitForConfirmation(txRes.hash, this.sorobanServer);
            return confirmation;
        } catch (error) {
            showErrorResultCodes(error);
            console.error(error);
        }
    }

    /**
     * Removes liquidity from a pool by invoking the `remove_liquidity` function of the soroswap router contract on the Soroban network.
     * @param args The arguments for removing liquidity.
     *             - source: The source account's keypair and public key.
     *             - tokenA: The address of the first token in the pool.
     *             - tokenB: The address of the second token in the pool.
     *             - liquidity: The amount of liquidity to remove.
     *             - amountAMin: The minimum amount of token A to receive.
     *             - amountBMin: The minimum amount of token B to receive.
     *             - to: The destination account's keypair and public key.
     * @returns A promise that resolves to the confirmation of the transaction.
     */
    async removeLiquiditySoroswap(args: removeLiquiditySoroswapArgs): Promise<any> {
        const source = await this.sorobanServer.getAccount(args.source.publicKey);
        const sourceKeypair = sdk.Keypair.fromSecret(args.source.privateKey);
        const routerContract = new sdk.Contract(this.routerContractAddress);

        const scValParams = [
            new sdk.Address(args.tokenA).toScVal(),
            new sdk.Address(args.tokenB).toScVal(),
            sdk.nativeToScVal(Number(args.liquidity), { type: "i128" }),
            sdk.nativeToScVal(Number(args.amountAMin), { type: "i128" }),
            sdk.nativeToScVal(Number(args.amountBMin), { type: "i128" }),
            new sdk.Address(args.to.publicKey).toScVal(),
            sdk.nativeToScVal(getCurrentTimePlusOneHour(), { type: "u64" }),
        ];

        const op = routerContract.call("remove_liquidity", ...scValParams);
        const transaction = this.buildTransaction(source, sourceKeypair, op);
        const preparedTransaction = await this.sorobanServer.prepareTransaction(transaction);
        preparedTransaction.sign(sourceKeypair);

        try {
            const txRes = await this.sorobanServer.sendTransaction(preparedTransaction);
            const confirmation = await waitForConfirmation(txRes.hash, this.sorobanServer);
            return confirmation;
        } catch (error) {
            showErrorResultCodes(error);
            console.error(error);
        }
    }
}