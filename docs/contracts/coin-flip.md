# coin-flip

## Public Methods

### `init`
Initialize the coin flip game.  `house_edge_bps`: house edge in basis points (e.g., 250 = 2.5%).

```rust
pub fn init(
```

### `place_bet`
Player places a bet. Tokens are transferred into the contract. A randomness request is submitted to the RNG contract.  `side`: 0 = Heads, 1 = Tails.

```rust
pub fn place_bet(
```

### `resolve_bet`
Resolve a game after the oracle has fulfilled the RNG request. Anyone can call this — no auth needed since the outcome is deterministic.

```rust
pub fn resolve_bet(env: Env, game_id: u64) -> Result<(), Error>
```

### `get_game`
View a game's state.

```rust
pub fn get_game(env: Env, game_id: u64) -> Result<Game, Error>
```

