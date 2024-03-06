import { copyFileSync, existsSync, mkdirSync } from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Function to copy file
function copyFile(source: string, destination: string) {
    copyFileSync(source, destination);
    console.log(`Copied ${path.basename(source)} to ${destination}`);
}

// Main function to handle the copying process
function main(network: string) {
    const sorobanDir = path.join(__dirname, '../../.soroban');
    const publicDir = path.join(__dirname, '../../public');

    // Ensure the public directory exists
    if (!existsSync(publicDir)) {
        mkdirSync(publicDir, { recursive: true });
    }

    // Define files to copy
    const filesToCopy = [`${network}.contracts.json`, 'tokens.json', 'random_tokens.json'];

    filesToCopy.forEach(file => {
        const sourcePath = path.join(sorobanDir, file);
        const destPath = path.join(publicDir, file);

        if (existsSync(sourcePath)) {
            copyFile(sourcePath, destPath);
        } else {
            console.warn(`Warning: File ${file} does not exist and cannot be copied.`);
        }
    });
}

// Extract network argument from command line
const network = process.argv[2];

if (!network) {
    console.error('Error: Network parameter is required.');
    process.exit(1);
}

main(network);
