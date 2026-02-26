# access-control

## Public Methods

### `init`
Initializes the contract with a super admin. This admin will have the power to grant and revoke any roles.

```rust
pub fn init(env: Env, admin: Address)
```

### `grant_role`
Grants a role to an account. Only accounts with ADMIN role (or the super admin) can call this.

```rust
pub fn grant_role(env: Env, role: Symbol, account: Address)
```

### `revoke_role`
Revokes a role from an account. Only accounts with ADMIN role can call this.

```rust
pub fn revoke_role(env: Env, role: Symbol, account: Address)
```

### `has_role`
Checks if an account has a specific role.

```rust
pub fn has_role(env: Env, role: Symbol, account: Address) -> bool
```

### `get_admin`
Retrieves the current super admin address.

```rust
pub fn get_admin(env: Env) -> Address
```

### `require_admin`
```rust
pub fn require_admin(env: &Env)
```

### `require_role`
```rust
pub fn require_role(env: &Env, role: Symbol, account: Address)
```

### `internal_grant_role`
```rust
pub fn internal_grant_role(env: &Env, role: Symbol, account: Address)
```

### `internal_revoke_role`
```rust
pub fn internal_revoke_role(env: &Env, role: Symbol, account: Address)
```

### `internal_has_role`
```rust
pub fn internal_has_role(env: &Env, role: Symbol, account: Address) -> bool
```

