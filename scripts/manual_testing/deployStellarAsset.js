console.log("executing deployStellarAsset.js...");
const StellarSdk = require("stellar-sdk");

// Get the arguments passed to the script
const args = process.argv.slice(2);

// Access the individual arguments
const network = args[0];
const userSecret = args[1];
const assetDeployerSecret = args[2];
console.log("network:", network)
console.log("userSecret:", userSecret)
console.log("assetDeployerSecret:", assetDeployerSecret)

const networkPassphrase = network === "testnet" ?
    StellarSdk.Networks.TESTNET : network == "standalone" ?
        StellarSdk.Networks.STANDALONE :
        StellarSdk.Networks.PUBLIC;
var server
if (network === "testnet") {
    server = new StellarSdk.Horizon.Server("https://horizon-testnet.stellar.org")
} else if (network === "standalone") {
    server = new StellarSdk.Horizon.Server("http://stellar:8000", { allowHttp: true })
}

// Keys for accounts to issue and receive the new asset
var issuingKeys = StellarSdk.Keypair.fromSecret(assetDeployerSecret);
var receivingKeys = StellarSdk.Keypair.fromSecret(userSecret);

// Create an object to represent the new asset
var astroDollar = new StellarSdk.Asset("AstroDollar", issuingKeys.publicKey());
server.loadAccount(issuingKeys.publicKey()).then(console.log)
// First, the receiving account must trust the asset
server
    .loadAccount(receivingKeys.publicKey())
    .then(function (receiver) {
        console.log("transaction 1....")
        var transaction = new StellarSdk.TransactionBuilder(receiver, {
            fee: 100,
            networkPassphrase: networkPassphrase,
        })
            // The `changeTrust` operation creates (or alters) a trustline
            // The `limit` parameter below is optional
            .addOperation(
                StellarSdk.Operation.changeTrust({
                    asset: astroDollar,
                    limit: "2500000",
                }),
            )
            // setTimeout is required for a transaction
            .setTimeout(100)
            .build();
        transaction.sign(receivingKeys);
        return server.submitTransaction(transaction);
    })
    .then(console.log)

    // Second, the issuing account actually sends a payment using the asset
    .then(function () {
        return server.loadAccount(issuingKeys.publicKey());
    })
    .then(function (issuer) {
        console.log("transaction 2....")

        console.log(issuer)
        var transaction = new StellarSdk.TransactionBuilder(issuer, {
            fee: 100,
            networkPassphrase: networkPassphrase,
        })
            .addOperation(
                StellarSdk.Operation.payment({
                    destination: receivingKeys.publicKey(),
                    asset: astroDollar,
                    amount: "2500000",
                }),
            )
            // setTimeout is required for a transaction
            .setTimeout(100)
            .build();
        transaction.sign(issuingKeys);
        return server.submitTransaction(transaction);
    })
    .then(console.log)
    .catch(function (error) {
        if (error.response) {
            const apiError = error.response.data;
            if (apiError && apiError.extras && apiError.extras.result_codes) {
                console.log('Result Codes:', apiError.extras.result_codes);
                // Handle the specifics of the result codes here
            } else {
                console.log('Error does not have the expected format');
            }
        } else {
            console.error('Non-API error occurred:', error);
        }
    });