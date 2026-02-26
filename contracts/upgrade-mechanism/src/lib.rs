#![no_std]
#![allow(unexpected_cfgs)]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, vec, Address, BytesN,
    Env, IntoVal, Symbol,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotAuthorized = 3,
    InvalidInput = 4,
    InvalidStateTransition = 5,
    DuplicateVersion = 6,
    UpgradeNotStaged = 7,
    ContractPaused = 8,
    AlreadyPaused = 9,
    NotPaused = 10,
    ContractKilled = 11,
    TestGateFailed = 12,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseRecord {
    pub version: u32,
    pub wasm_hash: BytesN<32>,
    pub schema_version: u32,
    pub migration_hash: BytesN<32>,
    pub changelog_hash: BytesN<32>,
    pub activated_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PendingUpgrade {
    pub version: u32,
    pub wasm_hash: BytesN<32>,
    pub target_schema_version: u32,
    pub migration_hash: BytesN<32>,
    pub changelog_hash: BytesN<32>,
    pub staged_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestGateConfig {
    pub enabled: bool,
    pub gate_contract: Address,
    pub suite: Symbol,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpgradeState {
    pub admin: Address,
    pub paused: bool,
    pub killed: bool,
    pub current_version: u32,
    pub current_schema_version: u32,
    pub current_wasm_hash: BytesN<32>,
    pub has_pending_upgrade: bool,
    pub pending_version: u32,
    pub has_rollback_point: bool,
    pub rollback_version: u32,
    pub has_test_gate: bool,
    pub test_gate_enabled: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Paused,
    Killed,
    CurrentVersion,
    CurrentSchemaVersion,
    CurrentWasmHash,
    PendingUpgrade,
    RollbackPoint,
    TestGate,
    Release(u32),
}

#[contractevent]
pub struct Initialized {
    pub admin: Address,
    pub version: u32,
    pub schema_version: u32,
    pub wasm_hash: BytesN<32>,
}

#[contractevent]
pub struct UpgradeStaged {
    pub version: u32,
    pub target_schema_version: u32,
    pub staged_at: u64,
}

#[contractevent]
pub struct UpgradeExecuted {
    pub from_version: u32,
    pub to_version: u32,
    pub schema_version: u32,
    pub activated_at: u64,
}

#[contractevent]
pub struct RollbackExecuted {
    pub from_version: u32,
    pub to_version: u32,
    pub reason_hash: BytesN<32>,
}

#[contractevent]
pub struct PauseChanged {
    pub paused: bool,
    pub admin: Address,
}

#[contractevent]
pub struct KillSwitchTriggered {
    pub admin: Address,
    pub reason_hash: BytesN<32>,
}

#[contractevent]
pub struct TestGateConfigured {
    pub enabled: bool,
    pub gate_contract: Address,
    pub suite: Symbol,
}

#[contract]
pub struct UpgradeMechanism;

#[contractimpl]
impl UpgradeMechanism {
    pub fn init(
        env: Env,
        admin: Address,
        initial_wasm_hash: BytesN<32>,
        initial_schema_version: u32,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if initial_schema_version == 0 {
            return Err(Error::InvalidInput);
        }

        admin.require_auth();

        let initial_version = 1u32;
        let record = ReleaseRecord {
            version: initial_version,
            wasm_hash: initial_wasm_hash.clone(),
            schema_version: initial_schema_version,
            migration_hash: initial_wasm_hash.clone(),
            changelog_hash: initial_wasm_hash.clone(),
            activated_at: env.ledger().timestamp(),
        };

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.storage().instance().set(&DataKey::Killed, &false);
        env.storage()
            .instance()
            .set(&DataKey::CurrentVersion, &initial_version);
        env.storage()
            .instance()
            .set(&DataKey::CurrentSchemaVersion, &initial_schema_version);
        env.storage()
            .instance()
            .set(&DataKey::CurrentWasmHash, &initial_wasm_hash);
        env.storage()
            .persistent()
            .set(&DataKey::Release(initial_version), &record);

        Initialized {
            admin,
            version: initial_version,
            schema_version: initial_schema_version,
            wasm_hash: initial_wasm_hash,
        }
        .publish(&env);

        Ok(())
    }

    pub fn pause(env: Env, admin: Address) -> Result<(), Error> {
        require_admin(&env, &admin)?;
        require_not_killed(&env)?;

        if is_paused(&env) {
            return Err(Error::AlreadyPaused);
        }

        env.storage().instance().set(&DataKey::Paused, &true);
        PauseChanged { paused: true, admin }.publish(&env);
        Ok(())
    }

    pub fn unpause(env: Env, admin: Address) -> Result<(), Error> {
        require_admin(&env, &admin)?;
        require_not_killed(&env)?;

        if !is_paused(&env) {
            return Err(Error::NotPaused);
        }

        env.storage().instance().set(&DataKey::Paused, &false);
        PauseChanged {
            paused: false,
            admin,
        }
        .publish(&env);
        Ok(())
    }

    pub fn trigger_kill_switch(
        env: Env,
        admin: Address,
        reason_hash: BytesN<32>,
    ) -> Result<(), Error> {
        require_admin(&env, &admin)?;

        env.storage().instance().set(&DataKey::Killed, &true);
        env.storage().instance().set(&DataKey::Paused, &true);

        KillSwitchTriggered { admin, reason_hash }.publish(&env);
        Ok(())
    }

    pub fn configure_test_gate(
        env: Env,
        admin: Address,
        gate_contract: Address,
        suite: Symbol,
        enabled: bool,
    ) -> Result<(), Error> {
        require_admin(&env, &admin)?;
        require_not_killed(&env)?;

        let cfg = TestGateConfig {
            enabled,
            gate_contract: gate_contract.clone(),
            suite: suite.clone(),
        };

        env.storage().instance().set(&DataKey::TestGate, &cfg);
        TestGateConfigured {
            enabled,
            gate_contract,
            suite,
        }
        .publish(&env);
        Ok(())
    }

    pub fn stage_upgrade(
        env: Env,
        admin: Address,
        version: u32,
        wasm_hash: BytesN<32>,
        target_schema_version: u32,
        migration_hash: BytesN<32>,
        changelog_hash: BytesN<32>,
    ) -> Result<(), Error> {
        require_admin(&env, &admin)?;
        require_ready_for_mutation(&env)?;

        if version == 0 || target_schema_version == 0 {
            return Err(Error::InvalidInput);
        }

        let current_version = get_u32(&env, &DataKey::CurrentVersion)?;
        let current_schema_version = get_u32(&env, &DataKey::CurrentSchemaVersion)?;

        if version <= current_version {
            return Err(Error::InvalidStateTransition);
        }

        if target_schema_version != current_schema_version
            && target_schema_version != current_schema_version + 1
        {
            return Err(Error::InvalidStateTransition);
        }

        if env.storage().persistent().has(&DataKey::Release(version)) {
            return Err(Error::DuplicateVersion);
        }

        if env.storage().instance().has(&DataKey::PendingUpgrade) {
            return Err(Error::InvalidStateTransition);
        }

        let pending = PendingUpgrade {
            version,
            wasm_hash,
            target_schema_version,
            migration_hash,
            changelog_hash,
            staged_at: env.ledger().timestamp(),
        };

        env.storage().instance().set(&DataKey::PendingUpgrade, &pending);

        UpgradeStaged {
            version: pending.version,
            target_schema_version: pending.target_schema_version,
            staged_at: pending.staged_at,
        }
        .publish(&env);

        Ok(())
    }

    pub fn execute_upgrade(env: Env, admin: Address) -> Result<(), Error> {
        require_admin(&env, &admin)?;
        require_ready_for_mutation(&env)?;

        let pending = get_pending_upgrade(&env)?;

        if let Some(cfg) = get_test_gate(&env) {
            if cfg.enabled {
                let ready = env.invoke_contract::<bool>(
                    &cfg.gate_contract,
                    &Symbol::new(&env, "is_release_ready"),
                    vec![&env, cfg.suite.clone().into_val(&env)],
                );
                if !ready {
                    return Err(Error::TestGateFailed);
                }
            }
        }

        let previous = current_release(&env)?;
        let activated_at = env.ledger().timestamp();

        let new_release = ReleaseRecord {
            version: pending.version,
            wasm_hash: pending.wasm_hash.clone(),
            schema_version: pending.target_schema_version,
            migration_hash: pending.migration_hash,
            changelog_hash: pending.changelog_hash,
            activated_at,
        };

        env.storage().instance().set(&DataKey::RollbackPoint, &previous);
        env.storage()
            .instance()
            .set(&DataKey::CurrentVersion, &new_release.version);
        env.storage()
            .instance()
            .set(&DataKey::CurrentSchemaVersion, &new_release.schema_version);
        env.storage()
            .instance()
            .set(&DataKey::CurrentWasmHash, &new_release.wasm_hash);
        env.storage()
            .persistent()
            .set(&DataKey::Release(new_release.version), &new_release);
        env.storage().instance().remove(&DataKey::PendingUpgrade);

        UpgradeExecuted {
            from_version: previous.version,
            to_version: new_release.version,
            schema_version: new_release.schema_version,
            activated_at,
        }
        .publish(&env);

        Ok(())
    }

    pub fn rollback(env: Env, admin: Address, reason_hash: BytesN<32>) -> Result<(), Error> {
        require_admin(&env, &admin)?;
        require_not_killed(&env)?;

        let rollback_point: ReleaseRecord = env
            .storage()
            .instance()
            .get(&DataKey::RollbackPoint)
            .ok_or(Error::InvalidStateTransition)?;

        let current_version = get_u32(&env, &DataKey::CurrentVersion)?;
        if current_version == rollback_point.version {
            return Err(Error::InvalidStateTransition);
        }

        env.storage()
            .instance()
            .set(&DataKey::CurrentVersion, &rollback_point.version);
        env.storage().instance().set(
            &DataKey::CurrentSchemaVersion,
            &rollback_point.schema_version,
        );
        env.storage()
            .instance()
            .set(&DataKey::CurrentWasmHash, &rollback_point.wasm_hash);

        env.storage().instance().remove(&DataKey::PendingUpgrade);

        RollbackExecuted {
            from_version: current_version,
            to_version: rollback_point.version,
            reason_hash,
        }
        .publish(&env);

        Ok(())
    }

    pub fn state(env: Env) -> Result<UpgradeState, Error> {
        require_initialized(&env)?;

        let pending_upgrade: Option<PendingUpgrade> = env.storage().instance().get(&DataKey::PendingUpgrade);
        let rollback_point: Option<ReleaseRecord> = env.storage().instance().get(&DataKey::RollbackPoint);
        let test_gate: Option<TestGateConfig> = env.storage().instance().get(&DataKey::TestGate);

        Ok(UpgradeState {
            admin: get_admin(&env)?,
            paused: is_paused(&env),
            killed: is_killed(&env),
            current_version: get_u32(&env, &DataKey::CurrentVersion)?,
            current_schema_version: get_u32(&env, &DataKey::CurrentSchemaVersion)?,
            current_wasm_hash: env
                .storage()
                .instance()
                .get(&DataKey::CurrentWasmHash)
                .ok_or(Error::NotInitialized)?,
            has_pending_upgrade: pending_upgrade.is_some(),
            pending_version: pending_upgrade.map_or(0, |p| p.version),
            has_rollback_point: rollback_point.is_some(),
            rollback_version: rollback_point.map_or(0, |r| r.version),
            has_test_gate: test_gate.is_some(),
            test_gate_enabled: test_gate.map_or(false, |t| t.enabled),
        })
    }

    pub fn get_release(env: Env, version: u32) -> Result<Option<ReleaseRecord>, Error> {
        require_initialized(&env)?;
        Ok(env.storage().persistent().get(&DataKey::Release(version)))
    }
}

fn require_initialized(env: &Env) -> Result<(), Error> {
    if !env.storage().instance().has(&DataKey::Admin) {
        return Err(Error::NotInitialized);
    }
    Ok(())
}

fn require_not_paused(env: &Env) -> Result<(), Error> {
    if is_paused(env) {
        return Err(Error::ContractPaused);
    }
    Ok(())
}

fn require_not_killed(env: &Env) -> Result<(), Error> {
    if is_killed(env) {
        return Err(Error::ContractKilled);
    }
    Ok(())
}

fn require_ready_for_mutation(env: &Env) -> Result<(), Error> {
    require_initialized(env)?;
    require_not_killed(env)?;
    require_not_paused(env)?;
    Ok(())
}

fn get_admin(env: &Env) -> Result<Address, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotInitialized)
}

fn require_admin(env: &Env, caller: &Address) -> Result<(), Error> {
    require_initialized(env)?;
    caller.require_auth();
    if get_admin(env)? != *caller {
        return Err(Error::NotAuthorized);
    }
    Ok(())
}

fn is_paused(env: &Env) -> bool {
    env.storage().instance().get(&DataKey::Paused).unwrap_or(false)
}

fn is_killed(env: &Env) -> bool {
    env.storage().instance().get(&DataKey::Killed).unwrap_or(false)
}

fn get_u32(env: &Env, key: &DataKey) -> Result<u32, Error> {
    env.storage().instance().get(key).ok_or(Error::NotInitialized)
}

fn get_pending_upgrade(env: &Env) -> Result<PendingUpgrade, Error> {
    env.storage()
        .instance()
        .get(&DataKey::PendingUpgrade)
        .ok_or(Error::UpgradeNotStaged)
}

fn current_release(env: &Env) -> Result<ReleaseRecord, Error> {
    let current_version = get_u32(env, &DataKey::CurrentVersion)?;
    env.storage()
        .persistent()
        .get(&DataKey::Release(current_version))
        .ok_or(Error::NotInitialized)
}

fn get_test_gate(env: &Env) -> Option<TestGateConfig> {
    env.storage().instance().get(&DataKey::TestGate)
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, BytesN as _},
        Address, Env,
    };

    #[contract]
    struct MockGate;

    #[contracttype]
    #[derive(Clone)]
    enum GateDataKey {
        Ready,
    }

    #[contractimpl]
    impl MockGate {
        pub fn set_ready(env: Env, ready: bool) {
            env.storage().instance().set(&GateDataKey::Ready, &ready);
        }

        pub fn is_release_ready(env: Env, _suite: Symbol) -> bool {
            env.storage().instance().get(&GateDataKey::Ready).unwrap_or(false)
        }
    }

    fn random_hash(env: &Env) -> BytesN<32> {
        BytesN::random(env)
    }

    fn setup(env: &Env) -> (UpgradeMechanismClient<'_>, Address) {
        let admin = Address::generate(env);
        let contract_id = env.register(UpgradeMechanism, ());
        let client = UpgradeMechanismClient::new(env, &contract_id);

        env.mock_all_auths();
        client.init(&admin, &random_hash(env), &1u32);

        (client, admin)
    }

    #[test]
    fn init_and_state_are_consistent() {
        let env = Env::default();
        let (client, admin) = setup(&env);

        let state = client.state();
        assert_eq!(state.admin, admin);
        assert_eq!(state.current_version, 1);
        assert_eq!(state.current_schema_version, 1);
        assert!(!state.paused);
        assert!(!state.killed);
    }

    #[test]
    fn unauthorized_stage_is_rejected() {
        let env = Env::default();
        let (client, _) = setup(&env);
        let attacker = Address::generate(&env);

        env.mock_all_auths();
        let result = client.try_stage_upgrade(
            &attacker,
            &2u32,
            &random_hash(&env),
            &2u32,
            &random_hash(&env),
            &random_hash(&env),
        );
        assert!(result.is_err());
    }

    #[test]
    fn stage_execute_and_get_release_work() {
        let env = Env::default();
        let (client, admin) = setup(&env);
        env.mock_all_auths();

        client.stage_upgrade(
            &admin,
            &2u32,
            &random_hash(&env),
            &2u32,
            &random_hash(&env),
            &random_hash(&env),
        );
        client.execute_upgrade(&admin);

        let state = client.state();
        assert_eq!(state.current_version, 2);
        assert_eq!(state.current_schema_version, 2);
        assert!(client.get_release(&2u32).is_some());
    }

    #[test]
    fn duplicate_version_is_rejected() {
        let env = Env::default();
        let (client, admin) = setup(&env);
        env.mock_all_auths();

        client.stage_upgrade(
            &admin,
            &2u32,
            &random_hash(&env),
            &1u32,
            &random_hash(&env),
            &random_hash(&env),
        );
        client.execute_upgrade(&admin);

        let result = client.try_stage_upgrade(
            &admin,
            &2u32,
            &random_hash(&env),
            &1u32,
            &random_hash(&env),
            &random_hash(&env),
        );
        assert!(result.is_err());
    }

    #[test]
    fn pause_blocks_mutations() {
        let env = Env::default();
        let (client, admin) = setup(&env);
        env.mock_all_auths();

        client.pause(&admin);
        let result = client.try_stage_upgrade(
            &admin,
            &2u32,
            &random_hash(&env),
            &1u32,
            &random_hash(&env),
            &random_hash(&env),
        );
        assert!(result.is_err());
    }

    #[test]
    fn rollback_restores_previous_release() {
        let env = Env::default();
        let (client, admin) = setup(&env);
        env.mock_all_auths();

        client.stage_upgrade(
            &admin,
            &2u32,
            &random_hash(&env),
            &2u32,
            &random_hash(&env),
            &random_hash(&env),
        );
        client.execute_upgrade(&admin);

        client.rollback(&admin, &random_hash(&env));

        let state = client.state();
        assert_eq!(state.current_version, 1);
        assert_eq!(state.current_schema_version, 1);
    }

    #[test]
    fn test_gate_can_block_execute() {
        let env = Env::default();
        let (client, admin) = setup(&env);

        let gate_id = env.register(MockGate, ());
        let gate_client = MockGateClient::new(&env, &gate_id);

        env.mock_all_auths();
        gate_client.set_ready(&false);
        client.configure_test_gate(&admin, &gate_id, &Symbol::new(&env, "core_suite"), &true);

        client.stage_upgrade(
            &admin,
            &2u32,
            &random_hash(&env),
            &1u32,
            &random_hash(&env),
            &random_hash(&env),
        );

        let blocked = client.try_execute_upgrade(&admin);
        assert!(blocked.is_err());

        gate_client.set_ready(&true);
        client.execute_upgrade(&admin);

        let state = client.state();
        assert_eq!(state.current_version, 2);
    }

    #[test]
    fn kill_switch_freezes_contract() {
        let env = Env::default();
        let (client, admin) = setup(&env);
        env.mock_all_auths();

        client.trigger_kill_switch(&admin, &random_hash(&env));
        let res = client.try_unpause(&admin);
        assert!(res.is_err());

        let state = client.state();
        assert!(state.killed);
        assert!(state.paused);
    }
}
