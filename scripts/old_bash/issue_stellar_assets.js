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

const name_parts = [
  "zim", "lay", "veo", "tak", "rud", "pia", "nov", "kul", "jor", "fyx",
  "bax", "wun", "voe", "quy", "pyr", "otz", "mil", "kra", "jix", "gex",
  "dex", "uxi", "tro", "siv", "rya", "nef", "laz", "kev", "jam", "fiz",
  "cyo", "vax", "uvi", "tez", "rog", "peq", "nyl", "lom", "kib", "jah"
];

// Function to generate a random name
function generateRandomName() {
  const part1 = name_parts[Math.floor(Math.random() * name_parts.length)];
  const part2 = name_parts[Math.floor(Math.random() * name_parts.length)];
  return part1 + part2;
}

function getAdminKeys() {
  let adminKeys = fs.readFileSync(`${sorobanDir}/token_admin_keys.json`, 'utf8');
  adminKeys = JSON.parse(adminKeys)
  return adminKeys.find((a) => a.network == network)
}

function createToken(name, issuerPublicKey) {
  return new StellarSdk.Asset(name, issuerPublicKey);
}

function createXTokens(adminKeys) {
  let tokens = []

  for (let i = 0; i < amountOfTokens; i++) {
    const name = generateRandomName();
    const symbol = name.substring(0, 4).toUpperCase();
    
    const token = createToken(symbol, adminKeys.admin_public)

    const toSave = {
      symbol: symbol,
      name: `${token.code}:${token.issuer}`,
      logoURI: "",
      asset: `${token.code}:${token.issuer}`
    }

    tokens.push(toSave)
  }

  const dataToSave = {
    tokens: tokens
  };

  fs.writeFile(`${sorobanDir}/generated_stellar_assets.json`, JSON.stringify(dataToSave, null, 2), 'utf8', function (err) {
    if (err) {
      console.error('Error writing file:', err);
    } else {
      console.log('Stellar Assets issued and saved to', `${sorobanDir}/generated_stellar_assets.json`);
    }
  });
}

function main() {
  const adminKeys = getAdminKeys()
  createXTokens(adminKeys)
}

if (network == 'standalone' | network == 'futurenet' | network == 'testnet') {
  main()
} else {
  console.log("Args missing, usage:")
  console.log("node issueStellarAssets.js <NETWORK> [<AMOUNT OF TOKENS>]")
  console.log("<NETWORK> options: 'standalone', 'futurenet', 'testnet'")
}
