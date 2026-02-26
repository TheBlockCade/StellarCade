#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, symbol_short, Address,
    Env, Symbol, Vec, Map,
};

use stellarcade_shared::calculate_fee;

// ---------------------------------------------------------------------------
// TTL / storage constants
// ---------------------------------------------------------------------------

const PERSISTENT_BUMP_LEDGERS: u32 = 518_400; // ~30 days
const PERSISTENT_BUMP_THRESHOLD: u32 = PERSISTENT_BUMP_LEDGERS - 100_800; // Renew ~7 days early

const BASIS_POINTS_DIVISOR: u32 = 10_000;

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
    RuleNotFound = 4,
    RuleDisabled = 5,
    Overflow = 6,
    InvalidFeeConfig = 7,
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeeTier {
    pub threshold: i128,
    pub fee_bps: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeeRuleConfig {
    pub base_fee_bps: u32,
    pub tiers: Option<Vec<FeeTier>>,
    pub enabled: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeeContext {
    pub multiplier_bps: u32, // 10000 = 1x
    pub additional_data: Map<Symbol, i128>,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    FeeRule(Symbol), // Keyed by game_id
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

#[contractevent]
pub struct ContractInitialized {
    #[topic]
    pub admin: Address,
}

#[contractevent]
pub struct FeeRuleSet {
    #[topic]
    pub game_id: Symbol,
    pub base_fee_bps: u32,
    pub has_tiers: bool,
}

#[contractevent]
pub struct FeeRuleStatusChanged {
    #[topic]
    pub game_id: Symbol,
    pub enabled: bool,
}

#[contractevent]
pub struct FeeComputed {
    #[topic]
    pub game_id: Symbol,
    pub original_amount: i128,
    pub fee_amount: i128,
    pub applied_bps: u32,
}

// ---------------------------------------------------------------------------
// Contract
// ---------------------------------------------------------------------------

#[contract]
pub struct DynamicFeePolicy;

#[contractimpl]
impl DynamicFeePolicy {
    /// Initialise the contract.
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);

        ContractInitialized { admin }.publish(&env);

        Ok(())
    }

    /// Set a fee rule for a game.
    pub fn set_fee_rule(
        env: Env,
        game_id: Symbol,
        rule_config: FeeRuleConfig,
    ) -> Result<(), Error> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();

        // Basic validation
        if rule_config.base_fee_bps > BASIS_POINTS_DIVISOR {
            return Err(Error::InvalidFeeConfig);
        }
        if let Some(ref tiers) = rule_config.tiers {
            for tier in tiers.iter() {
                if tier.fee_bps > BASIS_POINTS_DIVISOR {
                    return Err(Error::InvalidFeeConfig);
                }
            }
        }

        let key = DataKey::FeeRule(game_id.clone());
        env.storage().persistent().set(&key, &rule_config);
        env.storage().persistent().extend_ttl(
            &key,
            PERSISTENT_BUMP_THRESHOLD,
            PERSISTENT_BUMP_LEDGERS,
        );

        FeeRuleSet {
            game_id,
            base_fee_bps: rule_config.base_fee_bps,
            has_tiers: rule_config.tiers.is_some(),
        }
        .publish(&env);

        Ok(())
    }

    /// Compute the fee for a given amount and context.
    pub fn compute_fee(
        env: Env,
        game_id: Symbol,
        amount: i128,
        context: FeeContext,
    ) -> Result<i128, Error> {
        let key = DataKey::FeeRule(game_id.clone());
        let rule: FeeRuleConfig = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::RuleNotFound)?;

        if !rule.enabled {
            return Err(Error::RuleDisabled);
        }

        // 1. Determine base bps (check tiers)
        let mut applied_bps = rule.base_fee_bps;
        if let Some(tiers) = rule.tiers {
            let mut highest_threshold = -1i128;
            for tier in tiers.iter() {
                if amount >= tier.threshold && tier.threshold > highest_threshold {
                    highest_threshold = tier.threshold;
                    applied_bps = tier.fee_bps;
                }
            }
        }

        // 2. Apply context multiplier
        // final_bps = (applied_bps * multiplier_bps) / 10000
        let final_bps = applied_bps
            .checked_mul(context.multiplier_bps)
            .and_then(|v| v.checked_div(BASIS_POINTS_DIVISOR))
            .ok_or(Error::Overflow)?;

        // 3. Calculate actual fee
        let fee_amount = match calculate_fee(amount, final_bps) {
            Ok(fee) => fee,
            Err(_) => return Err(Error::Overflow),
        };

        FeeComputed {
            game_id,
            original_amount: amount,
            fee_amount,
            applied_bps: final_bps,
        }
        .publish(&env);

        Ok(fee_amount)
    }

    /// Enable a fee rule.
    pub fn enable_rule(env: Env, game_id: Symbol) -> Result<(), Error> {
        Self::set_enabled_status(env, game_id, true)
    }

    /// Disable a fee rule.
    pub fn disable_rule(env: Env, game_id: Symbol) -> Result<(), Error> {
        Self::set_enabled_status(env, game_id, false)
    }

    /// Query the state of a fee rule.
    pub fn fee_rule_state(env: Env, game_id: Symbol) -> Option<FeeRuleConfig> {
        env.storage().persistent().get(&DataKey::FeeRule(game_id))
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    fn require_admin(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }

    fn set_enabled_status(env: Env, game_id: Symbol, status: bool) -> Result<(), Error> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();

        let key = DataKey::FeeRule(game_id.clone());
        let mut rule: FeeRuleConfig = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::RuleNotFound)?;

        rule.enabled = status;
        env.storage().persistent().set(&key, &rule);

        FeeRuleStatusChanged {
            game_id,
            enabled: status,
        }
        .publish(&env);

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, vec, Map};

    struct Setup<'a> {
        _env: Env,
        client: DynamicFeePolicyClient<'a>,
        _admin: Address,
    }

    fn setup() -> Setup<'static> {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(DynamicFeePolicy, ());
        let client = DynamicFeePolicyClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.init(&admin);

        let client: DynamicFeePolicyClient<'static> = unsafe { core::mem::transmute(client) };

        Setup {
            _env: env,
            client,
            _admin: admin,
        }
    }

    #[test]
    fn test_compute_base_fee() {
        let s = setup();
        let game = symbol_short!("game1");
        
        s.client.set_fee_rule(&game, &FeeRuleConfig {
            base_fee_bps: 500, // 5%
            tiers: None,
            enabled: true,
        });

        let context = FeeContext {
            multiplier_bps: 10_000, // 1x
            additional_data: Map::new(&s._env),
        };

        let fee = s.client.compute_fee(&game, &1000, &context);
        assert_eq!(fee, 50);
    }

    #[test]
    fn test_compute_tiered_fee() {
        let s = setup();
        let game = symbol_short!("game1");
        
        let tiers = vec![&s._env, 
            FeeTier { threshold: 1000, fee_bps: 300 }, // 3% for >= 1000
            FeeTier { threshold: 5000, fee_bps: 100 }, // 1% for >= 5000
        ];

        s.client.set_fee_rule(&game, &FeeRuleConfig {
            base_fee_bps: 500, // 5% base
            tiers: Some(tiers),
            enabled: true,
        });

        let context = FeeContext {
            multiplier_bps: 10_000, // 1x
            additional_data: Map::new(&s._env),
        };

        // Case 1: Below threshold
        assert_eq!(s.client.compute_fee(&game, &500, &context), 25); // 5% of 500

        // Case 2: In first tier
        assert_eq!(s.client.compute_fee(&game, &2000, &context), 60); // 3% of 2000

        // Case 3: In second tier
        assert_eq!(s.client.compute_fee(&game, &10000, &context), 100); // 1% of 10000
    }

    #[test]
    fn test_compute_context_multiplier() {
        let s = setup();
        let game = symbol_short!("game1");
        
        s.client.set_fee_rule(&game, &FeeRuleConfig {
            base_fee_bps: 1000, // 10%
            tiers: None,
            enabled: true,
        });

        // Promo: half fees
        let context = FeeContext {
            multiplier_bps: 5000, // 0.5x
            additional_data: Map::new(&s._env),
        };

        let fee = s.client.compute_fee(&game, &1000, &context);
        assert_eq!(fee, 50); // 10% halved = 5% -> 5% of 1000 = 50
    }

    #[test]
    fn test_disabled_rule() {
        let s = setup();
        let game = symbol_short!("game1");
        
        s.client.set_fee_rule(&game, &FeeRuleConfig {
            base_fee_bps: 500,
            tiers: None,
            enabled: false,
        });

        let context = FeeContext {
            multiplier_bps: 10_000,
            additional_data: Map::new(&s._env),
        };

        let result = s.client.try_compute_fee(&game, &1000, &context);
        assert_eq!(result, Err(Ok(Error::RuleDisabled)));
    }
}
