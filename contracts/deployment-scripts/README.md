# Contract Deployment Scripts

This library provides deterministic deployment orchestration for Stellarcade contracts across multiple profiles (`dev`, `testnet`, `mainnet`). By persisting state to a machine-readable format (JSON), it manages complex environments, handles incremental contract sequencing, and integrates seamlessly with backend parsers.

## Features & Responsibilities

1. **State Transitions & Invariants**: Enforces strict state transitions (`Pending` ➔ `Deployed` ➔ `Initialized`). Duplicate initializations explicitly rejected. Arithmetic and bounds constraints consistently evaluated.
2. **Environment Configuration**: Multi-tier tracking for diverse network footprints. Allows separate state directories per target environment.
3. **Machine-Readable Outputs**: Addresses and hashes for deployed contracts immediately commit incrementally to a JSON dictionary.
4. **Authorization & Validation**: Explicit admin validation required. Role checks bound all state mutations to the deployment initiator.
5. **Composability**: Integrable as a Rust library into CLI interfaces or standard API wrappers.

## Storage
State is structurally retained using standard `serde_json` format, mapping string contract aliases to explicit runtime states.

## Events
Runtime occurrences dynamically output events via stdout for structured pipeline logs:
- `EVENT: Contract {name} deployed to {address} (WASM: {wasm_hash})`
- `EVENT: Contract {name} initialized`

## Backend Integration Assumptions
Backend services assume target deployment artifacts exist persistently through the network-specific JSON files instantiated dynamically. They routinely poll or cache these addresses directly using the provided TS/JS utilities located in `backend/src/utils/` and `backend/src/services/`.
