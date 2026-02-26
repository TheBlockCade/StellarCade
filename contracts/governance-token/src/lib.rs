#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, 
    Address, Env, String, Symbol,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotAuthorized = 1,
    AlreadyInitialized = 2,
    InsufficientBalance = 3,
    Overflow = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    Name,
    Symbol,
    Decimals,
    Balance(Address),
    TotalSupply,
}

#[contract]
pub struct GovernanceToken;

#[contractimpl]
impl GovernanceToken {
    pub fn init(
        env: Env, 
        admin: Address, 
        name: String, 
        symbol: String, 
        decimals: u32
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Decimals, &decimals);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
        Ok(())
    }

    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NotAuthorized)?;
        admin.require_auth();

        if amount <= 0 {
            return Err(Error::Overflow);
        }

        let balance = Self::balance(env.clone(), to.clone());
        let new_balance = balance.checked_add(amount).ok_or(Error::Overflow)?;
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &new_balance);

        let total_supply = Self::total_supply(env.clone());
        let new_total_supply = total_supply.checked_add(amount).ok_or(Error::Overflow)?;
        env.storage().instance().set(&DataKey::TotalSupply, &new_total_supply);

        env.events().publish((symbol_short!("mint"), to), amount);
        Ok(())
    }

    pub fn burn(env: Env, from: Address, amount: i128) -> Result<(), Error> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).ok_or(Error::NotAuthorized)?;
        admin.require_auth();

        let balance = Self::balance(env.clone(), from.clone());
        if balance < amount {
            return Err(Error::InsufficientBalance);
        }

        let new_balance = balance.checked_sub(amount).ok_or(Error::Overflow)?;
        env.storage().persistent().set(&DataKey::Balance(from.clone()), &new_balance);

        let total_supply = Self::total_supply(env.clone());
        let new_total_supply = total_supply.checked_sub(amount).ok_or(Error::Overflow)?;
        env.storage().instance().set(&DataKey::TotalSupply, &new_total_supply);

        env.events().publish((symbol_short!("burn"), from), amount);
        Ok(())
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        from.require_auth();

        let balance_from = Self::balance(env.clone(), from.clone());
        if balance_from < amount {
            return Err(Error::InsufficientBalance);
        }

        let new_balance_from = balance_from.checked_sub(amount).ok_or(Error::Overflow)?;
        env.storage().persistent().set(&DataKey::Balance(from.clone()), &new_balance_from);

        let balance_to = Self::balance(env.clone(), to.clone());
        let new_balance_to = balance_to.checked_add(amount).ok_or(Error::Overflow)?;
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &new_balance_to);

        env.events().publish((symbol_short!("transfer"), from, to), amount);
        Ok(())
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(id)).unwrap_or(0i128)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0i128)
    }

    pub fn name(env: Env) -> String {
        env.storage().instance().get(&DataKey::Name).unwrap()
    }

    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&DataKey::Symbol).unwrap()
    }

    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Decimals).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_token_flow() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        let contract_id = env.register(GovernanceToken, ());
        let client = GovernanceTokenClient::new(&env, &contract_id);

        client.init(&admin, &String::from_str(&env, "StellarCade Governance"), &String::from_str(&env, "SCG"), &18);

        client.mint(&user1, &1000);
        assert_eq!(client.balance(&user1), 1000);
        assert_eq!(client.total_supply(), 1000);

        client.transfer(&user1, &user2, &400);
        assert_eq!(client.balance(&user1), 600);
        assert_eq!(client.balance(&user2), 400);

        client.burn(&user2, &100);
        assert_eq!(client.balance(&user2), 300);
        assert_eq!(client.total_supply(), 900);
    }
}
