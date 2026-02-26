# reward-distribution

## Public Methods

### `init`
Initialise the contract.  Can only be called once.  * `admin`             — privileged account for campaign management. * `treasury_contract` — address of the on-chain treasury holding campaign budgets (stored for composability). * `balance_contract`  — address of the token / balance contract used to settle claims.

```rust
pub fn init(
```

### `define_reward_campaign`
Define a new reward campaign.  Admin only.  * `campaign_id` — unique numeric identifier. * `rules_hash`  — SHA-256 of the off-chain eligibility rules document. * `budget`      — maximum tokens distributable; must be > 0.

```rust
pub fn define_reward_campaign(
```

### `accrue_reward`
Record a pending reward for `user`.  Admin only.  The call is additive — repeated calls accumulate until the user claims. The campaign's `remaining` balance is decremented immediately to uphold the invariant `Σ accrued ≤ budget`.

```rust
pub fn accrue_reward(
```

### `claim_reward`
Claim all accrued rewards for `user` in a campaign.  * The user must authenticate. * The reentrancy guard (`Claimed` flag) is set **before** any external settlement call. * Returns the amount of tokens claimed.

```rust
pub fn claim_reward(env: Env, user: Address, campaign_id: u32) -> Result<i128, Error>
```

### `campaign_state`
Return the current state of a campaign, or `None` if it does not exist.

```rust
pub fn campaign_state(env: Env, campaign_id: u32) -> Option<CampaignData>
```

### `accrued_for`
Return the unclaimed accrued balance for `user` in a campaign.

```rust
pub fn accrued_for(env: Env, user: Address, campaign_id: u32) -> i128
```

### `has_claimed`
Return whether `user` has already claimed from `campaign_id`.

```rust
pub fn has_claimed(env: Env, user: Address, campaign_id: u32) -> bool
```

