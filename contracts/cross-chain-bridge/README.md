# Cross-Chain Bridge Contract

This contract facilitates secure asset transfers between the Stellar network and other supported blockchains.

## Features

- **Locking/Releasing**: For native Stellar assets.
- **Minting/Burning**: For wrapped assets representing tokens from other chains.
- **Validator Quorum**: Secure cross-chain verification via authorized validators.
- **Replay Protection**: Prevents double-spending using unique transfer identifiers.

## Methods

### Initialization
- `init(admin: Address, validators: Vec<Address>, token_mapping: Map<Symbol, Address>)`: Setup the initial bridge configuration.

### Outbound Transfers
- `lock(asset: Address, amount: i128, recipient_chain: Symbol, recipient: String) -> Result<(), Error>`: Lock native assets to be transferred cross-chain.
- `burn_wrapped(asset: Address, amount: i128, recipient_chain: Symbol, recipient: String) -> Result<(), Error>`: Burn wrapped assets to release them on their native chain.

### Inbound Transfers
- `mint_wrapped(asset: Symbol, amount: i128, recipient: Address, proof: BytesN<32>, signatures: Map<Address, BytesN<64>>) -> Result<(), Error>`: Mint wrapped assets based on validator proof.
- `release(asset: Address, amount: i128, recipient: Address, proof: BytesN<32>, signatures: Map<Address, BytesN<64>>) -> Result<(), Error>`: Release locked native assets based on validator proof.

## Security

- Quorum-based verification for all inbound transfers.
- Nonce/Proof deduplication to prevent replay attacks.
- Admin-controlled validator set and token mappings.
- Emergency pause functionality.

## Events

- `Locked(asset, from, amount, to_chain, to_address)`
- `Burned(asset, from, amount, to_chain, to_address)`
- `Minted(asset, to, amount, proof)`
- `Released(asset, to, amount, proof)`
