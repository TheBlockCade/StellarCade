# emergency-pause

## Public Methods

### `init`
Initialize with an admin who can pause/unpause. Can only be called once.

```rust
pub fn init(env: Env, admin: Address) -> Result<(), Error>
```

### `pause`
Pause the contract. Only callable by admin. Errors if already paused.

```rust
pub fn pause(env: Env, admin: Address) -> Result<(), Error>
```

### `unpause`
Unpause the contract. Only callable by admin. Errors if not paused.

```rust
pub fn unpause(env: Env, admin: Address) -> Result<(), Error>
```

### `is_paused`
Check if the contract is currently paused.

```rust
pub fn is_paused(env: Env) -> bool
```

### `require_not_paused`
Panics if the contract is paused. Call this at the top of any function that should be blocked during an emergency.  Usage from another contract: ```ignore use stellarcade_emergency_pause::require_not_paused; require_not_paused(&env); ```

```rust
pub fn require_not_paused(env: &Env)
```

### `is_paused_internal`
Read the pause flag from instance storage.

```rust
pub fn is_paused_internal(env: &Env) -> bool
```

