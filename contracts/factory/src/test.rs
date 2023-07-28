#![cfg(test)]
extern crate std;
mod token {
    soroban_sdk::contractimport!(file = "../token/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}

use soroban_sdk::{testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    xdr::ToXdr,
    Address, 
    BytesN, 
    Env,
    Bytes,
    IntoVal,
    Symbol};

    use crate::{pair, SoroswapFactoryClient};
use token::TokenClient;


fn create_token_contract<'a>(e: &'a Env, admin: &'a Address) -> TokenClient<'a> {
    TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
}

fn create_factory_contract<'a>(
    e: &'a Env,
    setter: &'a Address,
    pair_wasm_hash: &'a BytesN<32>
) -> SoroswapFactoryClient<'a> {
    let factory = SoroswapFactoryClient::new(e, &e.register_contract(None, crate::SoroswapFactory {}));
    factory.initialize(&setter, pair_wasm_hash);
    factory
}

fn pair_token_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}


// fn guess_contract_address(
//     e: &Env,
//     pair_wasm_hash: &BytesN<32>,
//     token_a: &BytesN<32>,
//     token_b: &BytesN<32>,
// ) -> BytesN<32> {
//     // Create a new Bytes instance using the current environment
//     let token_0;
//     let token_1;

//     if token_a < token_b {
//         token_0 = token_a;
//         token_1 = token_b;
//     }
//     else {
//         token_0 = token_b;
//         token_1 = token_a;
//     }
//     let mut salt = Bytes::new(e);

//     // Append the bytes of token_a and token_b to the salt
//     salt.append(&token_0.clone().into());
//     salt.append(&token_1.clone().into());

//     // Hash the salt using SHA256 to generate a new BytesN<32> value
//     let salt = e.crypto().sha256(&salt);

//     // Return the hash without deploying the contract
//     salt
// }

/*
Function that will guess the contract address.
Currently is not working as expected.
TODO: Fix
*/
pub fn guess_contract_address(
    e: &Env,
    factory: &Address,
    token_a: &Address,
    token_b: &Address,
) -> BytesN<32> {
    let token_0;
    let token_1;
    if token_a < token_b {
        token_0 = token_a;
        token_1 = token_b;
    }
    else {
        token_0 = token_b;
        token_1 = token_a;
    }
    let mut salt = Bytes::new(e);
    salt.append(&factory.to_xdr(e));
    salt.append(&token_0.to_xdr(e));
    salt.append(&token_1.to_xdr(e));
    let salt_hash = e.crypto().sha256(&salt);
    // let contract_address = Address::try_from(&salt_hash.as_ref()[12..]);
    // contract_address.unwrap_or_else(|_| BytesN::zero())
    salt_hash
}


fn create_pair(
                factory: &SoroswapFactoryClient,
                token_0: &Address,
                token_1: &Address) {
    factory.create_pair(&token_0, &token_1);
    
    // TODO: Test the event emmited
}

#[test]
fn test() {
    let e: Env = Default::default();
    e.mock_all_auths();

    let admin = Address::random(&e);
    let fake_admin = Address::random(&e);
    let new_admin = Address::random(&e);
    
    let pair_token_wasm_binding = pair_token_wasm(&e);  
    let factory = create_factory_contract(&e, &admin, &pair_token_wasm_binding);

    /*
    expect(await factory.feeTo()).to.eq(AddressZero)
    expect(await factory.feeToSetter()).to.eq(wallet.address)
    expect(await factory.allPairsLength()).to.eq(0)
    */

    // fee_to_setter is equal to admin / but is not equal to fake_admin
    assert_eq!(factory.fee_to_setter(), admin);
    assert_ne!(factory.fee_to_setter(), fake_admin);
    assert_eq!(factory.all_pairs_length(), 0);
    assert_eq!(factory.fees_enabled(), false);

    // if the admin changes the fee_to_setter, test require_auth
    factory.set_fee_to_setter(&new_admin);

    assert_eq!(
        e.auths(),
        std::vec![(
            admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    factory.address.clone(),
                    Symbol::new(&e, "set_fee_to_setter"),
                    (new_admin.clone(),).into_val(&e)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(factory.fee_to_setter(), new_admin);
    assert_ne!(factory.fee_to_setter(), admin);


     // if the new_admin changes the fees_enabled, test require_auth
     factory.set_fees_enabled(&true);

     assert_eq!(
         e.auths(),
         std::vec![(
             new_admin.clone(),
             AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    factory.address.clone(),
                    Symbol::new(&e, "set_fees_enabled"),
                    (true,).into_val(&e)
                )),
                sub_invocations: std::vec![]
            }
         )]
     );
     assert_eq!(factory.fees_enabled(), true);
     


    // The new admin changes the fee_to to he the factory itself
    // This is just to not to create a dummy BytesN<32>
    factory.set_fee_to(&factory.address);
    assert_eq!(
        e.auths(),
        std::vec![(
            new_admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    factory.address.clone(),
                    Symbol::new(&e, "set_fee_to"),
                    (factory.address.clone(),).into_val(&e)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(factory.fee_to(), factory.address);




    // TODO: Implement kind-of zero address to test:
    // assert_eq!(factory.fee_to(), ZERO_ADDRESS);
    
    // Create two tokens in order to create a pair using the factory
    let mut token_0 = create_token_contract(&e, &admin);
    let mut token_1 = create_token_contract(&e, &admin);

    create_pair(&factory, &token_0.address, &token_1.address);

    assert_eq!(factory.pair_exists(&token_0.address, &token_1.address), true);
    assert_eq!(factory.pair_exists(&token_1.address, &token_0.address), true);

    let _pair_expected_address = guess_contract_address( &e,
                                                        &factory.address, 
                                                        &token_1.address, 
                                                        &token_0.address);
    let pair_address = factory.get_pair(&token_0.address, &token_1.address);
    let pair_address_inverted = factory.get_pair(&token_1.address, &token_0.address);


    // expect(await factory.getPair(...tokens)).to.eq(create2Address)
    // expect(await factory.getPair(...tokens.slice().reverse())).to.eq(create2Address)
    assert_eq!(&pair_address, &pair_address_inverted);
    
    // TODO: fix the guess_contract_address function and uncomment the following line
    //  assert_eq!(&pair_expected_address, &pair_address);

    // expect(await factory.allPairs(0)).to.eq(create2Address)   
    // TODO: fix the guess_contract_address function and uncomment the following line
    // assert_eq!(&factory.all_pairs(&0), &pair_expected_address);
    assert_eq!(&factory.all_pairs(&0), &pair_address);

    // Test that all_pairs_length now is equal to 1
    // expect(await factory.allPairsLength()).to.eq(1)
    assert_eq!(factory.all_pairs_length(), 1);

    // TODO: Test that the pair:
    //      - has been correctly created
    //      - has the factory address correctly
    //      - token_0 is correct
    //      - token_1 is correct

    // const pair = new Contract(create2Address, JSON.stringify(UniswapV2Pair.abi), provider)
    let pair_client = pair::Client::new(&e, &pair_address);
    // expect(await pair.factory()).to.eq(factory.address)
    assert_eq!(pair_client.factory(), factory.address);

    
    // TODO; DONT USE RESET
    e.budget().reset_unlimited(); 
    
    // expect(await pair.token0()).to.eq(TEST_ADDRESSES[0])
    // expect(await pair.token1()).to.eq(TEST_ADDRESSES[1])
    // Before comparing the token_0 and token_1 saved in the pair contract, we need
    // to be sure if they are in the correct order
    if &token_1.address < &token_0.address {
        std::mem::swap(&mut token_0, &mut token_1);
    }
    assert_eq!(&pair_client.token_0(), &token_0.address);
    assert_eq!(&pair_client.token_1(), &token_1.address);

}

//Creating the same pair again should fail
//  await expect(factory.createPair(...tokens)).to.be.reverted // UniswapV2: PAIR_EXISTS
#[test]
#[should_panic(expected = "SoroswapFactory: pair already exist between token_0 and token_1")]
fn test_double_same_pair_not_possible() {
    let e: Env = Default::default();
    e.mock_all_auths();
    let admin = Address::random(&e);  
    let pair_token_wasm_binding = pair_token_wasm(&e);  
    let factory = create_factory_contract(&e, &admin, &pair_token_wasm_binding);
    let token_0 = create_token_contract(&e, &admin);
    let token_1 = create_token_contract(&e, &admin);

    factory.create_pair(&token_0.address, &token_1.address);

    // Second creation of same pair should fail
    factory.create_pair(&token_0.address, &token_1.address);
}

// Creating the same pair again (but in inverse order) should also fail
// await expect(factory.createPair(...tokens.slice().reverse())).to.be.reverted // UniswapV2: PAIR_EXISTS

#[test]
#[should_panic(expected = "SoroswapFactory: pair already exist between token_0 and token_1")]
fn test_double_inverse_pair_not_possible() {
    let e: Env = Default::default();
    e.mock_all_auths();
    let admin = Address::random(&e);    
    let pair_token_wasm_binding = pair_token_wasm(&e);  
    let factory = create_factory_contract(&e, &admin, &pair_token_wasm_binding);
    let token_0 = create_token_contract(&e, &admin);
    let token_1 = create_token_contract(&e, &admin);

    factory.create_pair(&token_0.address, &token_1.address);

    // Second creation of same pair (but now in reverse order) should fail
    factory.create_pair(&token_1.address, &token_0.address);
}

// TODO: Test: Should panic when other account tries to change the fee_to
// TODO: Test: Should panic when other account tries to change the fee_to_setter