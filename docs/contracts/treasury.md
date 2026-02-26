# treasury

## Public Methods

### `init`
```rust
pub fn init(env: Env, admin: Address, token_address: Address) -> Result<(), Error>
```

### `pause`
```rust
pub fn pause(env: Env, admin: Address) -> Result<(), Error>
```

### `unpause`
```rust
pub fn unpause(env: Env, admin: Address) -> Result<(), Error>
```

### `deposit`
```rust
pub fn deposit(env: Env, from: Address, amount: i128, reason: Symbol) -> Result<(), Error>
```

### `allocate`
```rust
pub fn allocate(
```

### `release`
```rust
pub fn release(env: Env, to: Address, amount: i128, purpose: Symbol) -> Result<(), Error>
```

### `treasury_state`
```rust
pub fn treasury_state(env: Env) -> Result<TreasuryState, Error>
```

