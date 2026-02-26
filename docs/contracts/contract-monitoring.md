# Contract Monitoring

The Contract Monitoring module tracks contract event ingestion and exposes health flags for backend dashboards.

## Health Flags

- `failed_settlement_alert` when failed settlements reach threshold.
- `high_error_rate` when error ratio crosses threshold.
- `paused` when the system is paused.

## Metrics

- `total_events`
- `settlement_success`
- `settlement_failed`
- `error_events`
- `paused_events`
