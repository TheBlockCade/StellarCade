#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address,
    Env, Symbol,
};

// ---------------------------------------------------------------------------
// TTL / storage constants
// ---------------------------------------------------------------------------

const PERSISTENT_BUMP_LEDGERS: u32 = 518_400; // ~30 days
const PERSISTENT_BUMP_THRESHOLD: u32 = PERSISTENT_BUMP_LEDGERS - 100_800; // Renew ~7 days early

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NotAuthorized = 3,
    CallDenied = 4,
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyKey {
    pub source: Address,
    pub target: Address,
    pub selector: Symbol,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Policy(PolicyKey),
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

#[contractevent]
pub struct ContractInitialized {
    pub admin: Address,
}

#[contractevent]
pub struct CallAllowed {
    pub source: Address,
    pub target: Address,
    pub selector: Symbol,
}

#[contractevent]
pub struct CallDenied {
    pub source: Address,
    pub target: Address,
    pub selector: Symbol,
}

// ---------------------------------------------------------------------------
// Contract
// ---------------------------------------------------------------------------

#[contract]
pub struct CrossContractCallGuard;

#[contractimpl]
impl CrossContractCallGuard {
    /// Initialize the guard with an admin address.
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);

        ContractInitialized { admin }.publish(&env);

        Ok(())
    }

    /// Allow a specific cross-contract call. Admin only.
    pub fn allow_call(
        env: Env,
        source: Address,
        target: Address,
        selector: Symbol,
    ) -> Result<(), Error> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();

        let key = DataKey::Policy(PolicyKey {
            source: source.clone(),
            target: target.clone(),
            selector: selector.clone(),
        });

        env.storage().persistent().set(&key, &true);
        env.storage().persistent().extend_ttl(
            &key,
            PERSISTENT_BUMP_THRESHOLD,
            PERSISTENT_BUMP_LEDGERS,
        );

        CallAllowed { source, target, selector }.publish(&env);

        Ok(())
    }

    /// Deny (remove permission for) a specific cross-contract call. Admin only.
    pub fn deny_call(
        env: Env,
        source: Address,
        target: Address,
        selector: Symbol,
    ) -> Result<(), Error> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();

        let key = DataKey::Policy(PolicyKey {
            source: source.clone(),
            target: target.clone(),
            selector: selector.clone(),
        });

        env.storage().persistent().remove(&key);

        CallDenied { source, target, selector }.publish(&env);

        Ok(())
    }

    /// Assert that a call is allowed. Traps/Errs if not found or explicitly denied.
    pub fn assert_allowed(
        env: Env,
        source: Address,
        target: Address,
        selector: Symbol,
    ) -> Result<(), Error> {
        let key = DataKey::Policy(PolicyKey {
            source,
            target,
            selector,
        });

        if !env.storage().persistent().get::<_, bool>(&key).unwrap_or(false) {
            return Err(Error::CallDenied);
        }

        Ok(())
    }

    /// Check the state of a specific policy.
    pub fn policy_state(
        env: Env,
        source: Address,
        target: Address,
        selector: Symbol,
    ) -> bool {
        let key = DataKey::Policy(PolicyKey {
            source,
            target,
            selector,
        });

        env.storage().persistent().get(&key).unwrap_or(false)
    }

    // ---------------------------------------------------------------------------
    // Internal helpers
    // ---------------------------------------------------------------------------

    fn require_admin(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, symbol_short};

    struct Setup<'a> {
        _env: Env,
        client: CrossContractCallGuardClient<'a>,
        _admin: Address,
    }

    fn setup() -> Setup<'static> {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CrossContractCallGuard, ());
        let client = CrossContractCallGuardClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.init(&admin);

        let client: CrossContractCallGuardClient<'static> = unsafe { core::mem::transmute(client) };

        Setup { _env: env, client, _admin: admin }
    }

    #[test]
    fn test_init() {
        let _s = setup();
    }

    #[test]
    fn test_policy_allow_deny() {
        let s = setup();
        let source = Address::generate(&s._env);
        let target = Address::generate(&s._env);
        let selector = symbol_short!("swap");

        // Should be false by default
        assert!(!s.client.policy_state(&source, &target, &selector));

        // Allow
        s.client.allow_call(&source, &target, &selector);
        assert!(s.client.policy_state(&source, &target, &selector));

        // Deny
        s.client.deny_call(&source, &target, &selector);
        assert!(!s.client.policy_state(&source, &target, &selector));
    }

    #[test]
    fn test_assert_allowed() {
        let s = setup();
        let source = Address::generate(&s._env);
        let target = Address::generate(&s._env);
        let selector = symbol_short!("transfer");

        // Assert should fail initially
        let result = s.client.try_assert_allowed(&source, &target, &selector);
        assert!(result.is_err());

        // Allow
        s.client.allow_call(&source, &target, &selector);
        
        // Assert should pass
        let result = s.client.try_assert_allowed(&source, &target, &selector);
        assert!(result.is_ok());
    }
}
