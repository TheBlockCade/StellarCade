# Comprehensive Test Suite Contract

The Comprehensive Test Suite contract tracks deterministic test-gate signals that can be consumed by release pipelines and contracts.

It standardizes:
- Suite registration and policy metadata.
- Per-build run recording with duplicate protection.
- Coverage thresholds and required test-class checks.
- A simple `is_release_ready(suite)` gate for dependent modules.

## Public Interface

### `init(admin, reporter, coverage_target_bps)`
Initializes contract once.

### `set_reporter(admin, reporter)`
Admin-only reporter role update.

### `pause(admin)` / `unpause(admin)`
Emergency controls that block run mutations.

### `trigger_kill_switch(admin, reason_hash)`
Irreversible safety stop for test-gate writes.

### `register_suite(admin, suite, min_cases, requires_integration, requires_property)`
Registers suite policy and expected signals.

### `update_suite(admin, suite, min_cases, requires_integration, requires_property, active)`
Mutates suite policy and activation state.

### `record_run(reporter, suite, build_id, passed_cases, failed_cases, coverage_bps, includes_integration, includes_property)`
Records run output once per `(suite, build_id)`.

### `is_release_ready(suite)`
Returns true if suite has a last successful run satisfying all constraints.

### `state()`
Returns administrative snapshot.

## Events

- `Initialized`
- `ReporterChanged`
- `SuiteRegistered`
- `SuiteUpdated`
- `RunRecorded`
- `PauseChanged`
- `KillSwitchTriggered`

## Storage

Instance:
- `Admin`, `Reporter`
- `Paused`, `Killed`
- `CoverageTargetBps`
- `TotalSuites`, `TotalRuns`
- `Suite(Symbol) -> SuiteConfig`
- `LastSuccessfulRun(Symbol) -> BytesN<32>`

Persistent:
- `Run(RunKey { suite, build_id }) -> RunRecord`

## Invariants

- Privileged methods require admin/reporter roles.
- `coverage_bps` is validated in `[0, 10_000]`.
- Runs are idempotent per `(suite, build_id)`.
- `is_release_ready` only turns true after a passing run meeting suite and global gates.

## CI Coverage Targets and Gates

Suggested CI gate for this module and dependent contracts:

```bash
cd contracts/comprehensive-test-suite
cargo test
```

Recommended policy:
- Enforce `coverage_target_bps >= 8000` (80%) in staging/mainnet pipelines.
- Require integration/property flags on suites that declare them.
- Block release if `is_release_ready("core_suite") == false`.

## Integration Assumptions

- Dependent contracts invoke `is_release_ready(suite: Symbol) -> bool`.
- `build_id` should be a content hash of CI run inputs to prevent replay.

## Build and Test

```bash
cd contracts/comprehensive-test-suite
cargo test
```
