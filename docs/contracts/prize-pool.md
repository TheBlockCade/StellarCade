# prize-pool

Per-game reservation record.

## Public Methods

### `init`
Initialize the prize pool. May only be called once.  `token` must be a deployed SEP-41 contract address (e.g., the USDC Stellar Asset Contract). All `fund` and `payout` operations transfer tokens through this contract exclusively.

```rust
pub fn init(env: Env, admin: Address, token: Address) -> Result<(), Error>
```

### `fund`
Transfer `amount` tokens from `from` into the pool.  Any address may fund the pool (house top-up, admin, or a game contract forwarding a player's wager). The caller must sign an auth tree covering both this invocation and the downstream `token.transfer` sub-call.

```rust
pub fn fund(env: Env, from: Address, amount: i128) -> Result<(), Error>
```

### `reserve`
Earmark `amount` tokens from the available pool for a specific game.  Moves `amount` from `available` into a `Reservation(game_id)` entry. Calling reserve with a `game_id` that already has a reservation returns `GameAlreadyReserved` — this is the idempotency guard preventing a buggy game contract from double-drawing from the pool.

```rust
pub fn reserve(
```

### `release`
Return `amount` from a game's reservation back to the available pool.  Used when a game ends with leftover funds (e.g., no winner, partial payout remainder, or game cancelled). A partial release (`amount < remaining`) is valid. When `remaining` reaches zero the reservation entry is removed to avoid stale storage.

```rust
pub fn release(
```

### `payout`
Transfer `amount` tokens to `to` from a game's reservation. Admin only.  Multiple calls against the same `game_id` are permitted (e.g., one call per winner in a multi-winner game). Each call decrements `remaining`; the reservation is removed when `remaining` hits zero.  All accounting state is updated BEFORE the external `token.transfer` to eliminate reentrancy risk: if the token call panics, state reflects the attempted debit, preventing a retry from double-paying.

```rust
pub fn payout(
```

### `get_pool_state`
Returns a point-in-time snapshot of the pool's accounting state.

```rust
pub fn get_pool_state(env: Env) -> Result<PoolState, Error>
```

