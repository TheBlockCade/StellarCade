# Contract Address Registry

A centralized registry for tracking deployed Stellarcade contract addresses with comprehensive version management and audit trail capabilities.

## Overview

The Contract Address Registry provides a single source of truth for all deployed contract addresses within the Stellarcade ecosystem. It enables dynamic contract resolution, supports seamless upgrades through versioning, and maintains an immutable historical record of all contract deployments.

## Features

- **Centralized Registry**: Single location for all contract addresses
- **Version Management**: Track contract upgrades with incremental versioning
- **Immutable History**: Complete audit trail of all contract versions
- **Dynamic Resolution**: Resolve contract addresses at runtime
- **Admin-Controlled**: Secure registration and updates
- **Public Queries**: Anyone can resolve addresses and query history

## Architecture

### Storage Schema

```rust
// Admin and initialization
DataKey::Admin -> Address
DataKey::Initialized -> bool

// Current contract records
DataKey::Contract(name) -> ContractRecord

// Historical records (immutable)
DataKey::ContractHistory(name, version) -> ContractRecord

// Version tracking
DataKey::LatestVersion(name) -> u32
```

### Data Structures

```rust
pub struct ContractRecord {
    pub address: Address,       // Contract address (C... format)
    pub version: u32,           // Version number (starts at 1)
    pub registered_at: u64,     // Ledger sequence timestamp
    pub registered_by: Address, // Admin who performed the action
}
```

## Public Interface

### Initialization

#### `init(admin: Address)`

Initialize the registry with an admin address. Must be called before any other operations.

**Authorization**: Requires `admin` signature  
**One-time**: Can only be called once  
**Events**: Emits `Initialized`

```rust
registry.init(&admin_address);
```

### Registration

#### `register(name: String, address: Address, version: u32)`

Register a new contract in the registry.

**Authorization**: Admin only  
**Constraints**:
- Name must be 1-64 characters
- Version must be > 0 (typically start at 1)
- Name must not already exist

**Events**: Emits `ContractRegistered`

```rust
registry.register(
    &env,
    String::from_str(&env, "prize-pool"),
    &contract_address,
    1
);
```

### Updates

#### `update(name: String, address: Address, version: u32)`

Update an existing contract to a new address and version.

**Authorization**: Admin only  
**Constraints**:
- Contract must already exist
- New version must be greater than current version
- Version increments should be sequential (enforced by validation)

**Events**: Emits `ContractUpdated`

```rust
registry.update(
    &env,
    String::from_str(&env, "prize-pool"),
    &new_contract_address,
    2
);
```

### Queries

#### `resolve(name: String) -> Address`

Get the current contract address for a given name.

**Authorization**: Public (no auth required)  
**Returns**: Current contract address  
**Error**: `ContractNotFound` if name doesn't exist

```rust
let address = registry.resolve(
    &env,
    String::from_str(&env, "prize-pool")
)?;
```

#### `history(name: String) -> Vec<ContractRecord>`

Get the complete version history for a contract.

**Authorization**: Public (no auth required)  
**Returns**: Vector of all historical records, ordered by version  
**Error**: `ContractNotFound` if name doesn't exist

```rust
let history = registry.history(
    &env,
    String::from_str(&env, "prize-pool")
)?;

for record in history.iter() {
    // Process each version
}
```

#### `get_version(name: String) -> u32`

Get the current version number for a contract.

**Authorization**: Public (no auth required)  
**Returns**: Current version number  
**Error**: `ContractNotFound` if name doesn't exist

```rust
let version = registry.get_version(
    &env,
    String::from_str(&env, "prize-pool")
)?;
```

#### `get_admin() -> Address`

Get the current admin address.

**Authorization**: Public (no auth required)  
**Returns**: Admin address

```rust
let admin = registry.get_admin(&env)?;
```

## Events

### Initialized
```rust
pub struct Initialized {
    pub admin: Address,
}
```
Emitted when the registry is first initialized.

### ContractRegistered
```rust
pub struct ContractRegistered {
    pub name: String,
    pub address: Address,
    pub version: u32,
    pub admin: Address,
}
```
Emitted when a new contract is registered.

### ContractUpdated
```rust
pub struct ContractUpdated {
    pub name: String,
    pub old_address: Address,
    pub new_address: Address,
    pub old_version: u32,
    pub new_version: u32,
    pub admin: Address,
}
```
Emitted when a contract is updated to a new version.

## Error Codes

| Code | Error | Description |
|------|-------|-------------|
| 1 | `AlreadyInitialized` | Registry has already been initialized |
| 2 | `NotInitialized` | Registry has not been initialized |
| 3 | `NotAuthorized` | Caller is not authorized (not admin) |
| 4 | `ContractNotFound` | Contract name not found in registry |
| 5 | `InvalidAddress` | Contract address format is invalid |
| 6 | `DuplicateRegistration` | Contract name already exists |
| 7 | `InvalidVersion` | Version number is invalid |
| 8 | `InvalidName` | Contract name format is invalid |

## Security Model

### Authorization

- **Admin-Only Operations**: `init`, `register`, `update`
- **Public Operations**: `resolve`, `history`, `get_version`, `get_admin`

### Invariants

1. **Initialization**: Registry can only be initialized once
2. **Name Uniqueness**: Each contract name can only be registered once
3. **Version Monotonicity**: Versions must always increase
4. **History Immutability**: Historical records are never modified or deleted
5. **Admin Authority**: Only admin can register or update contracts

### Validation

- **Name Length**: 1-64 characters
- **Version Range**: Must be > 0
- **Version Increment**: New version must be > old version
- **Address Format**: Standard Stellar contract address validation

## Integration Guide

### For Contract Developers

When your contract needs to call another contract:

```rust
// Instead of hardcoding addresses:
let prize_pool = Address::from_string("CABC...XYZ");

// Use the registry:
let registry = ContractAddressRegistryClient::new(&env, &registry_address);
let prize_pool = registry.resolve(&String::from_str(&env, "prize-pool"));
```

### For Deployment Scripts

```bash
# 1. Deploy the registry
stellar contract deploy --wasm registry.wasm

# 2. Initialize with admin
stellar contract invoke \
  --id REGISTRY_ID \
  --fn init \
  --arg admin:ADMIN_ADDRESS

# 3. Register contracts
stellar contract invoke \
  --id REGISTRY_ID \
  --fn register \
  --arg name:"prize-pool" \
  --arg address:PRIZE_POOL_ADDRESS \
  --arg version:1
```

### For Frontend/Backend

```typescript
// Resolve contract addresses dynamically
const registryContract = new Contract(REGISTRY_ADDRESS);
const prizePoolAddress = await registryContract.call('resolve', 'prize-pool');

// Check version before calling
const currentVersion = await registryContract.call('get_version', 'prize-pool');
if (currentVersion < REQUIRED_VERSION) {
  throw new Error('Contract version mismatch');
}
```

## Usage Examples

### Initial Deployment

```rust
// 1. Initialize registry
registry.init(&admin);

// 2. Register core contracts
registry.register(&env, "random-generator", &rng_addr, 1);
registry.register(&env, "prize-pool", &pool_addr, 1);
registry.register(&env, "coin-flip", &flip_addr, 1);
```

### Contract Upgrade

```rust
// Deploy new version of contract
let new_coin_flip_addr = deploy_contract(&env, new_wasm);

// Update registry
registry.update(&env, "coin-flip", &new_coin_flip_addr, 2);

// Old version still in history for audit
let history = registry.history(&env, "coin-flip");
// history[0] = version 1
// history[1] = version 2
```

### Cross-Contract Resolution

```rust
// In your game contract
let registry = ContractAddressRegistryClient::new(&env, &registry_addr);

// Resolve dependencies dynamically
let rng = RandomGeneratorClient::new(
    &env,
    &registry.resolve(&String::from_str(&env, "random-generator"))
);

let prize_pool = PrizePoolClient::new(
    &env,
    &registry.resolve(&String::from_str(&env, "prize-pool"))
);
```

## Testing

### Run Unit Tests

```bash
cd contracts/contract-address-registry
cargo test
```

### Test Coverage

The test suite covers:
- ✅ Initialization (success, duplicate)
- ✅ Registration (success, unauthorized, duplicate, invalid inputs)
- ✅ Updates (success, unauthorized, not found, invalid version)
- ✅ Resolution (success, not found)
- ✅ History queries (single version, multiple versions, not found)
- ✅ Version queries
- ✅ Admin queries
- ✅ Full lifecycle integration
- ✅ Multiple contract management

## Build and Deploy

### Build

```bash
cd contracts/contract-address-registry
cargo build --target wasm32-unknown-unknown --release
```

### Optimize WASM

```bash
stellar contract optimize \
  --wasm target/wasm32-unknown-unknown/release/stellarcade_contract_address_registry.wasm
```

### Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellarcade_contract_address_registry.wasm \
  --network testnet \
  --source ADMIN_SECRET_KEY
```

## Dependencies

### For Other Contracts

To use the registry in your contracts:

```toml
[dependencies]
stellarcade-contract-address-registry = { path = "../contract-address-registry" }
```

### In Contract Code

```rust
use stellarcade_contract_address_registry::ContractAddressRegistryClient;
```

## Future Enhancements

Potential improvements for future versions:

1. **Multi-Admin Support**: Add role-based access control
2. **Contract Tags**: Add metadata tags for categorization
3. **Deprecation Markers**: Flag old versions as deprecated
4. **Name Aliasing**: Support multiple names for same contract
5. **Batch Operations**: Register/update multiple contracts at once
6. **Event Subscriptions**: Enhanced event filtering and indexing

## Security Considerations

### Audit Recommendations

- ✅ Admin key management and rotation procedures
- ✅ Version monotonicity enforcement
- ✅ Name collision prevention
- ✅ Storage cost management for large registries
- ✅ Rate limiting on registration operations (if applicable)

### Upgrade Path

Since this is a critical infrastructure contract:
1. Deploy new version of registry
2. Migrate all contract records
3. Update all dependent contracts to use new registry address
4. Maintain old registry in read-only mode for historical queries

## Support and Contribution

For issues, feature requests, or contributions, please refer to the main [CONTRIBUTING.md](../../CONTRIBUTING.md) guide.

## License

MIT License - see [LICENSE](../../LICENSE) for details.
