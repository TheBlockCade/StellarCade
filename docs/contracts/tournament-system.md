# tournament-system

## Public Methods

### `init`
Initialize the tournament system. May only be called once.

```rust
pub fn init(
```

### `create_tournament`
Create a new tournament. Admin only.

```rust
pub fn create_tournament(
```

### `join_tournament`
Join an active tournament. Player pays entry fee.

```rust
pub fn join_tournament(env: Env, player: Address, id: u64) -> Result<(), Error>
```

### `record_result`
Record a score for a player in a tournament. Admin/Authorized only.

```rust
pub fn record_result(
```

### `finalize_tournament`
Finalize a tournament. Admin only. Prevents further joins or result recording.

```rust
pub fn finalize_tournament(env: Env, admin: Address, id: u64) -> Result<(), Error>
```

### `get_tournament`
```rust
pub fn get_tournament(env: Env, id: u64) -> Option<TournamentData>
```

### `get_score`
```rust
pub fn get_score(env: Env, id: u64, player: Address) -> Option<u64>
```

### `is_joined`
```rust
pub fn is_joined(env: Env, id: u64, player: Address) -> bool
```

