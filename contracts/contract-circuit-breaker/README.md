# Contract Circuit Breaker

The Contract Circuit Breaker provides a safety mechanism to halt operations of specific contracts when failure thresholds are exceeded.

## Methods

### `init(admin: Address, threshold: u32)`
Initializes the contract with an admin and a global failure threshold.

### `record_failure(contract_id: Address, code: u32)`
Records a failure for a specific contract.
- **Authorization**: Admin (in base version).
- **Logic**: Increments failure count. If `failure_count >= threshold`, the breaker status moves to `Open`.

### `trip(contract_id: Address)`
Forces a contract's circuit breaker to the `Open` state.
- **Authorization**: Admin.

### `reset(contract_id: Address)`
Resets a contract's circuit breaker to the `Closed` state and clears the failure count.
- **Authorization**: Admin.

### `breaker_state(contract_id: Address) -> Option<BreakerData>`
Returns the current state of a contract's circuit breaker.

## Data Structures

- `BreakerStatus`: `Closed` (normal), `Open` (tripped).
- `BreakerData`: Contains `failure_count`, `status`, and `last_failure_ledger`.

## Events

- `ContractInitialized`: Emitted on initialization.
- `FailureRecorded`: Emitted for every recorded failure.
- `BreakerTripped`: Emitted when a breaker moves to `Open`.
- `BreakerReset`: Emitted when a breaker is manually reset.

## Storage Model

- **Instance Storage**: `Admin`, `Threshold`.
- **Persistent Storage**: `Breaker(contract_id)` -> `BreakerData`.
