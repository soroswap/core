var SorobanClient = require('soroban-client');
var StellarSdk = require('@stellar/stellar-sdk');
var fs = require('fs');

const sorobanDir = "/workspace/.soroban"

let network = process.argv[2] || undefined;
let amountOfTokens = process.argv[3] || 4;
let server
let friendbot

switch (network) {
  case 'standalone':
    server = new StellarSdk.Horizon.Server("http://localhost:8000", { allowHttp: true });
    friendbot = `http://localhost:8000/friendbot?addr=`;
    break;
  
  case 'futurenet':
    server = new StellarSdk.Horizon.Server("https://horizon-futurenet.stellar.org");
    friendbot = `https://friendbot-futurenet.stellar.org?addr=`;
    break;

  case 'testnet':
    server = new StellarSdk.Horizon.Server("https://horizon-testnet.stellar.org");
    friendbot = `https://friendbot.stellar.org?addr=`;
    break;
  
  default:
    break;
}

function getAdminKeys() {
  let adminKeys = fs.readFileSync(`${sorobanDir}/token_admin_keys.json`, 'utf8');
  adminKeys = JSON.parse(adminKeys)
  return adminKeys.find((a) => a.network == network)
}


function main() {
  const adminKeys = getAdminKeys()

}

if (network == 'standalone' | network == 'futurenet' | network == 'testnet') {
  main()
} else {
  console.log("Args missing, usage:")
  console.log("node issueStellarAssets.js <NETWORK> [<AMOUNT OF TOKENS>]")
  console.log("<NETWORK> options: 'standalone', 'futurenet', 'testnet'")
}
