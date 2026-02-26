# higher-lower

## Public Methods

### `init`
```rust
pub fn init(
```

### `place_prediction`
```rust
pub fn place_prediction(
```

### `resolve_game`
```rust
pub fn resolve_game(env: Env, game_id: u64) -> Result<(), Error>
```

### `get_game`
```rust
pub fn get_game(env: Env, game_id: u64) -> Option<GameData>
```

### `set_result`
```rust
pub fn set_result(env: Env, game_id: u64, result: u32)
```

### `is_ready`
```rust
pub fn is_ready(env: Env, game_id: u64) -> bool
```

### `get_result`
```rust
pub fn get_result(env: Env, game_id: u64) -> u32
```

