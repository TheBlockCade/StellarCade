# Cross-Contract Call Guard

A specialized registry for managing and enforcing permissions for cross-contract interactions.

## Features

- **Fine-Grained Permissions**: Control calls based on source contract, target contract, and function selector.
- **Fail-Fast Enforcement**: Provides an `assert_allowed` method for contracts to verify incoming/outgoing calls synchronously.
- **Admin Managed**: Centralized control over calling policies.

## Methods

### `init(admin: Address)`
Initializes the guard with an administrator.

### `allow_call(source: Address, target: Address, selector: Symbol)`
Grants permission for `source` to call `target` with the specified `selector`.
- **Authorization**: Admin.

### `deny_call(source: Address, target: Address, selector: Symbol)`
Revokes permission for a specific call triple.
- **Authorization**: Admin.

### `assert_allowed(source: Address, target: Address, selector: Symbol)`
Validates that a call is permitted.
- **Returns**: `Ok(())` if allowed, `Err(CallDenied)` otherwise.
- **Usage**: Intended to be called via cross-contract call by the `target` or `source` contract before performing sensitive logic.

### `policy_state(source: Address, target: Address, selector: Symbol) -> bool`
View the permission state of a specific call triple.

## Data Structures

- `PolicyKey`: A triple of `(source: Address, target: Address, selector: Symbol)`.

## Events

- `ContractInitialized`: Emitted on registry creation.
- `CallAllowed`: Emitted when a new permission is granted.
- `CallDenied`: Emitted when a permission is revoked.

## Storage Model

- **Instance Storage**: `Admin`.
- **Persistent Storage**: 
    - `Policy(PolicyKey)`: Boolean flag indicating if the call is allowed.
