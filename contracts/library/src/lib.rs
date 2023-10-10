#![no_std]

mod test;

use soroban_sdk::{
    contract,
    contractimpl, Address, BytesN, Env,
    xdr::ToXdr, Vec, Bytes,
};


mod pair {
    soroban_sdk::contractimport!(
        file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
    );
}


fn pair_salt(e: &Env, token_a: Address, token_b: Address) -> BytesN<32> {
    let mut salt = Bytes::new(e);

    // Append the bytes of token_a and token_b to the salt
    salt.append(&token_a.clone().to_xdr(e)); // can be simplified to salt.append(&self.clone().to_xdr(e)); but changes the hash
    salt.append(&token_b.clone().to_xdr(e));

    // Hash the salt using SHA256 to generate a new BytesN<32> value
    e.crypto().sha256(&salt)
}




pub trait SoroswapLibraryTrait {
    
    // returns sorted token addresses, used to handle return values from pairs sorted in this order
    fn sort_tokens(token_a: Address, token_b: Address) -> (Address, Address);

    // calculates the deterministic address for a pair without making any external calls
    // check https://github.com/paltalabs/deterministic-address-soroban
    fn pair_for(e: Env, factory: Address, token_a: Address, token_b: Address) -> Address;

    // fetches and sorts the reserves for a pair
    fn get_reserves(e: Env,factory: Address, token_a: Address, token_b: Address) -> (i128, i128);

    // given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    fn quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> i128;

    // given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
    fn get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128;

    // given an output amount of an asset and pair reserves, returns a required input amount of the other asset
    fn get_amount_in(amount_out: i128, reserve_in: i128, reserve_out: i128) -> i128;

    // performs chained getAmountOut calculations on any number of pairs 
    fn get_amounts_out(e: Env, factory: Address, amount_in: i128, path: Vec<Address>) -> Vec<i128>;
    
    // performs chained getAmountIn calculations on any number of pairs
    fn get_amounts_in(e:Env, factory: Address, amount_out: i128, path: Vec<Address>) -> Vec<i128>;
    // function getAmountsIn(address factory, uint amountOut, address[] memory path) internal view returns (uint[] memory amounts) {


   
}

#[contract]
struct SoroswapLibrary;

#[contractimpl]
impl SoroswapLibraryTrait for SoroswapLibrary {

    // returns sorted token addresses, used to handle return values from pairs sorted in this order
    // function sortTokens(address tokenA, address tokenB) internal pure returns (address token0, address token1) {
    fn sort_tokens(token_a: Address, token_b: Address) -> (Address, Address) {
        //     require(tokenA != tokenB, 'UniswapV2Library: IDENTICAL_ADDRESSES');
        if token_a == token_b {
            panic!("SoroswapLibrary: token_a and token_b have identical addresses");
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

        let (token_0, token_1) = Self::sort_tokens(token_a, token_b);
        let salt = pair_salt(&e, token_0, token_1);
        let deployer_with_address = e.deployer().with_address(factory.clone(), salt);
        let deterministic_address = deployer_with_address.deployed_address();
        deterministic_address
    }


    // fetches and sorts the reserves for a pair
    // function getReserves(address factory, address tokenA, address tokenB) internal view returns (uint reserveA, uint reserveB) {
    fn get_reserves(e: Env,factory: Address, token_a: Address, token_b: Address) -> (i128, i128) {
        //     (address token0,) = sortTokens(tokenA, tokenB);
        let (token_0,token_1) = Self::sort_tokens(token_a.clone(), token_b.clone());

        //     (uint reserve0, uint reserve1,) = IUniswapV2Pair(pairFor(factory, tokenA, tokenB)).getReserves();
        let pair_address = Self::pair_for(e.clone(), factory, token_0.clone(), token_1.clone());
        let pair_client = pair::Client::new(&e, &pair_address);
        let (reserve_0, reserve_1, _block_timestamp_last) = pair_client.get_reserves();
        

         //   (reserveA, reserveB) = tokenA == token0 ? (reserve0, reserve1) : (reserve1, reserve0);
        let (reserve_a, reseve_b) =
            if token_a == token_0 {
                (reserve_0, reserve_1) 
            } else {
                (reserve_1, reserve_0) };

        (reserve_a, reseve_b)

    }

    // given some amount of an asset and pair reserves, returns an equivalent amount of the other asset
    // function quote(uint amountA, uint reserveA, uint reserveB) internal pure returns (uint amountB) {
    fn quote(amount_a: i128, reserve_a: i128, reserve_b: i128) -> i128 {
        //     require(amountA > 0, 'UniswapV2Library: INSUFFICIENT_AMOUNT');
        if amount_a <= 0 {
            panic!("SoroswapLibrary: insufficient amount");
        }
        //     require(reserveA > 0 && reserveB > 0, 'UniswapV2Library: INSUFFICIENT_LIQUIDITY');
        if reserve_a <= 0 && reserve_b <= 0 {
            panic!("SoroswapLibrary: insufficient liquidity");
        }
        //     amountB = amountA.mul(reserveB) / reserveA;
        amount_a.checked_mul(reserve_b).unwrap().checked_div(reserve_a).unwrap()
    }
    

    // given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset
    // function getAmountOut(uint amountIn, uint reserveIn, uint reserveOut) internal pure returns (uint amountOut) {
    fn get_amount_out(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128 {
        //     require(amountIn > 0, 'UniswapV2Library: INSUFFICIENT_INPUT_AMOUNT');
        if amount_in <= 0 {
            panic!("SoroswapLibrary: insufficient input amount");
        }
        
        //     require(reserveIn > 0 && reserveOut > 0, 'UniswapV2Library: INSUFFICIENT_LIQUIDITY');
        if reserve_in <= 0 || reserve_out <= 0 {
            panic!("SoroswapLibrary: insufficient liquidity");
        }

        //     uint amountInWithFee = amountIn.mul(997);
        let amount_in_with_fee = amount_in.checked_mul(997).unwrap();
        //     uint numerator = amountInWithFee.mul(reserveOut);
        let numerator = amount_in_with_fee.checked_mul(reserve_out).unwrap();

        //     uint denominator = reserveIn.mul(1000).add(amountInWithFee);
        let denominator = reserve_in.checked_mul(1000).unwrap().checked_add(amount_in_with_fee).unwrap();

        //     amountOut = numerator / denominator;
        numerator.checked_div(denominator).unwrap()
    }

    // // given an output amount of an asset and pair reserves, returns a required input amount of the other asset
    // function getAmountIn(uint amountOut, uint reserveIn, uint reserveOut) internal pure returns (uint amountIn) {
    fn get_amount_in(amount_out: i128, reserve_in: i128, reserve_out: i128) -> i128 {
        //     require(amountOut > 0, 'UniswapV2Library: INSUFFICIENT_OUTPUT_AMOUNT');
        if amount_out <= 0 {
            panic!("SoroswapLibrary: insufficient input amount");
        }
        //     require(reserveIn > 0 && reserveOut > 0, 'UniswapV2Library: INSUFFICIENT_LIQUIDITY');
        if reserve_in <= 0 || reserve_out <= 0 {
            panic!("SoroswapLibrary: insufficient liquidity");
        }
        //     uint numerator = reserveIn.mul(amountOut).mul(1000);
        let numerator = reserve_in.checked_mul(amount_out).unwrap().checked_mul(1000).unwrap();

        //     uint denominator = reserveOut.sub(amountOut).mul(997);
        let denominator = reserve_out.checked_sub(amount_out).unwrap().checked_mul(997).unwrap();

        //     amountIn = (numerator / denominator).add(1);
        numerator.checked_div(denominator).unwrap().checked_add(1).unwrap()
    }

    // performs chained getAmountOut calculations on any number of pairs 
    // function getAmountsOut(address factory, uint amountIn, address[] memory path) internal view returns (uint[] memory amounts) {
    fn get_amounts_out(e: Env, factory: Address, amount_in: i128, path: Vec<Address>) -> Vec<i128> {
        //     require(path.length >= 2, 'UniswapV2Library: INVALID_PATH');
        if path.len() < 2 {panic!("SoroswapLibrary: invalid path")};
        
        //     amounts = new uint[](path.length);
        //     amounts[0] = amountIn;
        let mut amounts =  Vec::new(&e);
        amounts.set(0,amount_in);  
        //     for (uint i; i < path.length - 1; i++) {
        
        for i in 0..path.len() - 1 { //  represents a half-open range, which includes the start value (0) but excludes the end value (path.len() - 1)
            // (uint reserveIn, uint reserveOut) = getReserves(factory, path[i], path[i + 1]);
            let (reserve_in, reserve_out) = Self::get_reserves(e.clone(), factory.clone(), path.get(i).unwrap(), path.get(i+1).unwrap());

            // amounts[i + 1] = getAmountOut(amounts[i], reserveIn, reserveOut);
            amounts.set(i+1, Self::get_amount_out(amounts.get(i).unwrap(), reserve_in, reserve_out))
        }
        amounts
    }

    // performs chained getAmountIn calculations on any number of pairs
    // function getAmountsIn(address factory, uint amountOut, address[] memory path) internal view returns (uint[] memory amounts) {
    fn get_amounts_in(e:Env, factory: Address, amount_out: i128, path: Vec<Address>) -> Vec<i128> {
        //     require(path.length >= 2, 'UniswapV2Library: INVALID_PATH');
        if path.len() < 2 {panic!("SoroswapLibrary: invalid path")};

        //     amounts = new uint[](path.length);
        //     amounts[amounts.length - 1] = amountOut;
        let mut amounts =  Vec::new(&e);
        amounts.set(0,amount_out); 

        // TODO: Find a more efficient way to do this
        // for (uint i = path.length - 1; i > 0; i--) {
        for i in (1..path.len()).rev() {
            // (uint reserveIn, uint reserveOut) = getReserves(factory, path[i - 1], path[i]);
            let (reserve_in, reserve_out) = Self::get_reserves( e.clone(), factory.clone(), path.get(i-1).unwrap(), path.get(i).unwrap());
            
            //  amounts[i - 1] = getAmountIn(amounts[i], reserveIn, reserveOut);
            let new_amount = Self::get_amount_in(amounts.get(0).unwrap(), reserve_in, reserve_out);
            // Adds the item to the front.
            // Increases the length by one, shifts all items up by one, and puts the item in the first position.
            amounts.push_front(new_amount)
        }
        amounts
    }

    /*
    function getAmountsIn(address factory, uint amountOut, address[] memory path) internal view returns (uint[] memory amounts) {
                require(path.length >= 2, 'UniswapV2Library: INVALID_PATH');
    
                amounts = new uint[](path.length);
                amounts[amounts.length - 1] = amountOut;
            
         
            for (uint i = path.length - 1; i > 0; i--) {
                (uint reserveIn, uint reserveOut) = getReserves(factory, path[i - 1], path[i]);
                amounts[i - 1] = getAmountIn(amounts[i], reserveIn, reserveOut);
            }
        }
    }    
     */


}
