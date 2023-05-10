#![cfg(test)]
extern crate std;

use crate::{SoroswapFactoryClient};
// use crate::{SoroswapPairClient};

use soroban_sdk::{testutils::Address as _,
                Address, 
                BytesN, 
                Env,
                token::Client as TokenClient}; // TODO; add when testing authorizations: IntoVal, Symbol};

fn create_token_contract(e: &Env, admin: &Address) -> TokenClient {
    TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
}



fn create_factory_contract(
    e: &Env,
    setter: &Address,
    pair_wasm_hash: BytesN<32>
) -> SoroswapFactoryClient {
    let factory = SoroswapFactoryClient::new(e, &e.register_contract(None, crate::SoroswapFactory {}));
    factory.initialize(&setter, &pair_wasm_hash);
    factory
}

fn pair_token_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../pair/target/wasm32-unknown-unknown/release/soroswap_pair_contract.wasm"
    );
    e.install_contract_wasm(WASM)
}

fn create_pair( e: &Env,
                factory: &SoroswapFactoryClient,
                token_0: &BytesN<32>,
                token_1: &BytesN<32>) {
    factory.create_pair(&token_0, &token_1);
    
    // TODO: Test the event emmited
}

#[test]
fn test() {
    let e: Env = Default::default();

    let mut admin = Address::random(&e);
    let mut fake_admin = Address::random(&e);
    
    let mut factory = create_factory_contract(&e, &admin, pair_token_wasm(&e));

    

    /*
    expect(await factory.feeTo()).to.eq(AddressZero)
    expect(await factory.feeToSetter()).to.eq(wallet.address)
    expect(await factory.allPairsLength()).to.eq(0)
    */

    // fee_to_setter is equal to admin / but is not equal to fake_admin
    assert_eq!(factory.fee_to_setter(), admin);
    assert_ne!(factory.fee_to_setter(), fake_admin);
    assert_eq!(factory.all_pairs_length(), 0);

    // TODO: Implement kind-of zero address to test:
    //assert_eq!(factory.fee_to(), ZERO_ADDRESS);
    
    // Create two tokens in order to create a pair using the factory
    let mut token_0 = create_token_contract(&e, &admin);
    let mut token_1 = create_token_contract(&e, &admin);

    create_pair(&e, &factory, &token_0.contract_id, &token_1.contract_id);

    // TODO: Test the created pair address:
    // expect(await factory.getPair(...tokens)).to.eq(create2Address)
    // expect(await factory.getPair(...tokens.slice().reverse())).to.eq(create2Address)

    // TODO: Test that the first all_pairs is the created address
    // expect(await factory.allPairs(0)).to.eq(create2Address)

    // TODO: Test that all_pairs_length now is equal to 1
    // expect(await factory.allPairsLength()).to.eq(1)
    assert_eq!(factory.all_pairs_length(), 1);

    // TODO: Test that the pair:
    //      - has been correctly created
    //      - has the factory address correctly
    //      - token_0 is correct
    //      - token_1 is correct

    // const pair = new Contract(create2Address, JSON.stringify(UniswapV2Pair.abi), provider)
    // expect(await pair.factory()).to.eq(factory.address)
    // expect(await pair.token0()).to.eq(TEST_ADDRESSES[0])
    // expect(await pair.token1()).to.eq(TEST_ADDRESSES[1])

}

// Creating the same pair again should fail
// await expect(factory.createPair(...tokens)).to.be.reverted // UniswapV2: PAIR_EXISTS
#[test]
#[should_panic(expected = "SoroswapFactory: pair already exist between token_0 and token_1")]
fn test_double_same_pair_not_possible() {
    let e: Env = Default::default();
    let mut admin = Address::random(&e);    
    let mut factory = create_factory_contract(&e, &admin, pair_token_wasm(&e));
    let mut token_0 = create_token_contract(&e, &admin);
    let mut token_1 = create_token_contract(&e, &admin);

    factory.create_pair(&token_0.contract_id, &token_1.contract_id);

    // Second creation of same pair should fail
    factory.create_pair(&token_0.contract_id, &token_1.contract_id);
}

// Creating the same pair again (but in inverse order) should also fail
// await expect(factory.createPair(...tokens.slice().reverse())).to.be.reverted // UniswapV2: PAIR_EXISTS

#[test]
#[should_panic(expected = "SoroswapFactory: pair already exist between token_0 and token_1")]
fn test_double_inverse_pair_not_possible() {
    let e: Env = Default::default();
    let mut admin = Address::random(&e);    
    let mut factory = create_factory_contract(&e, &admin, pair_token_wasm(&e));
    let mut token_0 = create_token_contract(&e, &admin);
    let mut token_1 = create_token_contract(&e, &admin);

    factory.create_pair(&token_0.contract_id, &token_1.contract_id);

    // Second creation of same pair (but now in reverse order) should fail
    factory.create_pair(&token_1.contract_id, &token_0.contract_id);
}

// TODO: Test: Should panic when other account tries to change the fee_to
// TODO: Test: Should panic when other account tries to change the fee_to_setter