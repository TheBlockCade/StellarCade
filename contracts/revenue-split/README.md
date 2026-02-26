# Revenue Split Contract

Soroban smart contract for deterministic revenue distribution to multiple recipients using basis-point weights.

## Storage

| Key | Type | Description |
|-----|------|-------------|
| `Admin` | `Address` | Contract administrator |
| `Token` | `Address` | Token used for splits |
| `SplitConfig(stream_id)` | `SplitConfig` | Recipient weights per stream (persistent) |
| `StreamBalance(stream_id)` | `i128` | Pending undistributed balance (persistent) |
| `RecipientBalance(stream_id, addr)` | `i128` | Cumulative distributed amount (persistent) |

## Methods

| Method | Auth | Description |
|--------|------|-------------|
| `init(admin, token_address)` | — | Initialize contract |
| `set_split_config(stream_id, recipients)` | admin | Define recipients and their BPS weights (must sum to 10000) |
| `deposit_revenue(depositor, stream_id, amount)` | depositor | Deposit tokens into a stream |
| `distribute(stream_id)` | admin | Distribute all pending revenue proportionally |
| `recipient_balance(stream_id, recipient)` | — | Query cumulative distributed amount |

## Events

| Topic | Data | Trigger |
|-------|------|---------|
| `scfg` | `SplitConfigured` | Split configuration set |
| `deposit` | `RevenueDeposited` | Revenue deposited into stream |
| `distrib` | `RevenueDistributed` | Revenue distributed to recipients |

## Invariants

- Recipient weights must sum to exactly **10000 BPS** (100%).
- Stream balance is zeroed **before** transfers (reentrancy guard).
- Distribution requires a positive pending balance.
- At least one recipient is required.

## Dependencies

- `soroban-sdk = "25.0.2"`
- Depends on treasury/fee management for depositing revenue into streams.
- Depends on a deployed SEP-41 token contract.
