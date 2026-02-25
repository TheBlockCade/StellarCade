#![no_std]
#![allow(unexpected_cfgs)]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, BytesN, Env,
    Symbol,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotAuthorized = 3,
    InvalidInput = 4,
    DuplicateSuite = 5,
    SuiteNotFound = 6,
    DuplicateRun = 7,
    Overflow = 8,
    InvalidState = 9,
    ContractPaused = 10,
    AlreadyPaused = 11,
    NotPaused = 12,
    ContractKilled = 13,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuiteConfig {
    pub suite: Symbol,
    pub min_cases: u32,
    pub requires_integration: bool,
    pub requires_property: bool,
    pub active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunRecord {
    pub suite: Symbol,
    pub build_id: BytesN<32>,
    pub passed_cases: u32,
    pub failed_cases: u32,
    pub coverage_bps: u32,
    pub includes_integration: bool,
    pub includes_property: bool,
    pub gate_passed: bool,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunKey {
    pub suite: Symbol,
    pub build_id: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuiteState {
    pub admin: Address,
    pub reporter: Address,
    pub paused: bool,
    pub killed: bool,
    pub coverage_target_bps: u32,
    pub total_suites: u32,
    pub total_runs: u32,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Reporter,
    Paused,
    Killed,
    CoverageTargetBps,
    TotalSuites,
    TotalRuns,
    Suite(Symbol),
    LastSuccessfulRun(Symbol),
    Run(RunKey),
}

#[contractevent]
pub struct Initialized {
    pub admin: Address,
    pub reporter: Address,
    pub coverage_target_bps: u32,
}

#[contractevent]
pub struct ReporterChanged {
    pub old_reporter: Address,
    pub new_reporter: Address,
}

#[contractevent]
pub struct SuiteRegistered {
    #[topic]
    pub suite: Symbol,
    pub min_cases: u32,
    pub requires_integration: bool,
    pub requires_property: bool,
}

#[contractevent]
pub struct SuiteUpdated {
    #[topic]
    pub suite: Symbol,
    pub min_cases: u32,
    pub requires_integration: bool,
    pub requires_property: bool,
    pub active: bool,
}

#[contractevent]
pub struct RunRecorded {
    #[topic]
    pub suite: Symbol,
    pub build_id: BytesN<32>,
    pub gate_passed: bool,
    pub coverage_bps: u32,
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

#[contract]
pub struct ComprehensiveTestSuite;

#[contractimpl]
impl ComprehensiveTestSuite {
    pub fn init(
        env: Env,
        admin: Address,
        reporter: Address,
        coverage_target_bps: u32,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if coverage_target_bps > 10_000 {
            return Err(Error::InvalidInput);
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Reporter, &reporter);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.storage().instance().set(&DataKey::Killed, &false);
        env.storage()
            .instance()
            .set(&DataKey::CoverageTargetBps, &coverage_target_bps);
        env.storage().instance().set(&DataKey::TotalSuites, &0u32);
        env.storage().instance().set(&DataKey::TotalRuns, &0u32);

        Initialized {
            admin,
            reporter,
            coverage_target_bps,
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

    pub fn set_reporter(env: Env, admin: Address, reporter: Address) -> Result<(), Error> {
        require_admin(&env, &admin)?;
        require_not_killed(&env)?;

        let old_reporter = get_reporter(&env)?;
        env.storage().instance().set(&DataKey::Reporter, &reporter);

        ReporterChanged {
            old_reporter,
            new_reporter: reporter,
        }
        .publish(&env);

        Ok(())
    }

    pub fn register_suite(
        env: Env,
        admin: Address,
        suite: Symbol,
        min_cases: u32,
        requires_integration: bool,
        requires_property: bool,
    ) -> Result<(), Error> {
        require_admin(&env, &admin)?;
        require_ready_for_mutation(&env)?;
        validate_suite_input(&suite, min_cases)?;

        let key = DataKey::Suite(suite.clone());
        if env.storage().instance().has(&key) {
            return Err(Error::DuplicateSuite);
        }

        let cfg = SuiteConfig {
            suite: suite.clone(),
            min_cases,
            requires_integration,
            requires_property,
            active: true,
        };

        env.storage().instance().set(&key, &cfg);

        let new_total = get_u32(&env, &DataKey::TotalSuites)?
            .checked_add(1)
            .ok_or(Error::Overflow)?;
        env.storage().instance().set(&DataKey::TotalSuites, &new_total);

        SuiteRegistered {
            suite,
            min_cases,
            requires_integration,
            requires_property,
        }
        .publish(&env);

        Ok(())
    }

    pub fn update_suite(
        env: Env,
        admin: Address,
        suite: Symbol,
        min_cases: u32,
        requires_integration: bool,
        requires_property: bool,
        active: bool,
    ) -> Result<(), Error> {
        require_admin(&env, &admin)?;
        require_ready_for_mutation(&env)?;
        validate_suite_input(&suite, min_cases)?;

        let key = DataKey::Suite(suite.clone());
        if !env.storage().instance().has(&key) {
            return Err(Error::SuiteNotFound);
        }

        let cfg = SuiteConfig {
            suite: suite.clone(),
            min_cases,
            requires_integration,
            requires_property,
            active,
        };
        env.storage().instance().set(&key, &cfg);

        SuiteUpdated {
            suite,
            min_cases,
            requires_integration,
            requires_property,
            active,
        }
        .publish(&env);

        Ok(())
    }

    pub fn record_run(
        env: Env,
        reporter: Address,
        suite: Symbol,
        build_id: BytesN<32>,
        passed_cases: u32,
        failed_cases: u32,
        coverage_bps: u32,
        includes_integration: bool,
        includes_property: bool,
    ) -> Result<bool, Error> {
        require_reporter(&env, &reporter)?;
        require_ready_for_mutation(&env)?;

        let cfg = get_suite(&env, &suite)?;
        if !cfg.active {
            return Err(Error::InvalidState);
        }

        if coverage_bps > 10_000 {
            return Err(Error::InvalidInput);
        }

        let total_cases = passed_cases
            .checked_add(failed_cases)
            .ok_or(Error::Overflow)?;
        if total_cases < cfg.min_cases {
            return Err(Error::InvalidState);
        }

        let run_key = RunKey {
            suite: suite.clone(),
            build_id: build_id.clone(),
        };
        let storage_key = DataKey::Run(run_key);
        if env.storage().persistent().has(&storage_key) {
            return Err(Error::DuplicateRun);
        }

        let coverage_target_bps = get_u32(&env, &DataKey::CoverageTargetBps)?;
        let has_required_integration = !cfg.requires_integration || includes_integration;
        let has_required_property = !cfg.requires_property || includes_property;
        let has_no_failures = failed_cases == 0;
        let meets_coverage = coverage_bps >= coverage_target_bps;

        let gate_passed = has_required_integration
            && has_required_property
            && has_no_failures
            && meets_coverage;

        let record = RunRecord {
            suite: suite.clone(),
            build_id: build_id.clone(),
            passed_cases,
            failed_cases,
            coverage_bps,
            includes_integration,
            includes_property,
            gate_passed,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&storage_key, &record);

        if gate_passed {
            env.storage()
                .instance()
                .set(&DataKey::LastSuccessfulRun(suite.clone()), &build_id);
        }

        let new_total_runs = get_u32(&env, &DataKey::TotalRuns)?
            .checked_add(1)
            .ok_or(Error::Overflow)?;
        env.storage().instance().set(&DataKey::TotalRuns, &new_total_runs);

        RunRecorded {
            suite,
            build_id,
            gate_passed,
            coverage_bps,
        }
        .publish(&env);

        Ok(gate_passed)
    }

    pub fn is_release_ready(env: Env, suite: Symbol) -> Result<bool, Error> {
        require_initialized(&env)?;

        let cfg = get_suite(&env, &suite)?;
        if !cfg.active {
            return Ok(false);
        }

        Ok(env
            .storage()
            .instance()
            .has(&DataKey::LastSuccessfulRun(suite)))
    }

    pub fn get_suite(env: Env, suite: Symbol) -> Result<Option<SuiteConfig>, Error> {
        require_initialized(&env)?;
        Ok(env.storage().instance().get(&DataKey::Suite(suite)))
    }

    pub fn get_run(
        env: Env,
        suite: Symbol,
        build_id: BytesN<32>,
    ) -> Result<Option<RunRecord>, Error> {
        require_initialized(&env)?;

        let key = DataKey::Run(RunKey { suite, build_id });
        Ok(env.storage().persistent().get(&key))
    }

    pub fn state(env: Env) -> Result<SuiteState, Error> {
        require_initialized(&env)?;

        Ok(SuiteState {
            admin: get_admin(&env)?,
            reporter: get_reporter(&env)?,
            paused: is_paused(&env),
            killed: is_killed(&env),
            coverage_target_bps: get_u32(&env, &DataKey::CoverageTargetBps)?,
            total_suites: get_u32(&env, &DataKey::TotalSuites)?,
            total_runs: get_u32(&env, &DataKey::TotalRuns)?,
        })
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

fn get_u32(env: &Env, key: &DataKey) -> Result<u32, Error> {
    env.storage().instance().get(key).ok_or(Error::NotInitialized)
}

fn get_admin(env: &Env) -> Result<Address, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotInitialized)
}

fn get_reporter(env: &Env) -> Result<Address, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Reporter)
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

fn require_reporter(env: &Env, caller: &Address) -> Result<(), Error> {
    require_initialized(env)?;
    caller.require_auth();
    if get_reporter(env)? != *caller {
        return Err(Error::NotAuthorized);
    }
    Ok(())
}

fn get_suite(env: &Env, suite: &Symbol) -> Result<SuiteConfig, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Suite(suite.clone()))
        .ok_or(Error::SuiteNotFound)
}

fn is_paused(env: &Env) -> bool {
    env.storage().instance().get(&DataKey::Paused).unwrap_or(false)
}

fn is_killed(env: &Env) -> bool {
    env.storage().instance().get(&DataKey::Killed).unwrap_or(false)
}

fn validate_suite_input(_suite: &Symbol, min_cases: u32) -> Result<(), Error> {
    if min_cases == 0 {
        return Err(Error::InvalidInput);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, BytesN as _, Events as _},
        Address, Env,
    };

    fn random_hash(env: &Env) -> BytesN<32> {
        BytesN::random(env)
    }

    fn setup(env: &Env) -> (ComprehensiveTestSuiteClient<'_>, Address, Address) {
        let admin = Address::generate(env);
        let reporter = Address::generate(env);
        let contract_id = env.register(ComprehensiveTestSuite, ());
        let client = ComprehensiveTestSuiteClient::new(env, &contract_id);

        env.mock_all_auths();
        client.init(&admin, &reporter, &8_000u32);

        (client, admin, reporter)
    }

    #[test]
    fn register_and_record_successful_run_enables_release_gate() {
        let env = Env::default();
        let (client, admin, reporter) = setup(&env);
        let suite = Symbol::new(&env, "core_suite");

        env.mock_all_auths();
        client.register_suite(&admin, &suite, &10u32, &true, &false);

        let gate_ok = client.record_run(
            &reporter,
            &suite,
            &random_hash(&env),
            &10u32,
            &0u32,
            &8_500u32,
            &true,
            &false,
        );

        assert!(gate_ok);
        assert!(client.is_release_ready(&suite));

        let state = client.state();
        assert_eq!(state.total_suites, 1);
        assert_eq!(state.total_runs, 1);
    }

    #[test]
    fn unauthorized_reporter_is_rejected() {
        let env = Env::default();
        let (client, admin, _) = setup(&env);
        let suite = Symbol::new(&env, "core_suite");
        let attacker = Address::generate(&env);

        env.mock_all_auths();
        client.register_suite(&admin, &suite, &1u32, &false, &false);

        let result = client.try_record_run(
            &attacker,
            &suite,
            &random_hash(&env),
            &1u32,
            &0u32,
            &9_000u32,
            &false,
            &false,
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_run_is_rejected() {
        let env = Env::default();
        let (client, admin, reporter) = setup(&env);
        let suite = Symbol::new(&env, "dup_suite");
        let build = random_hash(&env);

        env.mock_all_auths();
        client.register_suite(&admin, &suite, &1u32, &false, &false);

        assert!(client.record_run(
            &reporter,
            &suite,
            &build,
            &1u32,
            &0u32,
            &8_000u32,
            &false,
            &false,
        ));

        let dup = client.try_record_run(
            &reporter,
            &suite,
            &build,
            &1u32,
            &0u32,
            &8_000u32,
            &false,
            &false,
        );

        assert!(dup.is_err());
    }

    #[test]
    fn required_integration_signal_is_enforced() {
        let env = Env::default();
        let (client, admin, reporter) = setup(&env);
        let suite = Symbol::new(&env, "int_suite");

        env.mock_all_auths();
        client.register_suite(&admin, &suite, &5u32, &true, &false);

        let gate_ok = client.record_run(
            &reporter,
            &suite,
            &random_hash(&env),
            &5u32,
            &0u32,
            &9_000u32,
            &false,
            &false,
        );

        assert!(!gate_ok);
        assert!(!client.is_release_ready(&suite));
    }

    #[test]
    fn paused_contract_blocks_recording() {
        let env = Env::default();
        let (client, admin, reporter) = setup(&env);
        let suite = Symbol::new(&env, "pause_suite");

        env.mock_all_auths();
        client.register_suite(&admin, &suite, &1u32, &false, &false);
        client.pause(&admin);

        let blocked = client.try_record_run(
            &reporter,
            &suite,
            &random_hash(&env),
            &1u32,
            &0u32,
            &9_000u32,
            &false,
            &false,
        );
        assert!(blocked.is_err());
    }

    #[test]
    fn kill_switch_blocks_future_mutations() {
        let env = Env::default();
        let (client, admin, _) = setup(&env);

        env.mock_all_auths();
        client.trigger_kill_switch(&admin, &random_hash(&env));

        let res = client.try_register_suite(
            &admin,
            &Symbol::new(&env, "blocked"),
            &1u32,
            &false,
            &false,
        );
        assert!(res.is_err());

        let state = client.state();
        assert!(state.killed);

    }
}
