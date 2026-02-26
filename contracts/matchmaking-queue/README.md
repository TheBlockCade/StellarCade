# Matchmaking Queue Contract

Soroban smart contract for managing player matchmaking queues on-chain.

## Storage

| Key | Type | Description |
|-----|------|-------------|
| `Admin` | `Address` | Contract administrator |
| `NextMatchId` | `u64` | Auto-incremented match ID |
| `QueueState(queue_id)` | `MatchQueueState` | Per-queue player list (persistent) |
| `Match(match_id)` | `MatchRecord` | Completed match record (persistent) |

## Methods

| Method | Auth | Description |
|--------|------|-------------|
| `init(admin)` | — | Initialize contract (once only) |
| `enqueue_player(queue_id, player, criteria_hash)` | player | Join a named queue; rejects duplicates |
| `dequeue_player(caller, queue_id, player)` | player or admin | Remove player from queue |
| `create_match(queue_id, players)` | admin | Form a match and remove players from queue |
| `queue_state(queue_id)` | — | Read current queue state |
| `match_state(match_id)` | — | Read a match record |

## Events

| Topic | Data | Trigger |
|-------|------|---------|
| `enqueued` | `PlayerEnqueued` | Player joins queue |
| `dequeued` | `PlayerDequeued` | Player leaves queue |
| `matched` | `MatchCreated` | Match formed |

## Invariants

- A player may not appear twice in the same queue.
- Only admin or the player themselves may dequeue.
- Match creation removes matched players from the queue atomically.

## Dependencies

- `soroban-sdk = "25.0.2"`
