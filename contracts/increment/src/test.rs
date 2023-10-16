#![cfg(test)]

use super::{IncrementContract, IncrementContractClient};
use soroban_sdk::{testutils::Logs, Env};
use super::my_increment;

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(&1), 2);
    assert_eq!(client.increment(&2), 3);
    assert_eq!(client.increment(&3), 4);
    assert_eq!(my_increment(3), 4);

    std::println!("{}", env.logs().all().join("\n"));
}
