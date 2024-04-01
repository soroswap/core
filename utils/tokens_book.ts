import { existsSync, readFileSync, writeFileSync } from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export interface Token {
  name: string;
  code: string;
  issuer?: string;
  contract: string;
  org?: string;
  domain?: string;
  icon?: string;
  decimals: number;
}

interface NetworkTokens {
  network: string;
  assets: Token[];
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
          assets: []
        },
        {
          network: 'testnet',
          assets: []
        },
        {
          network: 'standalone',
          assets: []
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

  addToken(networkName: string, asset: Token) {
    const network = this.networks.find(n => n.network === networkName);    
    if (network) {
      const tokenExists = network.assets.some(t => t.contract === asset.contract);
      
      if (!tokenExists) {
        network.assets.push(asset);
      }
    } else {
      this.networks.push({
        network: networkName,
        assets: [asset]
      });
    }
  }

  prependToken(networkName: string, asset: Token) {
    const network = this.networks.find(n => n.network === networkName);
    if (network) {
      const tokenExists = network.assets.some(t => t.contract === asset.contract);
  
      if (!tokenExists) {
        network.assets.unshift(asset);
      }
    } else {
      this.networks.push({
        network: networkName,
        assets: [asset]
      });
    }
  }

  getTokensByNetwork(networkName: string): Token[] | undefined {
    const network = this.networks.find(n => n.network === networkName);
    return network?.assets;
  }

  resetNetworkTokens(networkName: string) {
    const networkIndex = this.networks.findIndex(n => n.network === networkName);
    if (networkIndex !== -1) {
      this.networks[networkIndex].assets = [];
    } else {
      this.networks.push({
        network: networkName,
        assets: []
      });
    }
  }
}
