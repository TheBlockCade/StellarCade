#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype,
    Address, Bytes, BytesN, Env, Vec,
};

#[contract]
pub struct OracleIntegration;

//
// ─────────────────────────────────────────────
// STORAGE
// ─────────────────────────────────────────────
//

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    OracleSources,
    Request(BytesN<32>),
    Latest(BytesN<32>),
}

#[derive(Clone)]
#[contracttype]
pub struct OracleRequest {
    pub feed_id: BytesN<32>,
    pub fulfilled: bool,
    pub payload: Bytes,
}

//
// ─────────────────────────────────────────────
// EVENTS
// ─────────────────────────────────────────────
//

#[contractevent]
pub struct Initialized {
    pub admin: Address,
}

#[contractevent]
pub struct RequestCreated {
    pub request_id: BytesN<32>,
    pub feed_id: BytesN<32>,
}

#[contractevent]
pub struct RequestFulfilled {
    pub request_id: BytesN<32>,
    pub feed_id: BytesN<32>,
}

//
// ─────────────────────────────────────────────
// ERRORS
// ─────────────────────────────────────────────
//

#[contracterror]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Error {
    AlreadyInitialized = 1,
    NotAuthorized = 2,
    RequestExists = 3,
    RequestNotFound = 4,
    AlreadyFulfilled = 5,
    InvalidInput = 6,
    OracleNotWhitelisted = 7,
}

//
// ─────────────────────────────────────────────
// CONTRACT IMPLEMENTATION
// ─────────────────────────────────────────────
//

#[contractimpl]
impl OracleIntegration {

    // ───────── INIT ─────────

    pub fn init(
        env: Env,
        admin: Address,
        oracle_sources_config: Vec<Address>,
    ) -> Result<(), Error> {

        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        if oracle_sources_config.is_empty() {
            return Err(Error::InvalidInput);
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::OracleSources, &oracle_sources_config);

        Initialized { admin }.publish(&env);

        Ok(())
    }

    // ───────── REQUEST DATA ─────────

    pub fn request_data(
        env: Env,
        caller: Address,
        feed_id: BytesN<32>,
        request_id: BytesN<32>,
    ) -> Result<(), Error> {

        caller.require_auth();

        let zero = BytesN::from_array(&env, &[0; 32]);
        if feed_id == zero || request_id == zero {
            return Err(Error::InvalidInput);
        }

        if env.storage().persistent().has(&DataKey::Request(request_id.clone())) {
            return Err(Error::RequestExists);
        }

        let request = OracleRequest {
            feed_id: feed_id.clone(),
            fulfilled: false,
            payload: Bytes::new(&env),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Request(request_id.clone()), &request);

        RequestCreated {
            request_id,
            feed_id,
        }
        .publish(&env);

        Ok(())
    }

    // ───────── FULFILL DATA ─────────

    pub fn fulfill_data(
        env: Env,
        caller: Address,
        request_id: BytesN<32>,
        payload: Bytes,
        _proof: Bytes,
    ) -> Result<(), Error> {

        if payload.is_empty() {
            return Err(Error::InvalidInput);
        }

        caller.require_auth();

        let sources: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::OracleSources)
            .ok_or(Error::NotAuthorized)?;

        if !sources.contains(&caller) {
            return Err(Error::OracleNotWhitelisted);
        }

        let mut request: OracleRequest = env
            .storage()
            .persistent()
            .get(&DataKey::Request(request_id.clone()))
            .ok_or(Error::RequestNotFound)?;

        if request.fulfilled {
            return Err(Error::AlreadyFulfilled);
        }

        request.fulfilled = true;
        request.payload = payload.clone();

        env.storage()
            .persistent()
            .set(&DataKey::Request(request_id.clone()), &request);

        env.storage()
            .persistent()
            .set(&DataKey::Latest(request.feed_id.clone()), &payload);

        let feed_id = request.feed_id.clone();

        RequestFulfilled {
        request_id,
        feed_id,
}
.publish(&env);

        Ok(())
    }

    // ───────── READ METHODS ─────────

    pub fn latest(env: Env, feed_id: BytesN<32>) -> Option<Bytes> {
        env.storage().persistent().get(&DataKey::Latest(feed_id))
    }

    pub fn get_request(
        env: Env,
        request_id: BytesN<32>,
    ) -> Option<OracleRequest> {
        env.storage().persistent().get(&DataKey::Request(request_id))
    }
}

//
// ─────────────────────────────────────────────
// TESTS
// ─────────────────────────────────────────────
//

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn setup() -> (Env, Address) {
        let env = Env::default();
        let contract_id = env.register(OracleIntegration, ());
        (env, contract_id)
    }

    #[test]
    fn test_full_flow() {
        let (env, contract_id) = setup();
        let client = OracleIntegrationClient::new(&env, &contract_id);

        let oracle = Address::generate(&env);
        let mut oracles = Vec::new(&env);
        oracles.push_back(oracle.clone());

        let feed = BytesN::from_array(&env, &[1; 32]);
        let req = BytesN::from_array(&env, &[2; 32]);
        let payload = Bytes::from_slice(&env, &[9, 9, 9]);

        env.mock_all_auths();

        client.init(&oracle, &oracles);
        client.request_data(&oracle, &feed, &req);
        client.fulfill_data(&oracle, &req, &payload, &Bytes::new(&env));

        let latest = client.latest(&feed).unwrap();
        assert_eq!(latest, payload);
    }

    #[test]
    fn test_duplicate_request() {
        let (env, contract_id) = setup();
        let client = OracleIntegrationClient::new(&env, &contract_id);

        let oracle = Address::generate(&env);
        let mut oracles = Vec::new(&env);
        oracles.push_back(oracle.clone());

        let feed = BytesN::from_array(&env, &[3; 32]);
        let req = BytesN::from_array(&env, &[4; 32]);

        env.mock_all_auths();

        client.init(&oracle, &oracles);
        client.request_data(&oracle, &feed, &req);

        let result = client.try_request_data(&oracle, &feed, &req);
        assert!(result.is_err());
    }

    #[test]
    fn test_refulfill_rejected() {
        let (env, contract_id) = setup();
        let client = OracleIntegrationClient::new(&env, &contract_id);

        let oracle = Address::generate(&env);
        let mut oracles = Vec::new(&env);
        oracles.push_back(oracle.clone());

        let feed = BytesN::from_array(&env, &[5; 32]);
        let req = BytesN::from_array(&env, &[6; 32]);
        let payload = Bytes::from_slice(&env, &[1, 2, 3]);

        env.mock_all_auths();

        client.init(&oracle, &oracles);
        client.request_data(&oracle, &feed, &req);
        client.fulfill_data(&oracle, &req, &payload, &Bytes::new(&env));

        let result = client.try_fulfill_data(&oracle, &req, &payload, &Bytes::new(&env));
        assert!(result.is_err());
    }
}