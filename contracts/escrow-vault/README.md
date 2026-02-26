# Escrow Vault Contract

Soroban smart contract for deterministic on-chain escrow between a payer and payee.

## Storage

| Key | Type | Description |
|-----|------|-------------|
| `Admin` | `Address` | Contract administrator |
| `Token` | `Address` | Accepted ERC-20/Stellar token |
| `NextId` | `u64` | Auto-incremented escrow ID counter |
| `Escrow(id)` | `EscrowState` | Per-escrow state (persistent) |

## Methods

| Method | Auth | Description |
|--------|------|-------------|
| `init(admin, token_address)` | — | Initialize contract (once only) |
| `create_escrow(payer, payee, amount, terms_hash)` | payer | Lock tokens into escrow; returns `escrow_id` |
| `release_escrow(caller, escrow_id)` | payer or admin | Release funds to payee |
| `cancel_escrow(escrow_id)` | admin | Cancel and return funds to payer |
| `escrow_state(escrow_id)` | — | Read escrow state |

## Events

| Topic | Data | Trigger |
|-------|------|---------|
| `created` | `EscrowCreated` | New escrow created |
| `released` | `EscrowReleased` | Escrow paid out to payee |
| `cancel` | `EscrowCancelled` | Escrow cancelled, refunded to payer |

## Invariants

- An escrow cannot be released or cancelled more than once.
- Only the payer or admin may release; only admin may cancel.
- `amount` must be positive.
- Double-`init` is rejected.

## Dependencies

- `soroban-sdk = "25.0.2"`
- Requires a deployed SEP-41 / Stellar asset token contract.
