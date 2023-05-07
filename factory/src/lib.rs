#![no_std]

// TODO: Implement the token interface in THIS contract
// TODO: Make Pair Trait
// TODO: Tell when token is a call of another contract (like token_a), and when it should be this PairToken
// Own tokens functions to be imported: balance, mint, transfer, initialize
// Client token functions: transfer

mod test;
//mod token;
mod create;

//use num_integer::Roots;
use soroban_sdk::{contractimpl, Address, Env, TryFromVal, RawVal, ConversionError, Vec, Map}; //Bytes, BytesN, ConversionError, Env, RawVal, TryFromVal, token::Client as TokenClient};
//use token::{Token, TokenTrait};

#[derive(Clone, Copy)]
#[repr(u32)]

pub enum DataKey {
    FeeTo = 0, // address public feeTo;
    FeeToSetter = 1, // address public feeToSetter;
    AllPairs = 2, //  address[] public allPairs;
    PairsMapping = 3,

}

impl TryFromVal<Env, DataKey> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

// TODO: Implement event PairCreated(address indexed token0, address indexed token1, address pair, uint);


fn get_fee_to(e: &Env) -> Address {
    e.storage().get_unchecked(&DataKey::FeeTo).unwrap()
}

fn get_fee_to_setter(e: &Env) -> Address {
    e.storage().get_unchecked(&DataKey::FeeToSetter).unwrap()
}

fn get_all_pairs(e: &Env) -> Vec<Address> {
    e.storage().get_unchecked(&DataKey::AllPairs).unwrap()
}

fn get_pairs_mapping(e: &Env) -> Map<Address, Map<Address,Address>> {
    e.storage().get_unchecked(&DataKey::PairsMapping).unwrap()
}

fn put_fee_to(e: &Env, to: Address) {
    e.storage().set(&DataKey::FeeTo, &to);
}

fn put_fee_to_setter(e: &Env, setter: Address) {
    e.storage().set(&DataKey::FeeToSetter, &setter);
}

fn put_all_pairs(e: &Env, all_pairs: Vec<Address>) {
    e.storage().set(&DataKey::AllPairs, &all_pairs);
}

fn put_pairs_mapping(e: &Env, pairs_mapping: Map<Address, Map<Address,Address>>) {
    e.storage().set(&DataKey::PairsMapping, &pairs_mapping)
}


pub trait SoroswapFactoryTrait{
    // Sets the fee_to_setter address
    fn initialize(e: Env, setter: Address);

    /*  *** Read only functions: *** */

    // feeTo is the recipient of the charge.
    // function feeTo() external view returns (address);
    fn fee_to(e: Env) -> Address;

    // The address allowed to change feeTo.
    // function feeToSetter() external view returns (address);
    fn fee_to_setter(e: Env) -> Address;

    // Returns the total number of pairs created through the factory so far.
    // function allPairsLength() external view returns (uint);  
    fn all_pairs_length(e: Env) -> u32;

    // Returns the address of the pair for token_a and token_b, if it has been created, else address(0) 
    // function getPair(address token_a, address token_b) external view returns (address pair);
    fn get_pair(e: Env, token_a: Address, token_b: Address) -> Address;

    // Returns the address of the nth pair (0-indexed) created through the factory, or address(0) if not enough pairs have been created yet.
    // function allPairs(uint) external view returns (address pair);
    fn all_pairs(e: Env, n: u32) -> Address;

    /*  *** State-Changing Functions: *** */

    // function setFeeTo(address) external;
    fn set_fee_to(e: Env, to: Address);

    // function setFeeToSetter(address) external;
    fn set_fee_to_setter(e: Env, setter: Address);
    
    //Creates a pair for token_a and token_b if one doesn't exist already.
    // function createPair(address token_a, address token_b) external returns (address pair);
    fn create_pair(e: Env, token_a: Address, token_b: Address) -> Address;
}

struct SoroswapFactory;

#[contractimpl]
impl SoroswapFactoryTrait for SoroswapFactory {
    // Sets the fee_to_setter address
    fn initialize(e: Env, setter: Address){
        // TODO: This should be called only once, and by the contract creator
        put_fee_to_setter(&e, setter);
    }

    /*  *** Read only functions: *** */

    // feeTo is the recipient of the charge.
    // function feeTo() external view returns (address);
    fn fee_to(e: Env) -> Address {
        get_fee_to(&e)
    }

    // The address allowed to change feeTo.
    // function feeToSetter() external view returns (address);
    fn fee_to_setter(e: Env) -> Address {
        get_fee_to_setter(&e)
    }

    // Returns the total number of pairs created through the factory so far.
    // function allPairsLength() external view returns (uint);  
    fn all_pairs_length(e: Env) -> u32{
        get_all_pairs(&e).len()
    }

    // Returns the address of the pair for token_a and token_b, if it has been created, else address(0) 
    // function getPair(address token_a, address token_b) external view returns (address pair);
    fn get_pair(e: Env, token_a: Address, token_b: Address) -> Address{
        /*
        In this code, unwrap() is called twice. The first unwrap()
        gets the Result<Map<Address, Address>, ConversionError> from
        the Option<Result<Map<Address, Address>, ConversionError>>,
        and the second unwrap() gets the Map<Address, Address> from the 
        Result<Map<Address, Address>, ConversionError>. 
        If either of these unwrap() calls fail because their respective
         values are None or Err, the function will panic.*/

        let first_map = get_pairs_mapping(&e).get(token_a).unwrap().unwrap();
        first_map.get(token_b).unwrap().unwrap()
    }

    // Returns the address of the nth pair (0-indexed) created through the factory, or address(0) if not enough pairs have been created yet.
    // function allPairs(uint) external view returns (address pair);
    fn all_pairs(e: Env, n: u32) -> Address{
        // TODO: Implement error if n does not exist
        get_all_pairs(&e).get_unchecked(n).unwrap()
    }

    /*  *** State-Changing Functions: *** */

    // function setFeeTo(address) external;
    fn set_fee_to(e: Env, to: Address){
        // TODO: Implement restriction
        // require(msg.sender == feeToSetter, 'UniswapV2: FORBIDDEN');
        
        put_fee_to(&e, to);
    }

    // function setFeeToSetter(address) external;
    fn set_fee_to_setter(e: Env, setter: Address){
        // TODO: Implement restriction
        // require(msg.sender == feeToSetter, 'UniswapV2: FORBIDDEN');
        
        put_fee_to_setter(&e, setter);
    }
    
    //Creates a pair for token_a and token_b if one doesn't exist already.
    // function createPair(address token_a, address token_b) external returns (address pair);
    fn create_pair(e: Env, token_a: Address, token_b: Address) -> Address{
        // TODO: Implement

        /*
        function createPair(address tokenA, address tokenB) external returns (address pair) {
        require(tokenA != tokenB, 'UniswapV2: IDENTICAL_ADDRESSES');
        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        require(token0 != address(0), 'UniswapV2: ZERO_ADDRESS');
        require(getPair[token0][token1] == address(0), 'UniswapV2: PAIR_EXISTS'); // single check is sufficient
        bytes memory bytecode = type(UniswapV2Pair).creationCode;
        bytes32 salt = keccak256(abi.encodePacked(token0, token1));
        assembly {
            pair := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }
        IUniswapV2Pair(pair).initialize(token0, token1);
        getPair[token0][token1] = pair;
        getPair[token1][token0] = pair; // populate mapping in the reverse direction
        allPairs.push(pair);
        emit PairCreated(token0, token1, pair, allPairs.length);
        }
        */
        e.current_contract_address()
    }
    

}
