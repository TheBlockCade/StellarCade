# staking

## Public Methods

### `init`
Initialise the staking contract.

```rust
pub fn init(
```

### `set_reward_rate`
Set the reward rate (admin only).

```rust
pub fn set_reward_rate(env: Env, admin: Address, rate: i128) -> Result<(), Error>
```

### `stake`
Stake tokens to earn rewards.

```rust
pub fn stake(env: Env, user: Address, amount: i128) -> Result<(), Error>
```

### `unstake`
Withdraw staked tokens and claim rewards.

```rust
pub fn unstake(env: Env, user: Address, amount: i128) -> Result<(), Error>
```

### `claim_rewards`
Claim accrued rewards.

```rust
pub fn claim_rewards(env: Env, user: Address) -> Result<i128, Error>
```

### `position_of`
View user position.

```rust
pub fn position_of(env: Env, user: Address) -> UserPosition
```

