# Epoch Scheduler

The Epoch Scheduler contract allows tasks to be scheduled for specific ledger-sequence-derived "epochs" and enforces that they are only executed once that epoch is reached.

## Epoch Calculation

Epochs are calculated using the ledger sequence:
`epoch = ledger_sequence / epoch_duration`

The `epoch_duration` is set during contract initialization.

## Methods

### `init(admin: Address, epoch_duration: u32)`
Initializes the contract with an admin and the duration (in ledgers) of each epoch.

### `current_epoch() -> u64`
Returns the current epoch index based on the current ledger sequence.

### `schedule_task(task_id: Symbol, epoch: u64, payload_hash: BytesN<32>)`
Schedules a task for execution in a future or the current epoch.
- **Constraints**: `epoch` must be greater than or equal to the current epoch.

### `mark_executed(task_id: Symbol)`
Marks a task as executed.
- **Authorization**: Admin.
- **Constraints**: 
    - The current epoch must be greater than or equal to the scheduled `epoch`.
    - Task must not have been executed already.

### `task_state(task_id: Symbol) -> Option<TaskData>`
Returns the configuration and status of a scheduled task.

## Data Structures

- `TaskData`: Contains the scheduled `epoch`, `payload_hash`, and `executed` status.

## Events

- `ContractInitialized`: Emitted on initialization.
- `TaskScheduled`: Emitted when a new task is added.
- `TaskExecuted`: Emitted when a task is marked as completed.

## Storage Model

- **Instance Storage**: `Admin`, `EpochDuration`.
- **Persistent Storage**: `Task(task_id)` -> `TaskData`.
