# Contract Monitoring

Contract-side monitoring state for event ingestion, anomaly alerts, and dashboard metrics.

## Public Interface

- `init(admin)` - Initialize monitoring config.
- `ingest_event(admin, event_id, kind)` - Ingests a unique event and updates metrics.
- `set_paused(admin, paused)` - Updates paused state.
- `get_metrics()` - Returns aggregate counters.
- `get_health()` - Returns alert flags for:
  - failed settlements (`>= 3`)
  - high error rate (`>= 20%` once at least 10 events exist)
  - paused state

## Event Kinds

- `SettlementSuccess`
- `SettlementFailed`
- `Error`
- `Paused`
- `Resumed`

## Storage

- `Admin` (instance)
- `Paused` (instance)
- `Metrics` (instance)
- `SeenEvent(event_id)` (persistent duplicate guard)

## Security and Invariants

- Only `admin` can ingest events and change pause state.
- Duplicate event IDs are rejected.
- Health rules are deterministic and computed from stored counters.

## Build and Test

```bash
cd contracts/contract-monitoring
cargo test
```
