# Session Nonce Manager

A foundational anti-replay primitive for StellarCade contracts and signature-based actions on Stellar / Soroban.

## Overview

Each nonce is scoped to a `(account, purpose)` pair, ensuring that a given operation intent cannot be replayed across different contexts. Once consumed a nonce cannot be reused. The admin may revoke a nonce before it is consumed.

## Public Interface

| Method | Caller | Description |
|---|---|---|
| `init(admin)` | Admin | Initialise the contract once. |
| `issue_nonce(account, purpose) -> u64` | Admin or account | Issue the next nonce for the pair and return its value. |
| `consume_nonce(account, nonce, purpose)` | Account | Mark a nonce as used. Fails if already consumed or revoked. |
| `is_nonce_valid(account, nonce, purpose) -> bool` | Anyone | Returns `true` if the nonce was issued and not yet consumed/revoked. |
| `revoke_nonce(account, nonce)` | Admin | Administratively revoke a nonce before consumption. |

## Storage Schema

| Key | Type | Description |
|---|---|---|
| `Admin` | `Address` | Privileged administrator. |
| `NextNonce(account, purpose)` | `u64` | Next nonce counter (persistent). |
| `NonceUsed(account, purpose, nonce)` | `bool` | Consumed flag (persistent). |
| `NonceRevoked(account, purpose, nonce)` | `bool` | Revoked flag (persistent). |

## Events

| Topic | Data | Description |
|---|---|---|
| `init` | `(admin)` | Contract initialised. |
| `issued` | `(account, purpose, nonce)` | Nonce issued. |
| `consumed` | `(account, purpose, nonce)` | Nonce consumed. |
| `revoked` | `(account, nonce)` | Nonce revoked by admin. |

## Error Codes

| Code | Meaning |
|---|---|
| `NotInitialized` | `init` has not been called. |
| `AlreadyInitialized` | Duplicate `init` attempt. |
| `Unauthorized` | Caller lacks required privileges. |
| `NonceAlreadyUsed` | Replay attempt detected. |
| `NonceRevoked` | Nonce was administratively revoked. |
| `NonceNotFound` | Nonce was never issued. |
| `InvalidPurpose` | Empty purpose string. |

## Invariants

- Nonce counters are monotonically increasing.
- A consumed nonce is permanently marked and cannot be re-consumed.
- Revocation is permanent; a revoked nonce cannot become valid again.

## Integration Assumptions

- Callers are responsible for signing the payload that includes the nonce value and purpose string.
- The purpose string should be a short, namespaced label (e.g., `"withdraw:v1"`) to avoid accidental cross-action reuse.
- This contract does **not** validate signatures — it only tracks nonce lifecycle.

## Dependencies

No external contract dependencies. Depends only on `soroban-sdk`.

## Running Tests

```bash
cd contracts/session-nonce-manager
cargo test
```

Closes #153
