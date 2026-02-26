# number-guess

Full state for a single number-guess game.

## Public Methods

### `init`
Initialize the contract. May only be called once.  - `rng_contract`: address of the deployed Random Generator contract. - `prize_pool_contract`: reserved for future prize-pool integration. - `balance_contract`: the SEP-41 token used for wagers and payouts. - `min_wager` / `max_wager`: inclusive wager bounds enforced in `start_game`. - `house_edge_bps`: house take in basis points (e.g. 250 = 2.5 %).

```rust
pub fn init(
```

### `start_game`
Start a new number-guess game.  The player selects a range `[min, max]` and places a `wager`.  Tokens are transferred from the player to this contract immediately.  A randomness request is submitted to the RNG contract using the `game_id` as the request identifier, so `game_id` must be globally unique across the RNG contract's request space.  `game_id` is caller-provided and must not collide with any existing pending or fulfilled RNG request.

```rust
pub fn start_game(
```

### `submit_guess`
Lock in the player's guess for an open game.  Authorization is required from `game.player` — only the player who started the game can submit their guess.  The guess must lie within the `[min, max]` range declared in `start_game`.  The guess must be submitted while the RNG request is still pending so that the oracle cannot choose a server seed after observing the guess.

```rust
pub fn submit_guess(env: Env, game_id: u64, guess: u32) -> Result<(), Error>
```

### `resolve_game`
Resolve a game after the oracle has fulfilled the RNG request.  No authorization required — the outcome is fully deterministic from on-chain data once the RNG is fulfilled.  The secret number is derived as `min + (rng_result % range_size)` where `rng_result` is the value stored by the RNG contract.  Payout state is written before any token transfer to prevent reentrancy.

```rust
pub fn resolve_game(env: Env, game_id: u64) -> Result<(), Error>
```

### `get_game`
Return the full game state, or `GameNotFound` if the id is unknown.

```rust
pub fn get_game(env: Env, game_id: u64) -> Result<Game, Error>
```

