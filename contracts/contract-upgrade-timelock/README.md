# Contract Upgrade Timelock

Soroban smart contract that enforces a mandatory delay between proposing and executing contract upgrades.

## Storage

| Key | Type | Description |
|-----|------|-------------|
| `Admin` | `Address` | Contract administrator |
| `MinDelay` | `u64` | Minimum delay in seconds before an upgrade may execute |
| `NextUpgradeId` | `u64` | Auto-incremented upgrade proposal ID |
| `Upgrade(id)` | `UpgradeRecord` | Per-upgrade record (persistent) |

## Methods

| Method | Auth | Description |
|--------|------|-------------|
| `init(admin, min_delay)` | — | Initialize contract |
| `queue_upgrade(target_contract, payload_hash, eta)` | admin | Propose an upgrade; `eta` must be ≥ `now + min_delay` |
| `cancel_upgrade(upgrade_id)` | admin | Cancel a queued upgrade |
| `execute_upgrade(upgrade_id)` | admin | Execute upgrade after timelock expires |
| `upgrade_state(upgrade_id)` | — | Read upgrade record |

## Events

| Topic | Data | Trigger |
|-------|------|---------|
| `queued` | `UpgradeQueued` | Upgrade proposal created |
| `cancel` | `UpgradeCancelled` | Upgrade cancelled |
| `executed` | `UpgradeExecuted` | Upgrade executed |

## Invariants

- `eta` must satisfy `eta >= now + min_delay`; rejected otherwise.
- Only `Queued` upgrades may be cancelled or executed.
- An upgrade may not be executed before its `eta`.
- Double-`init` is rejected.

## Dependencies

- `soroban-sdk = "25.0.2"`
- The actual WASM upgrade invocation is handled off-chain after `execute_upgrade` succeeds.
