# Governance Token Contract

This contract implements the governance token for the StellarCade platform. It provides standard token functionalities such as minting, burning, and transferring, with administrative controls for governance purposes.

## Methods

### `init(admin: Address, token_config: TokenConfig)`
Initializes the contract with an admin address and token configuration.

### `mint(to: Address, amount: i128)`
Mints new tokens to the specified address. Requires admin authorization.

### `burn(from: Address, amount: i128)`
Burns tokens from the specified address. Requires admin authorization.

### `transfer(from: Address, to: Address, amount: i128)`
Transfers tokens from one address to another. Requires authorization from the sender.

### `total_supply() -> i128`
Returns the current total supply of tokens.

### `balance_of(owner: Address) -> i128`
Returns the token balance of the specified owner.

## Storage

- `Admin`: The address with administrative privileges.
- `TotalSupply`: Current total number of tokens in circulation.
- `Balances`: Mapping of addresses to their respective token balances.

## Events

- `mint`: Emitted when new tokens are minted.
- `burn`: Emitted when tokens are burned.
- `transfer`: Emitted when tokens are transferred.
- `init`: Emitted when the contract is initialized.
