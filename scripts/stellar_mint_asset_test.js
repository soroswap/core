var StellarSdk = require('@stellar/stellar-sdk');
var fs = require('fs');
const fetch = require('node-fetch');

const sorobanDir = "/workspace/.soroban"

let network = process.argv[2] || undefined;
let assetString = process.argv[3] || undefined;
let server = new StellarSdk.Horizon.Server("http://stellar:8000", { allowHttp: true });
let friendbot = "http://stellar:8000/friendbot?addr="
let passphrase = "Standalone Network ; February 2017"

const freighterWallet = {
  public: "GDTRLSZ6AFVCLPGB2TRRPE5YIHFSUVSLBHW6SJQQADZ7JQADOYFBMFBZ",
  secret: "SAGELCTURYBOCWGFXM2PGUPWNEHZHI435ZKBVMCEKSPJZL2EINRZYPAI"
}

switch (network) {
  case 'standalone':
    server = new StellarSdk.Horizon.Server("http://stellar:8000", { allowHttp: true });
    friendbot = `http://stellar:8000/friendbot?addr=`;
    passphrase = "Standalone Network ; February 2017";
    break;
  
  case 'futurenet':
    server = new StellarSdk.Horizon.Server("https://horizon-futurenet.stellar.org");
    friendbot = `https://friendbot-futurenet.stellar.org?addr=`;
    passphrase = "Test SDF Future Network ; October 2022";
    break;

  case 'testnet':
    server = new StellarSdk.Horizon.Server("https://horizon-testnet.stellar.org");
    friendbot = `https://friendbot.stellar.org?addr=`;
    passphrase = "Test SDF Network ; September 2015";
    break;
  
  default:
    break;
}

function getAdminKeys() {
  let adminKeys = fs.readFileSync(`${sorobanDir}/token_admin_keys.json`, 'utf8');
  adminKeys = JSON.parse(adminKeys)
  return adminKeys.find((a) => a.network == network)
}

async function createTxBuilder(source) {
  try {
    const account = await server.loadAccount(source.publicKey());
    return new StellarSdk.TransactionBuilder(account, {
      fee: '10000',
      timebounds: { minTime: 0, maxTime: 0 },
      networkPassphrase: passphrase,
    });
  } catch (e) {
    console.error(e);
    throw Error('unable to create txBuilder');
  }
}

async function invokeClassicOp(operation, source) {
  const txBuilder = await createTxBuilder(source);
  txBuilder.addOperation(operation);
  const tx = txBuilder.build();
  tx.sign(source);
  try {
    let response = await server.submitTransaction(tx);
    // console.log(response)
    let status = response.status;
    const tx_hash = response.hash;
    // console.log(`Hash: ${tx_hash}\n`);
    // Poll this until the status is not "NOT_FOUND"
    while (status === 'PENDING' || status === 'NOT_FOUND') {
      // See if the transaction is complete
      await new Promise((resolve) => setTimeout(resolve, 2000));
      // console.log('checking tx...');
      response = await server.getTransaction(tx_hash);
      status = response.status;
    }
    // console.log('Transaction status:', response.status);
    if (status === 'ERROR') {
      console.log(response);
    }
  } catch (e) {
    console.error(e);
    throw Error('failed to submit classic op TX');
  }
}

async function classic_trustline(user, asset, amount) {
  const operation = StellarSdk.Operation.changeTrust({
    source: user.publicKey(),
    limit: amount,
    asset: asset,
  });
  await invokeClassicOp(operation, user);
}

async function classic_mint(user, asset, amount, source) {
  const operation = StellarSdk.Operation.payment({
    amount: amount,
    asset: asset,
    destination: user.publicKey(),
    source: source.publicKey(),
  });
  await invokeClassicOp(operation, source);
}

async function main() {
  const adminKeys = getAdminKeys()
  const adminKeyPair = StellarSdk.Keypair.fromSecret(adminKeys.admin_secret)
  const userKeyPair = StellarSdk.Keypair.fromSecret(freighterWallet.secret)

  console.log("-------------------------")
  console.log()
  console.log("We are setting trutlines and minting so this assets have records")
  console.log("you can test this with this account secret:", userKeyPair.secret())
  console.log()
  await fetch(`${friendbot}${userKeyPair.publicKey()}`)

  const assetParts = getClassicStellarAsset(assetString);
  const asset = new StellarSdk.Asset(assetParts.assetCode, assetParts.issuer)
  
  await classic_trustline(userKeyPair, asset, "1000")
  await classic_mint(userKeyPair, asset, "1000", adminKeyPair)
}

if (network == 'standalone' | network == 'futurenet' | network == 'testnet' && isClassicStellarAssetFormat(assetString)) {
  main()
} else {
  console.log("Args missing, usage:")
  console.log("node stellar_trustline_n_mint_test_asset.js <NETWORK> <ASSET>")
  console.log("<NETWORK> options: 'standalone', 'futurenet', 'testnet'")
  console.log("<ASSET> format: 'CODE:ISSUER'")
}

function isAddress(value) {
  try {
    return value.match(/^[A-Z0-9]{56}$/) ? value : false;
  } catch {
    return false;
  }
}

function isValidSymbol(code) {
  return /^[A-Za-z0-9]{2,}$/.test(code);
}

function isClassicStellarAssetFormat(value) {
  if (!value) return false;
  const parts = value.split(':');
  if (parts.length !== 2) {
    return false;
  }

  const [assetCode, issuer] = parts;
  return isValidSymbol(assetCode) && isAddress(issuer) !== false;
}

function getClassicStellarAsset(value) {
  if (!value) return false;
  const parts = value.split(':');
  if (parts.length !== 2) {
    return false;
  }

  const [assetCode, issuer] = parts;

  if (!isAddress(issuer)) return false;

  return {
    assetCode,
    issuer,
  };
}
