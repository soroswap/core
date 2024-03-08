import { AddressBook } from "../../utils/address_book.js";
import { airdropAccount } from "../../utils/contract.js";
import { config } from "../../utils/env_config.js";
import { TokensBook } from "../../utils/tokens_book.js";
import { addLiquidity } from "./addLiquidity.js";
import { mintTokens } from "./mint.js";
import { removeLiquidity } from "./removeLiquidity.js";
import { swap } from "./swap.js";

const testAll = async() => {
    const testAccount = loadedConfig.getUser('TESTING_ACCOUNT_SECRET_KEY');
    const tokensAdmin = loadedConfig.getUser('TEST_TOKENS_ADMIN_SECRET_KEY');
    if (network != 'mainnet') await airdropAccount(testAccount);
    if (network != 'mainnet') await airdropAccount(tokensAdmin);

    await mintTokens(network, tokensBook, loadedConfig);
    await addLiquidity(network, tokensBook, addressBook, loadedConfig);
    await swap(network, tokensBook, addressBook, loadedConfig);
    await removeLiquidity(network, tokensBook, addressBook, loadedConfig);
}

const network = process.argv[2];
const folder = process.argv[3];
let addressBook: AddressBook;
let tokensBook: TokensBook;

if (folder == 'public') {
    addressBook = AddressBook.loadFromFile(network, folder);
    tokensBook = TokensBook.loadFromFile(folder);
} else {
    addressBook = AddressBook.loadFromFile(network);
    tokensBook = TokensBook.loadFromFile();
}

const loadedConfig = config(network);

testAll()
