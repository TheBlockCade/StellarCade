# Contract Health Registry

Soroban smart contract for on-chain health monitoring and status reporting of other contracts.

## Storage

| Key | Type | Description |
|-----|------|-------------|
| `Admin` | `Address` | Contract administrator |
| `HealthPolicy(contract_id)` | `HealthPolicy` | Monitoring policy per contract (persistent) |
| `LatestHealth(contract_id)` | `HealthReport` | Most recent health report (persistent) |
| `HealthHistory(contract_id)` | `Vec<HealthReport>` | Bounded history of reports (persistent) |

## Methods

| Method | Auth | Description |
|--------|------|-------------|
| `init(admin)` | — | Initialize contract |
| `report_health(reporter, contract_id, status, details_hash)` | admin | Submit a health status report |
| `set_health_policy(contract_id, policy)` | admin | Configure `max_history` and `policy_type` for a contract |
| `health_of(contract_id)` | — | Get most recent health report |
| `history(contract_id)` | — | Get full bounded health history |

## Events

| Topic | Data | Trigger |
|-------|------|---------|
| `health` | `HealthReported` | New health report submitted |
| `policy` | `PolicySet` | Monitoring policy configured |

## Status Values

| Status | Meaning |
|--------|---------|
| `Healthy` | Contract operating normally |
| `Degraded` | Contract experiencing issues but functional |
| `Critical` | Contract in critical state; circuit-breaker action recommended |
| `Unknown` | No data or initial state |

## Invariants

- Only admin may report health (future: role-based reporters via circuit-breaker registry).
- History is trimmed to `max_history` (default: 10) — oldest entries removed first.
- `max_history` must be at least 1.
- Double-`init` is rejected.

## Dependencies

- `soroban-sdk = "25.0.2"`
- Feeds monitoring dashboards and circuit-breaker decisions across critical contracts.
