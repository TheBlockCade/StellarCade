# Penalty & Slashing Contract

Soroban smart contract for defining violation rules and applying on-chain token slashes.

## Storage

| Key | Type | Description |
|-----|------|-------------|
| `Admin` | `Address` | Contract administrator |
| `Treasury` | `Address` | Recipient of slashed tokens |
| `NextPenaltyId` | `u64` | Auto-incremented penalty ID |
| `Violation(code)` | `PenaltyRule` | Slash amount per violation code (persistent) |
| `Penalty(id)` | `PenaltyRecord` | Per-penalty state (persistent) |

## Methods

| Method | Auth | Description |
|--------|------|-------------|
| `init(admin, treasury_contract)` | — | Initialize contract |
| `define_violation(code, rule)` | admin | Create/update a violation and its slash amount |
| `apply_penalty(account, code, context_hash, token)` | admin | Slash tokens from account to treasury |
| `appeal_penalty(penalty_id)` | penalized account | Mark penalty as under appeal |
| `penalty_state(penalty_id)` | — | Read penalty record |

## Events

| Topic | Data | Trigger |
|-------|------|---------|
| `vdef` | `ViolationDefined` | Violation rule created/updated |
| `applied` | `PenaltyApplied` | Penalty applied and tokens slashed |
| `appealed` | `PenaltyAppealed` | Penalty under appeal |

## Invariants

- Undefined violation codes are rejected.
- Only `Applied` penalties may be appealed.
- Slash amount must be non-negative.
- Token transfer uses `mock_all_auths_allowing_non_root_auth` in tests (token transfer from non-root authority).

## Dependencies

- `soroban-sdk = "25.0.2"`
- Requires a deployed SEP-41 token contract for slashing.
- Depends on a treasury address to receive slashed funds.
