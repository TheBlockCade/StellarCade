#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Symbol,
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Role(Address, Symbol),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleAssigned {
    pub target: Address,
    pub role: Symbol,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoleRevoked {
    pub target: Address,
    pub role: Symbol,
}

#[contract]
pub struct ContractRoleRegistry;

#[contractimpl]
impl ContractRoleRegistry {
    /// Initializes the contract with an admin.
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Assigns a role to a given address. Requires admin authorization.
    pub fn assign_role(env: Env, target: Address, role: Symbol) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("Not initialized");
        admin.require_auth();

        let key = DataKey::Role(target.clone(), role.clone());
        if !env.storage().persistent().has(&key) {
            env.storage().persistent().set(&key, &());
            
            // Emit role assignment event
            env.events().publish((), RoleAssigned { target, role });
        }
    }

    /// Revokes a role. Requires admin authorization.
    pub fn revoke_role(env: Env, target: Address, role: Symbol) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("Not initialized");
        admin.require_auth();

        let key = DataKey::Role(target.clone(), role.clone());
        if env.storage().persistent().has(&key) {
            env.storage().persistent().remove(&key);
            
            // Emit role revocation event
            env.events().publish((), RoleRevoked { target, role });
        }
    }

    /// Public query method verifying if the target has the specific role.
    pub fn has_role(env: Env, target: Address, role: Symbol) -> bool {
        env.storage().persistent().has(&DataKey::Role(target, role))
    }

    /// Retrieves the current admin address.
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).expect("Not initialized")
    }
}

#[cfg(test)]
mod test;
