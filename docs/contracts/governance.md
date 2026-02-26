# governance

## Public Methods

### `init`
Initialize governance with token and parameters.  `voting_period`: ledgers for voting (e.g., 17280 = ~1 day at 5s/ledger) `timelock_delay`: ledgers before execution (e.g., 86400 = ~5 days) `quorum_bps`: minimum participation (e.g., 400 = 4% of supply) `threshold_bps`: minimum approval (e.g., 6000 = 60% of votes cast)

```rust
pub fn init(
```

### `propose`
Create a new proposal. Anyone can propose.  `payload_hash`: SHA-256 of the action to execute (verified at execution)

```rust
pub fn propose(
```

### `vote`
Cast a vote on an active proposal.  `support`: true = for, false = against `weight`: voter's token balance at time of vote (verified on-chain)

```rust
pub fn vote(
```

### `queue`
Queue a succeeded proposal into the timelock. Anyone can call.  Requirements: voting ended, quorum reached, threshold met

```rust
pub fn queue(env: Env, proposal_id: u64) -> Result<(), Error>
```

### `execute`
Execute a queued proposal after timelock. Anyone can call.  `payload_hash_verify`: must match stored hash (prevents bait-and-switch)

```rust
pub fn execute(
```

### `cancel`
Admin can cancel a proposal at any state (emergency function)

```rust
pub fn cancel(env: Env, admin: Address, proposal_id: u64) -> Result<(), Error>
```

### `get_proposal`
Get proposal details

```rust
pub fn get_proposal(env: Env, proposal_id: u64) -> Result<Proposal, Error>
```

### `has_voted`
Check if an address has voted on a proposal

```rust
pub fn has_voted(env: Env, proposal_id: u64, voter: Address) -> bool
```

