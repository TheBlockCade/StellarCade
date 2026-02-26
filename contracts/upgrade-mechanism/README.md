# Upgrade Mechanism Contract

The Upgrade Mechanism contract manages controlled releases for Stellarcade contracts.

It provides:
- Explicit upgrade state transitions (`stage -> execute -> rollback`).
- A version registry with migration metadata.
- Admin-only kill switch and pause controls.
- Optional test-gate checks via cross-contract call before activation.

## Public Interface

### `init(admin, initial_wasm_hash, initial_schema_version)`
Initializes state exactly once.

### `pause(admin)` / `unpause(admin)`
Admin emergency controls. Upgrade and rollback mutations are blocked when paused.

### `trigger_kill_switch(admin, reason_hash)`
Irreversible safety stop for faulty deployments. Contract becomes paused and frozen.

### `configure_test_gate(admin, gate_contract, suite, enabled)`
Configures optional release gate contract used during `execute_upgrade`.

### `stage_upgrade(admin, version, wasm_hash, target_schema_version, migration_hash, changelog_hash)`
Stages an upgrade candidate. Enforces deterministic migration policy:
- `target_schema_version` must be either current schema or current schema + 1.
- `version` must be strictly greater than current version.
- Duplicate version staging is rejected.

### `execute_upgrade()`
Admin-authenticated execution of staged release.
- Validates gate contract if enabled.
- Persists release in registry.
- Captures rollback point.
- Emits `UpgradeExecuted`.

### `rollback(admin, reason_hash)`
Rolls back to previous known-good release captured at execute time.

### `state()`
Returns current state snapshot.

### `get_release(version)`
Returns release metadata if the version exists.

## Events

- `Initialized`
- `UpgradeStaged`
- `UpgradeExecuted`
- `RollbackExecuted`
- `PauseChanged`
- `KillSwitchTriggered`
- `TestGateConfigured`

## Storage

Instance:
- `Admin`
- `Paused`
- `Killed`
- `CurrentVersion`
- `CurrentSchemaVersion`
- `CurrentWasmHash`
- `PendingUpgrade`
- `RollbackPoint`
- `TestGate`

Persistent:
- `Release(version) -> ReleaseRecord`

## Invariants

- Contract must be initialized before any privileged flow.
- Only admin can mutate privileged state.
- At most one staged upgrade exists at a time.
- Schema migration transitions are deterministic (same schema or +1 only).
- Rollback point always reflects last pre-upgrade active release.
- If kill switch is set, no mutable state transition can proceed.

## Integration Assumptions

- Gate contract exposes `is_release_ready(suite: Symbol) -> bool`.
- `migration_hash` and `changelog_hash` are content-addressed references to off-chain migration/change artifacts.

## Build and Test

```bash
cd contracts/upgrade-mechanism
cargo test
```
