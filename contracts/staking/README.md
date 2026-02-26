# Staking Contract

The `staking` contract allows users to stake a specific token and earn rewards over time in a potentially different reward token. It uses a fair "reward per token" distribution model.

## Public Interface

### `init(admin: Address, staking_token: Address, reward_token: Address)`
Initializes the contract with the administrator and token addresses.

### `stake(user: Address, amount: i128)`
Deposits `amount` of staking tokens from the `user` into the contract. Updates the user's reward accrual state.

### `unstake(user: Address, amount: i128)`
Withdraws `amount` of staked tokens back to the `user`. Accrued rewards are automatically calculated and added to the user's pending balance.

### `claim_rewards(user: Address) -> i128`
Transfers all accrued and pending rewards to the `user`. Returns the total amount claimed.

### `position_of(user: Address) -> UserPosition`
View function returning the user's current stake, reward debt, and pending rewards (including dynamic accruals since the last update).

### `set_reward_rate(admin: Address, rate: i128)`
Admin-only function to set the emission rate of reward tokens per second.

## Mathematical Model

The contract uses the standard accumulation model to avoid iterative loops:
1.  **Pool Update**: `reward_per_share_acc += (time_delta * reward_rate * precision) / total_staked`
2.  **User Reward Calculation**: `pending = (user_amount * reward_per_share_acc / precision) - user_reward_debt`
3.  **Debt Update**: `user_reward_debt = user_amount * reward_per_share_acc / precision`

## Storage Strategy

- **Instance Storage**: Stores global configuration and the `GlobalState` (pool data).
- **Persistent Storage**: Stores individual `UserPosition` data for stakers.

## Security & Invariants

- **Authorization**: `init` and `set_reward_rate` require admin authentication. Staking and claiming require the respective user's authentication.
- **Arithmetic Integrity**: Uses a `PRECISION` factor of 1e12 to prevent rounding errors in reward distribution.
- **Solvency**: The contract expects to be funded with reward tokens to satisfy claims.
