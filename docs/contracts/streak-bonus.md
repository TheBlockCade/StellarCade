# streak-bonus

## Public Methods

### `init`
Initialize with admin and reward contract address. Call once.

```rust
pub fn init(env: Env, admin: Address, reward_contract: Address) -> Result<(), Error>
```

### `record_activity`
Record an activity for a user. Caller must be the user (require_auth) or admin.

```rust
pub fn record_activity(
```

### `current_streak`
Return the current streak count for a user.

```rust
pub fn current_streak(env: Env, user: Address) -> u32
```

### `claim_streak_bonus`
Claim streak bonus for the current streak. User must authorize. Updates last_claimed_streak.

```rust
pub fn claim_streak_bonus(env: Env, user: Address) -> Result<i128, Error>
```

### `reset_rules`
Reset streak rules. Admin only.

```rust
pub fn reset_rules(env: Env, admin: Address, config: StreakRules) -> Result<(), Error>
```

