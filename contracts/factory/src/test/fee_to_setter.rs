extern crate std;
use crate::test::SoroswapFactoryTest;
use soroban_sdk::{
    testutils::{AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke},
    IntoVal, Symbol,
};

#[test]
fn changing_with_mock_auth() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    // Let's check basic info
    assert_eq!(test.contract.fee_to_setter(), test.admin);
    assert_ne!(test.contract.fee_to_setter(), test.user);

    assert_eq!(test.contract.fee_to(), test.admin);
    assert_ne!(test.contract.fee_to(), test.user);

    assert_eq!(test.contract.all_pairs_length(), 0);
    assert_eq!(test.contract.fees_enabled(), false);

    //  MOCK THE SPECIFIC AUTHORIZATION
    test.contract
        .mock_auths(&[MockAuth {
            address: &test.admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.contract.address.clone(),
                fn_name: "set_fee_to_setter",
                args: (test.user.clone(),).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_fee_to_setter(&test.user);

    // CHECK THAT WE SAW IT IN THE PREVIOUS AUTORIZED TXS
    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    Symbol::new(&test.env, "set_fee_to_setter"),
                    (test.user.clone(),).into_val(&test.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(test.contract.fee_to_setter(), test.user);
    assert_ne!(test.contract.fee_to_setter(), test.admin);

    //  MOCK THE SPECIFIC AUTHORIZATION
    test.contract
        .mock_auths(&[MockAuth {
            address: &test.user.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.contract.address.clone(),
                fn_name: "set_fee_to",
                args: (test.user.clone(),).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_fee_to(&test.user);

    // CHECK THAT WE SAW IT IN THE PREVIOUS AUTORIZED TXS
    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.user.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    Symbol::new(&test.env, "set_fee_to"),
                    (test.user.clone(),).into_val(&test.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(test.contract.fee_to(), test.user);
    assert_ne!(test.contract.fee_to(), test.admin);

    //  MOCK THE SPECIFIC AUTHORIZATION
    test.contract
        .mock_auths(&[MockAuth {
            address: &test.user.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.contract.address.clone(),
                fn_name: "set_fees_enabled",
                args: (true,).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_fees_enabled(&true);

    // CHECK THAT WE SAW IT IN THE PREVIOUS AUTORIZED TXS
    assert_eq!(
        test.env.auths(),
        std::vec![(
            test.user.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    test.contract.address.clone(),
                    Symbol::new(&test.env, "set_fees_enabled"),
                    (true,).into_val(&test.env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(test.contract.fees_enabled(), true);
}

#[test]
#[should_panic]
fn changing_fee_to_setter_with_mock_auth_not_allowed() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    test.contract
        .mock_auths(&[MockAuth {
            address: &test.user.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.contract.address.clone(),
                fn_name: "set_fee_to_setter",
                args: (test.user.clone(),).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_fee_to_setter(&test.user);
}

#[test]
#[should_panic]
fn changing_fee_to_with_mock_auth_not_allowed() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    test.contract
        .mock_auths(&[MockAuth {
            address: &test.user.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.contract.address.clone(),
                fn_name: "set_fee_to",
                args: (test.user.clone(),).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_fee_to(&test.user);
}

#[test]
#[should_panic]
fn changing_fees_enabled_with_mock_auth_not_allowed() {
    let test = SoroswapFactoryTest::setup();
    test.contract.initialize(&test.admin, &test.pair_wasm);

    test.contract
        .mock_auths(&[MockAuth {
            address: &test.user.clone(),
            invoke: &MockAuthInvoke {
                contract: &test.contract.address.clone(),
                fn_name: "set_fees_enabled",
                args: (false,).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_fees_enabled(&false);
}
