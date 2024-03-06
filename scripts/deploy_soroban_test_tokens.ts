import { readFileSync } from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { AddressBook } from "../utils/address_book.js";
import { bumpContractCode, installContract } from "../utils/contract.js";
import { config } from "../utils/env_config.js";
import { TokensBook } from "../utils/tokens_book.js";
import { deployToken } from "./deploy_token.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const network = process.argv[2];
const loadedConfig = config(network);

export async function deploySorobanTestTokens(
  numberOfTokens: number,
  resetTokensBook: boolean,
  tokensBook: TokensBook,
  addressBook: AddressBook,
) {
  const tokensAdminAccount = loadedConfig.getUser(
    "TEST_TOKENS_ADMIN_SECRET_KEY",
  );
  const fileName = `../../scripts/token_name_ideas.json`;
  // Instaling token contract
  await installContract("token", addressBook, tokensAdminAccount);
  await bumpContractCode("token", addressBook, tokensAdminAccount);

  try {
    if (resetTokensBook) {
      tokensBook.resetNetworkTokens(network);
    }

    const tokenNameIdeas = readFileSync(path.join(__dirname, fileName));
    const tokenNameIdeasObject = JSON.parse(tokenNameIdeas.toString());
    for (let i = 0; i < numberOfTokens; i++) {
      const token = tokenNameIdeasObject.tokens[i];
      const deployedToken = await deployToken(
        token.name,
        token.symbol,
        token.logoURI,
        tokensAdminAccount,
        addressBook,
      );
      tokensBook.addToken(network, deployedToken!);
      console.log(
        `🚀 Token ${deployedToken?.symbol} deployed successfully, address ${deployedToken?.address}`,
      );
    }
    tokensBook.writeToFile();
  } catch (error) {
    console.log("🚀 « error:", error);
  }
}
