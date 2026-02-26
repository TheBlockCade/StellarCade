# Gas Optimization Analysis

Lightweight analysis contract to track compute/storage cost per method and generate optimization guidance.

## Public Interface

- `init(admin)` - Initializes the analyzer.
- `record_sample(admin, method, cpu, read_bytes, write_bytes)` - Stores one measurement sample.
- `get_method_profile(method)` - Returns aggregate profile for a method.
- `get_hotspots(limit)` - Returns methods with computed hotspot scores.
- `get_recommendations(limit)` - Returns optimization recommendations with estimated savings.

## Recommendation Rules

- `split_method` when average CPU usage is high (`>= 50_000`).
- `cache_writes` when writes dominate reads (`avg_write > avg_read * 2`).

## Security and Validation

- Only admin can record samples.
- Samples with `cpu == 0` are rejected.
- All counters use saturating arithmetic.

## CI Artifact Integration

The output from `get_hotspots` and `get_recommendations` is deterministic and can be serialized by CI scripts as JSON artifacts for build reports.

## Build and Test

```bash
cd contracts/gas-optimization-analysis
cargo test
```
