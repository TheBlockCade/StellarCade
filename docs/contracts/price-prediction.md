# price-prediction

## Public Methods

### `init`
Initialize the price prediction game.  `house_edge_bps`: house edge in basis points (e.g., 500 = 5%).

```rust
pub fn init(
```

### `open_market`
Open a new prediction market round. Admin only.  Queries the oracle for the current price of `asset` to set the opening price. `close_time` must be in the future.

```rust
pub fn open_market(
```

### `place_prediction`
Player places a prediction on an open round.  `direction`: 0 = Up, 1 = Down. Tokens are transferred from the player to the contract as escrow. Each player may only bet once per round.

```rust
pub fn place_prediction(
```

### `settle_round`
Settle a round after `close_time` has passed. Anyone can call this — the outcome is deterministic from the oracle.  A round is a push (all bets refunded) when: - Close price equals open price (flat market). - No bets were placed. - Only one side has bets (no opposing risk).

```rust
pub fn settle_round(env: Env, round_id: u64) -> Result<(), Error>
```

### `claim`
Claim winnings for a settled round. Winners receive their proportional share of the net pool. In a push round, all players receive a full refund of their wager.  Losers cannot claim (returns `NoPayout`).

```rust
pub fn claim(env: Env, player: Address, round_id: u64) -> Result<(), Error>
```

### `get_round`
View a round's state.

```rust
pub fn get_round(env: Env, round_id: u64) -> Result<RoundData, Error>
```

### `get_bet`
View a player's bet in a round.

```rust
pub fn get_bet(env: Env, round_id: u64, player: Address) -> Result<BetData, Error>
```

