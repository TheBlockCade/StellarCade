# dice-roll

## Public Methods

### `init`
Initialize the dice roll game.  `house_edge_bps`: house edge in basis points (e.g., 250 = 2.5%).

```rust
pub fn init(
```

### `roll`
Player places a dice roll bet. Tokens are transferred into the contract. A randomness request is submitted to the RNG contract.  `prediction`: the die face the player predicts (1–6).

```rust
pub fn roll(
```

### `resolve_roll`
Resolve a game after the oracle has fulfilled the RNG request. Anyone can call this — no auth needed since the outcome is deterministic.

```rust
pub fn resolve_roll(env: Env, game_id: u64) -> Result<(), Error>
```

### `get_roll`
View a roll's state.

```rust
pub fn get_roll(env: Env, game_id: u64) -> Result<Roll, Error>
```

