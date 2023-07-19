#![cfg(test)]
extern crate std;

use crate::{SoroswapRouterClient};

use soroban_sdk::{  testutils::{Events, Ledger, LedgerInfo},
                    Vec,
                    RawVal,
                    vec,
                    testutils::Address as _,
                    Address, 
                    BytesN, 
                    Env,
                    IntoVal, Symbol};


#[test]
fn test() {
    let e: Env = Default::default();
    e.mock_all_auths();
    



    


}