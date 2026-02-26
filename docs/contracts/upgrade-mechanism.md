# upgrade-mechanism

## Public Methods

### `init`
```rust
pub fn init(
```

### `pause`
```rust
pub fn pause(env: Env, admin: Address) -> Result<(), Error>
```

### `unpause`
```rust
pub fn unpause(env: Env, admin: Address) -> Result<(), Error>
```

### `trigger_kill_switch`
```rust
pub fn trigger_kill_switch(
```

### `configure_test_gate`
```rust
pub fn configure_test_gate(
```

### `stage_upgrade`
```rust
pub fn stage_upgrade(
```

### `execute_upgrade`
```rust
pub fn execute_upgrade(env: Env, admin: Address) -> Result<(), Error>
```

### `rollback`
```rust
pub fn rollback(env: Env, admin: Address, reason_hash: BytesN<32>) -> Result<(), Error>
```

### `state`
```rust
pub fn state(env: Env) -> Result<UpgradeState, Error>
```

### `get_release`
```rust
pub fn get_release(env: Env, version: u32) -> Result<Option<ReleaseRecord>, Error>
```

### `set_ready`
```rust
pub fn set_ready(env: Env, ready: bool)
```

### `is_release_ready`
```rust
pub fn is_release_ready(env: Env, _suite: Symbol) -> bool
```

