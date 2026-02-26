#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{symbol_short, Env};

#[test]
fn test_init_and_admin() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let contract_id = env.register(ContractRoleRegistry, ());
    let client = ContractRoleRegistryClient::new(&env, &contract_id);

    client.init(&admin);
    assert_eq!(client.get_admin(), admin);
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_already_initialized() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let contract_id = env.register(ContractRoleRegistry, ());
    let client = ContractRoleRegistryClient::new(&env, &contract_id);

    client.init(&admin);
    client.init(&admin);
}

#[test]
fn test_role_assignment() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let target = Address::generate(&env);
    let role = symbol_short!("GAME");

    let contract_id = env.register(ContractRoleRegistry, ());
    let client = ContractRoleRegistryClient::new(&env, &contract_id);

    client.init(&admin);

    assert_eq!(client.has_role(&target, &role), false);

    client.assign_role(&target, &role);
    assert_eq!(client.has_role(&target, &role), true);

    client.revoke_role(&target, &role);
    assert_eq!(client.has_role(&target, &role), false);
}

#[test]
#[should_panic]
fn test_unauthorized_assignment() {
    let env = Env::default();
    // We don't mock auth here to let it fail or we mock it for the wrong user
    
    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    let target = Address::generate(&env);
    let role = symbol_short!("GAME");

    let contract_id = env.register(ContractRoleRegistry, ());
    let client = ContractRoleRegistryClient::new(&env, &contract_id);

    client.init(&admin);

    // This should panic because non_admin is not admin
    env.set_authorizer(&non_admin);
    client.assign_role(&target, &role);
}
