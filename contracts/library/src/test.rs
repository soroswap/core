#![cfg(test)]
extern crate std;

use soroban_sdk::{testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    xdr::ToXdr,
    Address, 
    BytesN, 
    Env,
    Bytes,
    IntoVal,
    Symbol};

#[test]
fn test() {
    let e: Env = Default::default();
    e.mock_all_auths();

    

}
