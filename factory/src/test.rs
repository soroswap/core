#![cfg(test)]
extern crate std;

use crate::{SoroswapFactoryClient};
// use crate::{SoroswapPairClient};

use soroban_sdk::{testutils::Address as _,
                Address, 
                BytesN, 
                Env}; // TODO; add when testing authorizations: IntoVal, Symbol};


// fn create_pair_contract(
//     e: &Env,
//     token_a: &BytesN<32>,
//     token_b: &BytesN<32>,
// ) -> SoroswapPairClient {
//     let pair = SoroswapPairClient::new(e, &e.register_contract(None, crate::SoroswapPair {}));
//     pair.initialize_pair(token_a, token_b);
//     pair
// }

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
