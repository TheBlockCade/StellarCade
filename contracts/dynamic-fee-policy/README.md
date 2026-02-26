# Dynamic Fee Policy Contract

The Dynamic Fee Policy contract manages customizable fee rules for games, supporting tiered fees and contextual multipliers.

## Methods

### `init(admin: Address)`
Initializes the contract with an admin.

### `set_fee_rule(game_id: Symbol, rule_config: FeeRuleConfig)`
Defines or updates a fee rule for a specific game.
- **Authorization**: Admin.
- **Validation**: BPS values must be within $[0, 10000]$.

### `compute_fee(game_id: Symbol, amount: i128, context: FeeContext) -> i128`
Calculates the applicable fee based on the stored rule and provided context.
- **Logic**: 
    1. Selects the highest tier threshold met by the `amount`.
    2. Falls back to `base_fee_bps` if no tier matches.
    3. Multiplies the resulting BPS by `context.multiplier_bps`.
    4. Applies the final BPS to the `amount`.

### `enable_rule(game_id: Symbol)` / `disable_rule(game_id: Symbol)`
Toggles the enabled status of a rule. Disabled rules cannot be used for fee computation.
- **Authorization**: Admin.

### `fee_rule_state(game_id: Symbol) -> Option<FeeRuleConfig>`
Returns the configuration for a game's fee rule.

## Data Structures

- `FeeRuleConfig`: Includes `base_fee_bps`, optional `tiers`, and `enabled` flag.
- `FeeTier`: Pair of `threshold` (min amount) and `fee_bps`.
- `FeeContext`: Includes `multiplier_bps` for dynamic adjustments (e.g., promotions).

## Events

- `ContractInitialized`: Emitted on initialization.
- `FeeRuleSet`: Emitted when a rule is created/updated.
- `FeeRuleStatusChanged`: Emitted when a rule is enabled/disabled.
- `FeeComputed`: Emitted when a fee is calculated.

## Storage Model

- **Instance Storage**: `Admin`.
- **Persistent Storage**: `FeeRule(game_id)` -> `FeeRuleConfig`.

## Invariants

- BPS values are always $\le 10000$.
- Rule must be enbaled to compute fees.
