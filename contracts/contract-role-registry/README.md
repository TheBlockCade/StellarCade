# Contract Role Registry

A Soroban smart contract to manage roles assigned to contract addresses within the StellarCade ecosystem.

## Methods

- `init(admin: Address)`: Initializes the contract with an admin.
- `assign_role(target: Address, role: Symbol)`: Assigns a role to a target address. Requires admin authentication.
- `revoke_role(target: Address, role: Symbol)`: Revokes a role from a target address. Requires admin authentication.
- `has_role(target: Address, role: Symbol) -> bool`: Checks if the target address has the specified role.
- `get_admin() -> Address`: Returns the current admin address.

## Events

- `RoleAssigned { target: Address, role: Symbol }`: Published when a role is assigned.
- `RoleRevoked { target: Address, role: Symbol }`: Published when a role is revoked.

## Storage

- `Admin`: Instance storage for the contract admin address.
- `Role(Address, Symbol)`: Persistent storage mapping an address and role to possession.

## Build

```bash
soroban contract build
```

## Test

```bash
cargo test
```
