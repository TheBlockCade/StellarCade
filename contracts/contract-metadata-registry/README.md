# Contract Metadata Registry

The Contract Metadata Registry provides a centralized location for storing and querying versioned metadata (schema hashes, documentation URIs) for contracts in the Stellarcade ecosystem.

## Features

- **Version Tracking**: Maintains a history of all metadata updates for each contract.
- **Integrity Verification**: Stores SHA-256 hashes of schemas (e.g., JSON ABI or interface definitions).
- **Documentation Links**: Links to off-chain documentation via URIs (IPFS, HTTPS, etc.).
- **Admin Controlled**: Only authorized administrators can register or update metadata.

## Methods

### `init(admin: Address)`
Initializes the registry with an admin account.

### `register_metadata(contract_id: Address, version: u32, schema_hash: BytesN<32>, docs_uri: String)`
Registers the initial metadata for a contract.
- **Constraints**: Contract must not be already registered. Version must be > 0.
- **Authorization**: Admin.

### `update_metadata(contract_id: Address, version: u32, schema_hash: BytesN<32>, docs_uri: String)`
Updates the metadata for an existing contract and increments the version.
- **Constraints**: Contract must exist. New version must be strictly greater than current version.
- **Authorization**: Admin.

### `metadata_of(contract_id: Address) -> Option<MetadataRecord>`
Returns the current (latest) metadata for a given contract.

### `history(contract_id: Address) -> Vec<MetadataRecord>`
Returns the complete historical list of metadata updates for a contract, ordered by version.

## Data Structures

- `MetadataRecord`: Contains `version`, `schema_hash`, `docs_uri`, and `updated_at` (ledger timestamp).

## Events

- `ContractInitialized`: Emitted on registry creation.
- `MetadataRegistered`: Emitted when a contract's initial metadata is added.
- `MetadataUpdated`: Emitted when a contract's metadata is incremented.

## Storage Model

- **Instance Storage**: `Admin`.
- **Persistent Storage**: 
    - `Metadata(contract_id)`: Current `MetadataRecord`.
    - `History(contract_id, version)`: Historical `MetadataRecord`.
