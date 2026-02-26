# contract-address-registry

## Public Methods

### `init`
Initialize the registry with an admin address.  # Arguments * `admin` - Address that will have full control over the registry  # Errors * `AlreadyInitialized` - If registry has already been initialized  # Events Emits `Initialized` event on success

```rust
pub fn init(env: Env, admin: Address) -> Result<(), Error>
```

### `register`
Register a new contract in the registry.  # Arguments * `name` - Unique identifier for the contract (e.g., "prize-pool") * `address` - Contract address (must start with 'C') * `version` - Initial version number (typically 1)  # Errors * `NotInitialized` - If registry hasn't been initialized * `NotAuthorized` - If caller is not the admin * `InvalidName` - If name is empty, too long, or invalid format * `InvalidAddress` - If address format is invalid * `DuplicateRegistration` - If contract name already exists * `InvalidVersion` - If version is 0  # Events Emits `ContractRegistered` event on success

```rust
pub fn register(env: Env, name: String, address: Address, version: u32) -> Result<(), Error>
```

### `update`
Update an existing contract to a new address and version.  # Arguments * `name` - Contract name to update * `address` - New contract address * `version` - New version number (must be greater than current)  # Errors * `NotInitialized` - If registry hasn't been initialized * `NotAuthorized` - If caller is not the admin * `ContractNotFound` - If contract name doesn't exist * `InvalidAddress` - If address format is invalid * `InvalidVersion` - If version is not greater than current version  # Events Emits `ContractUpdated` event on success

```rust
pub fn update(env: Env, name: String, address: Address, version: u32) -> Result<(), Error>
```

### `resolve`
Resolve the current address for a contract name.  # Arguments * `name` - Contract name to resolve  # Returns The current contract address  # Errors * `NotInitialized` - If registry hasn't been initialized * `ContractNotFound` - If contract name doesn't exist  # Note This is a public read operation - no authorization required

```rust
pub fn resolve(env: Env, name: String) -> Result<Address, Error>
```

### `history`
Get the full version history for a contract.  # Arguments * `name` - Contract name to query  # Returns Vector of all historical contract records, ordered by version  # Errors * `NotInitialized` - If registry hasn't been initialized * `ContractNotFound` - If contract name doesn't exist  # Note This is a public read operation - no authorization required

```rust
pub fn history(env: Env, name: String) -> Result<Vec<ContractRecord>, Error>
```

### `get_version`
Get the current version number for a contract.  # Arguments * `name` - Contract name to query  # Returns The current version number  # Errors * `NotInitialized` - If registry hasn't been initialized * `ContractNotFound` - If contract name doesn't exist

```rust
pub fn get_version(env: Env, name: String) -> Result<u32, Error>
```

### `get_admin`
Get the current admin address.  # Returns The admin address  # Errors * `NotInitialized` - If registry hasn't been initialized

```rust
pub fn get_admin(env: Env) -> Result<Address, Error>
```

