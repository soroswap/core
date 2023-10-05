#![no_std]

mod test;

use soroban_sdk::{
    contract,
    contractimpl, Address, BytesN, ConversionError, Map, Env, Val, TryFromVal, Vec,
    xdr::ToXdr,
    Bytes,
};

#[derive(Clone, Copy)]
#[repr(u32)]

pub enum DataKey {
    FeeTo = 0,        // address public feeTo;
    FeeToSetter = 1,  // address public feeToSetter;
    AllPairs = 2,     //  address[] public allPairs;
    PairsMapping = 3, // Map of pairs
    PairWasmHash = 4,
    FeesEnabled = 5, // bool is taking fees?
}


impl TryFromVal<Env, DataKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

fn pairSalt(e: &Env, token_a: Address, token_b: Address) -> BytesN<32> {
    let mut salt = Bytes::new(e);

    // Append the bytes of token_a and token_b to the salt
    salt.append(&token_a.clone().to_xdr(e)); // can be simplified to salt.append(&self.clone().to_xdr(e)); but changes the hash
    salt.append(&token_b.clone().to_xdr(e));

    // Hash the salt using SHA256 to generate a new BytesN<32> value
    e.crypto().sha256(&salt)
}


pub trait SoroswapLibraryTrait {
    
    // returns sorted token addresses, used to handle return values from pairs sorted in this order
    fn sortTokens(e: Env, token_a: Address, token_b: Address) -> (Address, Address);

    // calculates the deterministic address for a pair without making any external calls
    // check https://github.com/paltalabs/deterministic-address-soroban
    fn pairFor(e: Env, factory: Address, token_a: Address, token_b: Address) -> Address;

   
}

#[contract]
struct SoroswapLibrary;

#[contractimpl]
impl SoroswapLibraryTrait for SoroswapLibrary {

    // returns sorted token addresses, used to handle return values from pairs sorted in this order
    // function sortTokens(address tokenA, address tokenB) internal pure returns (address token0, address token1) {
    fn sortTokens(e: Env, token_a: Address, token_b: Address) -> (Address, Address) {
        //     require(tokenA != tokenB, 'UniswapV2Library: IDENTICAL_ADDRESSES');
        if token_a == token_b {
            panic!("SoroswapFactory: token_a and token_b have identical addresses");
        }
        
        //     (token0, token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        if token_a < token_b {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        }
        
        //     require(token0 != address(0), 'UniswapV2Library: ZERO_ADDRESS');
        // In Soroban we don't have the concept of ZERO_ADDRESS
    }


    // calculates the CREATE2 address for a pair without making any external calls
    // function pairFor(address factory, address tokenA, address tokenB) internal pure returns (address pair) {
    fn pairFor(e: Env, factory: Address, token_a: Address, token_b: Address) -> Address {

        let (token_0, token_1) = Self::sortTokens(e.clone(), token_a, token_b);
        let salt = pairSalt(&e, token_0, token_1);
        let deployer_with_address = e.deployer().with_address(factory.clone(), salt);
        let deterministic_address = deployer_with_address.deployed_address();
        deterministic_address
    }
    //     (address token0, address token1) = sortTokens(tokenA, tokenB);
    //     pair = address(uint(keccak256(abi.encodePacked(
    //             hex'ff',
    //             factory,
    //             keccak256(abi.encodePacked(token0, token1)),
    //             hex'96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f' // init code hash
    //         ))));
    


}
