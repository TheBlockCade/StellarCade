#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotAuthorized = 1,
    InsufficientBalance = 2,
    InvalidAmount = 3,
    Overflow = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenConfig {
    pub name: Symbol,
    pub symbol: Symbol,
    pub decimals: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    Supply,
    Config,
    Balance(Address),
}

#[contract]
pub struct GovernanceToken;

#[contractimpl]
impl GovernanceToken {
    /// Initializes the contract with the admin address and token setup.
    pub fn init(env: Env, admin: Address, config: TokenConfig) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NotAuthorized); // Already initialized
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().set(&DataKey::Supply, &0i128);

        env.events().publish(
            (symbol_short!("init"),),
            (admin, config)
        );
        Ok(())
    }

    /// Mints new tokens to a recipient. Only admin can call.
    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NotAuthorized)?;
        admin.require_auth();

        let mut balance = self::GovernanceToken::balance_of(env.clone(), to.clone());
        balance = balance.checked_add(amount).ok_or(Error::Overflow)?;
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &balance);

        let mut supply: i128 = env.storage().instance().get(&DataKey::Supply).unwrap_or(0);
        supply = supply.checked_add(amount).ok_or(Error::Overflow)?;
        env.storage().instance().set(&DataKey::Supply, &supply);

        env.events().publish(
            (symbol_short!("mint"),),
            (to, amount)
        );
        Ok(())
    }

    /// Burns tokens from an account. Only admin can call.
    pub fn burn(env: Env, from: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NotAuthorized)?;
        admin.require_auth();

        let mut balance = self::GovernanceToken::balance_of(env.clone(), from.clone());
        if balance < amount {
            return Err(Error::InsufficientBalance);
        }
        balance -= amount;
        env.storage().persistent().set(&DataKey::Balance(from.clone()), &balance);

        let mut supply: i128 = env.storage().instance().get(&DataKey::Supply).unwrap_or(0);
        supply -= amount;
        env.storage().instance().set(&DataKey::Supply, &supply);

        env.events().publish(
            (symbol_short!("burn"),),
            (from, amount)
        );
        Ok(())
    }

    /// Transfers tokens between accounts. Requires sender authorization.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        from.require_auth();

        let mut from_balance = self::GovernanceToken::balance_of(env.clone(), from.clone());
        if from_balance < amount {
            return Err(Error::InsufficientBalance);
        }

        let mut to_balance = self::GovernanceToken::balance_of(env.clone(), to.clone());
        
        from_balance -= amount;
        to_balance = to_balance.checked_add(amount).ok_or(Error::Overflow)?;

        env.storage().persistent().set(&DataKey::Balance(from.clone()), &from_balance);
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &to_balance);

        env.events().publish(
            (symbol_short!("transfer"),),
            (from, to, amount)
        );
        Ok(())
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Supply).unwrap_or(0)
    }

    pub fn balance_of(env: Env, owner: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(owner)).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Events, MockAuth, MockAuthInvoke};
    use soroban_sdk::{IntoVal};

    #[test]
    fn test_init() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let contract_id = env.register(GovernanceToken, ());
        let client = GovernanceTokenClient::new(&env, &contract_id);

        let config = TokenConfig {
            name: Symbol::new(&env, "Governance"),
            symbol: Symbol::new(&env, "GOV"),
            decimals: 18,
        };

        client.init(&admin, &config);

        assert_eq!(client.total_supply(), 0);
    }

    #[test]
    fn test_mint() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let contract_id = env.register(GovernanceToken, ());
        let client = GovernanceTokenClient::new(&env, &contract_id);

        let config = TokenConfig {
            name: Symbol::new(&env, "Governance"),
            symbol: Symbol::new(&env, "GOV"),
            decimals: 18,
        };
        client.init(&admin, &config);

        client.mint(&user, &1000);

        assert_eq!(client.balance_of(&user), 1000);
        assert_eq!(client.total_supply(), 1000);
    }

    #[test]
    fn test_burn() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let contract_id = env.register(GovernanceToken, ());
        let client = GovernanceTokenClient::new(&env, &contract_id);

        let config = TokenConfig {
            name: Symbol::new(&env, "Governance"),
            symbol: Symbol::new(&env, "GOV"),
            decimals: 18,
        };
        client.init(&admin, &config);

        client.mint(&user, &1000);
        client.burn(&user, &400);

        assert_eq!(client.balance_of(&user), 600);
        assert_eq!(client.total_supply(), 600);
    }

    #[test]
    fn test_transfer() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        let contract_id = env.register(GovernanceToken, ());
        let client = GovernanceTokenClient::new(&env, &contract_id);

        let config = TokenConfig {
            name: Symbol::new(&env, "Governance"),
            symbol: Symbol::new(&env, "GOV"),
            decimals: 18,
        };
        client.init(&admin, &config);

        client.mint(&user1, &1000);
        client.transfer(&user1, &user2, &300);

        assert_eq!(client.balance_of(&user1), 700);
        assert_eq!(client.balance_of(&user2), 300);
        assert_eq!(client.total_supply(), 1000);
    }

    #[test]
    #[should_panic(expected = "Error(Auth, InvalidAction)")]
    fn test_unauthorized_mint() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let malicious = Address::generate(&env);
        let contract_id = env.register(GovernanceToken, ());
        let client = GovernanceTokenClient::new(&env, &contract_id);

        client.init(&admin, &TokenConfig {
            name: Symbol::new(&env, "G"),
            symbol: Symbol::new(&env, "G"),
            decimals: 0,
        });

        // Use mock_auths to simulate authorization from malicious address
        client.mock_auths(&[
            MockAuth {
                address: &malicious,
                invoke: &MockAuthInvoke {
                    contract: &contract_id,
                    fn_name: "mint",
                    args: (user.clone(), 1000i128).into_val(&env),
                    sub_invokes: &[],
                },
            },
        ]);

        client.mint(&user, &1000);
    }
}
