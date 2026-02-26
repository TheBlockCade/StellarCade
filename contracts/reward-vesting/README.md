# Reward Vesting Contract

Deterministic linear vesting of game rewards with cliff support. Built on Stellar / Soroban.

## Overview

The admin creates vesting schedules on behalf of users by locking tokens into the contract. A user can claim whatever portion has vested so far at any time after the cliff elapses. The admin can revoke a schedule, returning unvested tokens.

## Public Interface

| Method | Caller | Description |
|---|---|---|
| `init(admin, token_address)` | Admin | Initialise the contract once. |
| `create_vesting_schedule(user, amount, start, cliff, duration) -> u64` | Admin | Lock `amount` tokens and create a vesting schedule. Returns the schedule ID. |
| `claim_vested(user) -> i128` | User | Transfer all currently vested tokens to the user. |
| `revoke_schedule(schedule_id) -> i128` | Admin | Cancel a schedule, returning unvested tokens to the admin. |
| `vesting_state(user) -> Vec<VestingSchedule>` | Anyone | Return all vesting schedules for a user. |

## Vesting Formula

```
vested = amount * min(elapsed, duration) / duration   (after cliff)
vested = 0                                             (before cliff)
```

Where `elapsed = now - start_timestamp`.

## VestingSchedule Fields

| Field | Type | Description |
|---|---|---|
| `schedule_id` | `u64` | Unique ID. |
| `user` | `Address` | Beneficiary. |
| `amount` | `i128` | Total tokens locked. |
| `start_timestamp` | `u64` | UNIX seconds when vesting begins. |
| `cliff_seconds` | `u64` | Seconds after start before any claim is possible. |
| `duration_seconds` | `u64` | Total vesting window. |
| `claimed` | `i128` | Cumulative amount claimed. |
| `revoked` | `bool` | Whether the schedule was revoked. |

## Storage Schema

| Key | Type | Description |
|---|---|---|
| `Admin` | `Address` | Privileged administrator. |
| `Token` | `Address` | Reward token address. |
| `NextScheduleId` | `u64` | Monotonic schedule counter. |
| `ScheduleMap` | `Map<u64, VestingSchedule>` | All schedules by ID. |
| `UserSchedules(address)` | `Vec<u64>` | Schedule IDs per user (persistent). |

## Events

| Topic | Data | Description |
|---|---|---|
| `init` | `(admin, token)` | Contract initialised. |
| `scheduled` | `(user, schedule_id, amount)` | New schedule created. |
| `claimed` | `(user, amount)` | Tokens claimed. |
| `revoked` | `(schedule_id, user, unvested)` | Schedule cancelled. |

## Error Codes

| Code | Meaning |
|---|---|
| `NotInitialized` | Contract not yet initialised. |
| `AlreadyInitialized` | Duplicate `init`. |
| `Unauthorized` | Caller lacks privileges. |
| `InvalidAmount` | Amount <= 0. |
| `InvalidDuration` | Duration is zero. |
| `ScheduleNotFound` | Schedule ID does not exist. |
| `ScheduleRevoked` | Schedule already revoked. |
| `NothingToClaim` | No vested tokens available. |
| `ArithmeticError` | Integer overflow. |

## Invariants

- `claimed` is always <= `vested_amount(now)`.
- A revoked schedule can never be claimed after revocation.
- Unvested tokens are always returned to admin on revocation.

## Integration Assumptions

- The admin must hold sufficient token balance and approve the transfer before calling `create_vesting_schedule`.
- Depends on a SEP-41 / Stellar Asset Contract compatible token.
- Downstream reward-distribution contracts should call `create_vesting_schedule` after computing award amounts.

## Dependencies

- `soroban-sdk` 25.x
- Reward distribution contract (upstream token minting)

## Running Tests

```bash
cd contracts/reward-vesting
cargo test
```

Closes #156
