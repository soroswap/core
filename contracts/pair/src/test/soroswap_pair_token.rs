#![cfg(test)]
extern crate std;
use crate::test::{SoroswapPairTest, deposit::add_liquidity  };
use crate::soroswap_pair_token::{SoroswapPairToken, SoroswapPairTokenClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, IntoVal, Symbol,
};

#[test]
fn test() {
    let test = SoroswapPairTest::setup();
    test.env.mock_all_auths();

    let user1 = test.user.clone();
    let user2 = Address::generate(&test.env);
    let user3 = Address::generate(&test.env);
    
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0 = 2000;
    let amount_1 = 2000;
    add_liquidity(&test, &amount_0, &amount_1);
    // Less min amount 1000 mint should be 1000
    assert_eq!(test.contract.balance(&user1), 1000);
    assert_eq!(test.contract.total_supply(), 2000); // Total supply should be 1000 + 1000 of minimum liquidity

    test.contract.approve(&user2, &user3, &500, &200);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 500_i128, 200_u32).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(test.contract.allowance(&user2, &user3), 500);

    test.contract.transfer(&user1, &user2, &600);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("transfer"),
                    (&user1, &user2, 600_i128).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(test.contract.balance(&user1), 400);
    assert_eq!(test.contract.balance(&user2), 600);

    test.contract.transfer_from(&user3, &user2, &user1, &400);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            user3.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    Symbol::new(&test.env, "transfer_from"),
                    (&user3, &user2, &user1, 400_i128).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(test.contract.balance(&user1), 800);
    assert_eq!(test.contract.balance(&user2), 200);

    test.contract.transfer(&user1, &user3, &300);
    assert_eq!(test.contract.balance(&user1), 500);
    assert_eq!(test.contract.balance(&user3), 300);

    // Increase to 500
    test.contract.approve(&user2, &user3, &500, &200);
    assert_eq!(test.contract.allowance(&user2, &user3), 500);
    test.contract.approve(&user2, &user3, &0, &200);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 0_i128, 200_u32).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(test.contract.allowance(&user2, &user3), 0);
}

#[test]
fn test_burn() {
    let test = SoroswapPairTest::setup();
    test.env.mock_all_auths();

    let user1 = test.user.clone();
    let user2 = Address::generate(&test.env);

    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0 = 2000;
    let amount_1 = 2000;
    add_liquidity(&test, &amount_0, &amount_1);
    // Less min amount 1000 mint should be 1000

    assert_eq!(test.contract.total_supply(), 2000); // minimum liquidity (1000) + 1000 of user
    assert_eq!(test.contract.balance(&user1), 1000);

    test.contract.approve(&user1, &user2, &500, &200);
    assert_eq!(test.contract.allowance(&user1, &user2), 500);

    test.contract.burn_from(&user2, &user1, &500);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("burn_from"),
                    (&user2, &user1, 500_i128).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(test.contract.total_supply(), 1500); // plus 1000 of minimum liquidity
    assert_eq!(test.contract.allowance(&user1, &user2), 0);
    assert_eq!(test.contract.balance(&user1), 500);
    assert_eq!(test.contract.balance(&user2), 0);

    test.contract.burn(&user1, &500);
    assert_eq!(
        test.env.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    symbol_short!("burn"),
                    (&user1, 500_i128).into_val(&test.env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(test.contract.total_supply(), 1000); // minimum liquidity
    assert_eq!(test.contract.balance(&user1), 0);
    assert_eq!(test.contract.balance(&user2), 0);
}

#[test]
// #[should_panic(expected = "insufficient balance")]
#[should_panic]
fn transfer_insufficient_balance() {
    let test = SoroswapPairTest::setup();

    test.env.mock_all_auths();

    let user1 = test.user.clone();
    let user2 = Address::generate(&test.env);

    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0 = 2000;
    let amount_1 = 2000;
    add_liquidity(&test, &amount_0, &amount_1);
    // Less min amount 1000 mint should be 1000

    assert_eq!(test.contract.total_supply(), 2000); // + minimum liquidity 1000
    assert_eq!(test.contract.balance(&user1), 1000);

    test.contract.transfer(&user1, &user2, &1001);
}

#[test]
// #[should_panic(expected = "insufficient allowance")]
#[should_panic]
fn transfer_from_insufficient_allowance() {
    let test = SoroswapPairTest::setup();

    test.env.mock_all_auths();

    let user1 = test.user.clone();
    let user2 = Address::generate(&test.env);
    let user3 = Address::generate(&test.env);
    
    test.contract.initialize_pair(&test.factory.address, &test.token_0.address, &test.token_1.address);
    let amount_0 = 2000;
    let amount_1 = 2000;
    add_liquidity(&test, &amount_0, &amount_1);
    // Less min amount 1000 mint should be 1000

    assert_eq!(test.contract.total_supply(), 2000); // + minimum liquidity 1000
    assert_eq!(test.contract.balance(&user1), 1000);

    test.contract.approve(&user1, &user3, &100, &200);
    assert_eq!(test.contract.allowance(&user1, &user3), 100);

    test.contract.transfer_from(&user3, &user1, &user2, &101);
}

#[test]
fn test_zero_allowance() {
    // Here we test that transfer_from with a 0 amount does not create an empty allowance
    let test = SoroswapPairTest::setup();
    test.env.mock_all_auths();

    let spender = Address::generate(&test.env);
    let from = Address::generate(&test.env);

    let token_client = SoroswapPairTokenClient::new(&test.env, &test.env.register_contract(&test.contract.address, SoroswapPairToken {}));

    test.contract.transfer_from(&spender, &from, &spender, &0);
    assert!(token_client.get_allowance(&from, &spender).is_none());
}
