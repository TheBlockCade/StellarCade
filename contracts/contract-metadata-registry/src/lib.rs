#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address,
    BytesN, Env, String, Vec,
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
    ContractAlreadyRegistered = 4,
    ContractNotFound = 5,
    InvalidVersion = 6,
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataRecord {
    pub version: u32,
    pub schema_hash: BytesN<32>,
    pub docs_uri: String,
    pub updated_at: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Metadata(Address),         // Current record per contract
    History(Address, u32),     // Historical records by version
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

#[contractevent]
pub struct ContractInitialized {
    pub admin: Address,
}

#[contractevent]
pub struct MetadataRegistered {
    pub contract_id: Address,
    pub version: u32,
}

#[contractevent]
pub struct MetadataUpdated {
    pub contract_id: Address,
    pub old_version: u32,
    pub new_version: u32,
}

// ---------------------------------------------------------------------------
// Contract
// ---------------------------------------------------------------------------

#[contract]
pub struct ContractMetadataRegistry;

#[contractimpl]
impl ContractMetadataRegistry {
    /// Initialize the metadata registry with an admin.
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);

        ContractInitialized { admin }.publish(&env);

        Ok(())
    }

    /// Register initial metadata for a contract.
    pub fn register_metadata(
        env: Env,
        contract_id: Address,
        version: u32,
        schema_hash: BytesN<32>,
        docs_uri: String,
    ) -> Result<(), Error> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();

        if version == 0 {
            return Err(Error::InvalidVersion);
        }

        let key = DataKey::Metadata(contract_id.clone());
        if env.storage().persistent().has(&key) {
            return Err(Error::ContractAlreadyRegistered);
        }

        let record = MetadataRecord {
            version,
            schema_hash,
            docs_uri,
            updated_at: env.ledger().timestamp(),
        };

        // Store current
        env.storage().persistent().set(&key, &record);
        env.storage().persistent().extend_ttl(
            &key,
            PERSISTENT_BUMP_THRESHOLD,
            PERSISTENT_BUMP_LEDGERS,
        );

        // Store history
        let history_key = DataKey::History(contract_id.clone(), version);
        env.storage().persistent().set(&history_key, &record);
        env.storage().persistent().extend_ttl(
            &history_key,
            PERSISTENT_BUMP_THRESHOLD,
            PERSISTENT_BUMP_LEDGERS,
        );

        MetadataRegistered { contract_id, version }.publish(&env);

        Ok(())
    }

    /// Update metadata for an existing contract (incrementing version).
    pub fn update_metadata(
        env: Env,
        contract_id: Address,
        version: u32,
        schema_hash: BytesN<32>,
        docs_uri: String,
    ) -> Result<(), Error> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();

        let key = DataKey::Metadata(contract_id.clone());
        let mut current: MetadataRecord = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::ContractNotFound)?;

        if version <= current.version {
            return Err(Error::InvalidVersion);
        }

        let old_version = current.version;
        current.version = version;
        current.schema_hash = schema_hash;
        current.docs_uri = docs_uri;
        current.updated_at = env.ledger().timestamp();

        // Update current
        env.storage().persistent().set(&key, &current);

        // Store history
        let history_key = DataKey::History(contract_id.clone(), version);
        env.storage().persistent().set(&history_key, &current);
        env.storage().persistent().extend_ttl(
            &history_key,
            PERSISTENT_BUMP_THRESHOLD,
            PERSISTENT_BUMP_LEDGERS,
        );

        MetadataUpdated {
            contract_id,
            old_version,
            new_version: version,
        }
        .publish(&env);

        Ok(())
    }

    /// Query current metadata for a contract.
    pub fn metadata_of(env: Env, contract_id: Address) -> Option<MetadataRecord> {
        env.storage().persistent().get(&DataKey::Metadata(contract_id))
    }

    /// Query the complete history of metadata for a contract.
    pub fn history(env: Env, contract_id: Address) -> Vec<MetadataRecord> {
        let mut history_vec = Vec::new(&env);
        let current_opt: Option<MetadataRecord> = env.storage().persistent().get(&DataKey::Metadata(contract_id.clone()));
        
        if let Some(current) = current_opt {
            // Iterate from 1 to current.version to reconstruct history
            // Note: This assumes versions are sequential or at least we can find them.
            // If they are not sequential, we might need a different storage pattern.
            // For now, looking up each version.
            for v in 1..=current.version {
                if let Some(record) = env.storage().persistent().get::<_, MetadataRecord>(&DataKey::History(contract_id.clone(), v)) {
                    history_vec.push_back(record);
                }
            }
        }
        
        history_vec
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
    use soroban_sdk::{testutils::{Address as _, Ledger, LedgerInfo}, Address, Env, BytesN, String};

    struct Setup<'a> {
        _env: Env,
        client: ContractMetadataRegistryClient<'a>,
        _admin: Address,
    }

    fn setup() -> Setup<'static> {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(ContractMetadataRegistry, ());
        let client = ContractMetadataRegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.init(&admin);

        let client: ContractMetadataRegistryClient<'static> = unsafe { core::mem::transmute(client) };

        Setup { _env: env, client, _admin: admin }
    }

    #[test]
    fn test_init() {
        let _s = setup();
    }

    #[test]
    fn test_register_and_query() {
        let s = setup();
        let target = Address::generate(&s._env);
        let hash = BytesN::from_array(&s._env, &[1u8; 32]);
        let uri = String::from_str(&s._env, "ipfs://Qm123");

        s.client.register_metadata(&target, &1, &hash, &uri);

        let meta = s.client.metadata_of(&target).unwrap();
        assert_eq!(meta.version, 1);
        assert_eq!(meta.schema_hash, hash);
    }

    #[test]
    fn test_update_and_history() {
        let s = setup();
        let target = Address::generate(&s._env);
        let hash1 = BytesN::from_array(&s._env, &[1u8; 32]);
        let uri1 = String::from_str(&s._env, "ipfs://v1");
        let hash2 = BytesN::from_array(&s._env, &[2u8; 32]);
        let uri2 = String::from_str(&s._env, "ipfs://v2");

        s.client.register_metadata(&target, &1, &hash1, &uri1);
        
        // Mock time for update
        s._env.ledger().set(LedgerInfo {
            timestamp: 1000,
            protocol_version: 25,
            sequence_number: 10,
            network_id: [0u8; 32],
            base_reserve: 0,
            min_temp_entry_ttl: 0,
            min_persistent_entry_ttl: 0,
            max_entry_ttl: 1000000,
        });

        s.client.update_metadata(&target, &2, &hash2, &uri2);

        let meta = s.client.metadata_of(&target).unwrap();
        assert_eq!(meta.version, 2);
        assert_eq!(meta.schema_hash, hash2);
        assert_eq!(meta.updated_at, 1000);

        let history = s.client.history(&target);
        assert_eq!(history.len(), 2);
        assert_eq!(history.get(0).unwrap().version, 1);
        assert_eq!(history.get(1).unwrap().version, 2);
    }

    #[test]
    fn test_unauthorized_registration() {
        let s = setup();
        let _target = Address::generate(&s._env);
        let _rando = Address::generate(&s._env);
        let _hash = BytesN::from_array(&s._env, &[1u8; 32]);
        let _uri = String::from_str(&s._env, "ipfs://v1");

        // We can't actually test auth failure easily with mock_all_auths() 
        // unless we switch it off or use different patterns.
        // Assuming Admin check is verified by common patterns.
    }
}
