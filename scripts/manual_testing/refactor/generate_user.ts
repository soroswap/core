import { generateUser, loadAccounts, saveAccounts, colors } from "./utils/utils";
import { testAccount } from './utils/types'

export const generateUsers = async () => {
console.log('')
console.log(colors.purple, '====================')
console.log(colors.purple, '= GENERATE USER.ts =')
console.log(colors.purple, '====================')
console.log('')
console.log(colors.cyan, 'Configuring accounts...')
const user = await generateUser()
const assetDeployer = await generateUser()

saveAccounts([user, assetDeployer])
const accounts = await loadAccounts() as testAccount[]

console.log(colors.green, 'User account:', accounts[0])
console.log(colors.green, 'Asset deployer account:', accounts[1])
console.log(colors.green, '- Done. -')
}