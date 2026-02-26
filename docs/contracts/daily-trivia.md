# daily-trivia

## Public Methods

### `init`
```rust
pub fn init(
```

### `open_round`
```rust
pub fn open_round(
```

### `submit_answer`
```rust
pub fn submit_answer(
```

### `close_round`
```rust
pub fn close_round(env: Env, round_id: u64) -> Result<(), Error>
```

### `claim_reward`
```rust
pub fn claim_reward(env: Env, player: Address, round_id: u64) -> Result<i128, Error>
```

### `get_round`
```rust
pub fn get_round(env: Env, round_id: u64) -> Option<RoundData>
```

### `reserve`
```rust
pub fn reserve(env: Env, _admin: Address, game_id: u64, amount: i128)
```

### `release`
```rust
pub fn release(env: Env, _admin: Address, game_id: u64, amount: i128)
```

### `payout`
```rust
pub fn payout(env: Env, _admin: Address, _to: Address, game_id: u64, amount: i128)
```

### `set_balance`
```rust
pub fn set_balance(env: Env, user: Address, amount: i128)
```

### `credit`
```rust
pub fn credit(env: Env, _game: Address, user: Address, amount: i128, _reason: Symbol)
```

### `debit`
```rust
pub fn debit(env: Env, _game: Address, user: Address, amount: i128, _reason: Symbol)
```

### `balance_of`
```rust
pub fn balance_of(env: Env, user: Address) -> i128
```

