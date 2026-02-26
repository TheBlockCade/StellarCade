# Settlement Queue Contract

The Settlement Queue contract manages a FIFO queue of settlement requests for rewards and treasury allocations.

## Methods

### `init(admin: Address, reward_contract: Address, treasury_contract: Address)`
Initializes the contract with the admin and dependent contract addresses. Resets the queue head and tail pointers.

### `enqueue_settlement(settlement_id: Symbol, account: Address, amount: i128, reason: Symbol)`
Enqueues a new settlement request.
- **Authorization**: Admin or RewardContract.
- **Validation**: `settlement_id` must be unique.

### `process_next(batch_size: u32) -> u32`
Processes up to `batch_size` pending settlements from the queue.
- **Authorization**: Admin.
- **Logic**: Poppa items from FIFO queue, updates status to `Processed`.

### `mark_failed(settlement_id: Symbol, error_code: u32)`
Marks a pending settlement as failed with an error code.
- **Authorization**: Admin.

### `settlement_state(settlement_id: Symbol) -> Option<SettlementData>`
Returns the current state of a settlement.

## Storage Model

- **Instance Storage**:
    - `Admin`: `Address`
    - `RewardContract`: `Address`
    - `TreasuryContract`: `Address`
    - `QueueHead`: `u64`
    - `QueueTail`: `u64`
- **Persistent Storage**:
    - `Settlement(settlement_id)`: `SettlementData`
    - `QueueItem(index)`: `Symbol` (points to `settlement_id`)

## Events

- `ContractInitialized`: Emitted on successful initialization.
- `SettlementEnqueued`: Emitted when a new settlement is added to the queue.
- `SettlementProcessed`: Emitted when a settlement is successfully processed.
- `SettlementFailed`: Emitted when a settlement is marked as failed.

## Invariants

- `QueueHead <= QueueTail`
- Every `QueueItem` between `QueueHead` and `QueueTail` points to a valid `Settlement`.
- Total settlements processed/failed + pending = Total enqueued.
