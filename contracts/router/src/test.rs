#![cfg(test)]
extern crate std;

use crate::{SoroswapRouterClient};

use soroban_sdk::{Env};


fn create_router_contract<'a>(
    e: &'a Env,
) -> SoroswapRouterClient<'a> {
    let router = SoroswapRouterClient::new(e, &e.register_contract(None, crate::SoroswapRouter {}));
    router
}

#[test]
fn test() {
    let e: Env = Default::default();
    e.mock_all_auths();

    let router = create_router_contract(&e);

    assert_eq!(router.my_bool(), true);
    
    



    


}