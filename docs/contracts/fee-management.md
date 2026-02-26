# fee-management

## Public Methods

### `init`
Initialize the fee management contract  # Arguments * `env` - The contract environment * `admin` - The admin address with full control * `treasury_contract` - The treasury contract address

```rust
pub fn init(env: Env, admin: Address, treasury_contract: Address) -> Result<(), Error>
```

### `set_fee_config`
Set fee configuration for a game

```rust
pub fn set_fee_config(
```

### `charge_fee`
Charge fee for a game transaction

```rust
pub fn charge_fee(
```

### `accrued_fees`
Get accrued fees for a game

```rust
pub fn accrued_fees(env: Env, game_id: Symbol) -> Result<i128, Error>
```

### `withdraw_fees`
Withdraw accrued fees for a game

```rust
pub fn withdraw_fees(
```

### `pause`
Pause the contract (admin only)

```rust
pub fn pause(env: Env, admin: Address) -> Result<(), Error>
```

### `unpause`
Unpause the contract (admin only)

```rust
pub fn unpause(env: Env, admin: Address) -> Result<(), Error>
```

