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

    // TODO: Expect to panic when trying to create the same pair
    // await expect(factory.createPair(...tokens)).to.be.reverted // UniswapV2: PAIR_EXISTS
    // SoroswapFactory: pair already exist between token_0 and token_1
    //factory.create_pair(&token_0, &token_1);



    // await expect(factory.createPair(...tokens.slice().reverse())).to.be.reverted // UniswapV2: PAIR_EXISTS
    // expect(await factory.getPair(...tokens)).to.eq(create2Address)
    // expect(await factory.getPair(...tokens.slice().reverse())).to.eq(create2Address)
    // expect(await factory.allPairs(0)).to.eq(create2Address)
    // expect(await factory.allPairsLength()).to.eq(1)
    // const pair = new Contract(create2Address, JSON.stringify(UniswapV2Pair.abi), provider)
    // expect(await pair.factory()).to.eq(factory.address)
    // expect(await pair.token0()).to.eq(TEST_ADDRESSES[0])
    // expect(await pair.token1()).to.eq(TEST_ADDRESSES[1])
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



    // let mut token1 = create_token_contract(&e, &admin1);
    // let mut token2 = create_token_contract(&e, &admin2);
//     if &token2.contract_id < &token1.contract_id {
//         std::mem::swap(&mut token1, &mut token2);
//         std::mem::swap(&mut admin1, &mut admin2);
//     }
//     let user1 = Address::random(&e);
//     let liqpool = create_liqpool_contract(
//         &e, 
//         &token1.contract_id,
//         &token2.contract_id,
//     );

//    // let contract_share: [u8; 32] = liqpool.share_id().into();
//     //let token_share = TokenClient::new(&e, &contract_share);
//     //let token_share = TokenClient::new(&e, &liqpool.contract_id); 

//     token1.mint(&user1, &1000);
//     assert_eq!(token1.balance(&user1), 1000);

//     token2.mint(&user1, &1000);
//     assert_eq!(token2.balance(&user1), 1000);

//     liqpool.deposit(&user1, &100, &100, &100, &100);
   
//     // TODO: Implement authorization tests after changing everything to own token
//     // assert_eq!(
//     //     e.recorded_top_authorizations(),
//     //     std::vec![(
//     //         user1.clone(),
//     //         liqpool.contract_id.clone(),
//     //         Symbol::short("deposit"),
//     //         (&user1, 100_i128, 100_i128, 100_i128, 100_i128).into_val(&e)
//     //     )]
//     // );

//     assert_eq!(liqpool.my_balance(&user1), 100);
//     assert_eq!(liqpool.my_balance(&liqpool.address()), 0);
//     assert_eq!(token1.balance(&user1), 900);
//     assert_eq!(token1.balance(&liqpool.address()), 100);
//     assert_eq!(token2.balance(&user1), 900);
//     assert_eq!(token2.balance(&liqpool.address()), 100);

//     liqpool.swap(&user1, &false, &49, &100);
//     // TODO: Implement authorization tests after changing everything to own token
//     // assert_eq!(
//     //     e.recorded_top_authorizations(),
//     //     std::vec![(
//     //         user1.clone(),
//     //         liqpool.contract_id.clone(),
//     //         Symbol::short("swap"),
//     //         (&user1, false, 49_i128, 100_i128).into_val(&e)
//     //     )]
//     // );

//     assert_eq!(token1.balance(&user1), 803);
//     assert_eq!(token1.balance(&liqpool.address()), 197);
//     assert_eq!(token2.balance(&user1), 949);
//     assert_eq!(token2.balance(&liqpool.address()), 51);

//     liqpool.withdraw(&user1, &100, &197, &51);

//     // TODO: Implement authorization tests after changing everything to own token
//     // assert_eq!(
//     //     e.recorded_top_authorizations(),
//     //     std::vec![(
//     //         user1.clone(),
//     //         liqpool.contract_id.clone(),
//     //         Symbol::short("withdraw"),
//     //         (&user1, 100_i128, 197_i128, 51_i128).into_val(&e)
//     //     )]
//     // );

//     assert_eq!(token1.balance(&user1), 1000);
//     assert_eq!(token2.balance(&user1), 1000);
//     assert_eq!(liqpool.my_balance(&user1), 0);
//     assert_eq!(token1.balance(&liqpool.address()), 0);
//     assert_eq!(token2.balance(&liqpool.address()), 0);
//     assert_eq!(liqpool.my_balance(&liqpool.address()), 0);
}

// Creating the same pair again should fail
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
#[test]
#[should_panic(expected = "SoroswapFactory: pair already exist between token_0 and token_1")]
fn test_double_inverse_pair_not_possible() {
    let e: Env = Default::default();
    let mut admin = Address::random(&e);    
    let mut factory = create_factory_contract(&e, &admin, pair_token_wasm(&e));
    let mut token_0 = create_token_contract(&e, &admin);
    let mut token_1 = create_token_contract(&e, &admin);

    factory.create_pair(&token_0.contract_id, &token_1.contract_id);

    // Second creation of same pair should fail
    factory.create_pair(&token_1.contract_id, &token_0.contract_id);
}