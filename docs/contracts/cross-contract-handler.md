# cross-contract-handler

## Public Methods

### `init`
Initialize with admin and optional registry contract. Call once.

```rust
pub fn init(env: Env, admin: Address, registry_contract: Address) -> Result<(), Error>
```

### `register_route`
Register a route: source_contract may dispatch to target_contract via selector. Admin only.

```rust
pub fn register_route(
```

### `dispatch`
Dispatch a request along a registered route. Caller must be admin or source_contract for that route.

```rust
pub fn dispatch(
```

### `acknowledge`
Acknowledge a pending request with a result. Caller must be admin or target_contract for that request's route.

```rust
pub fn acknowledge(
```

### `get_route`
Return the route for a given route_id, or None if not found.

```rust
pub fn get_route(env: Env, route_id: u32) -> Result<Route, Error>
```

