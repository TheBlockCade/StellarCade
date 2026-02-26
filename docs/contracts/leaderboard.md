# leaderboard

## Public Methods

### `init`
Initialize the leaderboard contract with an admin.

```rust
pub fn init(env: Env, admin: Address) -> Result<(), Error>
```

### `set_authorized`
Authorize or deauthorize an address (e.g., a game contract) to submit scores.

```rust
pub fn set_authorized(
```

### `set_game_active`
Set a game's active status.

```rust
pub fn set_game_active(
```

### `submit_score`
Submit a score for a player in a game. Only authorized callers can submit scores.

```rust
pub fn submit_score(
```

### `update_rankings`
Explicitly request a ranking update for a game. In this implementation, it's mostly a placeholder as submit_score handles it, but can be used to re-validate the top list.

```rust
pub fn update_rankings(env: Env, game_id: Symbol) -> Result<(), Error>
```

### `top_players`
Get the top players for a game, up to a certain limit.

```rust
pub fn top_players(env: Env, game_id: Symbol, limit: u32) -> Result<Vec<ScoreEntry>, Error>
```

### `player_rank`
Get the rank of a player in a specific game (1-indexed). Returns 0 if player is not in the top leaderboard.

```rust
pub fn player_rank(env: Env, game_id: Symbol, player: Address) -> Result<u32, Error>
```

### `get_player_score`
Get a player's raw score.

```rust
pub fn get_player_score(env: Env, game_id: Symbol, player: Address) -> u64
```

