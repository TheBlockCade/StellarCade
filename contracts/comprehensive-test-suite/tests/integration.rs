#![cfg(test)]

use soroban_sdk::{
    contract, contractimpl, vec, Address, BytesN, Env, IntoVal, Symbol,
    testutils::Address as _,
};
use stellarcade_comprehensive_test_suite::ComprehensiveTestSuite;

#[contract]
struct GateConsumer;

#[contractimpl]
impl GateConsumer {
    pub fn read_gate(env: Env, gate_contract: Address, suite: Symbol) -> bool {
        env.invoke_contract::<bool>(
            &gate_contract,
            &Symbol::new(&env, "is_release_ready"),
            vec![&env, suite.into_val(&env)],
        )
    }
}

#[test]
fn integration_gate_read_flow() {
    let env = Env::default();

    let admin = Address::generate(&env);
    let reporter = Address::generate(&env);

    let gate_contract_id = env.register(ComprehensiveTestSuite, ());
    let gate = stellarcade_comprehensive_test_suite::ComprehensiveTestSuiteClient::new(&env, &gate_contract_id);

    let consumer_id = env.register(GateConsumer, ());
    let consumer = GateConsumerClient::new(&env, &consumer_id);

    let suite = Symbol::new(&env, "core_suite");

    env.mock_all_auths();
    gate.init(&admin, &reporter, &8_000u32);
    gate.register_suite(&admin, &suite, &5u32, &true, &false);

    let before = consumer.read_gate(&gate_contract_id, &suite);
    assert!(!before);

    let build_id = BytesN::from_array(&env, &[7u8; 32]);
    let ok = gate.record_run(
        &reporter,
        &suite,
        &build_id,
        &5u32,
        &0u32,
        &9_000u32,
        &true,
        &false,
    );
    assert!(ok);

    let after = consumer.read_gate(&gate_contract_id, &suite);
    assert!(after);
}
