#![no_std]

mod test;

use soroban_sdk::{
    contract,
    contractimpl, Address, BytesN, ConversionError, Map, Env, Val, TryFromVal, Vec,
    xdr::ToXdr,
    Bytes,
};


mod pair {
    soroban_sdk::contractimport!(
        file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
    );
}

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
    fn sort_tokens(e: Env, token_a: Address, token_b: Address) -> (Address, Address);

    // calculates the deterministic address for a pair without making any external calls
    // check https://github.com/paltalabs/deterministic-address-soroban
    fn pair_for(e: Env, factory: Address, token_a: Address, token_b: Address) -> Address;

    // fetches and sorts the reserves for a pair
    fn get_reserves(e: Env,factory: Address, token_a: Address, token_b: Address) -> (i128, i128);

   
}

#[contract]
struct SoroswapLibrary;

#[contractimpl]
impl SoroswapLibraryTrait for SoroswapLibrary {

    // returns sorted token addresses, used to handle return values from pairs sorted in this order
    // function sortTokens(address tokenA, address tokenB) internal pure returns (address token0, address token1) {
    fn sort_tokens(e: Env, token_a: Address, token_b: Address) -> (Address, Address) {
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
    fn pair_for(e: Env, factory: Address, token_a: Address, token_b: Address) -> Address {
        //     (address token0, address token1) = sortTokens(tokenA, tokenB);
        //     pair = address(uint(keccak256(abi.encodePacked(
        //             hex'ff',
        //             factory,
        //             keccak256(abi.encodePacked(token0, token1)),
        //             hex'96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f' // init code hash
        //         ))));

        let (token_0, token_1) = Self::sort_tokens(e.clone(), token_a, token_b);
        let salt = pairSalt(&e, token_0, token_1);
        let deployer_with_address = e.deployer().with_address(factory.clone(), salt);
        let deterministic_address = deployer_with_address.deployed_address();
        deterministic_address
    }


    // // fetches and sorts the reserves for a pair
    // function getReserves(address factory, address tokenA, address tokenB) internal view returns (uint reserveA, uint reserveB) {
    fn get_reserves(e: Env,factory: Address, token_a: Address, token_b: Address) -> (i128, i128) {
        //     (address token0,) = sortTokens(tokenA, tokenB);
        let (token_0,token_1) = Self::sort_tokens(e.clone(), token_a, token_b);

        //     (uint reserve0, uint reserve1,) = IUniswapV2Pair(pairFor(factory, tokenA, tokenB)).getReserves();
        let pair_address = Self::pair_for(e.clone(), factory, token_0, token_1);
        let pair_client = pair::Client::new(&e, &pair_address);
        let (reserve_0, reserve_1, _block_timestamp_last) = pair_client.get_reserves();
        

         //   (reserveA, reserveB) = tokenA == token0 ? (reserve0, reserve1) : (reserve1, reserve0);
        let (reserve_a, reseve_b) =
            if token_a == token_0 {
                (reserve_0, reserve_1) 
            } else {
                (reserve_1, reserve_0) };

        (reserve_0, reserve_1)

    }

    // // given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    // function quote(uint amountA, uint reserveA, uint reserveB) internal pure returns (uint amountB) {
    //     require(amountA > 0, 'UniswapV2Library: INSUFFICIENT_AMOUNT');
    //     require(reserveA > 0 && reserveB > 0, 'UniswapV2Library: INSUFFICIENT_LIQUIDITY');
    //     amountB = amountA.mul(reserveB) / reserveA;
    // }

    // // given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
    // function getAmountOut(uint amountIn, uint reserveIn, uint reserveOut) internal pure returns (uint amountOut) {
    //     require(amountIn > 0, 'UniswapV2Library: INSUFFICIENT_INPUT_AMOUNT');
    //     require(reserveIn > 0 && reserveOut > 0, 'UniswapV2Library: INSUFFICIENT_LIQUIDITY');
    //     uint amountInWithFee = amountIn.mul(997);
    //     uint numerator = amountInWithFee.mul(reserveOut);
    //     uint denominator = reserveIn.mul(1000).add(amountInWithFee);
    //     amountOut = numerator / denominator;
    // }

    // // given an output amount of an asset and pair reserves, returns a required input amount of the other asset
    // function getAmountIn(uint amountOut, uint reserveIn, uint reserveOut) internal pure returns (uint amountIn) {
    //     require(amountOut > 0, 'UniswapV2Library: INSUFFICIENT_OUTPUT_AMOUNT');
    //     require(reserveIn > 0 && reserveOut > 0, 'UniswapV2Library: INSUFFICIENT_LIQUIDITY');
    //     uint numerator = reserveIn.mul(amountOut).mul(1000);
    //     uint denominator = reserveOut.sub(amountOut).mul(997);
    //     amountIn = (numerator / denominator).add(1);
    // }

    // // performs chained getAmountOut calculations on any number of pairs 
    // function getAmountsOut(address factory, uint amountIn, address[] memory path) internal view returns (uint[] memory amounts) {
    //     require(path.length >= 2, 'UniswapV2Library: INVALID_PATH');
    //     amounts = new uint[](path.length);
    //     amounts[0] = amountIn;
    //     for (uint i; i < path.length - 1; i++) {
    //         (uint reserveIn, uint reserveOut) = getReserves(factory, path[i], path[i + 1]);
    //         amounts[i + 1] = getAmountOut(amounts[i], reserveIn, reserveOut);
    //     }
    // }

    // // performs chained getAmountIn calculations on any number of pairs
    // function getAmountsIn(address factory, uint amountOut, address[] memory path) internal view returns (uint[] memory amounts) {
    //     require(path.length >= 2, 'UniswapV2Library: INVALID_PATH');
    //     amounts = new uint[](path.length);
    //     amounts[amounts.length - 1] = amountOut;
    //     for (uint i = path.length - 1; i > 0; i--) {
    //         (uint reserveIn, uint reserveOut) = getReserves(factory, path[i - 1], path[i]);
    //         amounts[i - 1] = getAmountIn(amounts[i], reserveIn, reserveOut);
    //     }
    // }
    
    


}
