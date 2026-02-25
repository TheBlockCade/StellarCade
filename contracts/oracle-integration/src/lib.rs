#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype,
    Address, Bytes, BytesN, Env, Vec,
};

#[contract]
pub struct OracleIntegration;

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

#[contractimpl]
impl OracleIntegration {

    pub fn init(
        env: Env,
        admin: Address,
        oracle_sources_config: Vec<Address>,
    ) -> Result<(), Error> {

        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::OracleSources, &oracle_sources_config);

        Ok(())
    }

    pub fn request_data(
        env: Env,
        feed_id: BytesN<32>,
        request_id: BytesN<32>,
    ) -> Result<(), Error> {

        if feed_id.is_empty() || request_id.is_empty() {
            return Err(Error::InvalidInput);
        }

        if env.storage().persistent().has(&DataKey::Request(request_id.clone())) {
            return Err(Error::RequestExists);
        }

        let request = OracleRequest {
            feed_id,
            fulfilled: false,
            payload: Bytes::new(&env),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Request(request_id), &request);

        Ok(())
    }

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

        Ok(())
    }

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

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_full_flow() {
        let env = Env::default();
        let contract_id = env.register(OracleIntegration, ());
        let client = OracleIntegrationClient::new(&env, &contract_id);

        let oracle = Address::generate(&env);
        let attacker = Address::generate(&env);

        let mut oracles = Vec::new(&env);
        oracles.push_back(oracle.clone());

        let feed = BytesN::from_array(&env, &[1; 32]);
        let req = BytesN::from_array(&env, &[2; 32]);
        let payload = Bytes::from_slice(&env, &[9, 9, 9]);

        env.mock_all_auths();

        // Init
        client.init(&oracle, &oracles);

        // Request
        client.request_data(&feed, &req);

        // Fulfill (authorized)
        client.fulfill_data(&oracle, &req, &payload, &Bytes::new(&env));

        let latest = client.latest(&feed).unwrap();
        assert_eq!(latest, payload);

        // Unauthorized attempt
        let result = client.try_fulfill_data(&attacker, &req, &payload, &Bytes::new(&env));
        assert!(result.is_err());
    }
}