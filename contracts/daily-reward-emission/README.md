# Daily Reward Emission Contract

Soroban smart contract for configuring epoch-based daily reward schedules and claiming rewards.

## Storage

| Key | Type | Description |
|-----|------|-------------|
| `Admin` | `Address` | Contract administrator |
| `RewardPool` | `Address` | Source of reward tokens |
| `Schedule(id)` | `EmissionConfig` | Emission schedule config (persistent) |
| `EpochState(id)` | `EmissionEpochState` | Current epoch tracking (persistent) |
| `Claimed(id, epoch, user)` | `bool` | Double-claim guard (persistent) |

## Methods

| Method | Auth | Description |
|--------|------|-------------|
| `init(admin, reward_pool_contract)` | — | Initialize contract |
| `configure_emission(schedule_id, config)` | admin | Create or update an emission schedule |
| `emit_for_epoch(schedule_id)` | admin | Finalize current epoch and pull rewards from pool |
| `claim_daily_reward(user, schedule_id, epoch_id, amount)` | user | Claim reward for a specific epoch |
| `emission_state(epoch_id)` | — | Read emission state |

## Events

| Topic | Data | Trigger |
|-------|------|---------|
| `ecfg` | `EmissionConfigured` | Schedule created/updated |
| `emitted` | `EpochEmitted` | Epoch finalized, rewards pulled from pool |
| `claimed` | `RewardClaimed` | User claims reward |

## Invariants

- Each (user, schedule, epoch) may only claim once (reentrancy guard set before transfer).
- Epoch cannot be finalized before its duration elapses.
- `rewards_per_epoch` and `epoch_duration` must be positive.
- Claimed flag is set **before** token transfer to prevent re-entrancy.

## Dependencies

- `soroban-sdk = "25.0.2"`
- Requires a reward pool address pre-funded with the reward token.
- Reward amounts per user are determined off-chain and passed at claim time.
