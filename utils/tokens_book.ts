import { existsSync, readFileSync, writeFileSync } from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export interface Token {
  name: string;
  contract: string;
  code: string;
  issuer?: string;
  icon?: string;
  decimals: number;
}

interface NetworkTokens {
  network: string;
  tokens: Token[];
}

export class TokensBook {
  private networks: NetworkTokens[];
  private fileName: string;

  constructor(networks: NetworkTokens[], fileName: string) {
    this.networks = networks;
    this.fileName = fileName;
  }

  static loadFromFile(folder: string = '.soroban', fileName: string = 'tokens.json') {
    const filePath = path.join(__dirname, '../../',folder, '/',fileName);
    let networks: NetworkTokens[];

    if (existsSync(filePath)) {
      const fileContent = readFileSync(filePath, { encoding: 'utf-8' });
      networks = JSON.parse(fileContent);
    } else {
      // If the file doesn't exist, create a new empty array for networks
      networks = [
        {
          network: 'mainnet',
          tokens: []
        },
        {
          network: 'testnet',
          tokens: []
        },
        {
          network: 'standalone',
          tokens: []
        }
      ];
    }

    return new TokensBook(networks, fileName);
  }

  writeToFile() {
    const filePath = path.join(__dirname, '../../.soroban/', this.fileName);
    const fileContent = JSON.stringify(this.networks, null, 2);
    writeFileSync(filePath, fileContent);
  }

  addToken(networkName: string, token: Token) {
    const network = this.networks.find(n => n.network === networkName);    
    if (network) {
      const tokenExists = network.tokens.some(t => t.contract === token.contract);
      
      if (!tokenExists) {
        network.tokens.push(token);
      }
    } else {
      this.networks.push({
        network: networkName,
        tokens: [token]
      });
    }
  }

  prependToken(networkName: string, token: Token) {
    const network = this.networks.find(n => n.network === networkName);
    if (network) {
      const tokenExists = network.tokens.some(t => t.contract === token.contract);
  
      if (!tokenExists) {
        network.tokens.unshift(token);
      }
    } else {
      this.networks.push({
        network: networkName,
        tokens: [token]
      });
    }
  }

  getTokensByNetwork(networkName: string): Token[] | undefined {
    const network = this.networks.find(n => n.network === networkName);
    return network?.tokens;
  }

  resetNetworkTokens(networkName: string) {
    const networkIndex = this.networks.findIndex(n => n.network === networkName);
    if (networkIndex !== -1) {
      this.networks[networkIndex].tokens = [];
    } else {
      this.networks.push({
        network: networkName,
        tokens: []
      });
    }
  }
}
